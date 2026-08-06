use std::fmt::Display;

use crate::instr::{EncInstr, Reg, Word};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Instr {
    // Load/store
    Lw(RdRs1Imm),
    Lh(RdRs1Imm),
    Lhu(RdRs1Imm),
    Lb(RdRs1Imm),
    Lbu(RdRs1Imm),
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
    Seqz(RdRs1),
    Not(RdRs1),
    Nop,
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
    // Multiplication
    Mul(RdRs1Rs2),
    Mulh(RdRs1Rs2),
    Mulhu(RdRs1Rs2),
    Mulhsu(RdRs1Rs2),
    Div(RdRs1Rs2),
    Divu(RdRs1Rs2),
    Rem(RdRs1Rs2),
    Remu(RdRs1Rs2),
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
    // Jump and link
    Jal(RdImm),
    Jalr(RdRs1Imm),
    Jump(Imm),
    Jr(Rs1Imm),
    Ret,
    // Illegal
    Illegal(EncInstr),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RdRs1Imm {
    pub rd: Reg,
    pub rs1: Reg,
    pub imm: Word,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rs1Rs2Imm {
    pub rs1: Reg,
    pub rs2: Reg,
    pub imm: Word,
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
    pub imm: Word,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Imm {
    pub imm: Word,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rs1Imm {
    pub rs1: Reg,
    pub imm: Word,
}

impl Instr {
    pub fn decode_with_pseudos(enc: EncInstr) -> Self {
        Self::decode(enc).add_pseudos()
    }

    pub fn decode(enc: EncInstr) -> Self {
        match enc.opcode() {
            0b0000011 => Self::decode_load(enc),
            0b0100011 => Self::decode_store(enc),
            0b0010011 => Self::decode_int_imm(enc),
            0b0110011 => Self::decode_int(enc),
            0b1100011 => Self::decode_branch(enc),
            0b1101111 => Self::Jal(RdImm { rd: enc.rd(), imm: enc.j_imm() }),
            0b1100111 => Self::Jalr(RdRs1Imm { rd: enc.rd(), rs1: enc.rs1(), imm: enc.i_imm() }),
            0b0110111 => Self::Lui(RdRs1Imm { rd: enc.rd(), rs1: enc.rs1(), imm: enc.u_imm() }),
            0b0010111 => Self::Auipc(RdRs1Imm { rd: enc.rd(), rs1: enc.rs1(), imm: enc.u_imm() }),
            0b0001111 => Self::decode_misc_mem(enc),
            _ => Self::Illegal(enc),
        }
    }

    fn decode_load(enc: EncInstr) -> Self {
        let args = RdRs1Imm { rd: enc.rd(), rs1: enc.rs1(), imm: enc.i_imm() };

        match enc.fn3() {
            0b000 => Self::Lb(args),
            0b001 => Self::Lh(args),
            0b010 => Self::Lw(args),
            0b100 => Self::Lbu(args),
            0b101 => Self::Lhu(args),
            _ => Self::Illegal(enc),
        }
    }

    fn decode_store(enc: EncInstr) -> Self {
        let args = Rs1Rs2Imm { rs1: enc.rs1(), rs2: enc.rs2(), imm: enc.s_imm() };

        match enc.fn3() {
            0b000 => Self::Sb(args),
            0b001 => Self::Sh(args),
            0b010 => Self::Sw(args),
            _ => Self::Illegal(enc),
        }
    }

    fn decode_int_imm(enc: EncInstr) -> Self {
        const F0: u32 = 0b0000000;
        const F1: u32 = 0b0100000;

        let args = RdRs1Imm { rd: enc.rd(), rs1: enc.rs1(), imm: enc.i_imm() };

        match (enc.fn3(), enc.fn7()) {
            (0b000, _) => Self::Addi(args),
            (0b001, F0) => Self::Slli(args),
            (0b010, _) => Self::Slti(args),
            (0b011, _) => Self::Sltiu(args),
            (0b100, _) => Self::Xori(args),
            (0b101, F0) => Self::Srli(args),
            (0b101, F1) => Self::Srai(args),
            (0b110, _) => Self::Ori(args),
            (0b111, _) => Self::Andi(args),
            _ => Self::Illegal(enc),
        }
    }

    fn decode_int(enc: EncInstr) -> Self {
        let args = RdRs1Rs2 { rd: enc.rd(), rs1: enc.rs1(), rs2: enc.rs2() };

        match enc.fn7() {
            0b0000000 => Self::decode_base_int(enc, args, false),
            0b0100000 => Self::decode_base_int(enc, args, true),
            0b0000001 => Self::decode_mul(enc, args),
            _ => Self::Illegal(enc),
        }
    }

    fn decode_base_int(enc: EncInstr, operands: RdRs1Rs2, alt: bool) -> Self {
        match (enc.fn3(), alt) {
            (0b000, false) => Self::Add(operands),
            (0b000, true) => Self::Sub(operands),
            (0b001, false) => Self::Sll(operands),
            (0b010, false) => Self::Slt(operands),
            (0b011, false) => Self::Sltu(operands),
            (0b100, false) => Self::Xor(operands),
            (0b101, false) => Self::Srl(operands),
            (0b101, true) => Self::Sra(operands),
            (0b110, false) => Self::Or(operands),
            (0b111, false) => Self::And(operands),
            _ => Self::Illegal(enc),
        }
    }

    fn decode_mul(enc: EncInstr, operands: RdRs1Rs2) -> Self {
        match enc.fn3() {
            0b000 => Self::Mul(operands),
            0b001 => Self::Mulh(operands),
            0b010 => Self::Mulhsu(operands),
            0b011 => Self::Mulhu(operands),
            0b100 => Self::Div(operands),
            0b101 => Self::Divu(operands),
            0b110 => Self::Rem(operands),
            0b111 => Self::Remu(operands),
            _ => Self::Illegal(enc),
        }
    }

    fn decode_branch(enc: EncInstr) -> Self {
        let args = Rs1Rs2Imm { rs1: enc.rs1(), rs2: enc.rs2(), imm: enc.b_imm() };

        match enc.fn3() {
            0b000 => Self::Beq(args),
            0b001 => Self::Bne(args),
            0b100 => Self::Blt(args),
            0b101 => Self::Bge(args),
            0b110 => Self::Bltu(args),
            0b111 => Self::Bgeu(args),
            _ => Self::Illegal(enc),
        }
    }

    fn decode_misc_mem(enc: EncInstr) -> Self {
        match enc.fn3() {
            0b000 | 0b001 => todo!(),
            _ => Self::Illegal(enc),
        }
    }
}

impl Instr {
    pub fn remove_pseudos(self) -> Self {
        match self {
            Self::Mv(RdRs1 { rd, rs1 }) => Self::Addi(RdRs1Imm { rd, rs1, imm: Word::ZERO }),
            Self::Seqz(RdRs1 { rd, rs1 }) => Self::Sltiu(RdRs1Imm { rd, rs1, imm: Word::ONE }),
            Self::Not(RdRs1 { rd, rs1 }) => Self::Xori(RdRs1Imm { rd, rs1, imm: Word::NEG_ONE }),
            Self::Nop => Self::Addi(RdRs1Imm { rd: Reg::ZERO, rs1: Reg::ZERO, imm: Word::ZERO }),
            Self::Snez(RdRs1 { rd, rs1 }) => Self::Sltu(RdRs1Rs2 { rd, rs1: Reg::ZERO, rs2: rs1 }),
            Self::Bgt(Rs1Rs2Imm { rs1, rs2, imm }) => Self::Blt(Rs1Rs2Imm { rs1: rs2, rs2: rs1, imm }),
            Self::Bgtu(Rs1Rs2Imm { rs1, rs2, imm }) => Self::Bltu(Rs1Rs2Imm { rs1: rs2, rs2: rs1, imm }),
            Self::Ble(Rs1Rs2Imm { rs1, rs2, imm }) => Self::Bge(Rs1Rs2Imm { rs1: rs2, rs2: rs1, imm }),
            Self::Bleu(Rs1Rs2Imm { rs1, rs2, imm }) => Self::Bgeu(Rs1Rs2Imm { rs1: rs2, rs2: rs1, imm }),
            Self::Jump(Imm { imm }) => Self::Jal(RdImm { rd: Reg::ZERO, imm }),
            Self::Jr(Rs1Imm { rs1, imm }) => Self::Jalr(RdRs1Imm { rd: Reg::ZERO, rs1, imm }),
            Self::Ret => Self::Jalr(RdRs1Imm { rd: Reg::ZERO, rs1: Reg::RA, imm: Word::ZERO }),
            _ => self,
        }
    }

    pub fn add_pseudos(self) -> Self {
        match self {
            Self::Addi(RdRs1Imm { rd, rs1, imm }) => match (rd, rs1, imm) {
                (Reg::ZERO, Reg::ZERO, Word::ZERO) => Self::Nop,
                (_, _, Word::ZERO) => Self::Mv(RdRs1 { rd, rs1 }),
                _ => self,
            },
            Self::Sltiu(RdRs1Imm { rd, rs1, imm }) => match imm {
                Word::ONE => Self::Seqz(RdRs1 { rd, rs1 }),
                _ => self,
            },
            Self::Xori(RdRs1Imm { rd, rs1, imm }) => match imm {
                Word::NEG_ONE => Self::Not(RdRs1 { rd, rs1 }),
                _ => self,
            },
            Self::Sltu(RdRs1Rs2 { rd, rs1, rs2 }) => match rs1 {
                Reg::ZERO => Self::Snez(RdRs1 { rd, rs1: rs2 }),
                _ => self,
            },
            Self::Jal(RdImm { rd, imm }) => match rd {
                Reg::ZERO => Self::Jump(Imm { imm }),
                _ => Self::Jal(RdImm { rd, imm }),
            },
            Self::Jalr(RdRs1Imm { rd, rs1, imm }) => match (rd, rs1, imm) {
                (Reg::ZERO, Reg::RA, Word::ZERO) => Self::Ret,
                (Reg::ZERO, _, _) => Self::Jr(Rs1Imm { rs1, imm }),
                _ => Self::Jalr(RdRs1Imm { rd, rs1, imm }),
            },
            _ => self,
        }
    }
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn write(f: &mut std::fmt::Formatter<'_>, mnem: &str, args: impl Display) -> std::fmt::Result {
            write!(f, "{mnem: <6}{args}")
        }

        match self {
            Self::Lw(args) => write(f, "lw", args),
            Self::Lh(args) => write(f, "lh", args),
            Self::Lhu(args) => write(f, "lhu", args),
            Self::Lb(args) => write(f, "lb", args),
            Self::Lbu(args) => write(f, "lbu", args),
            Self::Sw(args) => write(f, "sw", args),
            Self::Sh(args) => write(f, "sh", args),
            Self::Sb(args) => write(f, "sb", args),
            Self::Addi(args) => write(f, "addi", args),
            Self::Slti(args) => write(f, "slti", args),
            Self::Sltiu(args) => write(f, "sltiu", args),
            Self::Andi(args) => write(f, "andi", args),
            Self::Ori(args) => write(f, "ori", args),
            Self::Xori(args) => write(f, "xori", args),
            Self::Slli(args) => write(f, "slli", args),
            Self::Srli(args) => write(f, "srli", args),
            Self::Srai(args) => write(f, "srai", args),
            Self::Lui(args) => write(f, "lui", args),
            Self::Auipc(args) => write(f, "auipc", args),
            Self::Mv(args) => write(f, "mv", args),
            Self::Seqz(args) => write(f, "seqz", args),
            Self::Not(args) => write(f, "not", args),
            Self::Nop => write(f, "nop", ""),
            Self::Add(args) => write(f, "add", args),
            Self::Sub(args) => write(f, "sub", args),
            Self::Slt(args) => write(f, "slt", args),
            Self::Sltu(args) => write(f, "sltu", args),
            Self::And(args) => write(f, "and", args),
            Self::Or(args) => write(f, "or", args),
            Self::Xor(args) => write(f, "xor", args),
            Self::Sll(args) => write(f, "sll", args),
            Self::Srl(args) => write(f, "srl", args),
            Self::Sra(args) => write(f, "sra", args),
            Self::Snez(args) => write(f, "snez", args),
            Self::Mul(args) => write(f, "mul", args),
            Self::Mulh(args) => write(f, "mulh", args),
            Self::Mulhu(args) => write(f, "mulhu", args),
            Self::Mulhsu(args) => write(f, "mulhsu", args),
            Self::Div(args) => write(f, "div", args),
            Self::Divu(args) => write(f, "divu", args),
            Self::Rem(args) => write(f, "rem", args),
            Self::Remu(args) => write(f, "remu", args),
            Self::Beq(args) => write(f, "beq", args),
            Self::Bne(args) => write(f, "bne", args),
            Self::Blt(args) => write(f, "blt", args),
            Self::Bltu(args) => write(f, "bltu", args),
            Self::Bge(args) => write(f, "bge", args),
            Self::Bgeu(args) => write(f, "bgeu", args),
            Self::Bgt(args) => write(f, "bgt", args),
            Self::Bgtu(args) => write(f, "bgtu", args),
            Self::Ble(args) => write(f, "ble", args),
            Self::Bleu(args) => write(f, "bleu", args),
            Self::Jal(args) => write(f, "jal", args),
            Self::Jalr(args) => write(f, "jalr", args),
            Self::Jump(args) => write(f, "j", args),
            Self::Jr(args) => write(f, "jr", args),
            Self::Ret => write(f, "ret", ""),
            Self::Illegal(enc) => write!(f, "<illegal {:#010x}>", enc.u32()),
        }
    }
}

impl Display for RdRs1Imm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.rd, self.rs1, self.imm)
    }
}

impl Display for Rs1Rs2Imm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.rs1, self.rs2, self.imm)
    }
}

impl Display for RdRs1Rs2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.rd, self.rs1, self.rs2)
    }
}

impl Display for RdRs1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.rd, self.rs1)
    }
}

impl Display for RdImm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.rd, self.imm)
    }
}

impl Display for Imm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.imm)
    }
}

impl Display for Rs1Imm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.rs1, self.imm)
    }
}

impl Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x{}", self.u32())
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.i64())
    }
}
