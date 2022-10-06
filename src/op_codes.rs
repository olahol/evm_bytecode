#![allow(dead_code)]

pub const ISZERO: u8 = 0x15;
pub const EQ: u8 = 0x14;
pub const JUMPI: u8 = 0x57;
pub const PUSH1: u8 = 0x60;
pub const PUSH2: u8 = 0x61;
pub const PUSH4: u8 = 0x63;
pub const PUSH32: u8 = 0x7f;
pub const DUP1: u8 = 0x80;
pub const MLOAD: u8 = 0x51;
pub const SWAP3: u8 = 0x92;
pub const SWAP2: u8 = 0x91;
pub const LOG0: u8 = 0xa0;
pub const LOG4: u8 = 0xa4;
