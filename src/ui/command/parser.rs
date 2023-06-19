use chumsky::prelude::*;

use crate::canvas::curve::control_points::kind::interpolation::InterpolationNodes;
use crate::config::property::{ConvexHull, InterpolationNodesProperty, Property, Samples};
use crate::parser;

#[derive(Debug)]
pub struct CommandParser<'a> {
    input: &'a str,
}

type ParserError<'a> = extra::Err<Simple<'a, u8>>;

impl<'a> CommandParser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn parse(&mut self) -> Result<Command, Error> {
        log::debug!("{}", self.input);
        Self::parser()
            .parse(self.input.as_bytes())
            .into_result()
            .map_err(|error| {
                log::debug!("{error:?}");
                Error::ParserInternal(Vec::new())
            })
    }

    fn parser() -> impl Parser<'a, &'a [u8], Command, ParserError<'a>> {
        let bool = choice((Self::value(b"true", true), Self::value(b"false", false)));

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
        let toggle = choice((Self::get_property(ConvexHull).map(|_| Toggle::ConvexHull),));
        let rotate = parser::unsigned_parser().padded();
        let r#move = parser::f32_parser()
            .padded()
            .then(parser::f32_parser().padded());

        just(b':').ignore_then(choice((
            just(b"get").padded().ignore_then(get).map(Command::Get),
            just(b"set").padded().ignore_then(set).map(Command::Set),
            just(b"toggle")
                .padded()
                .ignore_then(toggle)
                .map(Command::Toggle),
            just(b"rotate")
                .padded()
                .ignore_then(rotate)
                .map(Command::Rotate),
            just(b"move")
                .padded()
                .ignore_then(r#move)
                .map(|(a, b)| Command::Move(a, b)),
        )))
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
    ParserInternal(Vec<EmptyErr>),
}

#[derive(Debug)]
pub enum Command {
    Get(Get),
    Set(Set),
    Toggle(Toggle),
    Rotate(u16),
    Move(f32, f32),
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
}
