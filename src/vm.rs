use std::fmt::Debug;

use object::{Object, ObjectSegment};

use crate::{
    decode::Instr,
    instr::{EncInstr, Reg, Width, Word},
};

const STACK_TOP: u32 = 0x10000;

pub struct Vm {
    pc: u32,
    next_pc: u32,
    registers: [Word; 32],
    memory: Vec<u8>,
    trap: bool,
}

impl Debug for Vm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vm")
            .field("pc", &self.pc)
            .field("next_pc", &self.next_pc)
            .field("registers", &self.registers)
            .finish()
    }
}

impl Vm {
    pub fn new(memory: Vec<u8>) -> Self {
        Self { pc: 0, next_pc: 0, registers: Default::default(), memory, trap: false }
    }

    fn read(&self, reg: Reg) -> Word {
        reg.get(&self.registers)
    }

    fn write(&mut self, reg: Reg, value: Word) {
        reg.set(&mut self.registers, value)
    }

    pub fn load(&self, addr: u32, width: Width) -> Word {
        match width {
            Width::Byte => i8::from_le_bytes(read_fixed(&self.memory, addr)).into(),
            Width::Half => i16::from_le_bytes(read_fixed(&self.memory, addr)).into(),
            Width::Word => i32::from_le_bytes(read_fixed(&self.memory, addr)).into(),
        }
    }

    pub fn load_unsigned(&self, addr: u32, width: Width) -> Word {
        match width {
            Width::Byte => u8::from_le_bytes(read_fixed(&self.memory, addr)).into(),
            Width::Half => u16::from_le_bytes(read_fixed(&self.memory, addr)).into(),
            Width::Word => u32::from_le_bytes(read_fixed(&self.memory, addr)).into(),
        }
    }

    fn store(&mut self, addr: u32, width: Width, value: Word) {
        match width {
            Width::Byte => write_fixed(&mut self.memory, addr, value.u8().to_le_bytes()),
            Width::Half => write_fixed(&mut self.memory, addr, value.u16().to_le_bytes()),
            Width::Word => write_fixed(&mut self.memory, addr, value.u32().to_le_bytes()),
        }
    }

