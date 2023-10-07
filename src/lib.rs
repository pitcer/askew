#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::clone_on_ref_ptr,
    clippy::dbg_macro,
    clippy::exit,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::get_unwrap,
    clippy::let_underscore_must_use,
    clippy::lossy_float_literal,
    clippy::map_err_ignore,
    clippy::mem_forget,
    clippy::multiple_inherent_impl,
    clippy::panic,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::rc_buffer,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_to_string,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::use_debug,
    clippy::verbose_file_reads,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::match_wildcard_for_single_variants,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::needless_pass_by_value,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::unused_self,
    clippy::unnecessary_wraps
)]

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

use crate::config::Config;
use crate::ui::frame::Frame;
use crate::ui::painter::Painter;
use crate::ui::runner::WindowRunner;
use crate::ui::window::Window;

pub mod canvas;
pub mod cli;
pub mod command;
pub mod config;
pub mod event;
pub mod ipc;
pub mod ui;
pub mod wasm;

pub fn main() -> Result<()> {
    let command = Command::parse();

    let filter = if command.debug { LevelFilter::Debug } else { LevelFilter::Info };
    initialize_logger(filter, command.log_ignore)?;

    let mut config = Config::from_file(command.config)?;
    match command.command {
        SubCommands::Run(run) => {
            config.overwrite_with_run(run);
            crate::run(config)
        }
        SubCommands::Ipc(ipc) => {
            let socket_path = ipc.socket_path.unwrap_or(config.ipc_socket_path);
            let client = IpcClient::new(socket_path)?;
            client.send(ipc.message)
        }
    }
}

fn run(config: Config) -> Result<()> {
    let event_loop = EventLoopBuilder::with_user_event().build()?;
    let window = WindowBuilder::new().with_title("askew").build(&event_loop)?;
    let window = Window::from_winit(window)?;
    let size = window.size_rectangle();
    let frame = Frame::new(size, config.frame, config.canvas)?;
    let painter = Painter::new(config.ui)?;

    // TODO: add option to disable IPC server
    let proxy = event_loop.create_proxy();
    let ipc_server = IpcServer::run(&config.ipc_socket_path, proxy)?;

    let proxy = event_loop.create_proxy();
    let mut handler =
        WindowRunner::new(config.startup_commands, window, frame, painter, ipc_server, proxy)?;

    event_loop.run(move |event, _, control_flow| {
        let result = handler.run(event, control_flow);
        result.expect("Error in event loop");
    })?;

    Ok(())
}

fn initialize_logger(level_filter: LevelFilter, ignore_filters: Vec<String>) -> Result<()> {
    let mut builder = ConfigBuilder::new();
    builder
        .set_enable_paris_formatting(true)
        .set_time_format_custom(simplelog::format_description!(
            "[hour]:[minute]:[second].[subsecond digits:3]"
        ))
        .set_time_offset_to_local()
        .unwrap_or_else(convert::identity);
    for filter in ignore_filters {
        builder.add_filter_ignore(filter);
    }
    let config = builder.build();

    TermLogger::init(level_filter, config, TerminalMode::Stdout, ColorChoice::Auto)?;
    Ok(())
}
