use crate::ohm::Precision;
use std::fmt::Display;

use crate::band::TempCoeff;

#[derive(Debug, PartialEq)]
pub struct Resistance {
    raw: Precision,
    tol: Precision,
    temp: Option<TempCoeff>,
}

impl Resistance {
    pub fn new(raw: Precision, tol: Precision, temp: Option<TempCoeff>) -> Self {
        Self {
            raw: raw,
            tol: tol,
            temp: temp,
        }
    }
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

impl Display for Resistance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} Ω ± {:?}% (min: {:?} Ω, max: {:?} Ω){}",
            self.raw,
            self.tol,
            self.minimum(),
            self.maximum(),
            {
                if let Some(t) = self.temp {
                    String::from(" ") + &t.to_string()
                } else {
                    String::new()
                }
            },
        )
    }
}
