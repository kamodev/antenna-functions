//! Pure-Rust antenna length calculations.
//!
//! Nothing in this module depends on Python, so it can be used and tested
//! directly from Rust. All lengths are stored in meters; use
//! [`meters_to_feet`] for imperial units.

use std::f64::consts::PI;
use std::fmt;

/// Speed of light in a vacuum, in meters per second.
pub const SPEED_OF_LIGHT_M_S: f64 = 299_792_458.0;

/// Exact length of one international foot, in meters.
pub const METERS_PER_FOOT: f64 = 0.3048;

/// Shortening factor applied to a free-space half wavelength to get the
/// physical length of a half-wave wire dipole.
///
/// The default reproduces the classic `468 / f(MHz)` feet formula, which
/// accounts for end effect on a typical wire dipole.
pub const DIPOLE_K_FACTOR: f64 = 468.0 * METERS_PER_FOOT / (SPEED_OF_LIGHT_M_S / 2.0e6);

/// Lengthening factor applied to a free-space wavelength to get the perimeter
/// of a full-wave wire loop.
///
/// The default reproduces the classic `1005 / f(MHz)` feet formula for
/// full-wave loops (delta, quad, and horizontal loops).
pub const LOOP_K_FACTOR: f64 = 1005.0 * METERS_PER_FOOT / (SPEED_OF_LIGHT_M_S / 1.0e6);

/// Default feed point position of an off-center fed dipole, as a fraction of
/// its total length measured from one end (the classic 1/3 : 2/3 Windom split).
pub const OCF_FEED_OFFSET: f64 = 1.0 / 3.0;

/// Reasons an antenna calculation can be rejected.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AntennaError {
    /// A parameter that must be a finite number greater than zero was not.
    NotPositive { name: &'static str, value: f64 },
    /// The off-center feed offset was outside `(0, 0.5]`.
    FeedOffsetOutOfRange(f64),
}

impl fmt::Display for AntennaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotPositive { name, value } => {
                write!(
                    f,
                    "{name} must be a finite number greater than 0, got {value}"
                )
            }
            Self::FeedOffsetOutOfRange(value) => write!(
                f,
                "feed_offset must be greater than 0 and at most 0.5 \
                 (fraction of the total length from the nearest end), got {value}"
            ),
        }
    }
}

impl std::error::Error for AntennaError {}

fn require_positive(name: &'static str, value: f64) -> Result<f64, AntennaError> {
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(AntennaError::NotPositive { name, value })
    }
}

/// Converts a length in meters to feet.
pub fn meters_to_feet(meters: f64) -> f64 {
    meters / METERS_PER_FOOT
}

/// Free-space wavelength in meters for a frequency in MHz.
pub fn wavelength_m(frequency_mhz: f64) -> Result<f64, AntennaError> {
    let frequency_mhz = require_positive("frequency_mhz", frequency_mhz)?;
    Ok(SPEED_OF_LIGHT_M_S / (frequency_mhz * 1.0e6))
}

/// A half-wave, center-fed dipole: two equal legs, normally fed with a 1:1
/// balun.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dipole {
    pub frequency_mhz: f64,
    pub k_factor: f64,
    pub total_length_m: f64,
}

impl Dipole {
    pub fn new(frequency_mhz: f64, k_factor: f64) -> Result<Self, AntennaError> {
        let k_factor = require_positive("k_factor", k_factor)?;
        let total_length_m = wavelength_m(frequency_mhz)? / 2.0 * k_factor;
        Ok(Self {
            frequency_mhz,
            k_factor,
            total_length_m,
        })
    }

    /// Length of each of the two legs (feed point to wire end).
    pub fn leg_length_m(&self) -> f64 {
        self.total_length_m / 2.0
    }
}

/// A half-wave dipole fed off center (Windom style), splitting the wire into a
/// short leg and a long leg.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OffCenterFedDipole {
    pub frequency_mhz: f64,
    pub k_factor: f64,
    /// Fraction of the total length between the nearest end and the feed point.
    pub feed_offset: f64,
    pub total_length_m: f64,
}

impl OffCenterFedDipole {
    pub fn new(frequency_mhz: f64, feed_offset: f64, k_factor: f64) -> Result<Self, AntennaError> {
        if !(feed_offset > 0.0 && feed_offset <= 0.5) {
            return Err(AntennaError::FeedOffsetOutOfRange(feed_offset));
        }
        let total_length_m = Dipole::new(frequency_mhz, k_factor)?.total_length_m;
        Ok(Self {
            frequency_mhz,
            k_factor,
            feed_offset,
            total_length_m,
        })
    }

