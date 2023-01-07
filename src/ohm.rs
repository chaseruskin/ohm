use std::str::FromStr;
use thiserror::Error;
use clif::Cli;
use clif::cmd::FromCli;
use std::fmt::Display;
use clif::arg::{Flag, Positional};

#[derive(Debug, PartialEq, Clone)]
enum Band {
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

trait FromBand {
    type Err;

    fn from_band(b: &Band) -> Result<Self, Self::Err> where Self: Sized;
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Digit {
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
enum Multiplier {
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
enum Tolerance {
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
enum TempCoeff {
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
enum BandError {
    #[error("invalid color code")]
    InvalidCode,
    #[error("color code requires 3 to 6 values but got {0}")]
    OutOfRange(usize),
    #[error("band {1} is not allowed as a {0}")]
    UnsupportedBand(String, Band),
}

const LEN_3_BAND: usize = 3;
const LEN_4_BAND: usize = 4;
const LEN_5_BAND: usize = 5;
const LEN_6_BAND: usize = 6;

/// Minimum support number of colors for a resistor.
const MIN_CODE_LEN: usize = LEN_3_BAND;

/// Maximum number of colors for a resistor.
const MAX_CODE_LEN: usize = LEN_6_BAND;

#[derive(Debug, PartialEq)]
struct Resistor {
    first: Digit,
    second: Digit,
    third: Option<Digit>,
    multiplier: Multiplier,
    tolerance: Tolerance,
    temp_coeff: Option<TempCoeff>,
}

impl Resistor {
    fn decode(vec: Vec<Band>) -> Result<Self, BandError> {
        // reverse to use the `pop` method
        let mut vec = vec;
        vec.reverse();
        // capture the state of how many bands are specified
        let band_count = vec.len();
        if band_count >= MIN_CODE_LEN && band_count <= MAX_CODE_LEN {
            Ok(Self { 
                first: Digit::from_band(&vec.pop().unwrap())?,
                second: Digit::from_band(&vec.pop().unwrap())?,
                third: {
                    match band_count {
                        LEN_3_BAND | LEN_4_BAND => None,
                        LEN_5_BAND | LEN_6_BAND => Some(Digit::from_band(&vec.pop().unwrap())?),
                        _ => panic!("unsupported band length {}", band_count)
                    }
                },
                multiplier: Multiplier::from_band(&vec.pop().unwrap())?,
                tolerance: match vec.pop() { Some(b) => Tolerance::from_band(&b)?, None => Tolerance::Default },
                temp_coeff: match vec.pop() { Some(b) => Some(TempCoeff::from_band(&b)?), None => None },
            })
        } else {
            Err(BandError::OutOfRange(band_count))
        }
    }
}

// #[derive(Error, Debug)]
// enum ResistorError {
//     #[error("color code requires 3 to 6 values but got {0}")]
//     OutOfRange(usize)
// }

#[derive(Debug, PartialEq)]
pub struct Ohm {
    resistor: Resistor
}

impl Ohm {
    pub fn compute(&self) -> Resistance {
        let mut result: usize = 0;

        result += <Digit as Into<u8>>::into(self.resistor.first) as usize;
        // shift digits to the left by 1 position
        result *= 10;
        result += <Digit as Into<u8>>::into(self.resistor.second) as usize;
        if let Some(third) = self.resistor.third {
            // shift digits to the left by 1 position
            result *= 10;
            result += <Digit as Into<u8>>::into(third) as usize;
        }
        let mult: i8 = self.resistor.multiplier.into();

        let result: Precision = match mult >= 0 {
            true => result as Precision * 10_usize.pow(mult.abs() as u32) as Precision,
            false => result as Precision / 10_usize.pow(mult.abs() as u32) as Precision,
        };

        Resistance {
            raw: result,
            tol: self.resistor.tolerance.into(),
            temp: self.resistor.temp_coeff,
        }
    }
}

impl FromCli for Ohm {

    fn from_cli<'c>(cli: &'c mut Cli<'_>) -> Result<Self, clif::Error<'c>> { 

        cli.check_help(
            clif::Help::new()
                .flag(Flag::new("help").switch('h'))
                .quick_text(QUICK_HELP)
            )?;

        let bands = cli.require_positional_all(Positional::new("band"))?;
        let app = Self {
            resistor: clif::Error::validate(Resistor::decode(bands))?,
        };

        // verify the cli is empty
        cli.is_empty()?;
        Ok(app)
    }
}

type Precision = f64;

#[derive(Debug, PartialEq)]
pub struct Resistance {
    raw: Precision,
    tol: Precision,
    temp: Option<TempCoeff>,
}

impl Resistance {
    /// Calculates the minimum-end of the allowed tolerance given the ideal `raw` value.
    fn minimum(&self) -> Precision {
        self.raw - self.percent_error()
    }

    /// Calculates the maximum-end of the allowed tolerance given the ideal `raw` value.
    fn maximum(&self) -> Precision {
        self.raw + self.percent_error()
    }

    /// Calculate the percent error allowed to devivate from the ideal value.
    fn percent_error(&self) -> Precision {
        self.raw * (self.tol / 100.0)
    }
}

/*
3-band: -[|||    ]-
4-band: -[|||  | ]-
5-band: -[|||| | ]-
6-band: -[|||| ||]-
*/  

impl Display for Resistance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} Ω ± {:?}% (min: {:?} Ω, max: {:?} Ω){}", self.raw, self.tol, self.minimum(), self.maximum(), {
            if let Some(t) = self.temp {
                String::from(" ") + &t.to_string()
            } else {
                String::new()
            }
        },
        )
    }
}

const QUICK_HELP: &str = "\
Resistor color code calculator.

Usage:
    ohm [options] <band>...

Arguments:
    <band>...       colors in order from left to right (between 3 and 6)  

Options:
    --help, -h      print quick help text
    --verbose
    --list          print the possible color codes
    --code          specify the <band> in terms of numerical code
";