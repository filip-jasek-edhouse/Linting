# inf_loop_lint

## What it does

Emits an error for loop expressions that use the loop keyword.

## Why is this bad?

Unbounded loops are often accidental and can hang a program.

## Known problems

This lint only targets loop and does not flag while or for loops.

## Example

Denied:

```rust
loop {
	do_work();
}
```

Allowed:

```rust
while should_continue() {
	do_work();
}
```

## Run

PowerShell:

```powershell
$env:DYLINT_RUSTFLAGS='-D inf_loop_lint'
cargo dylint --path inf_loop_lint --workspace -- --all-targets
```
