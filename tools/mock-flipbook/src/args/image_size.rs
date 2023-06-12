use std::{fmt::Display, str::FromStr};

use clap::error::ErrorKind;

#[derive(Clone, Debug)]
pub struct ImageSize {
    width: u16,
    height: u16,
}

impl Display for ImageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.width, self.height)
    }
}

impl Default for ImageSize {
    fn default() -> Self {
        Self {
            width: 1080,
            height: 1920,
        }
    }
}

impl FromStr for ImageSize {
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
            Ok(ImageSize {
                width: min,
                height: max,
            })
        } else {
            Err(clap::Error::new(ErrorKind::InvalidValue))
        }
    }
}
