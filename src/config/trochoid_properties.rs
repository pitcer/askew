use std::fmt;
use std::fmt::{Display, Formatter};
use std::str;

use anyhow::{anyhow, Result};

use crate::canvas::curve::formula::trochoid::TrochoidProperties;

pub fn parse(input: &str) -> Result<TrochoidProperties> {
    let properties = input
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;
    let [range_start, range_end, r_1, r_2, w_1, w_2] =
        <[f32; 6]>::try_from(properties).map_err(|_properties| anyhow!("6 properties required"))?;
    Ok(TrochoidProperties::new(
        range_start,
        range_end,
        r_1,
        r_2,
        w_1,
        w_2,
    ))
}

impl Display for TrochoidProperties {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},{},{},{},{},{}",
            self.range_start, self.range_end, self.r_1, self.r_2, self.w_1, self.w_2
        )
    }
}
