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
    clippy::unused_self,
    clippy::unnecessary_wraps
)]

use std::convert;

use anyhow::Result;
use clap::Parser;
use log::LevelFilter;
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use crate::config::Config;
use crate::ui::frame::Frame;
use crate::ui::window_event_handler::WindowEventHandler;

pub mod canvas;
pub mod config;
pub mod event;
pub mod parser;
pub mod ui;

pub fn main() -> Result<()> {
    initialize_logger()?;

    let config = Config::parse();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("askew")
        .build(&event_loop)?;
    let frame = Frame::new(window, &config)?;
    let mut handler = WindowEventHandler::new(frame, &config);
    event_loop.run(move |event, _, control_flow| {
        let result = handler.run(event, control_flow);
        result.expect("Error in event loop");
    });
}

fn initialize_logger() -> Result<()> {
    let logger_config = ConfigBuilder::new()
        .set_time_format_custom(simplelog::format_description!(
            "[hour]:[minute]:[second].[subsecond digits:3]"
        ))
        .set_time_offset_to_local()
        .unwrap_or_else(convert::identity)
        .build();
    TermLogger::init(
        LevelFilter::Debug,
        logger_config,
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )?;
    Ok(())
}