    pub fn short_leg_length_m(&self) -> f64 {
        self.total_length_m * self.feed_offset
    }

    pub fn long_leg_length_m(&self) -> f64 {
        self.total_length_m - self.short_leg_length_m()
    }
}

/// A full-wave delta loop shaped as an equilateral triangle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DeltaLoop {
    pub frequency_mhz: f64,
    pub k_factor: f64,
    pub perimeter_m: f64,
}

impl DeltaLoop {
    pub fn new(frequency_mhz: f64, k_factor: f64) -> Result<Self, AntennaError> {
        let k_factor = require_positive("k_factor", k_factor)?;
        let perimeter_m = wavelength_m(frequency_mhz)? * k_factor;
        Ok(Self {
            frequency_mhz,
            k_factor,
            perimeter_m,
        })
    }

    /// Length of each of the three equal sides.
    pub fn side_length_m(&self) -> f64 {
        self.perimeter_m / 3.0
    }

    /// Vertical distance from the base to the apex when the base is level.
    pub fn height_m(&self) -> f64 {
        self.side_length_m() * (PI / 3.0).sin()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 1e-9,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn default_factors_match_classic_formulas() {
        for f in [1.84, 3.6, 7.1, 14.175, 28.4, 50.125] {
            let dipole = Dipole::new(f, DIPOLE_K_FACTOR).unwrap();
            assert_close(meters_to_feet(dipole.total_length_m), 468.0 / f);

            let delta = DeltaLoop::new(f, LOOP_K_FACTOR).unwrap();
            assert_close(meters_to_feet(delta.perimeter_m), 1005.0 / f);
        }
    }

    #[test]
    fn wavelength_is_c_over_f() {
        assert_close(wavelength_m(299.792458).unwrap(), 1.0);
        assert_close(wavelength_m(14.0).unwrap(), 21.413747);
    }

    #[test]
    fn dipole_legs_are_half_the_total() {
        let dipole = Dipole::new(7.1, DIPOLE_K_FACTOR).unwrap();
        assert_close(dipole.leg_length_m() * 2.0, dipole.total_length_m);
    }

    #[test]
    fn k_factor_scales_free_space_half_wave() {
        let dipole = Dipole::new(10.0, 1.0).unwrap();
        assert_close(dipole.total_length_m, 14.9896229);
    }

    #[test]
    fn ocf_legs_split_by_offset() {
        let ocf = OffCenterFedDipole::new(3.6, OCF_FEED_OFFSET, DIPOLE_K_FACTOR).unwrap();
        let dipole = Dipole::new(3.6, DIPOLE_K_FACTOR).unwrap();
        assert_close(ocf.total_length_m, dipole.total_length_m);
        assert_close(ocf.short_leg_length_m(), ocf.total_length_m / 3.0);
        assert_close(ocf.long_leg_length_m(), ocf.total_length_m * 2.0 / 3.0);
    }

    #[test]
    fn ocf_at_half_offset_is_center_fed() {
        let ocf = OffCenterFedDipole::new(14.2, 0.5, DIPOLE_K_FACTOR).unwrap();
        assert_close(ocf.short_leg_length_m(), ocf.long_leg_length_m());
    }

    #[test]
    fn ocf_rejects_out_of_range_offsets() {
        for offset in [0.0, -0.1, 0.51, 1.0, f64::NAN] {
            let err = OffCenterFedDipole::new(7.1, offset, DIPOLE_K_FACTOR).unwrap_err();
            assert!(matches!(err, AntennaError::FeedOffsetOutOfRange(_)));
        }
    }

    #[test]
    fn delta_loop_geometry() {
        let delta = DeltaLoop::new(14.2, LOOP_K_FACTOR).unwrap();
        assert_close(delta.side_length_m() * 3.0, delta.perimeter_m);
        assert_close(delta.height_m(), delta.side_length_m() * 3f64.sqrt() / 2.0);
    }

    #[test]
    fn rejects_bad_frequency_and_k_factor() {
        for bad in [0.0, -7.1, f64::NAN, f64::INFINITY] {
            assert!(matches!(
                Dipole::new(bad, DIPOLE_K_FACTOR),
                Err(AntennaError::NotPositive {
                    name: "frequency_mhz",
                    ..
                })
            ));
            assert!(matches!(
                DeltaLoop::new(14.2, bad),
                Err(AntennaError::NotPositive {
                    name: "k_factor",
                    ..
                })
            ));
        }
    }
}
