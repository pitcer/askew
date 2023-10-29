use log::LevelFilter;
use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
#[command(version)]
pub struct Command {
    #[command(subcommand)]
    pub command: SubCommands,

    #[arg(long, default_value = "askew_config.toml")]
    pub config: PathBuf,

    /// Print debug logs
    #[arg(short = 'l', long, default_value_t = LevelFilter::Info)]
    pub log_level: LevelFilter,

    /// Simplelog allow filters
    #[arg(short = 'a', long)]
    pub log_allow: Vec<String>,

    /// Simplelog ignore filters
    #[arg(short = 'i', long)]
    pub log_ignore: Vec<String>,
}

#[derive(Debug, clap::Subcommand)]
pub enum SubCommands {
    /// Run and open editor's window
    Run(RunArguments),

    /// Execute commands on running editor instance
    Ipc(IpcArguments),
}

#[derive(Debug, clap::Args)]
pub struct IpcArguments {
    #[arg()]
    pub message: String,

    #[arg(short, long)]
    pub socket_path: Option<PathBuf>,
}

#[derive(Debug, clap::Args)]
pub struct RunArguments {
    #[arg(long)]
    pub canvas_curve_samples: Option<u32>,

    #[arg(short = 'b', long)]
    pub background_image_path: Option<PathBuf>,

    #[arg(short = 'o', long)]
    pub project_path: Option<PathBuf>,

    /// Command to execute on start, can be specified multiple times
    #[arg(short = 'c', long)]
    pub startup_commands: Vec<String>,

    #[arg(short = 'n', long)]
    pub random_points: Option<u32>,

    #[arg(long)]
    pub font_size: Option<u32>,

    #[arg(long)]
    pub font_path: Option<PathBuf>,

    #[arg(long)]
    pub ipc_socket_path: Option<PathBuf>,
}
