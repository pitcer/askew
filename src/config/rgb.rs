use anyhow::anyhow;
use anyhow::Result;
use atoi::FromRadix16;
use chumsky::prelude::*;
use chumsky::Parser;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

impl Rgb {
    #[must_use]
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub fn parse(input: &str) -> Result<Self> {
        let result = parser()
            .parse(input.as_bytes())
            .into_result()
            .map_err(|error| anyhow!("{:?}", error))?;
        Ok(result)
    }

    #[must_use]
    pub const fn red(self) -> u8 {
        self.red
    }

    #[must_use]
    pub const fn green(self) -> u8 {
        self.green
    }

    #[must_use]
    pub const fn blue(self) -> u8 {
        self.blue
    }
}

impl Default for Rgb {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Alpha(u8);

impl Alpha {
    #[must_use]
    pub const fn new(alpha: u8) -> Self {
        Self(alpha)
    }

    #[must_use]
    pub const fn max() -> Self {
        Self(255)
    }

    #[must_use]
    pub const fn min() -> Self {
        Self(0)
    }

    #[must_use]
    pub const fn alpha(self) -> u8 {
        self.0
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }
}

fn parser<'a>() -> impl Parser<'a, &'a [u8], Rgb> {
    let hex_digit = any().filter(u8::is_ascii_hexdigit);
    let color_byte = group([hex_digit, hex_digit])
        .map(|digits| u8::from_radix_16(&digits))
        .map(|(color_byte, _)| color_byte);
    just(b'#')
        .ignore_then(group((color_byte, color_byte, color_byte)))
        .map(|(red, green, blue)| Rgb::new(red, green, blue))
}
