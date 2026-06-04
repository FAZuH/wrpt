use clap::{ArgGroup, Args};
use std::path::PathBuf;

use clap::Args;

#[derive(Debug, Args)]
#[command(
    group(ArgGroup::new("endpoint_group").args(["endpoint", "endpoint_name"]).required(true)),
    group(ArgGroup::new("stack_group").args(["compose_file", "stack_dir"]).required(true)),
)]
pub struct StackDeployCommand {
    /// Name of the stack
    pub stack_name: String,

    /// Id of the environment (endpoint) that will be used
    #[arg(short = 'E', long)]
    pub endpoint: Option<u32>,

    /// Name of the environment (endpoint) that will be used
    #[arg(short = 'n', long)]
    pub endpoint_name: Option<String>,

    /// Path to docker compose/stack file
    #[arg(short, long, conflicts_with = "no_env")]
    pub compose_file: Option<PathBuf>,

    /// Path to directory containing docker compose and optional .env file
    #[arg(short, long)]
    pub stack_dir: Option<PathBuf>,

    /// When used with --stack-dir, skip loading the .env file from that directory
    #[arg(long)]
    pub no_env: bool,

    /// Path to a file of environment variables, to be used by the stack
    #[arg(short, long)]
    pub env_file: Option<PathBuf>,

    /// Whether to prune unused containers or not
    #[arg(long)]
    pub prune: bool,

    /// Force a pulling to current image with the original tag though the image is already the latest
    #[arg(long)]
    pub pull_image: bool,
}
