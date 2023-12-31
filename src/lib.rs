use std::convert;

use anyhow::Result;
use clap::Parser;
use log::LevelFilter;
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};
use winit::event_loop::EventLoopBuilder;
use winit::window::WindowBuilder;

use cli::{Command, SubCommands};
use ipc::client::IpcClient;
use ipc::server::IpcServer;
use ui::handler::WindowHandler;

use crate::config::Config;
use crate::ui::frame::Frame;
use crate::ui::painter::Painter;
use crate::ui::runner;
use crate::ui::runner::RunnerExitResult;
use crate::ui::shared::{SharedFrame, SharedState, SharedTasks};
use crate::ui::task::Tasks;
use crate::ui::window::Window;

pub mod canvas;
pub mod cli;
pub mod command;
pub mod config;
pub mod executor;
pub mod id_assigner;
pub mod ipc;
pub mod request;
pub mod ui;
pub mod wasm;

pub fn main() -> Result<()> {
    let command = Command::parse();
    initialize_logger(command.log_level, command.log_allow, command.log_ignore)?;

    let mut config = Config::from_file(command.config)?;
    match command.command {
        SubCommands::Run(arguments) => {
            config.overwrite_with_run(arguments);
            run(config)
        }
        SubCommands::Ipc(ipc) => {
            let Some(socket_path) = ipc.socket_path.or(config.ipc_socket_path) else {
                log::error!("Unknown IPC socket path.");
                return Ok(());
            };
            let client = IpcClient::new(socket_path)?;
            client.send(ipc.message)
        }
    }
}

fn run(config: Config) -> Result<()> {
    let event_loop = EventLoopBuilder::with_user_event().build()?;

    let window = WindowBuilder::new().with_title("askew").build(&event_loop)?;
    let window = Window::from_winit(&window)?;

    let size = window.size_rectangle();
    let frame = Frame::new(size, config.frame, config.canvas)?;
    let frame = SharedFrame::new(frame);

    let sender = event_loop.create_proxy();
    let frame_clone = SharedFrame::clone(&frame);
    let tasks = Tasks::new(sender, frame_clone)?;
    let tasks = SharedTasks::new(tasks);

    let state = SharedState::new(frame, tasks);

    if let Some(path) = config.ipc_socket_path {
        let state = SharedState::clone(&state);
        let sender = event_loop.create_proxy();
        IpcServer::run(path, state, sender)?;
    }

    let painter = Painter::new(config.ui)?;
    let sender = event_loop.create_proxy();
    let mut handler = WindowHandler::new(config.startup_commands, window, painter, sender, state)?;
    let run_future = runner::run(event_loop, &mut handler);

    let exited_runner: RunnerExitResult = executor::block_on_run(run_future)?;

    let exit_code = exited_runner.exit_code();
    log::info!("Loop exited with code {exit_code}.");

    Ok(())
}

fn initialize_logger(
    level_filter: LevelFilter,
    allow_filters: Vec<String>,
    ignore_filters: Vec<String>,
) -> Result<()> {
    let mut builder = ConfigBuilder::new();
    builder
        .set_enable_paris_formatting(true)
        .set_time_format_custom(simplelog::format_description!(
            "[hour]:[minute]:[second].[subsecond digits:3]"
        ))
        .set_time_offset_to_local()
        .unwrap_or_else(convert::identity);
    for filter in allow_filters {
        builder.add_filter_allow(filter);
    }
    for filter in ignore_filters {
        builder.add_filter_ignore(filter);
    }
    let config = builder.build();

    TermLogger::init(level_filter, config, TerminalMode::Stdout, ColorChoice::Auto)?;
    Ok(())
}
