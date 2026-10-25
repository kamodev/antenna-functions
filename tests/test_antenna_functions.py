import math

import pytest

import antenna_functions as af

FREQUENCIES_MHZ = [1.84, 3.6, 7.1, 10.125, 14.175, 18.1, 21.2, 24.94, 28.4, 50.125]


@pytest.mark.parametrize("freq", FREQUENCIES_MHZ)
def test_dipole_matches_468_over_f(freq):
    d = af.Dipole(freq)
    assert d.total_length_ft == pytest.approx(468.0 / freq)
    assert d.leg_length_ft == pytest.approx(234.0 / freq)
    assert d.total_length_m == pytest.approx(468.0 * 0.3048 / freq)
    assert d.leg_length_m * 2 == pytest.approx(d.total_length_m)


def test_dipole_custom_k_factor():
    d = af.Dipole(10.0, k_factor=1.0)
    # Free-space half wavelength at 10 MHz.
    assert d.total_length_m == pytest.approx(299.792458 / 10.0 / 2)
    assert d.k_factor == 1.0
    assert af.Dipole(10.0).k_factor == af.DIPOLE_K_FACTOR


@pytest.mark.parametrize("freq", FREQUENCIES_MHZ)
def test_ocf_default_split_is_one_third(freq):
    o = af.OffCenterFedDipole(freq)
    assert o.feed_offset == pytest.approx(1 / 3)
    assert o.total_length_ft == pytest.approx(468.0 / freq)
    assert o.short_leg_length_m == pytest.approx(o.total_length_m / 3)
    assert o.long_leg_length_m == pytest.approx(o.total_length_m * 2 / 3)
    assert o.short_leg_length_ft + o.long_leg_length_ft == pytest.approx(o.total_length_ft)


def test_ocf_custom_offset():
    o = af.OffCenterFedDipole(3.6, 0.36)
    assert o.short_leg_length_m == pytest.approx(o.total_length_m * 0.36)
    assert o.long_leg_length_m == pytest.approx(o.total_length_m * 0.64)
    assert af.OffCenterFedDipole(3.6, feed_offset=0.36).short_leg_length_m == o.short_leg_length_m


def test_ocf_half_offset_equals_center_fed_dipole():
    o = af.OffCenterFedDipole(14.2, 0.5)
    d = af.Dipole(14.2)
    assert o.short_leg_length_m == pytest.approx(d.leg_length_m)
    assert o.long_leg_length_m == pytest.approx(d.leg_length_m)


@pytest.mark.parametrize("offset", [0.0, -0.2, 0.6, 1.0, math.nan])
def test_ocf_rejects_bad_offset(offset):
    with pytest.raises(ValueError, match="feed_offset"):
        af.OffCenterFedDipole(7.1, offset)


@pytest.mark.parametrize("freq", FREQUENCIES_MHZ)
def test_delta_loop_matches_1005_over_f(freq):
    loop = af.DeltaLoop(freq)
    assert loop.perimeter_ft == pytest.approx(1005.0 / freq)
    assert loop.side_length_ft == pytest.approx(335.0 / freq)
    assert loop.side_length_m * 3 == pytest.approx(loop.perimeter_m)
    assert loop.height_m == pytest.approx(loop.side_length_m * math.sqrt(3) / 2)
    assert loop.height_ft == pytest.approx(loop.side_length_ft * math.sqrt(3) / 2)


def test_wavelength():
    assert af.wavelength_m(299.792458) == pytest.approx(1.0)
    assert af.wavelength_ft(14.0) == pytest.approx(299.792458 / 14.0 / 0.3048)


@pytest.mark.parametrize("cls", [af.Dipole, af.OffCenterFedDipole, af.DeltaLoop])
@pytest.mark.parametrize("freq", [0.0, -14.2, math.inf, math.nan])
def test_rejects_bad_frequency(cls, freq):
    with pytest.raises(ValueError, match="frequency_mhz"):
        cls(freq)


@pytest.mark.parametrize("cls", [af.Dipole, af.OffCenterFedDipole, af.DeltaLoop])
def test_rejects_bad_k_factor(cls):
    with pytest.raises(ValueError, match="k_factor"):
        cls(14.2, k_factor=0)


def test_k_factor_is_keyword_only():
    with pytest.raises(TypeError):
        af.Dipole(14.2, 0.95)


def test_results_are_read_only():
    d = af.Dipole(14.2)
    with pytest.raises(AttributeError):
        d.total_length_m = 1.0


def test_repr_shows_both_units():
    text = repr(af.Dipole(14.2))
    assert text.startswith("<Dipole 14.2 MHz")
    assert " m (" in text and " ft)" in text
    assert repr(af.OffCenterFedDipole(7.1)).startswith("<OffCenterFedDipole 7.1 MHz, feed at 33.3 %")
    assert repr(af.DeltaLoop(28.4)).startswith("<DeltaLoop 28.4 MHz")
