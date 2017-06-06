# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.6.0] - 2017-06-05

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

[Unreleased]: https://github.com/japaric/stm32f103xx/compare/v0.6.0...HEAD
[v0.6.0]: https://github.com/japaric/stm32f103xx/compare/v0.5.0...v0.6.0
[v0.5.0]: https://github.com/japaric/stm32f103xx/compare/v0.4.1...v0.5.0
[v0.4.1]: https://github.com/japaric/stm32f103xx/compare/v0.4.0...v0.4.1
[v0.4.0]: https://github.com/japaric/stm32f103xx/compare/v0.3.1...v0.4.0
[v0.3.1]: https://github.com/japaric/stm32f103xx/compare/v0.3.0...v0.3.1
[v0.3.0]: https://github.com/japaric/stm32f103xx/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/japaric/stm32f103xx/compare/v0.1.0...v0.2.0
