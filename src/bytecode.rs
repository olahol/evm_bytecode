use crate::op_codes::{PUSH1, PUSH32};
use std::ops::BitOr;

pub enum Pattern {
    Op(u8),
    Ops(Vec<u8>),
    OpAny,
}

impl PartialEq<u8> for &Pattern {
    fn eq(&self, other: &u8) -> bool {
        match self {
            Pattern::Op(v) => v == other,
            Pattern::Ops(vs) => vs.contains(other),
            Pattern::OpAny => true,
        }
    }
}

impl BitOr for Pattern {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Op(a), Self::Op(b)) => Self::Ops(vec![a, b]),
            (Self::Op(a), Self::Ops(b)) => Self::Ops([&[a], b.as_slice()].concat()),
            (Self::Ops(a), Self::Op(b)) => Self::Ops([a.as_slice(), &[b]].concat()),
            (Self::Ops(a), Self::Ops(b)) => Self::Ops([a, b].concat()),
            (Self::OpAny, _) => Self::OpAny,
            (_, Self::OpAny) => Self::OpAny,
        }
    }
}

pub struct Bytecode<'a> {
    code: &'a [u8],
}

impl<'a> Bytecode<'a> {
    pub fn new(code: &'a [u8]) -> Self {
        Bytecode { code }
    }

    pub fn extract_pattern(&self, pattern: &[Pattern]) -> Vec<Vec<&[u8]>> {
        let mut output: Vec<Vec<&[u8]>> = Vec::new();
        let mut buf: Vec<&[u8]> = Vec::new();
        let mut pc = 0;

        for (_, op, data) in self {
            if &pattern[pc] == op {
                if let Some(value) = data {
                    buf.push(value)
                }

                pc += 1;

                if pc == pattern.len() {
                    output.push(buf);

                    buf = Vec::new();
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

impl<'a> IntoIterator for &Bytecode<'a> {
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

#[cfg(test)]
mod tests {
    use crate::bytecode::*;
    use crate::op_codes::*;

    #[allow(clippy::assertions_on_constants)]
    #[test]
    fn empty_bytecode() {
        let b = Bytecode::new(&[]);

        assert_eq!(b.extract_pattern(&[Pattern::OpAny]).len(), 0);
        assert!(!b.has_pattern(&[Pattern::OpAny]));

        for (_, _, _) in &b {
            assert!(false)
        }
    }

    #[test]
    fn push_aware_iterator() {
        let s = hex::decode("61ffff000062ffff").unwrap();
        let b = Bytecode::new(&s);

        assert_eq!(b.into_iter().count(), 4);

        for (offset, op, data) in &b {
            if offset == 0 {
                assert_eq!(op, PUSH2);
                assert_ne!(data, None);
            } else if offset == 5 {
                assert_eq!(op, PUSH3);
                assert_eq!(data, None);
            } else {
                assert_eq!(op, STOP);
            }
        }
    }
}
