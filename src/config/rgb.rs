use std::fmt::{Display, Formatter};

use anyhow::{anyhow, Result};

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
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
        let ("#", input) = input.split_at(1) else {
            return Err(anyhow!("Rgb color should start with '#'"));
        };
        let color = u32::from_str_radix(input, 16)?;
        let red = (color & 0x00ff_0000) >> 16;
        let green = (color & 0x0000_ff00) >> 8;
        let blue = color & 0x0000_00ff;
        Ok(Self::new(red as u8, green as u8, blue as u8))
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

impl Display for Rgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
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
