use std::fs;
use std::path::Path;
use std::path::PathBuf;

use simplelog::debug;
use simplelog::info;

use crate::commands::consts;
use crate::commands::error::CliError;
use crate::commands::helpers::CliContext;
use crate::commands::helpers::build_table;
use crate::commands::helpers::choose_endpoint;
use crate::commands::helpers::construct_url;
use crate::commands::helpers::get_stack_id_from_name;
use crate::commands::helpers::get_swarm_id_from_endpoint_id;
use crate::commands::helpers::parse_api_response;
use crate::commands::helpers::parse_env_file;
use crate::commands::stacks::args::deploy::StackDeployCommand;
use crate::commands::stacks::models::deploy::Stack;
use crate::commands::stacks::models::deploy::StackDeployStandaloneCreatePayload;
use crate::commands::stacks::models::deploy::StackDeploySwarmCreatePayload;
use crate::commands::stacks::models::deploy::StackDeployUpdatePayload;

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
    let stack_id = get_stack_id_from_name(ctx, command.stack_name.as_str(), endpoint_id)?;

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
    if joined.exists() { Some(joined) } else { None }
}

struct StackPaths {
    compose: PathBuf,
    env: Option<PathBuf>,
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use tempfile::NamedTempFile;

    use super::*;

    fn create_temp_dir_with_files(files: &[(&str, &str)]) -> tempfile::TempDir {
        let dir = tempfile::TempDir::new().unwrap();
        for (name, content) in files {
            let path = dir.path().join(name);
            let mut f = fs::File::create(path).unwrap();
            f.write_all(content.as_bytes()).unwrap();
        }
        dir
    }

    #[test]
    fn both_compose_file_and_stack_dir_errors() {
        let result = get_stack_paths(
            Some(PathBuf::from("docker-compose.yml")),
            Some(PathBuf::from("/some/dir")),
            false,
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn compose_file_only_returns_path() {
        let result =
            get_stack_paths(Some(PathBuf::from("compose.yml")), None, false, None).unwrap();
        assert_eq!(result.compose, PathBuf::from("compose.yml"));
        assert!(result.env.is_none());
    }

    #[test]
    fn compose_file_with_env_returns_both() {
        let result = get_stack_paths(
            Some(PathBuf::from("compose.yml")),
            None,
            false,
            Some(PathBuf::from("custom.env")),
        )
        .unwrap();
        assert_eq!(result.compose, PathBuf::from("compose.yml"));
        assert_eq!(result.env, Some(PathBuf::from("custom.env")));
    }

    #[test]
    fn stack_dir_finds_docker_compose_yml() {
        let dir = create_temp_dir_with_files(&[("docker-compose.yml", "version: '3'")]);
        let result = get_stack_paths(None, Some(dir.path().to_path_buf()), false, None).unwrap();
        assert_eq!(result.compose, dir.path().join("docker-compose.yml"));
        assert!(result.env.is_none());
    }

    #[test]
    fn stack_dir_finds_docker_compose_yaml() {
        let dir = create_temp_dir_with_files(&[("docker-compose.yaml", "version: '3'")]);
        let result = get_stack_paths(None, Some(dir.path().to_path_buf()), false, None).unwrap();
        assert_eq!(result.compose, dir.path().join("docker-compose.yaml"));
        assert!(result.env.is_none());
    }

    #[test]
    fn stack_dir_prefers_yml_over_yaml() {
        let dir = create_temp_dir_with_files(&[
            ("docker-compose.yml", "v1"),
            ("docker-compose.yaml", "v2"),
        ]);
        let result = get_stack_paths(None, Some(dir.path().to_path_buf()), false, None).unwrap();
        assert_eq!(result.compose, dir.path().join("docker-compose.yml"));
    }

    #[test]
    fn stack_dir_no_compose_file_errors() {
        let dir = tempfile::TempDir::new().unwrap();
        let result = get_stack_paths(None, Some(dir.path().to_path_buf()), false, None);
        assert!(result.is_err());
    }

    #[test]
    fn stack_dir_not_a_directory_errors() {
        let file = NamedTempFile::new().unwrap();
        let result = get_stack_paths(None, Some(file.path().to_path_buf()), false, None);
        assert!(result.is_err());
    }

    #[test]
    fn stack_dir_reads_dot_env_when_present() {
        let dir = create_temp_dir_with_files(&[
            ("docker-compose.yml", "version: '3'"),
            (".env", "KEY=value"),
        ]);
        let result = get_stack_paths(None, Some(dir.path().to_path_buf()), false, None).unwrap();
        assert_eq!(result.env, Some(dir.path().join(".env")));
    }

    #[test]
    fn stack_dir_no_env_flag_skips_env() {
        let dir = create_temp_dir_with_files(&[
            ("docker-compose.yml", "version: '3'"),
            (".env", "KEY=value"),
        ]);
        let result = get_stack_paths(None, Some(dir.path().to_path_buf()), true, None).unwrap();
        assert!(result.env.is_none());
    }

    #[test]
    fn stack_dir_explicit_env_overrides_dot_env() {
        let dir = create_temp_dir_with_files(&[
            ("docker-compose.yml", "version: '3'"),
            (".env", "KEY=value"),
        ]);
        let result = get_stack_paths(
            None,
            Some(dir.path().to_path_buf()),
            false,
            Some(PathBuf::from("/custom/path/.env")),
        )
        .unwrap();
        assert_eq!(result.env, Some(PathBuf::from("/custom/path/.env")));
    }

    #[test]
    fn stack_dir_no_env_file_and_not_no_env() {
        let dir = create_temp_dir_with_files(&[("docker-compose.yml", "version: '3'")]);
        let result = get_stack_paths(None, Some(dir.path().to_path_buf()), false, None).unwrap();
        assert!(result.env.is_none());
    }

    #[test]
    fn neither_compose_nor_stack_dir_errors() {
        let result = get_stack_paths(None, None, false, None);
        assert!(result.is_err());
    }
}
