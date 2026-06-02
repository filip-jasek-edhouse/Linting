# test_lint

Lint name: test_linting

## What it does

Emits a warning when labeled continue is used.

## Why is this bad?

Labeled control-flow jumps are easy to miss in reviews and can make loops harder to reason about.

## Known problems

This lint intentionally allows unlabeled continue.

## Example

Warned:

```rust
'epic: loop {
	if should_skip() {
		continue 'epic;
	}
}
```

Preferred:

```rust
loop {
	if should_skip() {
		continue;
	}
}
```

## Run

PowerShell:

```powershell
$env:DYLINT_RUSTFLAGS='-D test_linting'
cargo dylint --path test_lint --workspace -- --all-targets
```
