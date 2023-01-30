# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

[Unreleased]: https://github.com/Nugine/rlimit/v0.9.1...HEAD

## [0.9.1] - 2023-01-30

[0.9.1]: https://github.com/Nugine/rlimit/compare/v0.9.0...v0.9.1

+ [PR #46](https://github.com/Nugine/rlimit/pull/46): `prlimit` and `ProcLimits` are available on Android.

## [0.9.0] - 2022-12-28

[0.9.0]: https://github.com/Nugine/rlimit/compare/v0.8.3...v0.9.0

+ rlimit v0.9.0 follows the latest libc definitions.
+ The MSRV of v0.9.* is explicitly guaranteed to be 1.59.0.

## [0.8.3] - 2022-04-06

[0.8.3]: https://github.com/Nugine/rlimit/compare/v0.8.2...v0.8.3

[PR #43](https://github.com/Nugine/rlimit/pull/43): Downgrade MSRV

## [0.8.2] - 2022-04-06

[0.8.2]: https://github.com/Nugine/rlimit/compare/v0.8.1...v0.8.2

rlimit v0.8.2 uses libc definitions again instead of incorrect custom bindings.

rlimit v0.8.0 and v0.8.1 are yanked now.

## [0.8.1] - 2022-04-01

[0.8.1]: https://github.com/Nugine/rlimit/compare/v0.8.0...v0.8.1

[PR #36](https://github.com/Nugine/rlimit/pull/36): Fix the bindings for aarch64-apple-darwin.

## [0.8.0] - 2022-03-31

[0.8.0]: https://github.com/Nugine/rlimit/compare/v0.7.0...v0.8.0

rlimit v0.8.0 uses custom ffi bindings instead of libc for rlimit symbols and constants. The custom bindings are kept in sync with system headers automatically.

All resource constants are available on all unix platforms.
Passing an unsupported resource to `[set|get|p]rlimit` will result in a custom IO error.

### Added

+ `Resource::is_supported`

### Changed

+ `Resource::as_raw` is a private method now.

### Removed

+ `Resource::available_names`
+ `Resource::available_resources`
+ `RawResource`

## [0.7.0] - 2022-02-13

[0.7.0]: https://github.com/Nugine/rlimit/compare/v0.6.2...v0.7.0

### Added

+ Windows support
  + [rlimit::getmaxstdio](https://docs.rs/rlimit/0.7.0/rlimit/fn.getmaxstdio.html)
  + [rlimit::setmaxstdio](https://docs.rs/rlimit/0.7.0/rlimit/fn.stdmaxstdio.html)

### Changed

+ [rlimit::utils::increase_nofile_limit] in v0.6.2 has been moved to [rlimit::increase_nofile_limit].

[rlimit::utils::increase_nofile_limit]: https://docs.rs/rlimit/0.6.2/rlimit/utils/fn.increase_nofile_limit.html

[rlimit::increase_nofile_limit]: https://docs.rs/rlimit/0.7.0/rlimit/fn.increase_nofile_limit.html

### Removed

+ [rlimit::utils::get_kern_max_files_per_proc] has been removed from public interfaces.

[rlimit::utils::get_kern_max_files_per_proc]: https://docs.rs/rlimit/0.6.2/x86_64-apple-darwin/rlimit/utils/fn.get_kern_max_files_per_proc.html
