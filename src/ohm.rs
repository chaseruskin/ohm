use crate::band::*;
use crate::resistance::Resistance;
use clif::arg::{Flag, Positional};
use clif::cmd::{Command, FromCli};
use clif::Cli;
use std::fmt::Display;

pub type Precision = f64;

enum BandGroup {
    R3(Band, Band, Band),
    R4(Band, Band, Band, Band),
    R5(Band, Band, Band, Band, Band),
    R6(Band, Band, Band, Band, Band, Band),
}

impl From<Vec<Band>> for BandGroup {
    fn from(vec: Vec<Band>) -> Self {
        let mut vec = vec;
        vec.reverse();
        match vec.len() {
            3 => Self::R3(vec.pop().unwrap(), vec.pop().unwrap(), vec.pop().unwrap()),
            4 => Self::R4(
                vec.pop().unwrap(),
                vec.pop().unwrap(),
                vec.pop().unwrap(),
                vec.pop().unwrap(),
            ),
            5 => Self::R5(
                vec.pop().unwrap(),
                vec.pop().unwrap(),
                vec.pop().unwrap(),
                vec.pop().unwrap(),
                vec.pop().unwrap(),
            ),
            6 => Self::R6(
                vec.pop().unwrap(),
                vec.pop().unwrap(),
                vec.pop().unwrap(),
                vec.pop().unwrap(),
                vec.pop().unwrap(),
                vec.pop().unwrap(),
            ),
            _ => panic!("unsupported band length {}", vec.len()),
        }
    }
}

impl Display for BandGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (match self {
                Self::R3(b0, b1, b2) => format!("-[{}{}{}    ]-", b0, b1, b2),
                Self::R4(b0, b1, b2, b3) => format!("-[{}{}{}  {} ]-", b0, b1, b2, b3),
                Self::R5(b0, b1, b2, b3, b4) => format!("-[{}{}{}{} {} ]-", b0, b1, b2, b3, b4),
                Self::R6(b0, b1, b2, b3, b4, b5) =>
                    format!("-[{}{}{}{} {}{}]-", b0, b1, b2, b3, b4, b5),
            })
        )
    }
}

#[derive(Debug, PartialEq)]
enum BandLength {
    L3,
    L4,
    L5,
    L6,
}

impl From<usize> for BandLength {
    fn from(f: usize) -> Self {
        match f {
            3 => Self::L3,
            4 => Self::L4,
            5 => Self::L5,
            6 => Self::L6,
            _ => panic!("unsupported band length {}", f),
        }
    }
}

impl Into<usize> for BandLength {
    fn into(self) -> usize {
        match self {
            Self::L3 => 3,
            Self::L4 => 4,
            Self::L5 => 5,
            Self::L6 => 6,
        }
    }
}

/// Minimum support number of colors for a resistor.
const MIN_CODE_LEN: BandLength = BandLength::L3;

/// Maximum number of colors for a resistor.
const MAX_CODE_LEN: BandLength = BandLength::L6;

#[derive(Debug, PartialEq)]
pub struct Ohm {
    resistor: Resistor,
}

impl Ohm {
    fn calculate_raw(res: &Resistor) -> Precision {
        let mut result: usize = 0;
        // add the first digit
        result += <Digit as Into<u8>>::into(res.first) as usize;

        // shift digits to the left by 1 position
        result = (result * 10) + <Digit as Into<u8>>::into(res.second) as usize;

        if let Some(third) = res.third {
            // shift digits to the left by 1 position
            result = (result * 10) + <Digit as Into<u8>>::into(third) as usize;
        }

        let mult: i8 = res.multiplier.into();
        match mult >= 0 {
            true => result as Precision * 10_usize.pow(mult.abs() as u32) as Precision,
            false => result as Precision / 10_usize.pow(mult.abs() as u32) as Precision,
        }
    }

    pub fn compute(&self) -> Resistance {
        Resistance::new(
            Self::calculate_raw(&self.resistor),
            self.resistor.tolerance.into(),
            self.resistor.temp_coeff,
        )
    }
}

