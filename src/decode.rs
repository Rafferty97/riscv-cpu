use std::fmt::Display;

use crate::instr::{EncInstr, Reg, Word};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Instr {
    // Integer register-immediate
    Addi(IntImm),
    Slti(IntImm),
    Sltiu(IntImm),
    Andi(IntImm),
    Ori(IntImm),
    Xori(IntImm),
    Slli(IntImm),
    Srli(IntImm),
    Srai(IntImm),
    Lui(IntImm),
    Auipc(IntImm),
    Mv { rd: Reg, rs1: Reg },
    // Integer register-register
    Add(IntReg),
    Sub(IntReg),
    Slt(IntReg),
    Sltu(IntReg),
    And(IntReg),
    Or(IntReg),
    Xor(IntReg),
    Sll(IntReg),
    Srl(IntReg),
    Sra(IntReg),
    // Multiplication
    Mul(IntReg),
    Mulh(IntReg),
    Mulhu(IntReg),
    Mulhsu(IntReg),
    Div(IntReg),
    Divu(IntReg),
    Rem(IntReg),
    Remu(IntReg),
    // Illegal
    Illegal,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct IntImm {
    pub rd: Reg,
    pub rs1: Reg,
    pub imm: Word,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct IntReg {
    pub rd: Reg,
    pub rs1: Reg,
    pub rs2: Reg,
}

impl Instr {
    pub fn decode(enc: EncInstr) -> Self {
        match enc.opcode() {
            // 0b0000011 => self.execute_load(ins),
            // 0b0100011 => self.execute_store(ins),
            0b0010011 => Self::decode_arith_imm(enc),
            0b0110011 => Self::decode_arith(enc),
            // 0b1100011 => self.execute_branch(ins),
            // 0b1101111 => self.execute_jal_imm(ins),
            // 0b1100111 => self.execute_jal_reg(ins),
            // 0b0110111 => self.execute_load_upper_imm(ins),
            // 0b0010111 => self.execute_add_upper_imm_to_pc(ins),
            // 0b0001111 => self.execute_misc_mem(ins),
            _ => Self::Illegal,
        }
    }

    fn decode_arith_imm(enc: EncInstr) -> Self {
        const F0: u32 = 0b0000000;
        const F1: u32 = 0b0100000;

        let operands = IntImm { rd: enc.rd(), rs1: enc.rs1(), imm: enc.i_imm() };

        match (enc.fn3(), enc.fn7()) {
            (0b000, _) => match operands.imm {
                Word::ZERO => Self::Mv { rd: operands.rd, rs1: operands.rs1 },
                _ => Self::Addi(operands),
            },
            (0b001, F0) => Self::Slli(operands),
            (0b010, _) => Self::Slti(operands),
            (0b011, _) => Self::Sltiu(operands),
            (0b100, _) => Self::Xori(operands),
            (0b101, F0) => Self::Srli(operands),
            (0b101, F1) => Self::Srai(operands),
            (0b110, _) => Self::Ori(operands),
            (0b111, _) => Self::Andi(operands),
            _ => Self::Illegal,
        }
    }

    fn decode_arith(enc: EncInstr) -> Self {
        let operands = IntReg { rd: enc.rd(), rs1: enc.rs1(), rs2: enc.rs2() };

        match enc.fn7() {
            0b0000000 => Self::decode_arith_int(enc, operands, false),
            0b0100000 => Self::decode_arith_int(enc, operands, true),
            0b0000001 => Self::decode_arith_mul(enc, operands),
            _ => Self::Illegal,
        }
    }

    fn decode_arith_int(enc: EncInstr, operands: IntReg, alt: bool) -> Self {
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
            _ => Self::Illegal,
        }
    }

    fn decode_arith_mul(enc: EncInstr, operands: IntReg) -> Self {
        match enc.fn3() {
            0b000 => Self::Mul(operands),
            0b001 => Self::Mulh(operands),
            0b010 => Self::Mulhsu(operands),
            0b011 => Self::Mulhu(operands),
            0b100 => Self::Div(operands),
            0b101 => Self::Divu(operands),
            0b110 => Self::Rem(operands),
            0b111 => Self::Remu(operands),
            _ => Self::Illegal,
        }
    }
}
