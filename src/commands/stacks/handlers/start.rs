use simplelog::debug;
use simplelog::info;

use crate::commands::consts;
use crate::commands::error::CliError;
use crate::commands::helpers::CliContext;
use crate::commands::helpers::choose_endpoint;
use crate::commands::helpers::construct_url;
use crate::commands::helpers::handle_api_response;
use crate::commands::helpers::resolve_stack;
use crate::commands::stacks::args::start::StackStartCommand;

pub(crate) fn handler(command: StackStartCommand, ctx: &CliContext) -> Result<(), CliError> {
    debug!("command = {:?}", command);

    let endpoint_id = choose_endpoint(ctx, command.endpoint, command.endpoint_name)?;

    info!("Getting stack info...");
    let stack_id = resolve_stack(ctx, &command.stack_name, endpoint_id)?;

    info!(
        "Stack \"{}\" exists (id = {})",
        command.stack_name, stack_id
    );

    info!("Starting stack \"{}\"", command.stack_name);

    let url = construct_url(
        &ctx.base_url,
        &consts::ENDPOINT_STACKS_START.replace("{id}", &stack_id.to_string()),
    )?;

    debug!("request = POST {:?}", url.as_str());

    let response = ctx
        .client
        .post(url)
        .query(&[("endpointId", endpoint_id)])
        .send()?;

    handle_api_response(response)?;

    info!("Done");

    Ok(())
}
