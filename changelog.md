# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- Embed the Python API module in the Rust crate so Python process implementations no longer require `PYTHONPATH` or a separate `anysystem.py` file at runtime.

### Fixed

- Restore docs.rs builds on current nightly Rust.

## 0.2.0 (2026-08-18)

### Added

- Add the `Process::on_start` callback for Rust and Python processes.
- Add `Message.get` and support `key in message` for Python messages.

### Changed

- Preserve insertion order in `System::process_names` and execution order in test results and failure output.
- Upgrade PyO3 from 0.19 to 0.29 and update the Python bindings for the new API.
- Align the Rust ping-pong example constructors with their Python counterparts.
- Update project URLs for the repository move to `osukhoroslov/anysystem`.
- Modernize the Rust toolchain, GitHub Actions, and dependency license checks used by CI.

### Breaking

- Make the Python `Process` class an enforced abstract base class and require implementations to define `on_start`.
- Change `TestSuite::run` to return an `IndexMap` instead of a `BTreeMap`.
- Change `PyProcessFactory::build` to use PyO3's `PyCallArgs` argument interface.

## 0.1.2 (2025-09-13)

### Fixed

- Improve calculation of inner data size for Python processes.
- Fix Clippy warnings.

## 0.1.1 (2024-10-05)

### Changed

- `TestSuite::run` returns the test results instead of terminating the process so that further processing of the results can be done.

## 0.1.0 (2024-07-28)

### Added

- Initial release based on code moved from DSLab project.
