use std::hint::unreachable_unchecked;

use bit_ops::bitops_u32;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Word(i32);

impl Word {
    pub fn u32(self) -> u32 {
        self.0 as u32
    }

    pub fn i32(self) -> i32 {
        self.0
    }
}

impl From<bool> for Word {
    fn from(value: bool) -> Self {
        Self(value as _)
    }
}

impl From<u32> for Word {
    fn from(value: u32) -> Self {
        Self(value as _)
    }
}

impl From<i32> for Word {
    fn from(value: i32) -> Self {
        Self(value as _)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Reg(u8);

impl Reg {
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
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Instr(u32);

impl Instr {
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
    pub fn funct3(self) -> u32 {
        (self.0 >> 12) & 7
    }

    #[inline(always)]
    pub fn funct7(self) -> u32 {
        self.0 >> 25
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
        Self::extend(a | b | c | d, 12)
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
        let e = self.extract(21, 1, 20);
        Self::extend(a | b | c | d | e, 21)
    }

    #[inline(always)]
    pub fn test_bit(self, bit: u32) -> bool {
        bitops_u32::is_set(self.0, bit)
    }

    #[inline(always)]
    fn extract(self, start: u32, len: u32, pos: u32) -> u32 {
        ((self.0 >> start) & (!0 >> (32 - len))) << pos
    }

    #[inline(always)]
    fn extend(value: u32, len: u32) -> Word {
        let shift = 32 - len;
        Word(((value as i32) << shift) >> shift)
    }
}
