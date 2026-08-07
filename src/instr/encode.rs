use super::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct InstrBuilder(u32);

impl InstrBuilder {
    pub fn new(opcode: u32) -> Self {
        debug_assert_eq!(opcode >> 7, 0);
        Self(opcode)
    }

    pub fn rd(self, rd: Reg) -> Self {
        Self(self.0 | ((rd.raw() as u32) << 7))
    }

    pub fn rs1(self, rs1: Reg) -> Self {
        Self(self.0 | ((rs1.raw() as u32) << 15))
    }

    pub fn rs2(self, rs2: Reg) -> Self {
        Self(self.0 | ((rs2.raw() as u32) << 20))
    }

    pub fn fn3(self, fn3: u32) -> Self {
        debug_assert_eq!(fn3 >> 3, 0);
        Self(self.0 | (fn3 << 12))
    }

    pub fn fn7(self, fn7: u32) -> Self {
        debug_assert_eq!(fn7 >> 7, 0);
        Self(self.0 | (fn7 << 25))
    }

    pub fn i_imm(self, imm: Imm) -> Self {
        debug_assert!(imm.fits(12));
        Self(self.0 | imm.extract(0, 12, 20))
    }

    pub fn s_imm(self, imm: Imm) -> Self {
        debug_assert!(imm.fits(12));
        let a = imm.extract(0, 5, 7);
        let b = imm.extract(5, 7, 25);
        Self(self.0 | a | b)
    }

    pub fn b_imm(self, imm: Imm) -> Self {
        debug_assert_eq!(imm.0 & 1, 0);
        debug_assert!(imm.fits(13));
        let a = imm.extract(11, 1, 7);
        let b = imm.extract(1, 4, 8);
        let c = imm.extract(5, 6, 25);
        let d = imm.extract(31, 1, 31);
        Self(self.0 | a | b | c | d)
    }

    pub fn u_imm(self, imm: Imm) -> Self {
        debug_assert_eq!(imm.0 & 0xfff, 0);
        Self(self.0 | imm.extract(12, 20, 12))
    }

    pub fn j_imm(self, imm: Imm) -> Self {
        debug_assert_eq!(imm.0 & 1, 0);
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

impl Imm {
    #[inline(always)]
    fn fits(self, bit_cnt: u32) -> bool {
        let shift = 32 - bit_cnt;
        ((self.0 << shift) >> shift) == self.0
    }

    #[inline(always)]
    fn extract(self, start: u32, len: u32, pos: u32) -> u32 {
        bit_utils::extract(self.0 as u32, start, len, pos)
    }
}
