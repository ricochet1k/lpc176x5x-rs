# lpc176x5x-rs

Rust device crate for the LPC176x and LPC175x family of ARM Cortex-M3
microcontrollers. The code is generated automatically from the [SVD
file](http://ds.arm.com/media/resources/db/chip/nxp/lpc1768/LPC176x5x.svd)
using [svd2rust](https://crates.io/crates/svd2rust).

## Creating The Bindings

The SVD file contained some duplicate definitions and some other errors which
needed to be fixed.

```bash
cargo install svd2rust form
svd2rust -i LPC176x5x.svd
rm -rf src
form -i lib.rs -o src/ && rm lib.rs
cargo fmt
```
