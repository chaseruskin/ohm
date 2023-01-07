use clif::Cli;
use clif::cmd::FromCli;
use crate::resistance::Resistance;
use clif::arg::{Flag, Positional};
use crate::band::*;

pub type Precision = f64;

const LEN_3_BAND: usize = 3;
const LEN_4_BAND: usize = 4;
const LEN_5_BAND: usize = 5;
const LEN_6_BAND: usize = 6;

/// Minimum support number of colors for a resistor.
const MIN_CODE_LEN: usize = LEN_3_BAND;

/// Maximum number of colors for a resistor.
const MAX_CODE_LEN: usize = LEN_6_BAND;

#[derive(Debug, PartialEq)]
pub struct Ohm {
    resistor: Resistor
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
    }
}