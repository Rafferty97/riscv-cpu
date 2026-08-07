use std::fmt::{Display, Formatter};
use std::sync::LazyLock;

use super::*;

#[derive(Clone, Copy, Debug)]
pub struct AsmPrinter {
    align: bool,
}

impl AsmPrinter {
    pub fn to_string(&self, ins: Instr) -> String {
        format!("{}", PrintAsm((ins, self)))
    }
}

struct PrintAsm<T>(T);

impl Display for PrintAsm<(Instr, &AsmPrinter)> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (ins, opts) = self.0;

        if let Instr::Illegal(enc) = ins {
            return write!(f, "<illegal {:#010x}>", enc.u32());
        }

        let (mnem, args) = ins.menm_and_args();
        match opts.align {
            true => write!(f, "{mnem: <10}{args}"),
            false => write!(f, "{mnem} {args}"),
        }
    }
}

impl Instr {
    fn menm_and_args(self) -> (&'static str, Arguments) {
        match self {
            Self::Ld(args) => ("ld", args.into()),
            Self::Lw(args) => ("lw", args.into()),
            Self::Lwu(args) => ("lwu", args.into()),
            Self::Lh(args) => ("lh", args.into()),
            Self::Lhu(args) => ("lhu", args.into()),
            Self::Lb(args) => ("lb", args.into()),
            Self::Lbu(args) => ("lbu", args.into()),
            Self::Sd(args) => ("sd", args.into()),
            Self::Sw(args) => ("sw", args.into()),
            Self::Sh(args) => ("sh", args.into()),
            Self::Sb(args) => ("sb", args.into()),
            Self::Addi(args) => ("addi", args.into()),
            Self::Slti(args) => ("slti", args.into()),
            Self::Sltiu(args) => ("sltiu", args.into()),
            Self::Andi(args) => ("andi", args.into()),
            Self::Ori(args) => ("ori", args.into()),
            Self::Xori(args) => ("xori", args.into()),
            Self::Slli(args) => ("slli", args.into()),
            Self::Srli(args) => ("srli", args.into()),
            Self::Srai(args) => ("srai", args.into()),
            Self::Lui(args) => ("lui", args.into()),
            Self::Auipc(args) => ("auipc", args.into()),
            Self::Mv(args) => ("mv", args.into()),
            Self::Li(args) => ("li", args.into()),
            Self::Seqz(args) => ("seqz", args.into()),
            Self::Not(args) => ("not", args.into()),
            Self::Nop => ("nop", Default::default()),
            Self::Addiw(args) => ("addiw", args.into()),
            Self::Slliw(args) => ("slliw", args.into()),
            Self::Srliw(args) => ("srliw", args.into()),
            Self::Sraiw(args) => ("sraiw", args.into()),
            Self::Sextw(args) => ("sext.w", args.into()),
            Self::Add(args) => ("add", args.into()),
            Self::Sub(args) => ("sub", args.into()),
            Self::Slt(args) => ("slt", args.into()),
            Self::Sltu(args) => ("sltu", args.into()),
            Self::And(args) => ("and", args.into()),
            Self::Or(args) => ("or", args.into()),
            Self::Xor(args) => ("xor", args.into()),
            Self::Sll(args) => ("sll", args.into()),
            Self::Srl(args) => ("srl", args.into()),
            Self::Sra(args) => ("sra", args.into()),
            Self::Snez(args) => ("snez", args.into()),
            Self::Sltz(args) => ("sltz", args.into()),
            Self::Sgtz(args) => ("sgtz", args.into()),
            Self::Neg(args) => ("neg", args.into()),
            Self::Addw(args) => ("addw", args.into()),
            Self::Subw(args) => ("subw", args.into()),
            Self::Sllw(args) => ("sllw", args.into()),
            Self::Srlw(args) => ("srlw", args.into()),
            Self::Sraw(args) => ("sraw", args.into()),
            Self::Mul(args) => ("mul", args.into()),
            Self::Mulh(args) => ("mulh", args.into()),
            Self::Mulhu(args) => ("mulhu", args.into()),
            Self::Mulhsu(args) => ("mulhsu", args.into()),
            Self::Div(args) => ("div", args.into()),
            Self::Divu(args) => ("divu", args.into()),
            Self::Rem(args) => ("rem", args.into()),
            Self::Remu(args) => ("remu", args.into()),
            Self::Mulw(args) => ("mulw", args.into()),
            Self::Divw(args) => ("divw", args.into()),
            Self::Divuw(args) => ("divuw", args.into()),
            Self::Remw(args) => ("remw", args.into()),
            Self::Remuw(args) => ("remuw", args.into()),
            Self::Beq(args) => ("beq", args.into()),
            Self::Bne(args) => ("bne", args.into()),
            Self::Blt(args) => ("blt", args.into()),
            Self::Bltu(args) => ("bltu", args.into()),
            Self::Bge(args) => ("bge", args.into()),
            Self::Bgeu(args) => ("bgeu", args.into()),
            Self::Bgt(args) => ("bgt", args.into()),
            Self::Bgtu(args) => ("bgtu", args.into()),
            Self::Ble(args) => ("ble", args.into()),
            Self::Bleu(args) => ("bleu", args.into()),
            Self::Beqz(args) => ("beqz", args.into()),
            Self::Bnez(args) => ("bnez", args.into()),
            Self::Blez(args) => ("blez", args.into()),
            Self::Bgez(args) => ("bgez", args.into()),
            Self::Bltz(args) => ("bltz", args.into()),
            Self::Bgtz(args) => ("bgtz", args.into()),
            Self::Jal(args) => ("jal", args.into()),
            Self::Jalr(args) => ("jalr", args.into()),
            Self::Jump(args) => ("j", args.into()),
            Self::Jr(args) => ("jr", args.into()),
            Self::Ret => ("ret", Default::default()),
            Self::Fence(args) => ("fence", args.into()),
            Self::FenceBare => ("fence", Default::default()),
            Self::FenceTso => ("fence.tso", Default::default()),
            Self::Pause => ("pause", Default::default()),
            Self::Fencei => ("fence.i", Default::default()),
            Self::Ecall => ("ecall", Default::default()),
            Self::Ebreak => ("ebreak", Default::default()),
            Self::Illegal(_) => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Default)]
struct Arguments {
    rd: Option<Reg>,
    rs1: Option<Reg>,
    rs2: Option<Reg>,
    imm: Option<Imm>,
    fence_pred: Option<FenceFlags>,
    fence_succ: Option<FenceFlags>,
}

impl From<super::RdRs1Imm> for Arguments {
    fn from(value: super::RdRs1Imm) -> Self {
        Self {
            rd: Some(value.rd),
            rs1: Some(value.rs1),
            imm: Some(value.imm),
            ..Default::default()
        }
    }
}

impl From<super::Rs1Rs2Imm> for Arguments {
    fn from(value: super::Rs1Rs2Imm) -> Self {
        Self {
            rs1: Some(value.rs1),
            rs2: Some(value.rs2),
            imm: Some(value.imm),
            ..Default::default()
        }
    }
}

impl From<super::RdRs1Rs2> for Arguments {
    fn from(value: super::RdRs1Rs2) -> Self {
        Self {
            rd: Some(value.rd),
            rs1: Some(value.rs1),
            rs2: Some(value.rs2),
            ..Default::default()
        }
    }
}

impl From<super::RdRs1> for Arguments {
    fn from(value: super::RdRs1) -> Self {
        Self { rd: Some(value.rd), rs1: Some(value.rs1), ..Default::default() }
    }
}

impl From<super::RdImm> for Arguments {
    fn from(value: super::RdImm) -> Self {
        Self { rd: Some(value.rd), imm: Some(value.imm), ..Default::default() }
    }
}

impl From<super::Rs1Imm> for Arguments {
    fn from(value: super::Rs1Imm) -> Self {
        Self { rs1: Some(value.rs1), imm: Some(value.imm), ..Default::default() }
    }
}

impl From<Imm> for Arguments {
    fn from(value: Imm) -> Self {
        Self { imm: Some(value), ..Default::default() }
    }
}

impl From<super::FenceArgs> for Arguments {
    fn from(value: super::FenceArgs) -> Self {
        Self { fence_pred: Some(value.pred), fence_succ: Some(value.succ), ..Default::default() }
    }
}

impl Display for Arguments {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn print(f: &mut Formatter<'_>, value: Option<impl Display>, first: &mut bool) -> std::fmt::Result {
            let Some(value) = value else {
                return Ok(());
            };

            match std::mem::replace(first, false) {
                true => write!(f, "{value}"),
                false => write!(f, ", {value}"),
            }
        }

