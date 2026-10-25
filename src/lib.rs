//! Python bindings for the wire antenna calculations in [`calc`].

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

pub mod calc;

use calc::{meters_to_feet, AntennaError};

impl From<AntennaError> for PyErr {
    fn from(err: AntennaError) -> Self {
        PyValueError::new_err(err.to_string())
    }
}

/// Half-wave, center-fed dipole with two equal legs, fed with a 1:1 balun.
///
/// Args:
///     frequency_mhz: Design frequency in MHz.
///     k_factor: Multiplier applied to the free-space half wavelength. The
///         default, DIPOLE_K_FACTOR, gives the classic 468 / f(MHz) feet.
#[pyclass(frozen, module = "antenna_functions")]
pub struct Dipole(calc::Dipole);

#[pymethods]
impl Dipole {
    #[new]
    #[pyo3(signature = (frequency_mhz, *, k_factor = calc::DIPOLE_K_FACTOR))]
    fn new(frequency_mhz: f64, k_factor: f64) -> PyResult<Self> {
        Ok(Self(calc::Dipole::new(frequency_mhz, k_factor)?))
    }

    /// Design frequency in MHz.
    #[getter]
    fn frequency_mhz(&self) -> f64 {
        self.0.frequency_mhz
    }

    /// Multiplier applied to the free-space half wavelength.
    #[getter]
    fn k_factor(&self) -> f64 {
        self.0.k_factor
    }

    /// End-to-end wire length in meters.
    #[getter]
    fn total_length_m(&self) -> f64 {
        self.0.total_length_m
    }

    /// End-to-end wire length in feet.
    #[getter]
    fn total_length_ft(&self) -> f64 {
        meters_to_feet(self.0.total_length_m)
    }

    /// Length of each leg (feed point to wire end) in meters.
    #[getter]
    fn leg_length_m(&self) -> f64 {
        self.0.leg_length_m()
    }

    /// Length of each leg (feed point to wire end) in feet.
    #[getter]
    fn leg_length_ft(&self) -> f64 {
        meters_to_feet(self.0.leg_length_m())
    }

    fn __repr__(&self) -> String {
        format!(
            "<Dipole {} MHz: total {:.3} m ({:.3} ft), legs {:.3} m ({:.3} ft)>",
            self.0.frequency_mhz,
            self.total_length_m(),
            self.total_length_ft(),
            self.leg_length_m(),
            self.leg_length_ft(),
        )
    }
}

/// Off-center fed (Windom style) half-wave dipole with a short and a long leg.
///
/// Args:
///     frequency_mhz: Design (lowest) frequency in MHz.
///     feed_offset: Fraction of the total length between the nearest end and
///         the feed point, in (0, 0.5]. Defaults to 1/3, the classic
///         33 % / 67 % split; 0.5 is an ordinary center-fed dipole.
///     k_factor: Multiplier applied to the free-space half wavelength. The
///         default, DIPOLE_K_FACTOR, gives the classic 468 / f(MHz) feet.
#[pyclass(frozen, module = "antenna_functions")]
pub struct OffCenterFedDipole(calc::OffCenterFedDipole);

#[pymethods]
impl OffCenterFedDipole {
    #[new]
    #[pyo3(signature = (
        frequency_mhz,
        feed_offset = calc::OCF_FEED_OFFSET,
        *,
        k_factor = calc::DIPOLE_K_FACTOR,
    ))]
    fn new(frequency_mhz: f64, feed_offset: f64, k_factor: f64) -> PyResult<Self> {
        Ok(Self(calc::OffCenterFedDipole::new(
            frequency_mhz,
            feed_offset,
            k_factor,
        )?))
    }

    /// Design (lowest) frequency in MHz.
    #[getter]
    fn frequency_mhz(&self) -> f64 {
        self.0.frequency_mhz
    }

    /// Multiplier applied to the free-space half wavelength.
    #[getter]
    fn k_factor(&self) -> f64 {
        self.0.k_factor
    }

    /// Fraction of the total length between the nearest end and the feed point.
    #[getter]
    fn feed_offset(&self) -> f64 {
        self.0.feed_offset
    }

    /// End-to-end wire length in meters.
    #[getter]
    fn total_length_m(&self) -> f64 {
        self.0.total_length_m
    }

    /// End-to-end wire length in feet.
    #[getter]
    fn total_length_ft(&self) -> f64 {
        meters_to_feet(self.0.total_length_m)
    }

    /// Short leg (feed point to nearest end) in meters.
    #[getter]
    fn short_leg_length_m(&self) -> f64 {
        self.0.short_leg_length_m()
    }

    /// Short leg (feed point to nearest end) in feet.
    #[getter]
    fn short_leg_length_ft(&self) -> f64 {
        meters_to_feet(self.0.short_leg_length_m())
    }

    /// Long leg (feed point to farthest end) in meters.
    #[getter]
    fn long_leg_length_m(&self) -> f64 {
        self.0.long_leg_length_m()
    }

    /// Long leg (feed point to farthest end) in feet.
    #[getter]
    fn long_leg_length_ft(&self) -> f64 {
        meters_to_feet(self.0.long_leg_length_m())
    }

    fn __repr__(&self) -> String {
        format!(
            "<OffCenterFedDipole {} MHz, feed at {:.1} %: total {:.3} m ({:.3} ft), \
             short leg {:.3} m ({:.3} ft), long leg {:.3} m ({:.3} ft)>",
            self.0.frequency_mhz,
            self.0.feed_offset * 100.0,
            self.total_length_m(),
            self.total_length_ft(),
            self.short_leg_length_m(),
            self.short_leg_length_ft(),
            self.long_leg_length_m(),
            self.long_leg_length_ft(),
        )
    }
}

