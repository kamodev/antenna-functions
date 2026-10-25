# antenna-functions

A Python module, written in Rust with [PyO3](https://pyo3.rs), that calculates wire lengths for
three common HF wire antennas:

| Antenna | Class | Default formula |
| --- | --- | --- |
| Half-wave dipole, center fed (1:1 balun) | `Dipole` | 468 / f(MHz) ft total |
| Off-center fed (Windom style) dipole | `OffCenterFedDipole` | 468 / f(MHz) ft total, fed 1/3 from one end |
| Full-wave equilateral delta loop | `DeltaLoop` | 1005 / f(MHz) ft perimeter |

Frequencies are in MHz. Every result is available in meters (`*_m`) and feet (`*_ft`).

## Install

You need a Rust toolchain (1.85+) and Python 3.9+. The wheel is built against the stable ABI, so
one build works for every Python version from 3.9 up.

```sh
pip install .
```

For development, install it into a virtual environment with [maturin](https://www.maturin.rs):

```sh
python -m venv .venv && source .venv/bin/activate
pip install maturin pytest
maturin develop --release
```

## Usage

```python
>>> import antenna_functions as af

>>> af.Dipole(7.1)
<Dipole 7.1 MHz: total 20.091 m (65.915 ft), legs 10.046 m (32.958 ft)>
>>> round(af.Dipole(7.1).leg_length_ft, 2)
32.96

>>> af.OffCenterFedDipole(3.6)
<OffCenterFedDipole 3.6 MHz, feed at 33.3 %: total 39.624 m (130.000 ft), short leg 13.208 m (43.333 ft), long leg 26.416 m (86.667 ft)>
>>> round(af.OffCenterFedDipole(3.6, 0.36).short_leg_length_m, 2)   # 36 % / 64 % split
14.26

>>> af.DeltaLoop(14.2)
<DeltaLoop 14.2 MHz: perimeter 21.572 m (70.775 ft), sides 7.191 m (23.592 ft), height 6.227 m (20.431 ft)>
```

### `Dipole(frequency_mhz, *, k_factor=DIPOLE_K_FACTOR)`

A half-wave dipole fed in the middle, so both legs are the same length. Fed with a 1:1 balun; the
feed point is roughly 50 to 75 ohms depending on height.

Attributes: `total_length_m/ft`, `leg_length_m/ft`, `frequency_mhz`, `k_factor`.

### `OffCenterFedDipole(frequency_mhz, feed_offset=1/3, *, k_factor=DIPOLE_K_FACTOR)`

The same total wire length as a half-wave dipole for `frequency_mhz` (the lowest band), but fed at
`feed_offset` of the total length from one end. The default 1/3 gives the classic 33 % / 67 % split;
other popular choices are 0.36 and 0.20. `feed_offset` must be in `(0, 0.5]`, and 0.5 is an ordinary
center-fed dipole. The off-center feed point is high impedance, so these are normally fed through a
4:1 or 6:1 balun.

Attributes: `total_length_m/ft`, `short_leg_length_m/ft`, `long_leg_length_m/ft`, `feed_offset`,
`frequency_mhz`, `k_factor`.

### `DeltaLoop(frequency_mhz, *, k_factor=LOOP_K_FACTOR)`

A one-wavelength loop shaped as an equilateral triangle. `height_m/ft` is the vertical distance from
a level base to the apex, which is useful for working out the mast height you need. A full-wave
loop presents about 100 to 125 ohms, commonly matched with a quarter-wave 75 ohm coax section or a
2:1 balun.

Attributes: `perimeter_m/ft`, `side_length_m/ft`, `height_m/ft`, `frequency_mhz`, `k_factor`.

### Helpers and constants

- `wavelength_m(frequency_mhz)`, `wavelength_ft(frequency_mhz)`: free-space wavelength.
- `DIPOLE_K_FACTOR` (about 0.9516): reproduces 468 / f ft for half-wave dipoles.
- `LOOP_K_FACTOR` (about 1.0218): reproduces 1005 / f ft for full-wave loops.
- `OCF_FEED_OFFSET` (1/3): default off-center feed position.

Invalid input (a frequency or `k_factor` that is zero, negative, NaN or infinite, or a
`feed_offset` outside `(0, 0.5]`) raises `ValueError`.

## How the lengths are calculated

All three start from the free-space wavelength, λ = c / f, scaled by a `k_factor`:

- Dipole and OCF dipole: total length = k × λ / 2. A wire half-wave dipole resonates a few percent
  shorter than λ / 2 because of end effect, and the default k reproduces the familiar 468 / f ft.
- Delta loop: perimeter = k × λ. A full-wave loop resonates slightly longer than λ, and the default
  k reproduces the familiar 1005 / f ft. Each side is a third of the perimeter.

These formulas are starting points. Height above ground, nearby objects, wire gauge and insulation
all shift resonance, so cut the wire a little long and trim to an SWR minimum. Insulated wire
usually needs to be a few percent shorter; pass a smaller `k_factor` to account for that.

## Using the calculations from Rust

The math lives in `src/calc.rs` with no Python dependency, and the crate also builds as an `rlib`:

```rust
use antenna_functions::calc::{Dipole, DIPOLE_K_FACTOR};

let dipole = Dipole::new(14.2, DIPOLE_K_FACTOR)?;
println!("each leg: {:.3} m", dipole.leg_length_m());
```

## Tests

```sh
cargo test          # Rust unit tests for the calculations
maturin develop     # build and install the module into the active venv
pytest              # Python tests for the bindings
```

## Releases

Pushing a version tag (for example `git tag v0.1.0 && git push origin v0.1.0`) runs
`.github/workflows/release.yml`. It runs the tests, builds wheels for Linux, macOS and Windows plus an
sdist, and publishes them as a GitHub Release for that tag.
