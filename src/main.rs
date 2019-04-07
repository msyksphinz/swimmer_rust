use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};

mod core_base;
mod riscv32_core;
mod riscv64_core;
mod riscv_csr;
mod riscv_csr_bitdef;
mod riscv_exception;
mod riscv_insts;
mod riscv64_insts;
mod riscv_mmu;
mod riscv_tracer;

use crate::riscv64_core::Riscv64Core;
use crate::riscv64_core::Riscv64Env;

use crate::riscv_insts::RiscvInsts;

use crate::riscv32_core::InstT;
use crate::riscv32_core::XlenT;
use crate::riscv32_core::DRAM_BASE;
use crate::riscv64_core::Xlen64T;

use crate::riscv32_core::MemResult;

fn main() -> Result<(), Box<std::error::Error>> {
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        writeln!(std::io::stderr(), "Usage: swimmer_rust_origin binfile").unwrap();
        std::process::exit(1);
    }

    let file = File::open(&args[1]).unwrap();
    let filebuf = BufReader::new(file);
    let mut hex_addr = 0;

    let mut riscv64_core = Riscv64Env::new();

    for result in filebuf.bytes() {
        let l = result?;
        riscv64_core.write_memory_byte(hex_addr + DRAM_BASE, l as Xlen64T);
        hex_addr = hex_addr + 1;
    }

    let mut count = 0;
    while count < 65535 && !riscv64_core.get_is_finish_cpu() {
        // println!("InstNo: {:10}", count);
        let (result, inst_data) = riscv64_core.fetch_bus();
        if result != MemResult::NoExcept {
            continue;
        }
        let inst_decode = riscv64_core.decode_inst(inst_data);
        riscv64_core.execute_inst(inst_decode, inst_data as InstT, count);

        count += 1;
    }

    if riscv64_core.get_tohost() == 1 {
        eprintln!("PASS : {}", args[1]);
    } else {
        eprintln!("FAIL : {}", args[1]);
    }

    Ok(())
}
