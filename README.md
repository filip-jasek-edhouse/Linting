# Linting

Custom Rust lints built with Dylint.

## Lints in this workspace

- test_lint: warns on labeled continue usage.
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

## External consumer quick-start

From a separate consumer repository/workspace:

1. rustup toolchain install nightly-2026-04-16 --component rustc-dev --component llvm-tools-preview
2. cargo +nightly-2026-04-16 install cargo-dylint dylint-link --locked
3. cargo +nightly-2026-04-16 dylint --git <repo_url> --workspace --all -- --all-targets --message-format=json

If you want warnings to fail CI in the consumer, pass deny flags for the lint names, for example:

- DYLINT_RUSTFLAGS='-D recursion_lint -D inf_loop_lint -D non_stack_lint -D test_lint' cargo +nightly-2026-04-16 dylint --git <repo_url> --workspace --all -- --all-targets --message-format=json

## Run a single lint

PowerShell examples:

- $env:DYLINT_RUSTFLAGS='-D recursion_lint'; cargo dylint --path recursion_lint --workspace -- --all-targets
- $env:DYLINT_RUSTFLAGS='-D inf_loop_lint'; cargo dylint --path inf_loop_lint --workspace -- --all-targets
- $env:DYLINT_RUSTFLAGS='-D non_stack_lint'; cargo dylint --path non_stack_lint --workspace -- --all-targets

To run test_lint as deny:

- $env:DYLINT_RUSTFLAGS='-D test_lint'; cargo dylint --path test_lint --workspace -- --all-targets

## Run tests

Each lint crate has compiletest UI tests.

- cargo +nightly-2026-04-16 test -p test_lint
- cargo +nightly-2026-04-16 test -p recursion_lint
- cargo +nightly-2026-04-16 test -p inf_loop_lint
- cargo +nightly-2026-04-16 test -p non_stack_lint

## CI behavior

The workflow in .github/workflows/custom-lints.yml runs all four lint crates explicitly.