        let mut first = true;
        print(f, self.rd.map(PrintAsm), &mut first)?;
        print(f, self.rs1.map(PrintAsm), &mut first)?;
        print(f, self.rs2.map(PrintAsm), &mut first)?;
        print(f, self.imm.map(PrintAsm), &mut first)?;
        print(f, self.fence_pred.map(PrintAsm), &mut first)?;
        print(f, self.fence_succ.map(PrintAsm), &mut first)?;

        Ok(())
    }
}

impl Display for PrintAsm<FenceFlags> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        static LOOKUP_TABLE: LazyLock<[&str; 16]> = LazyLock::new(|| {
            let buffer: Vec<u8> = (0..16)
                .map(|i| FenceFlags::new(i))
                .flat_map(|flags| {
                    let chars = [
                        flags.test(FenceFlags::INPUT).then_some(b'i'),
                        flags.test(FenceFlags::OUTPUT).then_some(b'o'),
                        flags.test(FenceFlags::READ).then_some(b'r'),
                        flags.test(FenceFlags::WRITE).then_some(b'w'),
                    ];
                    chars.into_iter().flatten().chain(std::iter::repeat(b' ')).take(4)
                })
                .collect();
            let buffer = Box::leak(Box::new(buffer));

            std::array::from_fn(|i| {
                let buf = &buffer[4 * i..][..4];
                std::str::from_utf8(buf.trim_ascii()).unwrap()
            })
        });

        write!(f, "{}", LOOKUP_TABLE[self.0.raw() as usize])
    }
}

