// Copyright: (c) 2020 Cedric Liegeois
// License: BSD3
use std::f64::consts::PI;

use crate::Measure;

/// A signed angle with a resolution of a microarcsecond.
/// When used as a latitude/longitude this roughly translate to a precision
/// of 0.03 millimetres at the equator.
///
/// `Angle` implements many traits, including [`Add`], [`Sub`], [`Mul`], and
/// [`Div`], among others.
// FIXME Display & FromStr
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Angle {
    /// Number of whole microarcseconds.
    microarcseconds: i64,
}

/// The error type returned by the [`Angle::from_dms`] function.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DmsError {
    /// Arcminutes are outside [0, 59].
    InvalidArcMinutes,
    /// Arcseconds are outside [0, 60].
    InvalidArcSeconds,
}

/// The number of microarcseconds in one degree.
const DG_TO_UAS: f64 = 3_600_000_000.0;

// FIXME parse
impl Angle {
    /// Equivalent to `Angle::from_decimal_degrees(0.0)`.
    ///
    /// ```rust
    /// # use jord::Angle;
    /// assert_eq!(Angle::from_decimal_degrees(0.0), Angle::zero());
    /// ```
    pub fn zero() -> Self {
        Angle { microarcseconds: 0 }
    }

    /// Create a new `Angle` with the given number of decimal degrees.
    pub fn from_decimal_degrees(dec: f64) -> Self {
        let uas = (dec * DG_TO_UAS).round() as i64;
        Angle {
            microarcseconds: uas,
        }
    }

    /// Create a new `Angle` with the given number of whole degrees, arcminutes and decimal arcseconds.
    /// Fails if given arcminutes are outside [0, 59] and/or arcseconds are outside [0, 60].
    ///
    ///  ```rust
    /// # use jord::Angle;
    /// assert_eq!(Ok(Angle::from_decimal_degrees(10.5125)), Angle::from_dms(10, 30, 45.0));
    /// ```
    pub fn from_dms(degs: i64, mins: i64, secs: f64) -> Result<Self, DmsError> {
        if mins < 0 || mins > 59 {
            Err(DmsError::InvalidArcMinutes)
        } else if secs < 0.0 || secs >= 60.0 {
            Err(DmsError::InvalidArcSeconds)
        } else {
            let d = degs.abs() as f64 + (mins as f64 / 60.0) + (secs / 3600.0);
            if degs < 0 {
                Ok(Angle::from_decimal_degrees(-d))
            } else {
                Ok(Angle::from_decimal_degrees(d))
            }
        }
    }

    /// Create a new `Angle` with the given number of radians.
    pub fn from_radians(rads: f64) -> Self {
        Angle::from_decimal_degrees(rads / PI * 180.0)
    }

    /// Returns the number of microarcseconds of this `Angle`.
    ///
    ///  ```rust
    /// # use jord::Angle;
    /// assert_eq!(3_600_000_000, Angle::from_decimal_degrees(1.0).microarcseconds());
    /// ```
    pub fn microarcseconds(self) -> i64 {
        self.microarcseconds
    }

    /// Converts this `Angle` to a number of radians.
    pub fn as_radians(self) -> f64 {
        self.as_decimal_degrees() * PI / 180.0
    }

    /// Converts this `Angle` to a number of decimal degrees.
    pub fn as_decimal_degrees(self) -> f64 {
        self.microarcseconds as f64 / DG_TO_UAS
    }

    /// Returns the degree component of this `Angle`.
    ///
    ///  ```rust
    /// # use jord::Angle;
    /// assert_eq!(-154, Angle::from_dms(-154, 3, 42.5).unwrap().whole_degrees());
    /// ```
    pub fn whole_degrees(self) -> i64 {
        let d = Angle::field(self, DG_TO_UAS, 360.0) as i64;
        if self.microarcseconds < 0 {
            -d
        } else {
            d
        }
    }

