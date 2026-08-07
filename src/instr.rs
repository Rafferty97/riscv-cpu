use std::ops::{BitAnd, BitOr};

use crate::bit_utils;

mod decode;
mod encode;
mod print;

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

impl Instr {
    pub fn remove_pseudos(self) -> Self {
        match self {
            Self::Mv(RdRs1 { rd, rs1 }) => Self::Addi(RdRs1Imm { rd, rs1, imm: Imm(0) }),
            Self::Li(RdImm { rd, imm }) => Self::Addi(RdRs1Imm { rd, rs1: Reg::ZERO, imm }),
            Self::Seqz(RdRs1 { rd, rs1 }) => Self::Sltiu(RdRs1Imm { rd, rs1, imm: Imm(1) }),
            Self::Not(RdRs1 { rd, rs1 }) => Self::Xori(RdRs1Imm { rd, rs1, imm: Imm(-1) }),
            Self::Nop => Self::Addiw(RdRs1Imm { rd: Reg::ZERO, rs1: Reg::ZERO, imm: Imm(0) }),
            Self::Sextw(RdRs1 { rd, rs1 }) => Self::Addi(RdRs1Imm { rd, rs1, imm: Imm(0) }),
            Self::Snez(RdRs1 { rd, rs1 }) => Self::Sltu(RdRs1Rs2 { rd, rs1: Reg::ZERO, rs2: rs1 }),
            Self::Sltz(RdRs1 { rd, rs1 }) => Self::Slt(RdRs1Rs2 { rd, rs1, rs2: Reg::ZERO }),
            Self::Sgtz(RdRs1 { rd, rs1 }) => Self::Slt(RdRs1Rs2 { rd, rs1: Reg::ZERO, rs2: rs1 }),
            Self::Neg(RdRs1 { rd, rs1 }) => Self::Sub(RdRs1Rs2 { rd, rs1: Reg::ZERO, rs2: rs1 }),
            Self::Bgt(Rs1Rs2Imm { rs1, rs2, imm }) => Self::Blt(Rs1Rs2Imm { rs1: rs2, rs2: rs1, imm }),
            Self::Bgtu(Rs1Rs2Imm { rs1, rs2, imm }) => Self::Bltu(Rs1Rs2Imm { rs1: rs2, rs2: rs1, imm }),
            Self::Ble(Rs1Rs2Imm { rs1, rs2, imm }) => Self::Bge(Rs1Rs2Imm { rs1: rs2, rs2: rs1, imm }),
            Self::Bleu(Rs1Rs2Imm { rs1, rs2, imm }) => Self::Bgeu(Rs1Rs2Imm { rs1: rs2, rs2: rs1, imm }),
            Self::Beqz(Rs1Imm { rs1, imm }) => Self::Beq(Rs1Rs2Imm { rs1, rs2: Reg::ZERO, imm }),
            Self::Bnez(Rs1Imm { rs1, imm }) => Self::Bne(Rs1Rs2Imm { rs1, rs2: Reg::ZERO, imm }),
            Self::Blez(Rs1Imm { rs1, imm }) => Self::Bge(Rs1Rs2Imm { rs1: Reg::ZERO, rs2: rs1, imm }),
            Self::Bgez(Rs1Imm { rs1, imm }) => Self::Bge(Rs1Rs2Imm { rs1, rs2: Reg::ZERO, imm }),
            Self::Bltz(Rs1Imm { rs1, imm }) => Self::Blt(Rs1Rs2Imm { rs1, rs2: Reg::ZERO, imm }),
            Self::Bgtz(Rs1Imm { rs1, imm }) => Self::Blt(Rs1Rs2Imm { rs1: Reg::ZERO, rs2: rs1, imm }),
            Self::Jump(imm) => Self::Jal(RdImm { rd: Reg::ZERO, imm }),
            Self::Jr(Rs1Imm { rs1, imm }) => Self::Jalr(RdRs1Imm { rd: Reg::ZERO, rs1, imm }),
            Self::Ret => Self::Jalr(RdRs1Imm { rd: Reg::ZERO, rs1: Reg::RA, imm: Imm(0) }),
            Self::FenceBare => Self::Fence(FenceArgs::ALL),
            _ => self,
        }
    }

