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