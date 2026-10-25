"""Wire antenna length calculations implemented in Rust."""

from typing import Final, final

__all__ = [
    "DIPOLE_K_FACTOR",
    "LOOP_K_FACTOR",
    "OCF_FEED_OFFSET",
    "DeltaLoop",
    "Dipole",
    "OffCenterFedDipole",
    "wavelength_ft",
    "wavelength_m",
]

DIPOLE_K_FACTOR: Final[float]
"""Default dipole factor; reproduces the classic 468 / f(MHz) feet formula."""

LOOP_K_FACTOR: Final[float]
"""Default full-wave loop factor; reproduces the classic 1005 / f(MHz) feet formula."""

OCF_FEED_OFFSET: Final[float]
"""Default off-center feed position (1/3 of the total length from one end)."""

def wavelength_m(frequency_mhz: float) -> float:
    """Free-space wavelength in meters for a frequency in MHz."""

def wavelength_ft(frequency_mhz: float) -> float:
    """Free-space wavelength in feet for a frequency in MHz."""

@final
class Dipole:
    """Half-wave, center-fed dipole with two equal legs, fed with a 1:1 balun."""

    def __new__(cls, frequency_mhz: float, *, k_factor: float = ...) -> Dipole: ...
    @property
    def frequency_mhz(self) -> float: ...
    @property
    def k_factor(self) -> float: ...
    @property
    def total_length_m(self) -> float: ...
    @property
    def total_length_ft(self) -> float: ...
    @property
    def leg_length_m(self) -> float: ...
    @property
    def leg_length_ft(self) -> float: ...

@final
class OffCenterFedDipole:
    """Off-center fed (Windom style) half-wave dipole with a short and a long leg."""

    def __new__(
        cls,
        frequency_mhz: float,
        feed_offset: float = ...,
        *,
        k_factor: float = ...,
    ) -> OffCenterFedDipole: ...
    @property
    def frequency_mhz(self) -> float: ...
    @property
    def k_factor(self) -> float: ...
    @property
    def feed_offset(self) -> float: ...
    @property
    def total_length_m(self) -> float: ...
    @property
    def total_length_ft(self) -> float: ...
    @property
    def short_leg_length_m(self) -> float: ...
    @property
    def short_leg_length_ft(self) -> float: ...
    @property
    def long_leg_length_m(self) -> float: ...
    @property
    def long_leg_length_ft(self) -> float: ...

@final
class DeltaLoop:
    """Full-wave delta loop shaped as an equilateral triangle."""

    def __new__(cls, frequency_mhz: float, *, k_factor: float = ...) -> DeltaLoop: ...
    @property
    def frequency_mhz(self) -> float: ...
    @property
    def k_factor(self) -> float: ...
    @property
    def perimeter_m(self) -> float: ...
    @property
    def perimeter_ft(self) -> float: ...
    @property
    def side_length_m(self) -> float: ...
    @property
    def side_length_ft(self) -> float: ...
    @property
    def height_m(self) -> float: ...
    @property
    def height_ft(self) -> float: ...
