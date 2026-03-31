# Agent Guidelines for Rust Code Quality

This document provides guidelines for maintaining high-quality Rust code. These rules MUST be followed by all AI coding agents and contributors.

## Your Core Principles

All code you write **MUST** be fully optimized. Generally this includes:

- **ALWAYS** maximize algorithmic big-O efficiency for memory and runtime, it's OK to ask which is a higher priority.
- **ALWAYS** use parallelization and SIMD _where appropriate_ , it's OK to ask if _appropriate_ is not clear.
- **ALWAYS** following proper style conventions for Rust.
- **NEVER** add extra code beyond what is absolutely necessary to solve the problem the user provides.
  - **If** a crate can be imported to significantly reduce the amount of new code required to implement a function at optimal performance, and the crate itself is small and does not have much overhead, **ALWAYS** use the crate instead.
- **ALWAYS** ensure a safe crate supply-chain, using `cargo audit` and using crates with a higher usage among alternatives.
- **ALWAYS** consider readability and maintainability, add comments where specific decisions have been made in data structure, algorithm, or crate choices.

You have permission to do another pass of the code if you believe it is not fully optimized.

### Agent Behavior

- Always check if there is an MCP tool or skill available before performing operations manually.
- If in doubt, ask.

## Preferred Tools

Tool specification **MAY** be externalized to [mise](https://mise.jdx.dev) by providing a `mise.toml` file. A default configuration is included in [`mise.toml`](./mise.toml).

### Build

- Use `cargo` for project management, building, and dependency management.
- Use `cargo test` for running tests
- Use `cargo doc` for generating documentation
- **MUST** use `rustfmt` for code formatting
- **MUST** use `clippy` for linting and follow its suggestions
- **MUST** ensure code compiles with no warnings (use `-D warnings` flag in CI, not `#![deny(warnings)]` in source)

Configure the rust, doc, and clippy lints by adding the contents of @[`Cargo-lints.toml`](./Cargo-lints.toml) to the top-most `Cargo.toml` file.

### Continuous Integration/Deployment

TBD

### Logging/Tracing

- **MUST** use the Tokio `tracing` module.
- For top-level, or API, functions use the tracing `#[instrument]` attribute, requires the tracing feature `attributes`.
- For complex, or coordination functions either use `#[instrument]` or manual spans.
- When reporting errors to the console, use `tracing::error!` or `log::error!` instead of `println!`.
- For async code, **ALWAYS** use `tracing-futures`.
- For file-based logs **ALWAYS** use `tracing-appender`.
- For test cases depending on trace/log output **ALWAYS** use `test-log` to initialize tracing in tests.

**NEVER** log sensitive information; e.g. passwords, tokens, PII.

More complex telemetry should:

- **ALWAYS** use `opentelemetry` (and `opentelemetry_sdk`).
  - Use `tracing-opentelemetry` to emit trace data through Open Telemetry. Also, use `opentelemetry-stdout` to emit a local copy of the traces.
  - Use `opentelemetry_otlp` to connect to a sink.
- **ALWAYS** encapsulate initialization in a `telemetry.rs` private module.
  - provide a common `init(...)` function, which calls `init_logging`, `init_metrics`, and `init_tracing`.

### Additional Commons

- **ALWAYS** use the `chrono` crate for date handling/formatting.
  - Use `humantime` for date formatting for readability.
- For data analytics:
  - **ALWAYS** use `polars` instead of other data frame libraries for tabular data manipulation.
  - If a `polars` dataframe will be printed, **NEVER** simultaneously print the number of entries in the dataframe nor the schema as it is redundant.
  - **NEVER** ingest more than 10 rows of a data frame at a time. Only analyze subsets of data to avoid overloading your memory context.
- For human-readable output, use the `i18n-embed` and `i18n-embed-fl` crates to provide for localized text.

## Code Generation

### Code Style and Formatting

- **MUST** use meaningful, descriptive variable and function names.
- **MUST** follow Rust API Guidelines and idiomatic Rust conventions.
- **MUST** use 4 spaces for indentation, **NEVER** tabs.
- **MUST** use snake_case for functions/variables/modules, PascalCase for types/traits, SCREAMING_SNAKE_CASE for constants.
- Limit line length to 100 characters (rustfmt default).
- **MUST** avoid including redundant comments which are tautological or self-demonstating; e.g. cases where it is easily parsable what the code does at a glance or its function name giving sufficient information as to what the code does, so the comment does nothing other than waste user time.
- **MUST** avoid including comments which leak what this file contains, or leak the original user prompt, ESPECIALLY if it's irrelevant to the output code.

The project should include an empty `rustfmt.toml` file such as @[`rustfmt.toml`](./rustfmt.toml).

Additional best practices include:

- **NEVER** use `unsafe` unless absolutely necessary; document safety invariants when used.
- **MUST** call `.clone()` explicitly on non-`Copy` types; avoid hidden clones in closures and iterators.
- **MUST** use pattern matching exhaustively; avoid catch-all `_` patterns when possible.
- **MUST** use `format!` macro for string formatting.
- Use iterators and iterator adapters over manual loops.
- Use `enumerate()` instead of manual counter variables.
- Prefer `if let` and `while let` for single-pattern matching.
  - Use the `if let ... && validity_check` pattern in edition 2024 to reduce nested if statements.

### Documentation

- **MUST** include doc comments for all public functions, structs, enums, and methods
- **MUST** document function parameters, return values, and errors
- Keep comments up-to-date with code changes
- Include examples in doc comments for complex functions
- Keep README file examples up-to-date:
  - with the code examples from `lib.rs` for library packages; use ` ```rust no_run` for examples with placeholders.
  - with command-line invocations with the `--help` option; use ` ```bash` for examples with placeholders.

Include the content of @[`Cargo-docs.toml`](./Cargo-docs.toml) in each package's `Cargo.toml` file.

To include additional fonts, styles, or scripts, use the `rustdoc-args` key in the`docs` metadata in `Cargo.toml`.

### Error Handling

- **NEVER** use `.unwrap()` in production code paths.
- **MUST** use `Result<T, E>` for fallible operations.
- **MUST** use `std::convert::Infallible`, **NOT** `()` for traits/functions that require an error type.
- **MUST** use `thiserror` for defining error types and `anyhow` for application-level errors.
- **MUST** propagate errors with `?` operator where appropriate.
- Include the `#[must_use]` annotation for API functions.
- Provide meaningful error messages with context using `.context()` from `anyhow`.

See [Error Module](#error-module) for the recommended module structure/content.

### Imports and Dependencies

- **MUST** avoid wildcard imports (`use module::*`) except for preludes, test modules (`use super::*`), and prelude re-exports.
- **MUST** document dependencies in `Cargo.toml` with version constraints
- Use hierarchical `use` statements, e.g. there should be one top-level `std::{...}` statement.
- Use `rustfmt` to automate import formatting.

### Type System

- **MUST** leverage Rust's type system to prevent bugs at compile time.
- **NEVER** use `.unwrap()` in library code; use `.expect()` only for invariant violations with a descriptive message.
- Use _newtypes_ to distinguish semantically different values of the same underlying type.
  - Implement `AsRef` for the `Inner` type where appropriate.
  - Implement `From<NewType>`  for the `Inner` type where appropriate.
  - For String-like newtypes, implement `FromStr` as a constructor.
  - For String-like newtypes, provide a method `is_valid(s: &str) -> bool` which at the very least is implemented as `Self::from_str(s).is_ok()` but where possible provides a more optimized way to check validity.
    - Alternatively, implement the validity check in `is_valid` and implement `FromStr` as `if Self::is_valid(s) { ... } else ...`.
- Prefer `Option<T>` over sentinel values.

### Function Design

- **MUST** keep functions focused on a single responsibility.
- **MUST** prefer borrowing (`&T`, `&mut T`) over ownership _when possible_.
- Limit function parameters to 5 or fewer; use a config struct for more.
- Return early to reduce nesting.
- Use iterators and combinators over explicit loops where clearer.

### Struct and Enum Design

- **MUST** keep types focused on a single responsibility.
- **MUST** derive common traits: `Debug`, `Clone`, `PartialEq`, except when necessary.
- Use `#[derive(Default)]` when a sensible/preferred default exists.
- Prefer composition over inheritance-like patterns.
- Make fields private by default; provide accessor methods _when needed_.
- Primary constructor methods should be named `new`.
- Additional constructor method names should start with `new_` or `new_with_`.
- Optional fields should have construction methods of the form `fn with_NAME(mut self, NAME: NAME_TYPE) -> Self` allowing them to be chained after `new`.
- Use builder pattern for complex construction cases.
  - For a type named `MyType` provide a `MyTypeBuilder` struct.
  - Add a method `fn builder() -> MyTypeBuilder` to `MyType`.
  - Implement _either_ `From<MyTypeBuilder>`, or possibly `TryFrom<MyTypeBuilder>` for MyType.
  - Add a method `build(self) -> MyType` to `MyTypeBuilder`, which may be called by the `From` or `TryFrom` implementation.
- Enumerations _should_ provide an `is_VARIANT(&self) -> bool` method for each variant.
- Enumerations _should_ provide an `as_VARIANT(&self) -> Option<&VARIANT_TYPE>` method for each variant.

### Memory and Performance

- **MUST** avoid unnecessary allocations; prefer `&str` over `String` when possible.
- **MUST** use `Cow<'_, str>` when ownership is conditionally needed.
- Use `Vec::with_capacity()` when the size is known.
- Prefer stack allocation over heap when appropriate.
- Use `Arc` and `Rc` judiciously; prefer borrowing.

### Concurrency

- **MUST** use `Send` and `Sync` bounds appropriately.
- **MUST** prefer `tokio` for async runtime in async applications.
- **MUST** use `rayon` for CPU-bound parallelism.
- Avoid `Mutex` when `RwLock` where lock-free alternatives are available.
- Use channels (`mpsc`, `crossbeam`) for message passing.

### Security

- **MUST** use environment variables for sensitive configuration via `std::env`
- Use `secrecy` crate for sensitive data types

### Support no_std

For libraries, whenever possible support `no_std` by having an explicit `std` feature and falling back to `core` and `alloc` when `cfg(not(feature = "std))`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"))]
extern crate alloc;
/*
 * The following is no longer supported as there is no known external crate named
 * `std` after approximately edition 2018. However, it does make a nice logical
 * follow-on from the line above for the `alloc` crate.
 *
 *  ```
 * #[cfg(any(test, feature = "std"))]
 * extern crate std;
 * ```
 */

#[cfg(not(feature = "std"))]
use alloc::{
    boxed::Box,
    string::{String, ToString},
};
```

## Testing

- **MUST** write unit tests for all new functions and types.
- **MUST** mock external dependencies (APIs, databases, file systems).
- **MUST** use the built-in `#[test]` attribute and `cargo test`.
- Do not commit commented-out tests.
- Use `#[cfg(test)]` sub-modules for unit test code (see @[`plain.rs`](./templates/plain.rs)).
- Use the `test-log` attribute for test cases that require tracing initialization (see @[`integration_test.rs`](./templates/integration_test.rs)).

### Code Coverage

- Use `cargo-tarpaulin` to run coverage tests.
- Include a @[`codecov.yml`](./codecov.yml) file with target set to **AT LEAST** 80%.

### Benchmarks

- **NEVER** run benchmarks in parallel, as the benchmarks will compete for resources and the results will be invalid.
- **NEVER** game the benchmarks. Do not manipulate the benchmarks themselves to satisfy any required performance constraints.
- **NEVER** run benchmarks with `target-cpu=native` or any other `RUSTFLAGS`.
- If benchmarking against another crate or library, ensure the benchmarks are apples-to-apples comparisons.
- Ensure benchmark tests are independent. If the tests are dependent due to a feature (e.g. caching), ensure the feature is disabled.

## Version Control

**ALWAYS** write clear, descriptive commit messages, following [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) style recommendations. Use the template @[`git-cc-message.txt`](../common/git-cc-message.txt) for Git commit messages.

The file @[`gitignore`](./gitignore) contains a minimal top-level `.gitignore` file for Rust.

### Code Reviews

Not just during a request for review, but always be looking at the code for quality, and improvements.

- Identifying potential bugs or safety issues
- Suggesting improvements for idiomatic Rust patterns

### Commit Checklist

- [ ] No compiler warnings (`cargo build`).
- [ ] All tests pass (`cargo test`).
- [ ] Clippy passes (`cargo clippy -- -D warnings`).
- [ ] Code is formatted (`cargo fmt --check`).
- [ ] No security advisories (`cargo audit`).
- [ ] No outdated _direct_ dependencies (`cargo outdated --depth 1`).
- [ ] All public items have doc comments.
- [ ] Documentation generates without errors (`cargo doc --all-features --no-deps`).
- [ ] No commented-out code or debug statements (`println!` statements or `dbg!` macros).
- [ ] No hardcoded credentials.
- **If** the project creates a WASM package and Rust code is touched:
  - [ ] rebuild the WASM package (`wasm-pack build --target web --out-dir web/pkg`)

### Release Management

Use the following tools, as configured in the example repository [orhun/automated-rust-releases](https://github.com/orhun/automated-rust-releases).

- `git-cliff` Automates the changelog generation.
- `release-plz` Handles dependency updates, version management, and crates.io release.
- `cargo-dist` Creates GitHub releases and packaging for various platforms.
- `Dependabot` Updates the Rust and GitHub Actions dependencies.
- `Mergify` Automatically merges the Dependabot pull requests.

## Appendix: Use Cases

### CLI

A key design goal is to separate the description of the command-line interface from the command implementation. The interface is defined using `clap`, and the validated fields are then parameters to a command implementation.

- Use a module `cli` to model the command-line itself. For complex cases this may have sub-modules per command.
  - Use `clap` for command-line definition.
  - Use `clap-verbosity-flag` to provide `--verbose` and `--quiet` options.
  - Use `clap_complete` to provide a `completions` command.
- Use a module `telemetry` to initialize logging, tracing, metrics, etc.
  - Use `tracing_subscriber` to initialize tracing.
  - See section [Logging/Tracing](#loggingtracing) for more details.
- Use a module `commands` to contain the actual command implementation. For complex cases this may have sub-modules per command.

The file @[`main.rs`](./templates/main.rs) demonstrates a very clean implementation with clear separations of concern.

Include the following trait in the module `main`. This trait is implemented by the top-level CLI structure and any command structures. Additionally each command implementation is a struct, with a `new` function taking parameters from the cli, and implementing `OnceCommand`.

As in the `main` function above, CLI functions that propogate errors _should_ use `ExitCode` to denote success/failure regardless of errors.

### Text UI

- Use `ratatui` and `crossterm` for terminal applications/TUIs.
  - Include logical and intuitive mouse controls for all TUIs.
  - **ALWAYS** account for interface scrolling offsets when calculating click locations.
- Use `indicatif` to track long-running operations with progress bars. The message should be contextually sensitive.
- Use `rustyline` and `rustyline-derive` for reading input from the user.

### Web Server

- Use `axum` for creating any web servers or HTTP APIs.
  - Keep request handlers async, returning `Result<Response, AppError>` to centralize error handling.
  - Use layered extractors and shared state structs instead of global mutable data.
  - Add `tower` middleware (timeouts, tracing, compression) for observability and resilience.
  - Offload CPU-bound work to `tokio::task::spawn_blocking` or background services to avoid blocking the reactor.
- See section [Logging/Tracing](#loggingtracing) for more.

## Appendix: Module Structures

### Library Module

The root of any library package, see @[`lib.rs`](./templates/lib.rs).

### Error Module

The file @[`error.rs`](./templates/error.rs) is at the same level as the `lib.rs` or `main.rs` file. This makes use of
the crate `thiserror` to define the error enumeration.

### General Module

Any non-error module should follow the template @[`plain.rs`](./templates/plain.rs).

### Unit Tests

Unit tests should be included at the end of any general module, see the template @[`plain.rs`](./templates/plain.rs).

### Test Files

Integration tests are found in the `tests` directory of the crate and are all named `test_*.rs` using the template @[`integration_test.rs`](./templates/integration_test.rs).
