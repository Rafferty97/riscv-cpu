use std::{fmt::Debug, hint::unreachable_unchecked, ops::BitAnd};

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Word(i32);

impl Word {
    pub const NEG_ONE: Self = Self(-1);
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(1);

    pub fn i8(self) -> i8 {
        self.0 as i8
    }

    pub fn i16(self) -> i16 {
        self.0 as i16
    }

    pub fn i32(self) -> i32 {
        self.0
    }

    pub fn i64(self) -> i64 {
        self.0 as i64
    }

    pub fn u8(self) -> u8 {
        self.0 as u8
    }

    pub fn u16(self) -> u16 {
        self.0 as u16
    }

    pub fn u32(self) -> u32 {
        self.0 as u32
    }

    pub fn u64(self) -> u64 {
        self.0 as u64
    }

    pub fn instr(self) -> EncInstr {
        EncInstr(self.0 as u32)
    }

    pub fn fits(self, bit_cnt: u32) -> bool {
        let shift = 32 - bit_cnt;
        ((self.i32() << shift) >> shift) == self.i32()
    }

    #[inline(always)]
    fn extract(self, start: u32, len: u32, pos: u32) -> u32 {
        extract(self.u32(), start, len, pos)
    }
}

impl Debug for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08x}", self.0)
    }
}

impl From<bool> for Word {
    fn from(value: bool) -> Self {
        Self(value as _)
    }
}

impl From<i8> for Word {
    fn from(value: i8) -> Self {
        Self(value as _)
    }
}

impl From<i16> for Word {
    fn from(value: i16) -> Self {
        Self(value as _)
    }
}

impl From<i32> for Word {
    fn from(value: i32) -> Self {
        Self(value as _)
    }
}

impl From<i64> for Word {
    fn from(value: i64) -> Self {
        Self(value as _)
    }
}

impl From<u8> for Word {
    fn from(value: u8) -> Self {
        Self(value as u32 as _)
    }
}

impl From<u16> for Word {
    fn from(value: u16) -> Self {
        Self(value as u32 as _)
    }
}

impl From<u32> for Word {
    fn from(value: u32) -> Self {
        Self(value as _)
    }
}

impl From<u64> for Word {
    fn from(value: u64) -> Self {
        Self(value as _)
    }
}

