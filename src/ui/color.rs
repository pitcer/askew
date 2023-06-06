#[derive(Debug, Copy, Clone)]
pub struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

impl Rgb {
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub const fn red(self) -> u8 {
        self.red
    }

    pub const fn green(self) -> u8 {
        self.green
    }

    pub const fn blue(self) -> u8 {
        self.blue
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Alpha(u8);

impl Alpha {
    pub const fn new(alpha: u8) -> Self {
        Self(alpha)
    }

    pub const fn max() -> Self {
        Self(255)
    }

    pub const fn min() -> Self {
        Self(0)
    }

    pub const fn alpha(self) -> u8 {
        self.0
    }
}
