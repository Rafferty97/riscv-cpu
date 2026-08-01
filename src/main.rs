use std::path::PathBuf;

use clap::Parser;

use crate::{
    instr::Width,
    vm::{Vm, load_elf},
};

mod instr;
mod vm;

#[derive(Parser, Debug)]
struct Args {
    /// Input file
    filename: PathBuf,
}

fn main() {
    let args = Args::parse();
    let bytes = std::fs::read(args.filename).unwrap();

    let mut vm = Vm::new(vec![0; 64 * 1024]);
    load_elf(&mut vm, &bytes).unwrap();

    vm.run();
    vm.print_registers();
    let result = vm.load(0x1000, Width::Word);
    println!("result = {}", result.u32());
}
