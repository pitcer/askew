use crate::canvas::curve::control_points::kind::interpolation::InterpolationNodes;
use atoi::FromRadix10;
use chumsky::prelude::*;
use chumsky::text;

use crate::config::property::{ConvexHull, InterpolationNodesProperty, Property, Samples};

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
        fn get_property<'a>(
            property: impl Property,
        ) -> impl Parser<'a, &'a [u8], &'a [u8], ParserError<'a>> {
            just(property.name().as_bytes()).padded()
        }

        fn set_property<'a, T>(
            property: impl Property,
            value: impl Parser<'a, &'a [u8], T, ParserError<'a>>,
        ) -> impl Parser<'a, &'a [u8], T, ParserError<'a>> {
            get_property(property).ignore_then(value)
        }

        fn value<T>(input: &[u8], value: T) -> impl Parser<'_, &[u8], T, ParserError<'_>>
        where
            T: Copy,
        {
            just(input).padded().ignored().map(move |_| value)
        }

        let bool = choice((value(b"true", true), value(b"false", false)));
        let uint = text::int(10)
            .map(u32::from_radix_10)
            .map(|(number, _)| number);
        let interpolation_nodes = choice((
            value(b"equally_spaced", InterpolationNodes::EquallySpaced),
            value(b"chebyshev", InterpolationNodes::Chebyshev),
        ));

        let get = choice((
            get_property(ConvexHull).map(|_| Get::ConvexHull),
            get_property(InterpolationNodesProperty).map(|_| Get::InterpolationNodes),
            get_property(Samples).map(|_| Get::Samples),
        ));
        let set = choice((
            set_property(ConvexHull, bool).map(Set::ConvexHull),
            set_property(InterpolationNodesProperty, interpolation_nodes)
                .map(Set::InterpolationNodes),
            set_property(Samples, uint).map(Set::Samples),
        ));
        let toggle = choice((get_property(ConvexHull).map(|_| Toggle::ConvexHull),));
        just(b':').ignore_then(choice((
            just(b"get").padded().ignore_then(get).map(Command::Get),
            just(b"set").padded().ignore_then(set).map(Command::Set),
            just(b"toggle")
                .padded()
                .ignore_then(toggle)
                .map(Command::Toggle),
        )))
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
