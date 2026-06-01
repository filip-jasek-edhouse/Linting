# non_stack_lint

## What it does

Emits an error on common non-stack allocation entry points.

Current coverage includes:

- Box::new
- Vec::new and Vec::with_capacity
- String::new and String::with_capacity
- Rc::new
- Arc::new
- std::alloc allocation functions

## Why is this bad?

Heap allocations can increase latency, memory pressure, and runtime nondeterminism.

## Known problems

This lint is conservative and does not catch every possible heap allocation route.

## Example

Denied:

```rust
let _boxed = Box::new(1u32);
let _vec = Vec::<u8>::with_capacity(8);
```

Preferred:

```rust
let x = 1u32;
let buf = [0u8; 8];
```

## Run

PowerShell:

```powershell
$env:DYLINT_RUSTFLAGS='-D non_stack_lint'
cargo dylint --path non_stack_lint --workspace -- --all-targets
```