impl BitAnd for Word {
    type Output = Word;

    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Width {
    Byte,
    Half,
    Word,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Reg(u8);

impl Reg {
    pub const ZERO: Self = Reg(0);
    pub const RA: Self = Reg(1);
    pub const SP: Self = Reg(2);

    #[inline(always)]
    pub fn get(self, file: &[Word; 32]) -> Word {
        match self.0 {
            i @ 0..32 => file[i as usize],
            _ => unsafe { unreachable_unchecked() },
        }
    }

    #[inline(always)]
    pub fn set(self, file: &mut [Word; 32], value: Word) {
        match self.0 {
            0 => {}
            i @ 1..32 => file[i as usize] = value,
            _ => unsafe { unreachable_unchecked() },
        }
    }

    pub fn u32(self) -> u32 {
        self.0 as u32
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct EncInstr(u32);

impl EncInstr {
    #[inline(always)]
    pub fn opcode(self) -> u32 {
        self.0 & 127
    }

    #[inline(always)]
    pub fn rd(self) -> Reg {
        Reg(((self.0 >> 7) & 31) as u8)
    }

    #[inline(always)]
    pub fn rs1(self) -> Reg {
        Reg(((self.0 >> 15) & 31) as u8)
    }

    #[inline(always)]
    pub fn rs2(self) -> Reg {
        Reg(((self.0 >> 20) & 31) as u8)
    }

    #[inline(always)]
    pub fn fn3(self) -> u32 {
        (self.0 >> 12) & 7
    }

    #[inline(always)]
    pub fn fn7(self) -> u32 {
        self.0 >> 25
    }

    #[inline(always)]
    pub fn fn12(self) -> u32 {
        self.0 >> 20
    }

    #[inline(always)]
    pub fn i_imm(self) -> Word {
        Self::extend(self.extract(20, 12, 0), 12)
    }

    #[inline(always)]
    pub fn s_imm(self) -> Word {
        let a = self.extract(7, 5, 0);
        let b = self.extract(25, 7, 5);
        Self::extend(a | b, 12)
    }

    #[inline(always)]
    pub fn b_imm(self) -> Word {
        let a = self.extract(8, 4, 1);
        let b = self.extract(25, 6, 5);
        let c = self.extract(7, 1, 11);
        let d = self.extract(31, 1, 12);
        Self::extend(a | b | c | d, 13)
    }

    #[inline(always)]
    pub fn u_imm(self) -> Word {
        Self::extend(self.extract(12, 20, 12), 32)
    }

    #[inline(always)]
    pub fn j_imm(self) -> Word {
        let a = self.extract(21, 4, 1);
        let b = self.extract(25, 6, 5);
        let c = self.extract(20, 1, 11);
        let d = self.extract(12, 8, 12);
        let e = self.extract(31, 1, 20);
        Self::extend(a | b | c | d | e, 21)
    }

    #[inline(always)]
    pub fn u32(self) -> u32 {
        self.0
    }

    #[inline(always)]
    fn extract(self, start: u32, len: u32, pos: u32) -> u32 {
        extract(self.0, start, len, pos)
    }

    #[inline(always)]
    fn extend(value: u32, len: u32) -> Word {
        let shift = 32 - len;
        Word(((value as i32) << shift) >> shift)
    }
}

impl FromIterator<EncInstr> for Vec<u8> {
    fn from_iter<T: IntoIterator<Item = EncInstr>>(iter: T) -> Self {
        iter.into_iter().flat_map(|ins| ins.0.to_le_bytes()).collect()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct InstrBuilder(u32);

impl InstrBuilder {
    pub fn new(opcode: u32) -> Self {
        debug_assert_eq!(opcode >> 7, 0);
        Self(opcode)
    }

    pub fn rd(self, rd: Reg) -> Self {
        Self(self.0 | (rd.u32() << 7))
    }

    pub fn rs1(self, rs1: Reg) -> Self {
        Self(self.0 | (rs1.u32() << 15))
    }

    pub fn rs2(self, rs2: Reg) -> Self {
        Self(self.0 | (rs2.u32() << 20))
    }

    pub fn fn3(self, fn3: u32) -> Self {
        debug_assert_eq!(fn3 >> 3, 0);
        Self(self.0 | (fn3 << 12))
    }

    pub fn fn7(self, fn7: u32) -> Self {
        debug_assert_eq!(fn7 >> 7, 0);
        Self(self.0 | (fn7 << 25))
    }

    pub fn i_imm(self, imm: impl Into<Word>) -> Self {
        let imm = imm.into();
        debug_assert!(imm.fits(12));
        Self(self.0 | imm.extract(0, 12, 20))
    }

    pub fn s_imm(self, imm: impl Into<Word>) -> Self {
        let imm = imm.into();
        debug_assert!(imm.fits(12));
        let a = imm.extract(0, 5, 7);
        let b = imm.extract(5, 7, 25);
        Self(self.0 | a | b)
    }

    pub fn b_imm(self, imm: impl Into<Word>) -> Self {
        let imm = imm.into();
        debug_assert_eq!(imm.u32() & 1, 0);
        debug_assert!(imm.fits(13));
        let a = imm.extract(11, 1, 7);
        let b = imm.extract(1, 4, 8);
        let c = imm.extract(5, 6, 25);
        let d = imm.extract(31, 1, 31);
        Self(self.0 | a | b | c | d)
    }

    pub fn u_imm(self, imm: impl Into<Word>) -> Self {
        let imm = imm.into();
        debug_assert_eq!(imm.u32() & 0xfff, 0);
        Self(self.0 | imm.extract(12, 20, 12))
    }

    pub fn j_imm(self, imm: impl Into<Word>) -> Self {
        let imm = imm.into();
        debug_assert_eq!(imm.u32() & 1, 0);
        debug_assert!(imm.fits(21));
        let a = imm.extract(12, 8, 12);
        let b = imm.extract(11, 1, 20);
        let c = imm.extract(1, 10, 21);
        let d = imm.extract(31, 1, 31);
        Self(self.0 | a | b | c | d)
    }

    pub fn build(self) -> EncInstr {
        EncInstr(self.0)
    }
}

#[inline(always)]
fn extract(value: u32, start: u32, len: u32, pos: u32) -> u32 {
    ((value >> start) & (!0 >> (32 - len))) << pos
}
