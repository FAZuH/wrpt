use clap::Args;

#[derive(Debug, Args)]
pub struct StackStopCommand {
    /// Name of the stack
    pub stack_name: String,

    /// Id of the environment (endpoint) that will be used
    #[arg(short = 'E', long)]
    pub endpoint: Option<u32>,

    /// Name of the environment (endpoint) that will be used
    #[arg(short = 'n', long)]
    pub endpoint_name: Option<String>,
}
