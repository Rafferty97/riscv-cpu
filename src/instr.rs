use std::ops::{BitAnd, BitOr};

use crate::bit_utils;

mod decode;
mod encode;
mod print;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Reg(u8);

impl Reg {
    pub const fn new(index: u8) -> Self {
        assert!(index < 32);
        Self(index)
    }

    pub fn raw(self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Imm(pub i32);

impl From<i32> for Imm {
    fn from(value: i32) -> Self {
        Imm(value)
    }
}

impl From<Imm> for i32 {
    fn from(value: Imm) -> Self {
        value.0
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
    pub fn i_imm(self) -> Imm {
        Self::extend(self.extract(20, 12, 0), 12)
    }

    #[inline(always)]
    pub fn s_imm(self) -> Imm {
        let a = self.extract(7, 5, 0);
        let b = self.extract(25, 7, 5);
        Self::extend(a | b, 12)
    }

    #[inline(always)]
    pub fn b_imm(self) -> Imm {
        let a = self.extract(8, 4, 1);
        let b = self.extract(25, 6, 5);
        let c = self.extract(7, 1, 11);
        let d = self.extract(31, 1, 12);
        Self::extend(a | b | c | d, 13)
    }

    #[inline(always)]
    pub fn u_imm(self) -> Imm {
        Self::extend(self.extract(12, 20, 12), 32)
    }

    #[inline(always)]
    pub fn j_imm(self) -> Imm {
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
    fn extract(self, start: u32, len: u32, pos: u32) -> i32 {
        bit_utils::extract(self.0, start, len, pos) as i32
    }

    #[inline(always)]
    fn extend(value: i32, len: u32) -> Imm {
        let shift = 32 - len;
        Imm((value << shift) >> shift)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Instr {
    // Load/store
    Ld(RdRs1Imm),
    Lw(RdRs1Imm),
    Lwu(RdRs1Imm),
    Lh(RdRs1Imm),
    Lhu(RdRs1Imm),
    Lb(RdRs1Imm),
    Lbu(RdRs1Imm),
    Sd(Rs1Rs2Imm),
    Sw(Rs1Rs2Imm),
    Sh(Rs1Rs2Imm),
    Sb(Rs1Rs2Imm),
    // Integer register-immediate
    Addi(RdRs1Imm),
    Slti(RdRs1Imm),
    Sltiu(RdRs1Imm),
    Andi(RdRs1Imm),
    Ori(RdRs1Imm),
    Xori(RdRs1Imm),
    Slli(RdRs1Imm),
    Srli(RdRs1Imm),
    Srai(RdRs1Imm),
    Lui(RdRs1Imm),
    Auipc(RdRs1Imm),
    Mv(RdRs1),
    Li(RdImm),
    Seqz(RdRs1),
    Not(RdRs1),
    Nop,
    // Integer register-immediate (rv64)
    Addiw(RdRs1Imm),
    Slliw(RdRs1Imm),
    Srliw(RdRs1Imm),
    Sraiw(RdRs1Imm),
    Sextw(RdRs1),
    // Integer register-register
    Add(RdRs1Rs2),
    Sub(RdRs1Rs2),
    Slt(RdRs1Rs2),
    Sltu(RdRs1Rs2),
    And(RdRs1Rs2),
    Or(RdRs1Rs2),
    Xor(RdRs1Rs2),
    Sll(RdRs1Rs2),
    Srl(RdRs1Rs2),
    Sra(RdRs1Rs2),
    Snez(RdRs1),
    Sltz(RdRs1),
    Sgtz(RdRs1),
    Neg(RdRs1),
    // Integer register-register (rv64)
    Addw(RdRs1Rs2),
    Subw(RdRs1Rs2),
    Sllw(RdRs1Rs2),
    Srlw(RdRs1Rs2),
    Sraw(RdRs1Rs2),
    // Multiplication
    Mul(RdRs1Rs2),
    Mulh(RdRs1Rs2),
    Mulhu(RdRs1Rs2),
    Mulhsu(RdRs1Rs2),
    Div(RdRs1Rs2),
    Divu(RdRs1Rs2),
    Rem(RdRs1Rs2),
    Remu(RdRs1Rs2),
    // Multiplication
    Mulw(RdRs1Rs2),
    Divw(RdRs1Rs2),
    Divuw(RdRs1Rs2),
    Remw(RdRs1Rs2),
    Remuw(RdRs1Rs2),
    // Branch
    Beq(Rs1Rs2Imm),
    Bne(Rs1Rs2Imm),
    Blt(Rs1Rs2Imm),
    Bltu(Rs1Rs2Imm),
    Bge(Rs1Rs2Imm),
    Bgeu(Rs1Rs2Imm),
    Bgt(Rs1Rs2Imm),
    Bgtu(Rs1Rs2Imm),
    Ble(Rs1Rs2Imm),
    Bleu(Rs1Rs2Imm),
    Beqz(Rs1Imm),
    Bnez(Rs1Imm),
    Blez(Rs1Imm),
    Bgez(Rs1Imm),
    Bltz(Rs1Imm),
    Bgtz(Rs1Imm),
    // Jump and link
    Jal(RdImm),
    Jalr(RdRs1Imm),
    Jump(Imm),
    Jr(Rs1Imm),
    Ret,
    // Misc mem
    Fence(FenceArgs),
    FenceBare,
    FenceTso,
    Pause,
    Fencei,
    // System
    Ecall,
    Ebreak,
    // Illegal
    Illegal(EncInstr),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RdRs1Imm {
    pub rd: Reg,
    pub rs1: Reg,
    pub imm: Imm,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rs1Rs2Imm {
    pub rs1: Reg,
    pub rs2: Reg,
    pub imm: Imm,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RdRs1Rs2 {
    pub rd: Reg,
    pub rs1: Reg,
    pub rs2: Reg,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RdRs1 {
    pub rd: Reg,
    pub rs1: Reg,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RdImm {
    pub rd: Reg,
    pub imm: Imm,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rs1Imm {
    pub rs1: Reg,
    pub imm: Imm,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FenceArgs {
    pub pred: FenceFlags,
    pub succ: FenceFlags,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FenceFlags(u8);

impl FenceFlags {
    pub const INPUT: Self = Self(0b1000);
    pub const OUTPUT: Self = Self(0b0100);
    pub const READ: Self = Self(0b0010);
    pub const WRITE: Self = Self(0b0001);

    pub const NONE: Self = Self(0b0000);
    pub const ALL: Self = Self(0b1111);

    pub fn new(raw: u8) -> Self {
        assert!(raw < 16);
        Self(raw)
    }

    pub fn test(self, flags: Self) -> bool {
        (self.0 & flags.0) == flags.0
    }

    pub fn raw(self) -> u8 {
        self.0
    }
}

impl BitOr for FenceFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitAnd for FenceFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}
