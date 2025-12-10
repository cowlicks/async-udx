# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Added

Added ability to create a "half-open" - preventing messages with the half-open stream id from being routed as socket messages.
The JavaScript implementation does something similar.

### Changed

`UdxSocket::bind` is no longer async. It didn't need to be, but it does need have a tokio runtime running.

Fixes a bug where we were getting the socket address of sender wrong

### Removed


<!-- next-url -->
[Unreleased]: https://github.com/datrs/async-udx/compare/v0.1.0...HEAD
