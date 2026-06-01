# recursion_lint

## What it does

Emits an error for direct recursion in function and method calls.

## Why is this bad?

Direct recursion can cause stack overflows and is often accidental.

## Known problems

This lint currently focuses on direct recursion only. It does not detect mutual recursion across multiple functions.

## Example

Denied:

```rust
fn factorial(n: u32) -> u32 {
	if n <= 1 { 1 } else { n * factorial(n - 1) }
}
```

Preferred:

```rust
fn factorial(mut n: u32) -> u32 {
	let mut acc = 1;
	while n > 1 {
		acc *= n;
		n -= 1;
	}
	acc
}
```

## Run

PowerShell:

```powershell
$env:DYLINT_RUSTFLAGS='-D recursion_lint'
cargo dylint --path recursion_lint --workspace -- --all-targets
```
