use crate::instr::{Instr, Reg, Width, Word};

mod instr;

fn main() {
    println!("Hello, world!");
}

pub struct Vm {
    pc: u32,
    next_pc: u32,
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
        match (width, U) {
            (Width::Byte, false) => i8::from_le_bytes(read_fixed(&self.memory, addr)).into(),
            (Width::Byte, true) => u8::from_le_bytes(read_fixed(&self.memory, addr)).into(),
            (Width::Half, false) => i16::from_le_bytes(read_fixed(&self.memory, addr)).into(),
            (Width::Half, true) => u16::from_le_bytes(read_fixed(&self.memory, addr)).into(),
            (Width::Word, false) => i32::from_le_bytes(read_fixed(&self.memory, addr)).into(),
            (Width::Word, true) => u32::from_le_bytes(read_fixed(&self.memory, addr)).into(),
        }
    }

    fn store(&mut self, addr: u32, width: Width, value: Word) {
        match width {
            Width::Byte => write_fixed(&mut self.memory, addr, value.u8().to_le_bytes()),
            Width::Half => write_fixed(&mut self.memory, addr, value.u16().to_le_bytes()),
            Width::Word => write_fixed(&mut self.memory, addr, value.u32().to_le_bytes()),
        }
    }

    fn fetch(&mut self) -> Instr {
        self.pc = self.next_pc;
        let instr = self.load::<false>(self.pc, Width::Word).instr();
        self.next_pc = self.pc.wrapping_add(4);
        instr
    }

    fn read_pc(&mut self) -> u32 {
        self.pc
    }

    fn write_pc(&mut self, pc: u32) {
        self.next_pc = pc;
    }
}

impl Vm {
    fn run(&mut self) {
        loop {
            let instr = self.fetch();
            self.execute(instr);
        }
    }

    fn execute(&mut self, ins: Instr) {
        match ins.opcode() {
            0b0000011 => self.execute_load(ins),
            0b0100011 => self.execute_store(ins),
            0b0010011 => self.execute_arith::<true>(ins),
            0b0110011 => self.execute_arith::<false>(ins),
            0b1100011 => self.execute_branch(ins),
            0b1100111 => self.execute_jal_imm(ins),
            0b1101111 => self.execute_jal_reg(ins),
            0b0110111 => self.execute_load_upper_imm(ins),
            0b0010111 => self.execute_add_upper_imm_to_pc(ins),
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
        let lhs = self.read(ins.rs1());
        let rhs = self.read(ins.rs2());

        let cond = match ins.fn3() {
            0b000 => lhs == rhs,
            0b001 => lhs != rhs,
            0b100 => lhs.i32() < rhs.i32(),
            0b101 => lhs.i32() >= rhs.i32(),
            0b110 => lhs.u32() < rhs.u32(),
            0b111 => lhs.u32() >= rhs.u32(),
            _ => return self.illegal_instr(),
        };

        if cond {
            let pc = self.read_pc();
            let offset = ins.b_imm().i32();
            self.write_pc(pc.wrapping_add_signed(offset));
        }
    }

    fn execute_jal_imm(&mut self, ins: Instr) {
        let pc = self.read_pc();
        let offset = ins.j_imm().i32();

        self.write(ins.rd(), pc.wrapping_add(4).into());
        self.write_pc(pc.wrapping_add_signed(offset));
    }

    fn execute_jal_reg(&mut self, ins: Instr) {
        let pc = self.read_pc();
        let rs1 = self.read(ins.rs1()).u32();
        let imm = ins.i_imm().i32();

        self.write(ins.rd(), pc.wrapping_add(4).into());
        self.write_pc(rs1.wrapping_add_signed(imm) & !1);
    }

    fn execute_load_upper_imm(&mut self, ins: Instr) {
        self.write(ins.rd(), ins.u_imm());
    }

    fn execute_add_upper_imm_to_pc(&mut self, ins: Instr) {
        let pc = self.read_pc();
        let result = pc.wrapping_add_signed(ins.u_imm().i32());
        self.write(ins.rd(), result.into());
    }

    fn illegal_instr(&mut self) {
        panic!("illegal instruction");
    }
}

fn read_fixed<const N: usize>(bytes: &[u8], offset: u32) -> [u8; N] {
    bytes[offset as usize..][..N].try_into().unwrap()
}

fn write_fixed<const N: usize>(bytes: &mut [u8], offset: u32, data: [u8; N]) {
    bytes[offset as usize..][..N].copy_from_slice(&data);
}