    fn fetch(&mut self) -> EncInstr {
        self.pc = self.next_pc;
        let instr = self.load(self.pc, Width::Word).instr();
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
    pub fn run(&mut self) {
        let xlen = rvdasm::disassembler::Xlen::XLEN32;
        let dis = rvdasm::disassembler::Disassembler::new(xlen);

        while !self.trap {
            print!("{:08x}: ", self.next_pc);
            let instr = self.fetch();
            match dis.disassmeble_one(instr.u32()) {
                Some(i) => println!("{}", i.to_string()),
                None => println!("unknown <{:08x}>", instr.u32()),
            }
            println!("    {:?}", Instr::decode(instr));
            self.execute(instr);
            if instr.u32() == 0x0000006f {
                break;
            }
            // self.print_registers();
        }
    }

    pub fn print_registers(&self) {
        for i in 0..8 {
            for j in 0..4 {
                let r = 8 * j + i;
                let s = if r < 10 { " " } else { "" };
                print!("{}x{r}: {:?}\t", s, self.registers[r]);
            }
            println!();
        }
    }

    fn execute(&mut self, ins: EncInstr) {
        // println!("opcode: {:07b}", ins.opcode());
        match ins.opcode() {
            0b0000011 => self.execute_load(ins),
            0b0100011 => self.execute_store(ins),
            0b0010011 => self.execute_arith_imm(ins),
            0b0110011 => self.execute_arith(ins),
            0b1100011 => self.execute_branch(ins),
            0b1101111 => self.execute_jal_imm(ins),
            0b1100111 => self.execute_jal_reg(ins),
            0b0110111 => self.execute_load_upper_imm(ins),
            0b0010111 => self.execute_add_upper_imm_to_pc(ins),
            0b0001111 => self.execute_misc_mem(ins),
            _ => self.illegal_instr(),
        }
    }

    fn execute_load(&mut self, ins: EncInstr) {
        let addr = self.read(ins.rs1()).u32().wrapping_add(ins.i_imm().u32());

        let value = match ins.fn3() {
            0b000 => self.load(addr, Width::Byte),
            0b001 => self.load(addr, Width::Half),
            0b010 => self.load(addr, Width::Word),
            0b100 => self.load_unsigned(addr, Width::Byte),
            0b101 => self.load_unsigned(addr, Width::Half),
            _ => return self.illegal_instr(),
        };

        self.write(ins.rd(), value);
    }

    fn execute_store(&mut self, ins: EncInstr) {
        let addr = self.read(ins.rs1()).u32().wrapping_add(ins.s_imm().u32());
        let value = self.read(ins.rs2());

        match ins.fn3() {
            0b000 => self.store(addr, Width::Byte, value),
            0b001 => self.store(addr, Width::Half, value),
            0b010 => self.store(addr, Width::Word, value),
            _ => return self.illegal_instr(),
        }
    }

    fn execute_arith_imm(&mut self, ins: EncInstr) {
        let lhs = self.read(ins.rs1());
        let rhs = ins.i_imm();

        const F0: u32 = 0b0000000;
        const F1: u32 = 0b0100000;

        let result = match (ins.fn3(), ins.fn7()) {
            (0b000, _) => lhs.i32().wrapping_add(rhs.i32()).into(),
            (0b001, F0) => (lhs.i32() << (rhs.u32() & 31)).into(),
            (0b010, _) => (lhs.i32() < rhs.i32()).into(),
            (0b011, _) => (lhs.u32() < rhs.u32()).into(),
            (0b100, _) => (lhs.u32() ^ rhs.u32()).into(),
            (0b101, F0) => (lhs.u32() >> (rhs.u32() & 31)).into(),
            (0b101, F1) => (lhs.i32() >> (rhs.u32() & 31)).into(),
            (0b110, _) => (lhs.u32() | rhs.u32()).into(),
            (0b111, _) => (lhs.u32() & rhs.u32()).into(),
            _ => return self.illegal_instr(),
        };

        self.write(ins.rd(), result);
    }

    fn execute_arith(&mut self, ins: EncInstr) {
        let lhs = self.read(ins.rs1());
        let rhs = self.read(ins.rs2());
        match ins.fn7() {
            0b0000000 => self.execute_arith_int(ins, lhs, rhs, false),
            0b0100000 => self.execute_arith_int(ins, lhs, rhs, true),
            0b0000001 => self.execute_arith_mul(ins, lhs, rhs),
            _ => return self.illegal_instr(),
        }
    }

    fn execute_arith_int(&mut self, ins: EncInstr, lhs: Word, rhs: Word, alt: bool) {
        let result = match (ins.fn3(), alt) {
            (0b000, false) => lhs.i32().wrapping_add(rhs.i32()).into(),
            (0b000, true) => lhs.i32().wrapping_sub(rhs.i32()).into(),
            (0b001, false) => (lhs.i32() << (rhs.u32() & 31)).into(),
            (0b010, false) => (lhs.i32() < rhs.i32()).into(),
            (0b011, false) => (lhs.u32() < rhs.u32()).into(),
            (0b100, false) => (lhs.u32() ^ rhs.u32()).into(),
            (0b101, false) => (lhs.u32() >> (rhs.u32() & 31)).into(),
            (0b101, true) => (lhs.i32() >> (rhs.u32() & 31)).into(),
            (0b110, false) => (lhs.u32() | rhs.u32()).into(),
            (0b111, false) => (lhs.u32() & rhs.u32()).into(),
            _ => return self.illegal_instr(),
        };
        self.write(ins.rd(), result);
    }

    fn execute_arith_mul(&mut self, ins: EncInstr, lhs: Word, rhs: Word) {
        let result = match ins.fn3() {
            0b000 => lhs.i32().wrapping_mul(rhs.i32()).into(),
            0b001 => (lhs.i64().wrapping_mul(rhs.i64()) >> 32).into(),
            0b010 => (lhs.i64().wrapping_mul(rhs.u64() as i64) >> 32).into(),
            0b011 => (lhs.u64().wrapping_mul(rhs.u64()) >> 32).into(),
            0b100 => match (lhs.i32(), rhs.i32()) {
                (_, 0) => -1,
                (i32::MIN, -1) => i32::MIN,
                (lhs, rhs) => lhs / rhs,
            }
            .into(),
            0b101 => lhs.u32().checked_div(rhs.u32()).unwrap_or(!0).into(),
            0b110 => match (lhs.i32(), rhs.i32()) {
                (x, 0) => x,
                (i32::MIN, -1) => 0,
                (lhs, rhs) => lhs / rhs,
            }
            .into(),
            0b111 => lhs.u32().checked_rem(rhs.u32()).unwrap_or(lhs.u32()).into(),
            _ => return self.illegal_instr(),
        };
        self.write(ins.rd(), result);
    }

    fn execute_branch(&mut self, ins: EncInstr) {
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

    fn execute_jal_imm(&mut self, ins: EncInstr) {
        let pc = self.read_pc();
        let offset = ins.j_imm().i32();

        self.write(ins.rd(), pc.wrapping_add(4).into());
        self.write_pc(pc.wrapping_add_signed(offset));
    }

    fn execute_jal_reg(&mut self, ins: EncInstr) {
        let pc = self.read_pc();
        let rs1 = self.read(ins.rs1()).u32();
        let imm = ins.i_imm().i32();

        self.write(ins.rd(), pc.wrapping_add(4).into());
        self.write_pc(rs1.wrapping_add_signed(imm) & !1);
    }

    fn execute_load_upper_imm(&mut self, ins: EncInstr) {
        self.write(ins.rd(), ins.u_imm());
    }

    fn execute_add_upper_imm_to_pc(&mut self, ins: EncInstr) {
        let pc = self.read_pc();
        let result = pc.wrapping_add_signed(ins.u_imm().i32());
        self.write(ins.rd(), result.into());
    }

    fn execute_misc_mem(&mut self, ins: EncInstr) {
        match ins.fn3() {
            0b000 | 0b001 => {}
            _ => return self.illegal_instr(),
        }
    }

    fn illegal_instr(&mut self) {
        eprintln!("illegal instruction");
        self.trap = true;
    }
}

fn read_fixed<const N: usize>(bytes: &[u8], offset: u32) -> [u8; N] {
    bytes[offset as usize..][..N].try_into().unwrap()
}

fn write_fixed<const N: usize>(bytes: &mut [u8], offset: u32, data: [u8; N]) {
    bytes[offset as usize..][..N].copy_from_slice(&data);
}

pub fn load_elf(vm: &mut Vm, bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let obj = object::File::parse(bytes)?;

    for segment in obj.segments() {
        let addr = segment.address() as usize;
        let file_data = segment.data()?;
        let mem_size = segment.size() as usize; // memsz — may exceed file_data.len()

        // Copy the file-backed portion
        vm.memory[addr..addr + file_data.len()].copy_from_slice(file_data);

        // Zero-fill the rest (this covers .bss, and any padding
        // within a segment where memsz > filesz)
        if mem_size > file_data.len() {
            let zero_start = addr + file_data.len();
            let zero_end = addr + mem_size;
            vm.memory[zero_start..zero_end].fill(0);
        }
    }

    let entry = obj.entry() as u32;
    vm.pc = entry;
    vm.next_pc = entry;
    vm.write(Reg::SP, STACK_TOP.into());

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::instr::{InstrBuilder, Reg};
    use crate::vm::Vm;

    #[test]
    fn basic_test() {
        use InstrBuilder as I;

        let instrs = [
            I::new(0b0010011).fn3(0).rd(Reg::RA).rs1(Reg::ZERO).i_imm(2).build(),
            I::new(0b0010011).fn3(0).rd(Reg::RA).rs1(Reg::RA).i_imm(3).build(),
            I::new(0b1110011).build(),
        ];

        let mut vm = Vm::new(instrs.into_iter().collect());
        vm.run();

        assert_eq!(vm.read(Reg::RA), 5.into());
    }
}
