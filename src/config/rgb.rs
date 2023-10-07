use std::num::ParseIntError;
use std::str::FromStr;

use anyhow::Result;

#[derive(
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    parse_display::Display,
    parse_display::FromStr,
)]
#[display("#{red}{green}{blue}")]
pub struct Rgb {
    #[from_str(regex = "[0-9a-f]{2}")]
    red: Hex,
    #[from_str(regex = "[0-9a-f]{2}")]
    green: Hex,
    #[from_str(regex = "[0-9a-f]{2}")]
    blue: Hex,
}

impl Rgb {
    #[must_use]
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red: Hex(red), green: Hex(green), blue: Hex(blue) }
    }

    #[must_use]
    pub const fn red(self) -> u8 {
        self.red.0
    }

    #[must_use]
    pub const fn green(self) -> u8 {
        self.green.0
    }

    #[must_use]
    pub const fn blue(self) -> u8 {
        self.blue.0
    }
}

impl Default for Rgb {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

pub mod serde_pretty {
    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serializer};

    use super::Rgb;

    pub fn serialize<S>(rgb: &Rgb, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string = rgb.to_string();
        serializer.serialize_str(&string)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Rgb, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(Error::custom)
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

#[derive(
    Debug, Copy, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, parse_display::Display,
)]
#[display("{0:02x}")]
struct Hex(u8);

impl FromStr for Hex {
    type Err = ParseIntError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let number = u8::from_str_radix(string, 16)?;
        Ok(Hex(number))
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_parse_rgb1() {
        assert_eq!(Rgb::new(0, 0, 0), "#000000".parse::<Rgb>().unwrap());
    }

    #[test]
    fn test_parse_rgb2() {
        assert_eq!(Rgb::new(15, 14, 16), "#0f0e10".parse::<Rgb>().unwrap());
    }

    #[test]
    #[should_panic]
    fn test_parse_rgb_wrong1() {
        assert_eq!(Rgb::new(0, 0, 0), "#000".parse::<Rgb>().unwrap());
    }

    #[test]
    #[should_panic]
    fn test_parse_rgb_wrong2() {
        assert_eq!(Rgb::new(0, 0, 0), "f000ff".parse::<Rgb>().unwrap());
    }
}
