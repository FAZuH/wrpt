use clap::{ArgGroup, Args};

#[derive(Debug, Args)]
#[command(group(
    ArgGroup::new("endpoint_group")
        .args(["endpoint", "endpoint_name"])
        .required(true)
))]
pub struct StackStartCommand {
    /// Name of the stack
    pub stack_name: String,

    /// Id of the environment (endpoint) that will be used
    #[arg(short = 'E', long)]
    pub endpoint: Option<u32>,

    /// Name of the environment (endpoint) that will be used
    #[arg(short = 'n', long)]
    pub endpoint_name: Option<String>,
}
