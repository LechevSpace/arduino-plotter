# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/LechevSpace/arduino-plotter/releases/tag/v0.1.0) - 2024-05-13

### Added
- *(protocol)* Improve docs and EoL
- Client/Server and better protocol implementation
- initial working protocol

### Fixed
- *(ci)* msrv - comment for default branch to `main`
- *(example)* run data lines
- rustfmt
- *(ci)* msrv and build workflows + set Rust msrv to 1.70
- *(Cargo.toml)* add msrv and rt-multi-thread feature for tokio dev-dep
- rustfmt

### Other
- README & lib.rs documentation + badges
- *(dependabot)* check for outdated actions
- fix build workflow - building docs should use nightly
- build and msrv workflows
- lib.rs crate attributes
- *(docs)* Server/Client docs improvements
- *(Cargo.toml)* Dependencies and package fields
- add rustfmt.toml
- fix README, docs and clean up
- *(examples)* add minimal usage example
- *(Cargo.toml)* add workspace key
- *(README)* add requirements, running the plotter app and the `run` example
- remove settings example
- *(Cargo)* Add more dependencies for:
