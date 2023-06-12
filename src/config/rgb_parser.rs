use anyhow::{anyhow, Result};
use atoi::FromRadix16;
use chumsky::prelude::*;
use chumsky::Parser;

use crate::ui::color::Rgb;

pub fn parse_rgb(input: &str) -> Result<Rgb> {
    let result = parser()
        .parse(input.as_bytes())
        .into_result()
        .map_err(|error| anyhow!("{:?}", error))?;
    Ok(result)
}

fn parser<'a>() -> impl Parser<'a, &'a [u8], Rgb> {
    let color_byte = any()
        .filter(u8::is_ascii_hexdigit)
        .repeated()
        .exactly(2)
        .map_slice(|digits| u8::from_radix_16(digits).0);
    just(b'#')
        .ignore_then(group((color_byte, color_byte, color_byte)))
        .then_ignore(end())
        .map(|(red, green, blue)| Rgb::new(red, green, blue))
}
