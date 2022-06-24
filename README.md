# EFM32PG (Pearl Gecko) support for Rust

[![PACs](https://github.com/efm32-rs/efm32pg-pacs/actions/workflows/pacs.yml/badge.svg)](https://github.com/efm32-rs/efm32pg-pacs/actions/workflows/pacs.yml)

This repository contains Peripheral Access Crates (PACs) for Silabs' EFM32 series of Cortex-M microcontrollers.

All these crates are automatically generated using [svd2rust](https://github.com/rust-embedded/svd2rust).

Refer to the [CHANGELOG](CHANGELOG.md) to see what changed in the last releases.

## Crates

Every EFM32PG chip has its own PAC, listed below:

| Crate               | Docs                                                                                         | crates.io                                                                                                         | target                      |
|---------------------|----------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------|-----------------------------|
| `efm32pg12b500-pac` | [![docs.rs](https://docs.rs/efm32pg12b500-pac/badge.svg)](https://docs.rs/efm32pg12b500-pac) | [![crates.io](https://img.shields.io/crates/d/efm32pg12b500-pac.svg)](https://crates.io/crates/efm32pg12b500-pac) | `thumbv7em-none-eabihf`     |
| `efm32pg1b100-pac`  | [![docs.rs](https://docs.rs/efm32pg1b100-pac/badge.svg)](https://docs.rs/efm32pg1b100-pac)   | [![crates.io](https://img.shields.io/crates/d/efm32pg1b100-pac.svg)](https://crates.io/crates/efm32pg1b100-pac)   | `thumbv7em-none-eabihf`     |
| `efm32pg1b200-pac`  | [![docs.rs](https://docs.rs/efm32pg1b200-pac/badge.svg)](https://docs.rs/efm32pg1b200-pac)   | [![crates.io](https://img.shields.io/crates/d/efm32pg1b200-pac.svg)](https://crates.io/crates/efm32pg1b200-pac)   | `thumbv7em-none-eabihf`     |
| `efm32pg22c200-pac` | _To be done_                                                                                 | _To be done_                                                                                                      | `thumbv8m.main-none-eabihf` |
| `efm32pg23b200-pac` | _To be done_                                                                                 | _To be done_                                                                                                      | `thumbv8m.main-none-eabihf` |
| `efm32pg23b210-pac` | _To be done_                                                                                 | _To be done_                                                                                                      | `thumbv8m.main-none-eabihf` |
| `efm32pg23b310-pac` | _To be done_                                                                                 | _To be done_                                                                                                      | `thumbv8m.main-none-eabihf` |

Process finished with exit code 0

## Device Reference Manuals from Silabs

**WIP**

## License

The included SVD files are sourced from https://www.silabs.com/documents/public/cmsis-packs and
are licensed under the Zlib (see [LICENSE-3RD-PARTY](LICENSE-3RD-PARTY-Zlib)).

The remainder of the code is under:

- 3-Clause BSD license ([LICENSE-3BSD](LICENSE-3BSD) or https://opensource.org/licenses/BSD-3-Clause)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the BSD-3-Clause license without any additional terms or conditions.