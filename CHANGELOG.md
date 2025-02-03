# Changelog

## [Unreleased]

* Update MSRV to 1.60
* Allow timestamp to be accessed in all modes
* Implement transceiver delay compensation configuration

## [v0.2.1] 2024-09-04

* Bugfix: Fix transmit pause configuration [#41]
* Update depreciated method `set_interrupt_line_config` to `select_interrupt_line_1` [#46]

## [v0.2.0] 2023-07-09

* **Breaking:** Rename method `set_interrupt_line_config` to `select_interrupt_line_1` [#27]
* bors bot replaced with GH merge queue
* Update txbuffer element.rs : to data length [#23]
* Expose protocol status and error properties [#25]
* Fix undefined behaviour in write_mailbox. [#32]
* Use volatile reads and writes for rx and tx buffers [#34]
* message_ram: fix SFIDx read/write mask [#36]

## [v0.1.2] 2022-09-13

* Fix DLC field (frame length) for FDCAN frames [#21]

## [v0.1.1] 2022-07-27

* Fix mask for STANDARD ID, was applied incorrectly

## [v0.1.0] 2022-03-18

Initial release.

Callbacks replaced with parameter buffer #10

[Unreleased]: https://github.com/stm32-rs/fdcan/compare/v0.2.1...HEAD
[v0.2.1]: https://github.com/stm32-rs/fdcan/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/stm32-rs/fdcan/compare/v0.1.2...v0.2.0
[v0.1.2]: https://github.com/stm32-rs/fdcan/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/stm32-rs/fdcan/compare/v0.1.0...v0.1.1

[#18]: https://github.com/stm32-rs/fdcan/pull/18
[#21]: https://github.com/stm32-rs/fdcan/pull/21
[#23]: https://github.com/stm32-rs/fdcan/pull/23
[#25]: https://github.com/stm32-rs/fdcan/pull/25
[#27]: https://github.com/stm32-rs/fdcan/pull/27
[#32]: https://github.com/stm32-rs/fdcan/pull/32
[#34]: https://github.com/stm32-rs/fdcan/pull/34
[#36]: https://github.com/stm32-rs/fdcan/pull/36
[#41]: https://github.com/stm32-rs/fdcan/pull/41
[#46]: https://github.com/stm32-rs/fdcan/pull/46
