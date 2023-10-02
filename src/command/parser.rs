use std::path::PathBuf;
use std::{iter, str};

use crate::canvas::curve::formula::trochoid::TrochoidProperties;
use crate::config::property::{ConvexHull, InterpolationNodesProperty, Property, Samples};
use crate::config::CurveType;

#[derive(Debug)]
pub struct CommandParser<'a> {
    input: &'a str,
}

impl<'a> CommandParser<'a> {
    #[must_use]
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn parse(&mut self) -> Result<Command, Error> {
        log::debug!("<cyan>Internal command input:</> '{}'", self.input);

        let input = self.input.split_whitespace();
        let input = iter::once("").chain(input);
        <Command as clap::Parser>::try_parse_from(input).map_err(|error| {
            let error_rendered = error.render();
            let error_printable = error_rendered.ansi();
            log::debug!("Parse error:\n{error_printable}");
            Error::ParserInternal(String::from("Parse error"))
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Internal parser error: {0:?}")]
    ParserInternal(String),
}

#[derive(Debug, clap::Parser)]
pub enum Command {
    #[command(subcommand)]
    Get(Get),

    #[command(subcommand)]
    Set(Set),

    #[command(subcommand)]
    Toggle(Toggle),

    #[command()]
    Rotate {
        #[arg()]
        angle: u16,
        #[arg()]
        curve_id: Option<usize>,
    },

    #[command()]
    Move {
        #[arg()]
        horizontal: f32,
        #[arg()]
        vertical: f32,
    },

    #[command()]
    Save {
        #[arg()]
        path: Option<PathBuf>,
    },

    #[command()]
    Open {
        #[arg()]
        path: Option<PathBuf>,
    },

    #[command()]
    SetCurveType {
        #[arg()]
        curve_type: CurveType,
    },

    #[command()]
    GetCurvesLength,

    #[command()]
    GetLength {
        #[arg()]
        curve_id: usize,
    },

    #[command()]
    GetPoint {
        #[arg()]
        curve_id: usize,
        #[arg()]
        point_id: usize,
    },

    #[command()]
    MovePoint {
        #[arg()]
        curve_id: usize,
        #[arg()]
        point_id: usize,
        #[arg()]
        horizontal: f32,
        #[arg()]
        vertical: f32,
    },

    #[command()]
    TrochoidProperties(TrochoidProperties),

    #[command()]
    Execute {
        #[arg()]
        path: PathBuf,
    },
}

#[derive(Debug, clap::Subcommand)]
pub enum Get {
    #[command()]
    ConvexHull,

    #[command()]
    InterpolationNodes,

    #[command()]
    Samples,
}

#[derive(Debug, clap::Subcommand)]
pub enum Set {
    #[command()]
    ConvexHull {
        #[arg()]
        value: <ConvexHull as Property>::Type,
    },

    #[command()]
    InterpolationNodes {
        #[arg()]
        value: <InterpolationNodesProperty as Property>::Type,
    },

    #[command()]
    Samples {
        #[arg()]
        value: <Samples as Property>::Type,
    },
}

#[derive(Debug, clap::Subcommand)]
pub enum Toggle {
    #[command()]
    ConvexHull,

    #[command()]
    ControlLine,
}
