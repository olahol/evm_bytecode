use std::ops::RangeInclusive;

const PUSHN: RangeInclusive<u8> = 0x60..=0x75;
const DUP1: u8 = 0x80;
const PUSH4: u8 = 0x63;
const EQ: u8 = 0x14;

fn parse_selector(i: usize, input: &[u8]) -> Option<u32> {
    let op1 = *input.get(i)?;
    let op2 = *input.get(i + 1)?;
    let op3 = *input.get(i + 6)?;

    if op1 == DUP1 && op2 == PUSH4 && op3 == EQ {
        Some(u32::from_be_bytes([
            *input.get(i + 2)?,
            *input.get(i + 3)?,
            *input.get(i + 4)?,
            *input.get(i + 5)?,
        ]))
    } else {
        None
    }
}

/// Get a list of 4 byte selectors from EVM bytecode
pub fn selectors_from_bytecode(input: Vec<u8>) -> Vec<u32> {
    let mut v: Vec<u32> = vec![];

    let mut i = 0;

    while i < input.len() {
        let op = input[i];

        if let Some(x) = parse_selector(i, &input) {
            v.push(x)
        }

        i += if PUSHN.contains(&op) {
            (op - 0x5e) as usize
        } else {
            1
        };
    }

    v
}

#[cfg(test)]
mod tests {
    use crate::selectors_from_bytecode;

    #[test]
    fn empty_input() {
        selectors_from_bytecode(vec![]);
    }
}
