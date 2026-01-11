# Verus Quick Start Guide

This guide will help you quickly build Verus and verify example code.

## Prerequisites

- **Rust**: You need `rustup` installed. If not, get it from https://rustup.rs
- **Git**: To clone the repository

## Building Verus

### 1. Change to the source directory

```bash
cd source
```

### 2. Download Z3 (SMT solver)

```bash
./tools/get-z3.sh    # On Linux/macOS
# or
./tools/get-z3.ps1   # On Windows PowerShell
```

### 3. Activate the development environment

```bash
source ../tools/activate        # for bash and zsh
# or
source ../tools/activate.fish   # for fish
# or
..\tools\activate.bat          # for Windows
```

This will build `vargo`, a cargo wrapper, and add it to your PATH.

### 4. Build Verus

```bash
vargo build --release
```

This will take several minutes and will build:
- The `rust_verify` binary (Verus verifier)
- Additional libraries (`builtin`, `builtin_macros`, `state_machines_macros`)
- The Verus standard library (`vstd`)

## Running Examples

### Verify an example file

From the `source` directory:

```bash
vargo run -p rust_verify --release -- ../examples/vectors.rs
```

Or run the verifier directly:

```bash
./target-verus/release/verus ../examples/vectors.rs
```

Expected output:
```
verification results:: 9 verified, 0 errors
```

### Compile and run an example

```bash
./target-verus/release/verus ../examples/doubly_linked_xor.rs --compile
./doubly_linked_xor
```

This will:
1. Verify the code (checking all specifications)
2. Compile it to a native binary via `rustc`
3. Run the compiled binary

## More Examples

The `examples` directory contains many example programs demonstrating various Verus features:

- `examples/vectors.rs` - Basic vector operations
- `examples/generics.rs` - Generic types and functions
- `examples/calc.rs` - Calculation proofs
- `examples/basic_lock1.rs` - Concurrency primitives
- `examples/doubly_linked_xor.rs` - Data structure verification

## Next Steps

- Read the [Tutorial and Reference](https://verus-lang.github.io/verus/guide/)
- Check the [API documentation for vstd](https://verus-lang.github.io/verus/verusdoc/vstd/)
- Join the community on [Zulip](https://verus-lang.zulipchat.com/)

## Troubleshooting

If you encounter issues:

1. Make sure you're using the correct Rust toolchain (1.91.0 as specified in `rust-toolchain.toml`)
2. Ensure Z3 is downloaded and accessible at `source/z3` or `source/z3.exe`
3. Always run from the `source` directory when using `vargo`
4. Check that the `vargo` wrapper is in your PATH after running the activate script

For more detailed build instructions, see [BUILD.md](BUILD.md).
