use std::fmt;
use std::fmt::{Display, Formatter};
use std::str;

use anyhow::{anyhow, Result};
use chumsky::prelude::*;

use crate::canvas::curve::formula::trochoid::TrochoidProperties;
use crate::parser;

pub fn parse(input: &str) -> Result<TrochoidProperties> {
    let result = parser()
        .parse(input.as_bytes())
        .into_result()
        .map_err(|error| anyhow!("{:?}", error))?;
    Ok(result)
}

impl Display for TrochoidProperties {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},{},{},{},{},{}",
            self.range.0, self.range.1, self.r_1, self.r_2, self.w_1, self.w_2
        )
    }
}

fn parser<'a>() -> impl Parser<'a, &'a [u8], TrochoidProperties> {
    let number = parser::f32_parser();
    let comma = just(b',');

    group((
        number, comma, number, comma, number, comma, number, comma, number, comma, number,
    ))
    .map(|(range_0, _, range_1, _, r_1, _, r_2, _, w_1, _, w_2)| {
        TrochoidProperties::new((range_0, range_1), r_1, r_2, w_1, w_2)
    })
}
