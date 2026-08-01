use crate::instr::{Instr, Reg, Word};

mod instr;

fn main() {
    println!("Hello, world!");
}

pub struct Vm {
    pc: usize,
    reg: [Word; 32],
}

impl Vm {
    fn read(&self, reg: Reg) -> Word {
        reg.get(&self.reg)
    }

    fn write(&mut self, reg: Reg, value: Word) {
        reg.set(&mut self.reg, value)
    }

    fn load(&self, addr: u32) -> Word {
        todo!()
    }

    fn store(&mut self, addr: u32) -> Word {
        todo!()
    }

    fn read_pc(&self) -> usize {
        self.pc
    }

    fn write_pc(&mut self, pc: usize) {
        self.pc = pc;
    }

    fn execute(&mut self, ins: Instr) {
        match ins.opcode() {
            0b0010011 => self.execute_arith::<true>(ins),
            0b0110011 => self.execute_arith::<false>(ins),
            _ => todo!(),
        }
    }

    fn execute_arith<const IMM: bool>(&mut self, ins: Instr) {
        let lhs = self.read(ins.rs1());
        let rhs = match IMM {
            false => self.read(ins.rs2()),
            true => ins.i_imm(),
        };

        let result = match ins.funct3() {
            0b000 => match !IMM && ins.test_bit(30) {
                false => lhs.i32().wrapping_add(rhs.i32()).into(),
                true => lhs.i32().wrapping_sub(rhs.i32()).into(),
            },
            0b001 => (lhs.i32() << (rhs.u32() & 31)).into(),
            0x010 => (lhs.i32() < rhs.i32()).into(),
            0x011 => (lhs.u32() < rhs.u32()).into(),
            0x100 => (lhs.u32() ^ rhs.u32()).into(),
            0x101 => match ins.test_bit(30) {
                false => (lhs.u32() >> (rhs.u32() & 31)).into(),
                true => (lhs.i32() >> (rhs.u32() & 31)).into(),
            },
            0x110 => (lhs.u32() | rhs.u32()).into(),
            0x111 => (lhs.u32() & rhs.u32()).into(),
            _ => unreachable!(),
        };

        self.write(ins.rd(), result);
    }
}
