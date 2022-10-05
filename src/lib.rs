use bytecode::Analyzer;
use bytecode::Pattern::OpCode;
use op_codes::*;

pub mod bytecode;
pub mod op_codes;

/// Get 4-byte selectors from EVM bytecode
pub fn selectors_from_bytecode(input: &[u8]) -> Vec<u32> {
    let solidity: Vec<_> = Analyzer::new(input)
        .extract_pattern(&[
            OpCode(DUP1),
            OpCode(PUSH4),
            OpCode(EQ),
            OpCode(PUSH1) | OpCode(PUSH2),
            OpCode(JUMPI),
        ])
        .into_iter()
        .filter(|bs| bs.len() == 4)
        .map(|bs| u32::from_be_bytes(bs.try_into().unwrap()))
        .collect();

    let vyper: Vec<_> = Analyzer::new(input)
        .extract_pattern(&[
            OpCode(PUSH4),
            OpCode(PUSH1),
            OpCode(MLOAD),
            OpCode(EQ),
            OpCode(ISZERO),
            OpCode(PUSH2),
            OpCode(JUMPI),
        ])
        .into_iter()
        .filter(|bs| bs.len() == 4)
        .map(|bs| u32::from_be_bytes(bs.try_into().unwrap()))
        .collect();

    if vyper.len() > solidity.len() {
        vyper
    } else {
        solidity
    }
}

#[cfg(test)]
mod tests {
    use crate::selectors_from_bytecode;

    #[test]
    fn empty_input() {
        selectors_from_bytecode(&[]);
    }
}
