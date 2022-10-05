use crate::op_codes::{PUSH1, PUSH32};
use std::ops::BitOr;

pub enum Pattern {
    OpCode(u8),
    OpCodes(Vec<u8>),
}

impl PartialEq<u8> for &Pattern {
    fn eq(&self, other: &u8) -> bool {
        match self {
            Pattern::OpCode(v) => v == other,
            Pattern::OpCodes(vs) => vs.contains(other),
        }
    }
}

impl BitOr for Pattern {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::OpCode(a), Self::OpCode(b)) => Self::OpCodes(vec![a, b]),
            (Self::OpCode(a), Self::OpCodes(b)) => Self::OpCodes([&[a], b.as_slice()].concat()),
            (Self::OpCodes(a), Self::OpCode(b)) => Self::OpCodes([a.as_slice(), &[b]].concat()),
            (Self::OpCodes(a), Self::OpCodes(b)) => Self::OpCodes([a, b].concat()),
        }
    }
}

pub struct Analyzer<'a> {
    code: &'a [u8],
}

impl<'a> Analyzer<'a> {
    pub fn new(code: &'a [u8]) -> Self {
        Analyzer { code }
    }

    pub fn extract_pattern(&self, pattern: &[Pattern]) -> Vec<&[u8]> {
        let mut output: Vec<&[u8]> = Vec::new();
        let mut buf: Vec<&[u8]> = Vec::new();
        let mut pc = 0;

        for (_, op, data) in self {
            if &pattern[pc] == op {
                if let Some(value) = data {
                    buf.push(value)
                }

                pc += 1;

                if pc == pattern.len() {
                    output.append(&mut buf);
                    pc = 0;
                }
            } else if pc > 0 {
                pc = 0;
                buf.clear();
            }
        }

        output
    }

    pub fn has_pattern(&self, pattern: &[Pattern]) -> bool {
        let mut pc = 0;

        for (_, op, _) in self {
            if &pattern[pc] == op {
                pc += 1;

                if pc == pattern.len() {
                    return true;
                }
            } else if pc > 0 {
                pc = 0;
            }
        }

        false
    }
}

impl<'a> IntoIterator for &Analyzer<'a> {
    type Item = (usize, u8, Option<&'a [u8]>);
    type IntoIter = BytecodeIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BytecodeIterator::new(self.code)
    }
}

pub struct BytecodeIterator<'a> {
    offset: usize,
    code: &'a [u8],
}

impl<'a> BytecodeIterator<'a> {
    pub fn new(code: &'a [u8]) -> Self {
        BytecodeIterator { offset: 0, code }
    }
}

impl<'a> Iterator for BytecodeIterator<'a> {
    type Item = (usize, u8, Option<&'a [u8]>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset < self.code.len() {
            let offset = self.offset;
            let op = self.code[self.offset];
            let is_push = (PUSH1..=PUSH32).contains(&op);

            self.offset += 1;

            if is_push {
                let size: usize = (op - 0x5f) as usize;

                self.offset += size;

                if offset + size < self.code.len() {
                    let num: &[u8] = &self.code[(offset + 1)..=(offset + size)];

                    Some((offset, op, Some(num)))
                } else {
                    Some((offset, op, None))
                }
            } else {
                Some((offset, op, None))
            }
        } else {
            None
        }
    }
}