    pub fn add_pseudos(self) -> Self {
        match self {
            Self::Addi(RdRs1Imm { rd, rs1, imm }) => match (rd, rs1, imm) {
                (Reg::ZERO, Reg::ZERO, Imm(0)) => Self::Nop,
                (_, Reg::ZERO, _) => Self::Li(RdImm { rd, imm }),
                (_, _, Imm(0)) => Self::Mv(RdRs1 { rd, rs1 }),
                _ => self,
            },
            Self::Sltiu(RdRs1Imm { rd, rs1, imm }) => match imm {
                Imm(1) => Self::Seqz(RdRs1 { rd, rs1 }),
                _ => self,
            },
            Self::Xori(RdRs1Imm { rd, rs1, imm }) => match imm {
                Imm(-1) => Self::Not(RdRs1 { rd, rs1 }),
                _ => self,
            },
            Self::Addiw(RdRs1Imm { rd, rs1, imm }) => match imm {
                Imm(0) => Self::Sextw(RdRs1 { rd, rs1 }),
                _ => self,
            },
            Self::Sub(RdRs1Rs2 { rd, rs1, rs2 }) => match rs1 {
                Reg::ZERO => Self::Neg(RdRs1 { rd, rs1: rs2 }),
                _ => self,
            },
            Self::Slt(RdRs1Rs2 { rd, rs1, rs2 }) => match (rs1, rs2) {
                (_, Reg::ZERO) => Self::Sltz(RdRs1 { rd, rs1 }),
                (Reg::ZERO, _) => Self::Sgtz(RdRs1 { rd, rs1: rs2 }),
                _ => self,
            },
            Self::Sltu(RdRs1Rs2 { rd, rs1, rs2 }) => match rs1 {
                Reg::ZERO => Self::Snez(RdRs1 { rd, rs1: rs2 }),
                _ => self,
            },
            Self::Beq(Rs1Rs2Imm { rs1, rs2, imm }) => match rs2 {
                Reg::ZERO => Self::Beqz(Rs1Imm { rs1, imm }),
                _ => self,
            },
            Self::Bne(Rs1Rs2Imm { rs1, rs2, imm }) => match rs2 {
                Reg::ZERO => Self::Bnez(Rs1Imm { rs1, imm }),
                _ => self,
            },
            Self::Blt(Rs1Rs2Imm { rs1, rs2, imm }) => match rs2 {
                Reg::ZERO => Self::Bltz(Rs1Imm { rs1, imm }),
                _ => self,
            },
            Self::Bge(Rs1Rs2Imm { rs1, rs2, imm }) => match rs2 {
                Reg::ZERO => Self::Bgez(Rs1Imm { rs1, imm }),
                _ => self,
            },
            Self::Jal(RdImm { rd, imm }) => match rd {
                Reg::ZERO => Self::Jump(imm),
                _ => self,
            },
            Self::Jalr(RdRs1Imm { rd, rs1, imm }) => match (rd, rs1, imm) {
                (Reg::ZERO, Reg::RA, Imm(0)) => Self::Ret,
                (Reg::ZERO, _, _) => Self::Jr(Rs1Imm { rs1, imm }),
                _ => self,
            },
            Self::Fence(FenceArgs::ALL) => Self::FenceBare,
            _ => self,
        }
    }
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

impl FenceArgs {
    pub const ALL: Self = Self { pred: FenceFlags::ALL, succ: FenceFlags::ALL };
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
        Self(raw & 15)
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Reg(u8);

impl Reg {
    pub const ZERO: Self = Self(0);
    pub const RA: Self = Self(0);

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
