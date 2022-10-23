# evm_bytecode

> Tools for working with EVM bytecode

## Example

```rust
// Extract all 160 or 256 bit values
use evm_bytecode::bytecode::{Bytecode, Pattern::Op};
use evm_bytecode::op_codes::*;

fn main() {
    let code = hex::decode("...").unwrap();
    let bytecode = Bytecode::new(&code);

    for m in bytecode.extract_pattern(&[Op(PUSH20) | Op(PUSH32)]) {
        println!("0x{}", hex::encode(m[0]));
    }
}
```

```rust
// Extract all 4-byte selectors
fn main() {
    let code = hex::decode("...").unwrap();

    for s in evm_bytecode::heuristic::selectors_from_bytecode(&code) {
        println!("0x{}", hex::encode(s));
    }
}
```