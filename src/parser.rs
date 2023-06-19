use std::str;

use atoi::{FromRadix10, FromRadix16};
use chumsky::extra::ParserExtra;
use chumsky::prelude::*;

use crate::config::rgb::Rgb;

pub fn rgb_parser<'a>() -> impl Parser<'a, &'a [u8], Rgb> + Copy {
    let hex_digit = any().filter(u8::is_ascii_hexdigit);
    let color_byte = group([hex_digit, hex_digit])
        .map(|digits| u8::from_radix_16(&digits))
        .map(|(color_byte, _)| color_byte);
    just(b'#')
        .ignore_then(group((color_byte, color_byte, color_byte)))
        .map(|(red, green, blue)| Rgb::new(red, green, blue))
}

#[must_use]
pub fn f32_parser<'a, E>() -> impl Parser<'a, &'a [u8], f32, E> + Copy
where
    E: ParserExtra<'a, &'a [u8]>,
{
    let digits = text::digits(10);
    let fractional = just(b'.').then(digits);
    just(b'-')
        .or_not()
        .then(text::int(10))
        .then(fractional.or_not())
        .map_slice(|slice| str::from_utf8(slice).expect("slice should be an utf8 string"))
        .map(|string| string.parse().expect("string should be in decimal form"))
}

pub fn unsigned_parser<'a, T, E>() -> impl Parser<'a, &'a [u8], T, E> + Copy
where
    T: FromRadix10,
    E: ParserExtra<'a, &'a [u8]>,
{
    text::int(10)
        .map(T::from_radix_10)
        .map(|(number, _)| number)
}
