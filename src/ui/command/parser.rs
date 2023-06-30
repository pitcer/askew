use std::str;

use chumsky::prelude::*;

use crate::canvas::curve::control_points::kind::interpolation::InterpolationNodes;
use crate::canvas::curve::formula::trochoid::TrochoidProperties;
use crate::config::property::{
    ControlLine, ConvexHull, InterpolationNodesProperty, Property, Samples,
};
use crate::config::CurveType;
use crate::{config, parser};

#[derive(Debug)]
pub struct CommandParser<'a> {
    input: &'a str,
}

type ParserError<'a> = extra::Err<Simple<'a, u8>>;

impl<'a> CommandParser<'a> {
    #[must_use]
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn parse(&mut self) -> Result<Command<'a>, Error> {
        log::debug!("{}", self.input);
        Self::parser()
            .parse(self.input.as_bytes())
            .into_result()
            .map_err(|error| {
                log::debug!("{error:?}");
                Error::ParserInternal(format!("{error:?}"))
            })
    }

    fn parser() -> impl Parser<'a, &'a [u8], Command<'a>, ParserError<'a>> {
        let bool = choice((Self::value(b"true", true), Self::value(b"false", false)));
        let maybe_word = any()
            .repeated()
            .at_least(1)
            .map_slice(|slice| str::from_utf8(slice).expect("slice should be an utf8 string"))
            .or_not();

        let interpolation_nodes = choice((
            Self::value(b"equally_spaced", InterpolationNodes::EquallySpaced),
            Self::value(b"chebyshev", InterpolationNodes::Chebyshev),
        ));

        let get = choice((
            Self::get_property(ConvexHull).map(|_| Get::ConvexHull),
            Self::get_property(InterpolationNodesProperty).map(|_| Get::InterpolationNodes),
            Self::get_property(Samples).map(|_| Get::Samples),
        ));
        let set = choice((
            Self::set_property(ConvexHull, bool).map(Set::ConvexHull),
            Self::set_property(InterpolationNodesProperty, interpolation_nodes)
                .map(Set::InterpolationNodes),
            Self::set_property(Samples, parser::unsigned_parser()).map(Set::Samples),
        ));
        let toggle = choice((
            Self::get_property(ConvexHull).map(|_| Toggle::ConvexHull),
            Self::get_property(ControlLine).map(|_| Toggle::ControlLine),
        ));
        let rotate = parser::unsigned_parser()
            .padded()
            .then(parser::unsigned_parser().padded().or_not());
        let r#move = parser::f32_parser()
            .padded()
            .then(parser::f32_parser().padded());
        let curve_type = choice((
            just(b"polyline").padded().map(|_| CurveType::Polyline),
            just(b"convex_hull").padded().map(|_| CurveType::ConvexHull),
            just(b"interpolation")
                .padded()
                .map(|_| CurveType::Interpolation),
            just(b"bezier").padded().map(|_| CurveType::Bezier),
            just(b"rational_bezier")
                .padded()
                .map(|_| CurveType::RationalBezier),
        ));
        let get_length = parser::unsigned_parser().padded();
        let get_point = parser::unsigned_parser()
            .padded()
            .then(parser::unsigned_parser().padded());
        let move_point = group((
            parser::unsigned_parser().padded(),
            parser::unsigned_parser().padded(),
            parser::f32_parser().padded(),
            parser::f32_parser().padded(),
        ));

        choice((
            just(b"get").padded().ignore_then(get).map(Command::Get),
            just(b"set").padded().ignore_then(set).map(Command::Set),
            just(b"toggle")
                .padded()
                .ignore_then(toggle)
                .map(Command::Toggle),
            just(b"rotate")
                .padded()
                .ignore_then(rotate)
                .map(|(angle, curve)| Command::Rotate(angle, curve)),
            just(b"move")
                .padded()
                .ignore_then(r#move)
                .map(|(horizontal, vertical)| Command::Move(horizontal, vertical)),
            just(b"save")
                .padded()
                .ignore_then(maybe_word)
                .map(Command::Save),
            just(b"open")
                .padded()
                .ignore_then(maybe_word)
                .map(Command::Open),
            just(b"set_curve_type")
                .padded()
                .ignore_then(curve_type)
                .map(Command::SetCurveType),
            just(b"get_curves_length")
                .padded()
                .ignored()
                .map(|_| Command::GetCurvesLength),
            just(b"get_length")
                .padded()
                .ignore_then(get_length)
                .map(Command::GetLength),
            just(b"get_point")
                .padded()
                .ignore_then(get_point)
                .map(|(curve_id, point_id)| Command::GetPoint(curve_id, point_id)),
            just(b"move_point")
                .padded()
                .ignore_then(move_point)
                .map(|(curve_id, point_id, x, y)| Command::MovePoint(curve_id, point_id, x, y)),
            just(b"trochoid_properties")
                .padded()
                .ignore_then(config::trochoid_properties::parser())
                .map(Command::TrochoidProperties),
        ))
    }

    fn get_property<'b>(
        property: impl Property,
    ) -> impl Parser<'b, &'b [u8], &'b [u8], ParserError<'b>> {
        just(property.name().as_bytes()).padded()
    }

    fn set_property<'b, T>(
        property: impl Property,
        value: impl Parser<'b, &'b [u8], T, ParserError<'b>>,
    ) -> impl Parser<'b, &'b [u8], T, ParserError<'b>> {
        Self::get_property(property).ignore_then(value)
    }

    fn value<T>(value_text: &[u8], value: T) -> impl Parser<'_, &[u8], T, ParserError<'_>>
    where
        T: Copy,
    {
        just(value_text).padded().ignored().map(move |_| value)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Internal parser error: {0:?}")]
    ParserInternal(String),
}

#[derive(Debug)]
pub enum Command<'a> {
    Get(Get),
    Set(Set),
    Toggle(Toggle),
    Rotate(u16, Option<usize>),
    Move(f32, f32),
    Save(Option<&'a str>),
    Open(Option<&'a str>),
    SetCurveType(CurveType),
    GetCurvesLength,
    GetLength(usize),
    GetPoint(usize, usize),
    MovePoint(usize, usize, f32, f32),
    TrochoidProperties(TrochoidProperties),
}

#[derive(Debug)]
pub enum Get {
    ConvexHull,
    InterpolationNodes,
    Samples,
}

#[derive(Debug)]
pub enum Set {
    ConvexHull(<ConvexHull as Property>::Type),
    InterpolationNodes(<InterpolationNodesProperty as Property>::Type),
    Samples(<Samples as Property>::Type),
}

#[derive(Debug)]
pub enum Toggle {
    ConvexHull,
    ControlLine,
}
