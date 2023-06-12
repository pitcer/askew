use anyhow::{anyhow, Result};
use atoi::FromRadix16;
use chumsky::error::Cheap;
use chumsky::prelude::*;
use chumsky::Parser;

use crate::ui::color::Rgb;

pub fn parse_rgb(input: &str) -> Result<Rgb> {
    let result = parser()
        .parse(input.as_bytes())
        .map_err(|error| anyhow!("{:?}", error))?;
    Ok(result)
}

fn parser() -> impl Parser<u8, Rgb, Error = Cheap<u8>> {
    let hex_digit = filter(u8::is_ascii_hexdigit);
    let color_byte = hex_digit
        .chain(hex_digit)
        .map(|digits| u8::from_radix_16(&digits).0);
    just(b'#')
        .ignore_then(color_byte)
        .then(color_byte)
        .then(color_byte)
        .then_ignore(end())
        .map(|((red, green), blue)| Rgb::new(red, green, blue))
}
