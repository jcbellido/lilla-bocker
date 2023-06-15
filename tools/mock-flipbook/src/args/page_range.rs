use std::{fmt::Display, str::FromStr};

use clap::error::ErrorKind;

#[derive(Clone, Debug)]
pub struct PageRange {
    pub min: u16,
    pub max: u16,
}

impl Display for PageRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.min, self.max)
    }
}

impl Default for PageRange {
    fn default() -> Self {
        Self { min: 16, max: 32 }
    }
}

impl FromStr for PageRange {
    type Err = clap::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() == 2 {
            let Ok(min) = u16::from_str(parts[0].trim()) else {
                return Err(clap::Error::new(ErrorKind::InvalidValue));
            };
            let Ok(max) = u16::from_str(parts[1].trim()) else {
                return Err(clap::Error::new(ErrorKind::InvalidValue));
            };
            if min == max && min == 0 {
                return Err(clap::Error::new(ErrorKind::InvalidValue));
            }
            if max < min {
                return Err(clap::Error::new(ErrorKind::InvalidValue));
            }
            Ok(PageRange { min, max })
        } else {
            Err(clap::Error::new(ErrorKind::InvalidValue))
        }
    }
}
