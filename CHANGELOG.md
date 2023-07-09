# Changelog

## Unreleased

* bors bot replaced with GH merge queue
* Rename method `set_interrupt_line_config` to `select_interrupt_line_1` [#27]

## [v0.1.2] 2022-09-13

* Fix DLC field (frame length) for FDCAN frames [#21]

## [v0.1.1] 2022-07-27

* Fix mask for STANDARD ID, was applied incorrectly

## [v0.1.0] 2022-03-18

Initial release.

Callbacks replaced with parameter buffer #10

[Unreleased]: https://github.com/stm32-rs/fdcan/compare/v0.1.2...HEAD
[v0.1.2]: https://github.com/stm32-rs/stm32h7xx-hal/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/stm32-rs/stm32h7xx-hal/compare/v0.1.0...v0.1.1

[#18]: https://github.com/stm32-rs/stm32h7xx-hal/pull/18
[#21]: https://github.com/stm32-rs/stm32h7xx-hal/pull/21
[#27]: https://github.com/stm32-rs/stm32h7xx-hal/pull/27
