# Linting

Custom Rust lints built with Dylint.

## Lints in this workspace

- test_linting: warns on labeled continue usage.
- recursion_lint: denies direct recursion (a function/method calling itself).
- inf_loop_lint: denies loop expressions that use the loop keyword.
- non_stack_lint: denies common non-stack allocation entry points.

## Toolchain requirements

These lints use rustc internals and require nightly.

- nightly-2026-04-16
- rustc-dev
- llvm-tools-preview

Recommended setup from repository root:

1. rustup override set nightly-2026-04-16
2. rustup component add rustc-dev llvm-tools-preview
3. cargo install cargo-dylint dylint-link --locked

## Run a single lint

PowerShell examples:

- $env:DYLINT_RUSTFLAGS='-D recursion_lint'; cargo dylint --path recursion_lint --workspace -- --all-targets
- $env:DYLINT_RUSTFLAGS='-D inf_loop_lint'; cargo dylint --path inf_loop_lint --workspace -- --all-targets
- $env:DYLINT_RUSTFLAGS='-D non_stack_lint'; cargo dylint --path non_stack_lint --workspace -- --all-targets

To run test_linting as deny:

- $env:DYLINT_RUSTFLAGS='-D test_linting'; cargo dylint --path test_linting --workspace -- --all-targets

## Run tests

Each lint crate has compiletest UI tests.

- cargo +nightly-2026-04-16 test -p test_linting
- cargo +nightly-2026-04-16 test -p recursion_lint
- cargo +nightly-2026-04-16 test -p inf_loop_lint
- cargo +nightly-2026-04-16 test -p non_stack_lint

## CI behavior

The workflow in .github/workflows/custom-lints.yml runs all lints by iterating over top-level folders matching *_lint and also includes test_linting.
