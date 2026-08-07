use super::*;

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