impl Display for PrintAsm<Reg> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x{}", self.0.raw())
    }
}

impl Display for PrintAsm<Imm> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.0)
    }
}

#[cfg(test)]
mod test {
    use std::fmt::Display;

    use crate::instr::{FenceArgs, RdRs1, RdRs1Imm};

    use super::*;

    fn print<T>(value: T) -> String
    where
        PrintAsm<T>: Display,
    {
        PrintAsm(value).to_string()
    }

    #[test]
    fn test_instructions() {
        const X0: Reg = Reg::new(0);
        const X1: Reg = Reg::new(1);

        let printer = AsmPrinter { align: false };

        assert_eq!(
            printer.to_string(Instr::Addi(RdRs1Imm { rd: X1, rs1: X0, imm: Imm::from(-32) })),
            "addi x1, x0, -32"
        );
        assert_eq!(printer.to_string(Instr::Mv(RdRs1 { rd: X0, rs1: X1 })), "mv x0, x1");
        assert_eq!(
            printer.to_string(Instr::Fence(FenceArgs {
                pred: FenceFlags::READ,
                succ: FenceFlags::OUTPUT | FenceFlags::WRITE
            })),
            "fence r, ow"
        );
    }

    #[test]
    fn test_fence_flags() {
        assert_eq!(print(FenceFlags::INPUT), "i");
        assert_eq!(print(FenceFlags::OUTPUT), "o");
        assert_eq!(print(FenceFlags::READ), "r");
        assert_eq!(print(FenceFlags::WRITE), "w");

        assert_eq!(print(FenceFlags::NONE), "");
        assert_eq!(print(FenceFlags::ALL), "iorw");

        assert_eq!(print(FenceFlags::INPUT | FenceFlags::READ), "ir");
        assert_eq!(print(FenceFlags::OUTPUT | FenceFlags::WRITE), "ow");
    }

    #[test]
    fn test_registers() {
        assert_eq!(print(Reg::new(0)), "x0");
        assert_eq!(print(Reg::new(1)), "x1");
        assert_eq!(print(Reg::new(14)), "x14");
        assert_eq!(print(Reg::new(15)), "x15");
    }
}
