use std::path::PathBuf;
use std::str;

use crate::canvas::shape::interpolation::InterpolationNodes;
use crate::canvas::shape::trochoid::TrochoidCurveProperties;
use crate::config::ShapeType;

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
        let input_split = shlex::split(self.input)
            .ok_or_else(|| Error::ParserInternal("Invalid command input".to_owned()))?;
        <Command as clap::Parser>::try_parse_from(input_split).map_err(|error| {
            let error_rendered = error.render();
            let error_printable = error_rendered.ansi();
            log::info!("Parse error:\n{error_printable}");
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
#[command(multicall(true), arg_required_else_help(true))]
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
    SaveImage {
        #[arg()]
        path: Option<PathBuf>,
    },

    #[command()]
    SetCurveType {
        #[arg()]
        curve_type: ShapeType,
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
    TrochoidProperties(TrochoidCurveProperties),

    /// Creates new task
    #[command()]
    Execute {
        #[arg()]
        path: PathBuf,
        #[arg()]
        argument: Option<String>,
    },

    #[command(subcommand)]
    Task(Task),

    #[command()]
    Quit,
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
        value: bool,
    },

    #[command()]
    InterpolationNodes {
        #[arg()]
        value: InterpolationNodes,
    },

    #[command()]
    Samples {
        #[arg()]
        value: u32,
    },
}

#[derive(Debug, clap::Subcommand)]
pub enum Toggle {
    #[command()]
    ConvexHull,

    #[command()]
    ControlLine,
}

#[derive(Debug, clap::Subcommand)]
pub enum Task {
    #[command()]
    List,

    #[command()]
    Kill {
        #[arg()]
        task_id: usize,
    },
}