    /// Returns the arcminutes component of this `Angle`.
    ///
    ///  ```rust
    /// # use jord::Angle;
    /// assert_eq!(45, Angle::from_dms(-154, 45, 42.5).unwrap().arcminutes());
    /// ```
    pub fn arcminutes(self) -> u8 {
        Angle::field(self, 60000000.0, 60.0) as u8
    }

    /// Returns the arcseconds component of this `Angle`.
    ///
    ///  ```rust
    /// # use jord::Angle;
    /// assert_eq!(42, Angle::from_dms(-154, 45, 42.5).unwrap().arcseconds());
    /// ```
    pub fn arcseconds(self) -> u8 {
        Angle::field(self, 1000000.0, 60.0) as u8
    }

    /// Returns the arcmilliseconds component of this `Angle`.
    ///
    ///  ```rust
    /// # use jord::Angle;
    /// assert_eq!(500, Angle::from_dms(-154, 45, 42.5).unwrap().arcmilliseconds());
    /// ```
    pub fn arcmilliseconds(self) -> u16 {
        Angle::field(self, 1000.0, 1000.0) as u16
    }

    fn field(self, div: f64, modu: f64) -> u64 {
        (self.microarcseconds.abs() as f64 / div % modu) as u64
    }
}

impl Measure for Angle {
    fn from_default_unit(amount: f64) -> Self {
        Angle::from_decimal_degrees(amount)
    }

    fn from_resolution_unit(amount: i64) -> Self {
        Angle {
            microarcseconds: amount,
        }
    }

    fn as_default_unit(self) -> f64 {
        self.as_decimal_degrees()
    }

    fn as_resolution_unit(self) -> i64 {
        self.microarcseconds
    }
}

impl_measure! { Angle }

#[cfg(test)]
mod test {

    use crate::Angle;

    #[test]
    fn one_microarcsecond() {
        assert_eq!(
            Angle::from_decimal_degrees(60.0),
            Angle::from_decimal_degrees(59.9999999999)
        );
        assert_ne!(
            Angle::from_decimal_degrees(60.0),
            Angle::from_decimal_degrees(59.999999998)
        );
    }

    #[test]
    fn one_arcmillisecond() {
        let a = Angle::from_decimal_degrees(1.0 / 3600000.0);
        assert_eq!(0, a.whole_degrees());
        assert_eq!(0, a.arcminutes());
        assert_eq!(0, a.arcseconds());
        assert_eq!(1, a.arcmilliseconds());
    }

    #[test]
    fn one_arcsecond() {
        let a = Angle::from_decimal_degrees(1000.0 / 3600000.0);
        assert_eq!(0, a.whole_degrees());
        assert_eq!(0, a.arcminutes());
        assert_eq!(1, a.arcseconds());
        assert_eq!(0, a.arcmilliseconds());
    }

    #[test]
    fn one_arcminute() {
        let a = Angle::from_decimal_degrees(60000.0 / 3600000.0);
        assert_eq!(0, a.whole_degrees());
        assert_eq!(1, a.arcminutes());
        assert_eq!(0, a.arcseconds());
        assert_eq!(0, a.arcmilliseconds());
    }

    #[test]
    fn one_degrees() {
        let a = Angle::from_decimal_degrees(1.0);
        assert_eq!(1, a.whole_degrees());
        assert_eq!(0, a.arcminutes());
        assert_eq!(0, a.arcseconds());
        assert_eq!(0, a.arcmilliseconds());
    }

    #[test]
    fn positve_value() {
        let a = Angle::from_decimal_degrees(154.9150300);
        assert_eq!(154, a.whole_degrees());
        assert_eq!(54, a.arcminutes());
        assert_eq!(54, a.arcseconds());
        assert_eq!(108, a.arcmilliseconds());
    }

    #[test]
    fn negative_value() {
        let a = Angle::from_decimal_degrees(-154.915);
        assert_eq!(-154, a.whole_degrees());
        assert_eq!(54, a.arcminutes());
        assert_eq!(54, a.arcseconds());
        assert_eq!(0, a.arcmilliseconds());
    }
}
