use crate::instr::{Instr, Reg, Width, Word};

mod instr;

fn main() {
    println!("Hello, world!");
}

pub struct Vm {
    pc: usize,
    reg_file: [Word; 32],
    memory: Vec<u8>,
}

impl Vm {
    fn read(&self, reg: Reg) -> Word {
        reg.get(&self.reg_file)
    }

    fn write(&mut self, reg: Reg, value: Word) {
        reg.set(&mut self.reg_file, value)
    }

    fn load<const U: bool>(&self, addr: u32, width: Width) -> Word {
        let mem = &self.memory[(addr as usize)..];
        match (width, U) {
            (Width::Byte, false) => i8::from_le_bytes(mem.try_into().unwrap()).into(),
            (Width::Byte, true) => u8::from_le_bytes(mem.try_into().unwrap()).into(),
            (Width::Half, false) => i16::from_le_bytes(mem.try_into().unwrap()).into(),
            (Width::Half, true) => u16::from_le_bytes(mem.try_into().unwrap()).into(),
            (Width::Word, false) => i32::from_le_bytes(mem.try_into().unwrap()).into(),
            (Width::Word, true) => u32::from_le_bytes(mem.try_into().unwrap()).into(),
        }
    }

    fn store(&mut self, addr: u32, width: Width, value: Word) {
        let mem = &mut self.memory[(addr as usize)..];
        match width {
            Width::Byte => mem[..1].copy_from_slice(&value.u8().to_le_bytes()),
            Width::Half => mem[..2].copy_from_slice(&value.u16().to_le_bytes()),
            Width::Word => mem[..4].copy_from_slice(&value.u32().to_le_bytes()),
        }
    }

    fn read_pc(&self) -> usize {
        self.pc
    }

    fn write_pc(&mut self, pc: usize) {
        self.pc = pc;
    }
}

impl Vm {
    fn execute(&mut self, ins: Instr) {
        match ins.opcode() {
            0b0000011 => self.execute_load(ins),
            0b0100011 => self.execute_store(ins),
            0b0010011 => self.execute_arith::<true>(ins),
            0b0110011 => self.execute_arith::<false>(ins),
            0b1100011 => self.execute_branch(ins),
            0b1100111 => self.execute_jump_imm(ins),
            0b1101111 => self.execute_jump_reg(ins),
            _ => self.illegal_instr(),
        }
    }

    fn execute_load(&mut self, ins: Instr) {
        let addr = self.read(ins.rs1()).u32().wrapping_add(ins.i_imm().u32());

        let value = match ins.fn3() {
            0b000 => self.load::<false>(addr, Width::Byte),
            0b001 => self.load::<false>(addr, Width::Half),
            0b010 => self.load::<false>(addr, Width::Word),
            0b100 => self.load::<true>(addr, Width::Byte),
            0b101 => self.load::<true>(addr, Width::Half),
            _ => return self.illegal_instr(),
        };

        self.write(ins.rd(), value);
    }

    fn execute_store(&mut self, ins: Instr) {
        let addr = self.read(ins.rs1()).u32().wrapping_add(ins.s_imm().u32());
        let value = self.read(ins.rs2());

        match ins.fn3() {
            0b000 => self.store(addr, Width::Byte, value),
            0b001 => self.store(addr, Width::Half, value),
            0b010 => self.store(addr, Width::Word, value),
            _ => return self.illegal_instr(),
        }
    }

    fn execute_arith<const IMM: bool>(&mut self, ins: Instr) {
        let lhs = self.read(ins.rs1());
        let rhs = match IMM {
            false => self.read(ins.rs2()),
            true => ins.i_imm(),
        };
        let fn3 = ins.fn3();
        let fn7 = match (IMM, fn3) {
            (false, _) => ins.fn7(),
            (true, 0b001 | 0b101) => ins.fn7(),
            (true, _) => 0,
        };

        const F0: u32 = 0b0000000;
        const F1: u32 = 0b0100000;

        let result = match (fn3, fn7) {
            (0b000, F0) => lhs.i32().wrapping_add(rhs.i32()).into(),
            (0b000, F1) => lhs.i32().wrapping_sub(rhs.i32()).into(),
            (0b001, F0) => (lhs.i32() << (rhs.u32() & 31)).into(),
            (0b010, F0) => (lhs.i32() < rhs.i32()).into(),
            (0b011, F0) => (lhs.u32() < rhs.u32()).into(),
            (0b100, F0) => (lhs.u32() ^ rhs.u32()).into(),
            (0b101, F0) => (lhs.u32() >> (rhs.u32() & 31)).into(),
            (0b101, F1) => (lhs.i32() >> (rhs.u32() & 31)).into(),
            (0b110, F0) => (lhs.u32() | rhs.u32()).into(),
            (0b111, F0) => (lhs.u32() & rhs.u32()).into(),
            _ => return self.illegal_instr(),
        };

        self.write(ins.rd(), result);
    }

    fn execute_branch(&mut self, ins: Instr) {
        // todo
    }

    fn execute_jump_imm(&mut self, ins: Instr) {
        // todo
    }

    fn execute_jump_reg(&mut self, ins: Instr) {
        // todo
    }

    fn illegal_instr(&mut self) {
        panic!("illegal instruction");
    }
}
