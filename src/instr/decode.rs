use super::*;

#[derive(Clone, Copy, Debug)]
pub struct InstrDecoder {
    pub with_pseudos: bool,
}

impl InstrDecoder {
    pub fn decode(&self, enc: EncInstr) -> Instr {
        let ins = Instr::decode(enc);
        match self.with_pseudos {
            true => ins.add_pseudos(),
            false => ins,
        }
    }
}

impl Instr {
    fn decode(enc: EncInstr) -> Self {
        match enc.opcode() {
            0b0000011 => Self::decode_load(enc),
            0b0100011 => Self::decode_store(enc),
            0b0010011 => Self::decode_op_imm(enc),
            0b0011011 => Self::decode_op_imm_32(enc),
            0b0110011 => Self::decode_op(enc),
            0b0111011 => Self::decode_op_32(enc),
            0b1100011 => Self::decode_branch(enc),
            0b1101111 => Self::Jal(RdImm { rd: enc.rd(), imm: enc.j_imm() }),
            0b1100111 => Self::Jalr(RdRs1Imm { rd: enc.rd(), rs1: enc.rs1(), imm: enc.i_imm() }),
            0b0010111 => Self::Auipc(RdRs1Imm { rd: enc.rd(), rs1: enc.rs1(), imm: enc.u_imm() }),
            0b0110111 => Self::Lui(RdRs1Imm { rd: enc.rd(), rs1: enc.rs1(), imm: enc.u_imm() }),
            0b0001111 => Self::decode_misc_mem(enc),
            0b1110011 => Self::decode_system(enc),
            _ => Self::Illegal(enc),
        }
    }

    fn decode_load(enc: EncInstr) -> Self {
        let args = RdRs1Imm { rd: enc.rd(), rs1: enc.rs1(), imm: enc.i_imm() };

        match enc.fn3() {
            0b000 => Self::Lb(args),
            0b001 => Self::Lh(args),
            0b010 => Self::Lw(args),
            0b011 => Self::Ld(args),
            0b100 => Self::Lbu(args),
            0b101 => Self::Lhu(args),
            0b110 => Self::Lwu(args),
            _ => Self::Illegal(enc),
        }
    }

    fn decode_store(enc: EncInstr) -> Self {
        let args = Rs1Rs2Imm { rs1: enc.rs1(), rs2: enc.rs2(), imm: enc.s_imm() };

        match enc.fn3() {
            0b000 => Self::Sb(args),
            0b001 => Self::Sh(args),
            0b010 => Self::Sw(args),
            0b011 => Self::Sd(args),
            _ => Self::Illegal(enc),
        }
    }

    fn decode_op_imm(enc: EncInstr) -> Self {
        const F0: u32 = 0b000000;
        const F1: u32 = 0b010000;

        let (rd, rs1, imm) = (enc.rd(), enc.rs1(), enc.i_imm());
        let args = RdRs1Imm { rd, rs1, imm };

        match (enc.fn3(), enc.fn7() >> 1) {
            (0b000, _) => Self::Addi(args),
            (0b001, F0) => Self::Slli(RdRs1Imm { rd, rs1, imm: Imm(imm.0 & 63) }),
            (0b010, _) => Self::Slti(args),
            (0b011, _) => Self::Sltiu(args),
            (0b100, _) => Self::Xori(args),
            (0b101, F0) => Self::Srli(RdRs1Imm { rd, rs1, imm: Imm(imm.0 & 63) }),
            (0b101, F1) => Self::Srai(RdRs1Imm { rd, rs1, imm: Imm(imm.0 & 63) }),
            (0b110, _) => Self::Ori(args),
            (0b111, _) => Self::Andi(args),
            _ => Self::Illegal(enc),
        }
    }

    fn decode_op_imm_32(enc: EncInstr) -> Self {
        const F0: u32 = 0b0000000;
        const F1: u32 = 0b0100000;

        let (rd, rs1, imm) = (enc.rd(), enc.rs1(), enc.i_imm());
        let args = RdRs1Imm { rd, rs1, imm };

        match (enc.fn3(), enc.fn7()) {
            (0b000, _) => Self::Addiw(args),
            (0b001, F0) => Self::Slliw(RdRs1Imm { rd, rs1, imm: Imm(imm.0 & 31) }),
            (0b101, F0) => Self::Srliw(RdRs1Imm { rd, rs1, imm: Imm(imm.0 & 31) }),
            (0b101, F1) => Self::Sraiw(RdRs1Imm { rd, rs1, imm: Imm(imm.0 & 31) }),
            _ => Self::Illegal(enc),
        }
    }

    fn decode_op(enc: EncInstr) -> Self {
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

    fn decode_op_32(enc: EncInstr) -> Self {
        let args = RdRs1Rs2 { rd: enc.rd(), rs1: enc.rs1(), rs2: enc.rs2() };

        match enc.fn7() {
            0b0000000 => Self::decode_base_int_32(enc, args, false),
            0b0100000 => Self::decode_base_int_32(enc, args, true),
            0b0000001 => Self::decode_mul_32(enc, args),
            _ => Self::Illegal(enc),
        }
    }

    fn decode_base_int_32(enc: EncInstr, operands: RdRs1Rs2, alt: bool) -> Self {
        match (enc.fn3(), alt) {
            (0b000, false) => Self::Addw(operands),
            (0b000, true) => Self::Subw(operands),
            (0b001, false) => Self::Sllw(operands),
            (0b101, false) => Self::Srlw(operands),
            (0b101, true) => Self::Sraw(operands),
            _ => Self::Illegal(enc),
        }
    }

    fn decode_mul_32(enc: EncInstr, operands: RdRs1Rs2) -> Self {
        match enc.fn3() {
            0b000 => Self::Mulw(operands),
            0b100 => Self::Divw(operands),
            0b101 => Self::Divuw(operands),
            0b110 => Self::Remw(operands),
            0b111 => Self::Remuw(operands),
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
            0b000 => Self::decode_fence(enc),
            0b001 => Self::Fencei,
            _ => Self::Illegal(enc),
        }
    }

    fn decode_fence(enc: EncInstr) -> Self {
        let imm = enc.i_imm().0;
        match imm {
            0b0000_0001_0000 => Self::Pause,
            0b0000_0000_0000..=0b0000_1111_1111 => {
                let pred = FenceFlags::new((imm >> 4) as u8);
                let succ = FenceFlags::new((imm >> 0) as u8);
                Self::Fence(FenceArgs { pred, succ })
            }
            0b1000_0011_0011 => Self::FenceTso,
            _ => Self::Illegal(enc),
        }
    }

    fn decode_system(enc: EncInstr) -> Self {
        if enc.rd() != Reg::ZERO || enc.rs1() != Reg::ZERO || enc.fn3() != 0 {
            return Self::Illegal(enc);
        }

        match enc.fn12() {
            0b0000_0000_0000 => Self::Ecall,
            0b0000_0000_0001 => Self::Ebreak,
            _ => Self::Illegal(enc),
        }
    }
}

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
