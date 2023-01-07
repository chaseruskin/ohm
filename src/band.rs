use std::str::FromStr;
use std::fmt::Display;
use thiserror::Error;

use crate::ohm::Precision;

#[derive(Debug, PartialEq, Clone)]
pub enum Band {
    Black,
    Brown,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Violet,
    Grey,
    White,
    Gold,
    Silver
}

impl Display for Band {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait FromBand {
    type Err;

    fn from_band(b: &Band) -> Result<Self, Self::Err> where Self: Sized;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Digit {
    Black,
    Brown,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Violet,
    Grey,
    White,
}

impl FromBand for Digit {
    type Err = BandError;

    fn from_band(b: &Band) -> Result<Self, Self::Err> where Self: Sized {
        Ok(match b {
            Band::Black => Self::Black,
            Band::Brown => Self::Brown,
            Band::Red => Self::Red,
            Band::Orange => Self::Orange,
            Band::Yellow => Self::Yellow,
            Band::Green => Self::Green,
            Band::Blue => Self::Blue,
            Band::Violet => Self::Violet,
            Band::Grey => Self::Grey,
            Band::White => Self::White,
            _ => return Err(BandError::UnsupportedBand(String::from("digit"), b.clone())),
        })
    }
}

impl Into<u8> for Digit {
    fn into(self) -> u8 {
        match self {
            Self::Black  => 0,
            Self::Brown  => 1,
            Self::Red    => 2,
            Self::Orange => 3,
            Self::Yellow => 4,
            Self::Green  => 5,
            Self::Blue   => 6,
            Self::Violet => 7,
            Self::Grey   => 8,
            Self::White  => 9,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Multiplier {
    Black,
    Brown,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Violet,
    Grey,
    White,
    Gold,
    Silver,
}

impl FromBand for Multiplier {
    type Err = BandError;

    fn from_band(b: &Band) -> Result<Self, Self::Err> where Self: Sized {
        Ok(match b {
            Band::Black => Self::Black,
            Band::Brown => Self::Brown,
            Band::Red => Self::Red,
            Band::Orange => Self::Orange,
            Band::Yellow => Self::Yellow,
            Band::Green => Self::Green,
            Band::Blue => Self::Blue,
            Band::Violet => Self::Violet,
            Band::Grey => Self::Grey,
            Band::White => Self::White,
            Band::Gold => Self::Gold,
            Band::Silver => Self::Silver,
        })
    }
}

impl Into<i8> for Multiplier {
    fn into(self) -> i8 {
        match self {
            Self::Black  => 0,
            Self::Brown  => 1,
            Self::Red    => 2,
            Self::Orange => 3,
            Self::Yellow => 4,
            Self::Green  => 5,
            Self::Blue   => 6,
            Self::Violet => 7,
            Self::Grey   => 8,
            Self::White  => 9,
            Self::Gold   => -1,
            Self::Silver => -2,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Tolerance {
    // Black,
    Brown,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Violet,
    Grey,
    // White,
    Gold,
    Silver,
    /// Default tolerance for a 3-band resistor.
    Default,
}

impl FromBand for Tolerance {
    type Err = BandError;

    fn from_band(b: &Band) -> Result<Self, Self::Err> where Self: Sized {
        Ok(match b {
            Band::Brown => Self::Brown,
            Band::Red => Self::Red,
            Band::Orange => Self::Orange,
            Band::Yellow => Self::Yellow,
            Band::Green => Self::Green,
            Band::Blue => Self::Blue,
            Band::Violet => Self::Violet,
            Band::Grey => Self::Grey,
            Band::Gold => Self::Gold,
            Band::Silver => Self::Silver,
            _ => return Err(BandError::UnsupportedBand(String::from("tolerance"), b.clone())),
        })
    }
}

impl Into<Precision> for Tolerance {
    fn into(self) -> Precision {
        match self {
            Self::Brown => 1.0,
            Self::Red => 2.0,
            Self::Orange => 3.0,
            Self::Yellow => 4.0,
            Self::Green => 0.5,
            Self::Blue => 0.25,
            Self::Violet => 0.1,
            Self::Grey => 0.05,
            Self::Gold => 5.0,
            Self::Silver => 10.0,
            Self::Default => 20.0,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TempCoeff {
    Black,
    Brown,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Violet,
    Grey,
    // White,
    // Gold,
    // Silver
}

impl FromBand for TempCoeff {
    type Err = BandError;

    fn from_band(b: &Band) -> Result<Self, Self::Err> where Self: Sized {
        Ok(match b {
            Band::Black => Self::Black,
            Band::Brown => Self::Brown,
            Band::Red => Self::Red,
            Band::Orange => Self::Orange,
            Band::Yellow => Self::Yellow,
            Band::Green => Self::Green,
            Band::Blue => Self::Blue,
            Band::Violet => Self::Violet,
            Band::Grey => Self::Grey,
            _ => return Err(BandError::UnsupportedBand(String::from("temperature coefficient"), b.clone())),
        })
    }
}

impl Display for TempCoeff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ppm/K",  <TempCoeff as Into<u8>>::into(*self))
    }
}

impl Into<u8> for TempCoeff {
    fn into(self) -> u8 {
        match self {
            Self::Black => 250,
            Self::Brown => 100,
            Self::Red => 50,
            Self::Orange => 15,
            Self::Yellow => 25,
            Self::Green => 20,
            Self::Blue => 10,
            Self::Violet => 5,
            Self::Grey => 1,
        }
    }
}

impl FromStr for Band {
    type Err = BandError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> { 
        Ok(match s.to_ascii_lowercase().as_ref() {
            "black" => Self::Black,
            "brown" => Self::Brown,
            "red" => Self::Red,
            "orange" => Self::Orange,
            "yellow" => Self::Yellow,
            "green" => Self::Green,
            "blue" => Self::Blue,
            "violet" => Self::Violet,
            "grey" | "gray" => Self::Grey,
            "white" => Self::White,
            "gold" => Self::Gold,
            "silver" => Self::Silver,
            _ => return Err(BandError::InvalidCode)
        })
    }
}

#[derive(Error, Debug)]
pub enum BandError {
    #[error("invalid color code")]
    InvalidCode,
    #[error("color code requires 3 to 6 values but got {0}")]
    OutOfRange(usize),
    #[error("band {1} is not allowed as a {0}")]
    UnsupportedBand(String, Band),
}