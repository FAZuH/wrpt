use simplelog::debug;
use simplelog::info;

use crate::commands::consts;
use crate::commands::error::CliError;
use crate::commands::helpers::CliContext;
use crate::commands::helpers::construct_url;
use crate::commands::helpers::handle_api_response;
use crate::commands::helpers::resolve_stack;
use crate::commands::stacks::args::remove::StackRemoveCommand;

pub(crate) fn handler(command: StackRemoveCommand, ctx: &CliContext) -> Result<(), CliError> {
    debug!("command = {:?}", command);

    info!("Getting stack info...");
    let stack_id = resolve_stack(ctx, &command.stack_name)?;

    info!(
        "Stack \"{}\" exists (id = {})",
        command.stack_name, stack_id
    );

    info!("Deleting stack \"{}\"", command.stack_name);

    let url = construct_url(
        &ctx.base_url,
        &consts::ENDPOINT_STACKS_REMOVE.replace("{id}", &stack_id.to_string()),
    )?;

    debug!("request = DELETE {:?}", url.as_str());

    let response = ctx
        .client
        .delete(url)
        .query(&[("endpointId", command.endpoint)])
        .send()?;

    handle_api_response(response)?;

    info!("Done");

    Ok(())
}