/// Full-wave delta loop shaped as an equilateral triangle.
///
/// Args:
///     frequency_mhz: Design frequency in MHz.
///     k_factor: Multiplier applied to the free-space wavelength. The default,
///         LOOP_K_FACTOR, gives the classic 1005 / f(MHz) feet perimeter.
#[pyclass(frozen, module = "antenna_functions")]
pub struct DeltaLoop(calc::DeltaLoop);

#[pymethods]
impl DeltaLoop {
    #[new]
    #[pyo3(signature = (frequency_mhz, *, k_factor = calc::LOOP_K_FACTOR))]
    fn new(frequency_mhz: f64, k_factor: f64) -> PyResult<Self> {
        Ok(Self(calc::DeltaLoop::new(frequency_mhz, k_factor)?))
    }

    /// Design frequency in MHz.
    #[getter]
    fn frequency_mhz(&self) -> f64 {
        self.0.frequency_mhz
    }

    /// Multiplier applied to the free-space wavelength.
    #[getter]
    fn k_factor(&self) -> f64 {
        self.0.k_factor
    }

    /// Total wire length around the loop in meters.
    #[getter]
    fn perimeter_m(&self) -> f64 {
        self.0.perimeter_m
    }

    /// Total wire length around the loop in feet.
    #[getter]
    fn perimeter_ft(&self) -> f64 {
        meters_to_feet(self.0.perimeter_m)
    }

    /// Length of each of the three sides in meters.
    #[getter]
    fn side_length_m(&self) -> f64 {
        self.0.side_length_m()
    }

    /// Length of each of the three sides in feet.
    #[getter]
    fn side_length_ft(&self) -> f64 {
        meters_to_feet(self.0.side_length_m())
    }

    /// Base-to-apex height in meters.
    #[getter]
    fn height_m(&self) -> f64 {
        self.0.height_m()
    }

    /// Base-to-apex height in feet.
    #[getter]
    fn height_ft(&self) -> f64 {
        meters_to_feet(self.0.height_m())
    }

    fn __repr__(&self) -> String {
        format!(
            "<DeltaLoop {} MHz: perimeter {:.3} m ({:.3} ft), sides {:.3} m ({:.3} ft), \
             height {:.3} m ({:.3} ft)>",
            self.0.frequency_mhz,
            self.perimeter_m(),
            self.perimeter_ft(),
            self.side_length_m(),
            self.side_length_ft(),
            self.height_m(),
            self.height_ft(),
        )
    }
}

/// Free-space wavelength in meters for a frequency in MHz.
#[pyfunction]
fn wavelength_m(frequency_mhz: f64) -> PyResult<f64> {
    Ok(calc::wavelength_m(frequency_mhz)?)
}

/// Free-space wavelength in feet for a frequency in MHz.
#[pyfunction]
fn wavelength_ft(frequency_mhz: f64) -> PyResult<f64> {
    Ok(meters_to_feet(calc::wavelength_m(frequency_mhz)?))
}

/// Wire antenna length calculations implemented in Rust.
///
/// Classes:
///     Dipole: half-wave center-fed dipole (1:1 balun).
///     OffCenterFedDipole: off-center fed (Windom style) dipole.
///     DeltaLoop: full-wave equilateral delta loop.
///
/// All frequencies are in MHz. Every result is available in meters (``*_m``)
/// and feet (``*_ft``).
#[pymodule]
mod antenna_functions {
    #[pymodule_export]
    use super::{wavelength_ft, wavelength_m, DeltaLoop, Dipole, OffCenterFedDipole};

    #[pymodule_export]
    const DIPOLE_K_FACTOR: f64 = super::calc::DIPOLE_K_FACTOR;

    #[pymodule_export]
    const LOOP_K_FACTOR: f64 = super::calc::LOOP_K_FACTOR;

    #[pymodule_export]
    const OCF_FEED_OFFSET: f64 = super::calc::OCF_FEED_OFFSET;
}
