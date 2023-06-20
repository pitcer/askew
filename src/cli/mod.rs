use crate::config::Config;

#[derive(Debug, clap::Parser)]
#[command(version)]
pub struct Command {
    #[command(subcommand)]
    pub command: SubCommands,
}

#[derive(Debug, clap::Subcommand)]
pub enum SubCommands {
    /// Run and open editor's window
    Run(Config),
    /// Execute commands on running editor instance
    Ipc(IpcArguments),
}

#[derive(Debug, clap::Args)]
pub struct IpcArguments {
    pub socket_path: String,
    pub message: String,
}
