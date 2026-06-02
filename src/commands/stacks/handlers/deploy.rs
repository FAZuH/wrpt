use crate::commands::consts;
use crate::commands::error::CliError;
use crate::commands::helpers::{
    build_table, choose_endpoint, construct_url, get_stack_id_from_name,
    get_swarm_id_from_endpoint_id, parse_api_response, parse_env_file, CliContext,
};
use crate::commands::stacks::args::deploy::StackDeployCommand;
use crate::commands::stacks::models::deploy::{
    Stack, StackDeployStandaloneCreatePayload, StackDeploySwarmCreatePayload,
    StackDeployUpdatePayload,
};
use simplelog::{debug, info};
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn handler(command: StackDeployCommand, ctx: &CliContext) -> Result<(), CliError> {
    debug!("command = {:?}", command);

    let StackPaths { compose, env } = get_stack_paths(
        command.compose_file,
        command.stack_dir,
        command.no_env,
        command.env_file,
    )?;

    let stack_file_content = fs::read_to_string(&compose).map_err(|e| {
        CliError::Io(format!(
            "unable to read compose file \"{}\": {}",
            compose.display(),
            e
        ))
    })?;
    debug!("stack_file_content = {:?}", stack_file_content);

    let env_file = parse_env_file(env).unwrap_or_default();
    debug!("env_file = {:?}", env_file);

    let endpoint_id = choose_endpoint(ctx, command.endpoint, command.endpoint_name)?;

    info!("Getting stack info...");
    let stack_id = get_stack_id_from_name(ctx, command.stack_name.as_str())?;

    let stack: Vec<Stack> = if stack_id.is_none() {
        info!("Stack \"{}\" does not exist", command.stack_name);

        info!("Getting Docker info...");
        let swarm_id = get_swarm_id_from_endpoint_id(ctx, endpoint_id)?;

        match swarm_id {
            Some(swarm_id) => {
                info!("Swarm cluster found : {}", swarm_id);

                info!("Preparing stack JSON...");
                let stack_create_payload = StackDeploySwarmCreatePayload {
                    env: env_file,
                    from_app_template: false,
                    name: command.stack_name.clone(),
                    stack_file_content,
                    swarm_id,
                };
                debug!("stack JSON = {:?}", stack_create_payload);

                info!("Creating Swarm stack \"{}\"", command.stack_name);
                create_stack(
                    ctx,
                    stack_create_payload,
                    endpoint_id,
                    consts::ENDPOINT_STACKS_CREATE_SWARM_STRING,
                )?
            }
            None => {
                info!("Swarm cluster not found");

                info!("Preparing stack JSON...");
                let stack_create_payload = StackDeployStandaloneCreatePayload {
                    env: env_file,
                    from_app_template: false,
                    name: command.stack_name.clone(),
                    stack_file_content,
                };
                debug!("stack JSON = {:?}", stack_create_payload);

                info!("Creating standalone stack \"{}\"", command.stack_name);
                create_stack(
                    ctx,
                    stack_create_payload,
                    endpoint_id,
                    consts::ENDPOINT_STACKS_CREATE_STANDALONE_STRING,
                )?
            }
        }
    } else {
        info!(
            "Stack \"{}\" exists (id = {})",
            command.stack_name,
            stack_id.unwrap_or_default()
        );

        info!("Preparing stack JSON...");
        let stack_update_payload = StackDeployUpdatePayload {
            env: env_file,
            stack_file_content,
            pull_image: command.pull_image,
            prune: command.prune,
        };
        debug!("stack JSON = {:?}", stack_update_payload);

        info!("Updating stack \"{}\"", command.stack_name);
        update_stack(
            ctx,
            stack_update_payload,
            stack_id.unwrap_or_default(),
            endpoint_id,
        )?
    };

    info!("Done");

    build_table(
        &stack,
        Some(&[
            "Id",
            "Name",
            "Type",
            "Status",
            "SwarmId",
            "EndpointId",
            // "ResourceControl",
            "CreationDate",
            "CreatedBy",
            "UpdateDate",
            "UpdatedBy",
        ]),
    )
    .printstd();

    Ok(())
}

pub(crate) fn create_stack<T: serde::Serialize>(
    ctx: &CliContext,
    stack_create_payload: T,
    endpoint_id: u32,
    endpoint: &str,
) -> Result<Vec<Stack>, CliError> {
    let url = construct_url(&ctx.base_url, endpoint)?;

    debug!("request = POST {:?}", url.as_str());

    let response = ctx
        .client
        .post(url)
        .json(&stack_create_payload)
        .query(&[("endpointId", endpoint_id)])
        .send()?;

    parse_api_response(response)
}

pub(crate) fn update_stack(
    ctx: &CliContext,
    stack_update_payload: StackDeployUpdatePayload,
    stack_id: u32,
    endpoint_id: u32,
) -> Result<Vec<Stack>, CliError> {
    let url = construct_url(
        &ctx.base_url,
        &consts::ENDPOINT_STACKS_UPDATE.replace("{id}", &stack_id.to_string()),
    )?;

    debug!("request = PUT {:?}", url.as_str());

    let response = ctx
        .client
        .put(url)
        .json(&stack_update_payload)
        .query(&[("endpointId", endpoint_id)])
        .send()?;

    parse_api_response(response)
}

fn get_stack_paths(
    compose_file: Option<PathBuf>,
    stack_dir: Option<PathBuf>,
    no_env: bool,
    env_file: Option<PathBuf>,
) -> Result<StackPaths, CliError> {
    if compose_file.is_some() && stack_dir.is_some() {
        // ambiguous
        return Err(CliError::Config(
            "cannot specify both --compose-file and --stack-dir".to_string(),
        ));
    }

    if let Some(compose) = compose_file {
        return Ok(StackPaths {
            compose,
            env: env_file,
        });
    }

    if let Some(dir) = stack_dir {
        if !dir.is_dir() {
            return Err(CliError::Config(format!(
                "{} is not a directory",
                dir.display()
            )));
        }

        let compose = join_exists(&dir, "docker-compose.yml")
            .or_else(|| join_exists(&dir, "docker-compose.yaml"))
            .ok_or_else(|| {
                CliError::Config(format!(
                    "neither docker-compose.yml nor docker-compose.yaml exists on {}",
                    dir.display()
                ))
            })?;

        let env = if !no_env {
            env_file.or_else(|| join_exists(&dir, ".env"))
        } else {
            None
        };

        return Ok(StackPaths { compose, env });
    }

    Err(CliError::Config(
        "requires --compose-file or --stack-dir".to_string(),
    ))
}

/// Returns the joined path if it exists.
fn join_exists(path: impl AsRef<Path>, join: impl AsRef<Path>) -> Option<PathBuf> {
    let joined = path.as_ref().join(join);
    if joined.exists() {
        Some(joined)
    } else {
        None
    }
}

struct StackPaths {
    compose: PathBuf,
    env: Option<PathBuf>,
}
