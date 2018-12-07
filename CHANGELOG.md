# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.11.0] - 2018-12-07

### Changed

- [breaking-change] re-generated using svd2rust v0.14.0. The `interrupt` macro
  has become an attribute.

## [v0.10.0] - 2018-05-12

- [breaking-change] re-generated using svd2rust v0.13.0. This crate now compiles on stable.

- [breaking-change] the minor versions of the cortex-m, cortex-m-rt and bare-metal dependencies have
  been increased.

### Changed

## [v0.9.1] - 2018-05-03

### Changed

- Re-generate using svd2rust HEAD

## [v0.9.0] - 2018-04-14

### Changed

- Bumped the `cortex-m-rt` dependency to v0.4.0

## [v0.8.0] - 2018-01-15

### Added

- enumeratedValues information to some DMA, SPI, I2C and RCC registers

### Changed

- [breaking-change] Re-generated using svd2rust v0.12.0
- bumped the version of the bare-metal dependency to v0.1.1

### Fixed

- Typo in some TIM1 registers

## [v0.7.5] - 2017-09-08

### Added

- Some `<enumeratedValues>` information to USB.

### Changed

- Re-generated using svd2rust v0.11.4

## [v0.7.4] - 2017-08-24

### Added

- Some `<enumeratedValues>` information to RCC.

## [v0.7.3] - 2017-08-01

### Changed

- Re-generated using svd2rust v0.11.3

### Fixed

- Overrides of interrupt handles were being ignored if LTO was not enabled.

## [v0.7.2] - 2017-07-21 - YANKED

### Changed

- Re-generated using svd2rust v0.11.2

## [v0.7.1] - 2017-07-07

### Changed

- Re-generated using svd2rust v0.11.1

## [v0.7.0] - 2017-07-07

### Changed

- [breaking-change] Re-generated using svd2rust [v0.11.0][svd2rust-11].

[svd2rust-11]: https://github.com/japaric/svd2rust/blob/master/CHANGELOG.md#v0110---2017-07-07

## [v0.6.1] - 2017-06-05

### Fixed

- Re-generated using svd2rust v0.9.1. The types of core peripheral register
  blocks are now uppercase as well.

## [v0.6.0] - 2017-06-05 - YANKED

### Changed

- [breaking-change] Re-generated using svd2rust v0.9.0. The types of
  peripherals, registers and bitfields are now normalized to uppercase.

## [v0.5.0] - 2017-06-02

### Added

- enumeratedValues information to timers.

### Changed

- [breaking-change] Re-generated using svd2rust v0.8.1. The API of 1-bit fields
  changed.

## [v0.4.1] - 2017-05-10

### Changed

- Re-generated using svd2rust v0.7.2.

## [v0.4.0] - 2017-04-25

### Changed

- [breaking-change] Re-generated using svd2rust v0.7.0. NVIC API changed.

### Added

- API for the rest of the core peripherals. This API is just a re-export of the
  cortex-m one.

## [v0.3.1] - 2017-04-15

### Changed

- [breaking-change] Re-generated using svd2rust v0.6.1

## [v0.3.0] - 2017-04-11

### Changed

- [breaking-change] Re-generated using svd2rust v0.6.0

## [v0.2.0] - 2017-03-27

### Changed

- [breaking-change] Re-generated using svd2rust v0.5.0

## v0.1.0 - 2017-03-12

- Initial release

[Unreleased]: https://github.com/japaric/stm32f103xx/compare/v0.11.0...HEAD
[v0.11.0]: https://github.com/japaric/stm32f103xx/compare/v0.10.0...v0.11.0
[v0.10.0]: https://github.com/japaric/stm32f103xx/compare/v0.9.1...v0.10.0
[v0.9.1]: https://github.com/japaric/stm32f103xx/compare/v0.9.0...v0.9.1
[v0.9.0]: https://github.com/japaric/stm32f103xx/compare/v0.8.0...v0.9.0
[v0.8.0]: https://github.com/japaric/stm32f103xx/compare/v0.7.5...v0.8.0
[v0.7.4]: https://github.com/japaric/stm32f103xx/compare/v0.7.4...v0.7.5
[v0.7.4]: https://github.com/japaric/stm32f103xx/compare/v0.7.3...v0.7.4
[v0.7.3]: https://github.com/japaric/stm32f103xx/compare/v0.7.2...v0.7.3
[v0.7.2]: https://github.com/japaric/stm32f103xx/compare/v0.7.1...v0.7.2
[v0.7.1]: https://github.com/japaric/stm32f103xx/compare/v0.7.0...v0.7.1
[v0.7.0]: https://github.com/japaric/stm32f103xx/compare/v0.6.1...v0.7.0
[v0.6.1]: https://github.com/japaric/stm32f103xx/compare/v0.6.0...v0.6.1
[v0.6.0]: https://github.com/japaric/stm32f103xx/compare/v0.5.0...v0.6.0
[v0.5.0]: https://github.com/japaric/stm32f103xx/compare/v0.4.1...v0.5.0
[v0.4.1]: https://github.com/japaric/stm32f103xx/compare/v0.4.0...v0.4.1
[v0.4.0]: https://github.com/japaric/stm32f103xx/compare/v0.3.1...v0.4.0
[v0.3.1]: https://github.com/japaric/stm32f103xx/compare/v0.3.0...v0.3.1
[v0.3.0]: https://github.com/japaric/stm32f103xx/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/japaric/stm32f103xx/compare/v0.1.0...v0.2.0
