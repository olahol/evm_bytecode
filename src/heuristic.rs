use crate::bytecode::Analyzer;
use crate::bytecode::Pattern::OpCode;
use crate::op_codes::*;

/// Get event signatures from EVM bytecode
pub fn events_from_bytecode(input: &[u8]) -> Vec<[u8; 32]> {
    let mut o = Vec::new();

    let mut t: &[u8] = &[];
    let mut c = -1;

    for (_, op, data) in &Analyzer::new(input) {
        if op == PUSH32 {
            if let Some(value) = data {
                t = value;
                c = 0;
            }
        }

        if c > 15 {
            c = -1;
            continue;
        }

        if c > -1 {
            if (0xa0..=0xa4).contains(&op) {
                o.push(t.try_into().unwrap());
                c = -1;
            }

            c += 1;
        }
    }

    o.sort();
    o.dedup();

    o
}

/// Get selectors from EVM bytecode
pub fn selectors_from_bytecode(input: &[u8]) -> Vec<[u8; 4]> {
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
        .map(|bs| bs.try_into().unwrap())
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
        .map(|bs| bs.try_into().unwrap())
        .collect();

    if vyper.len() > solidity.len() {
        vyper
    } else {
        solidity
    }
}

#[cfg(test)]
mod tests {
    use crate::heuristic::*;

    #[test]
    fn empty_input() {
        selectors_from_bytecode(&[]);
        events_from_bytecode(&[]);
    }
}