impl FromCli for Ohm {
    fn from_cli(cli: &mut Cli) -> Result<Self, clif::Error> {
        // check for 1st overall help flag
        {
            cli.check_help(
                clif::Help::new()
                    .flag(Flag::new("help").switch('h'))
                    .quick_text(QUICK_HELP),
            )?;
            cli.raise_help()?;
            cli.clear_help();
        }
        // check for 2nd quick color code list flag
        {
            cli.check_help(
                clif::Help::new()
                    .flag(Flag::new("list"))
                    .quick_text(BAND_LIST),
            )?;
            cli.raise_help()?;
            cli.clear_help();
        }
        // return to overall help flag
        cli.check_help(
            clif::Help::new()
                .flag(Flag::new("help").switch('h'))
                .quick_text(QUICK_HELP),
        )?;

        // parse cli into `Ohm` struct
        let bands = cli.require_positional_all(Positional::new("band"))?;
        let app = Self {
            resistor: clif::Error::validate(Resistor::decode(bands.clone()))?,
        };

        // verify the cli is empty
        cli.is_empty()?;

        let group = BandGroup::from(bands.clone());
        println!("Identification: {}", group);

        Ok(app)
    }
}

impl Command<()> for Ohm {
    type Status = u8;
    fn exec(&self, _: &()) -> <Self as clif::cmd::Command<()>>::Status {
        let resistance = self.compute();
        println!("Resistance: {}", resistance);
        0
    }
}

const QUICK_HELP: &str = "\
Resistor color code calculator.

Usage:
    ohm [options] <band>...

Arguments:
    <band>...       colors from left to right (expects between 3 and 6)  

Options:
    --help, -h      print quick help text
    --list          print the possible color codes
";

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
        if band_count >= MIN_CODE_LEN.into() && band_count <= MAX_CODE_LEN.into() {
            Ok(Self {
                first: Digit::from_band(&vec.pop().unwrap())?,
                second: Digit::from_band(&vec.pop().unwrap())?,
                third: {
                    match BandLength::from(band_count) {
                        BandLength::L3 | BandLength::L4 => None,
                        BandLength::L5 | BandLength::L6 => {
                            Some(Digit::from_band(&vec.pop().unwrap())?)
                        }
                    }
                },
                multiplier: Multiplier::from_band(&vec.pop().unwrap())?,
                tolerance: match vec.pop() {
                    Some(b) => Tolerance::from_band(&b)?,
                    None => Tolerance::Default,
                },
                temp_coeff: match vec.pop() {
                    Some(b) => Some(TempCoeff::from_band(&b)?),
                    None => None,
                },
            })
        } else {
            Err(BandError::OutOfRange(band_count))
        }
    }

    #[allow(dead_code)]
    fn size(&self) -> BandLength {
        if self.tolerance != Tolerance::Default {
            if self.third.is_some() {
                if self.temp_coeff.is_some() {
                    BandLength::L6
                } else {
                    BandLength::L5
                }
            } else {
                BandLength::L4
            }
        } else {
            BandLength::L3
        }
    }
}

// #[derive(Error, Debug)]
// enum ResistorError {
//     #[error("color code requires 3 to 6 values but got {0}")]
//     OutOfRange(usize)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_raw_resistance_3_band() {
        let r = Resistor {
            first: Digit::Brown,
            second: Digit::Black,
            third: None,
            multiplier: Multiplier::Brown,
            tolerance: Tolerance::Default,
            temp_coeff: None,
        };

        assert_eq!(Ohm::calculate_raw(&r), 100.0);
        assert_eq!(r.size(), BandLength::L3);
    }

    #[test]
    fn ut_raw_resistance_4_band() {
        let r = Resistor {
            first: Digit::Brown,
            second: Digit::Red,
            third: None,
            multiplier: Multiplier::Green,
            tolerance: Tolerance::Gold,
            temp_coeff: None,
        };

        assert_eq!(Ohm::calculate_raw(&r), 1200_000.0);
        assert_eq!(r.size(), BandLength::L4);
    }

    #[test]
    fn ut_raw_resistance_5_band() {
        let r = Resistor {
            first: Digit::Red,
            second: Digit::Red,
            third: Some(Digit::Black),
            multiplier: Multiplier::Black,
            tolerance: Tolerance::Gold,
            temp_coeff: None,
        };

        assert_eq!(Ohm::calculate_raw(&r), 220.0);
        assert_eq!(r.size(), BandLength::L5);
    }

    #[test]
    fn ut_raw_resistance_6_band() {
        let r = Resistor {
            first: Digit::Red,
            second: Digit::Violet,
            third: Some(Digit::Yellow),
            multiplier: Multiplier::Black,
            tolerance: Tolerance::Red,
            temp_coeff: Some(TempCoeff::Black),
        };

        assert_eq!(Ohm::calculate_raw(&r), 274.0);
        assert_eq!(r.size(), BandLength::L6);
    }
}
