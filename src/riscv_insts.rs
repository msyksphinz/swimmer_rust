use crate::riscv_csr::CsrAddr;

use crate::riscv_tracer::RiscvTracer;

use crate::riscv_core::UXlenT;
use crate::riscv_core::XlenT;
use crate::riscv_core::UXlen64T;
use crate::riscv_core::Addr64T;
use crate::riscv_core::Xlen64T;

use crate::riscv_core::PrivMode;

use crate::riscv_core::InstT;

use crate::riscv_core::Riscv64Core;
use crate::riscv_core::Riscv64Env;

use crate::riscv_exception::ExceptCode;
use crate::riscv_exception::RiscvException;

use crate::riscv_inst_id::RiscvInstId;

use crate::riscv_insts_int::Riscv64InstsInt;
use crate::riscv_insts_fpu::Riscv64InstsFpu;
use crate::riscv_insts_amo::Riscv64InstsAmo;
use crate::riscv_insts_mem::Riscv64InstsMem;

use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SIE_LSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SIE_MSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SPIE_LSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SPIE_MSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SPP_LSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SPP_MSB;

pub trait RiscvInsts {
    fn execute_inst(&mut self, dec_inst: RiscvInstId, inst: InstT, step: u32);
}


impl RiscvInsts for Riscv64Env {
    fn execute_inst(&mut self, dec_inst: RiscvInstId, inst: InstT, step: u32) {
        self.m_trace.m_executed_pc = self.m_pc;
        self.m_trace.m_inst_hex = inst;
        self.m_trace.m_dec_inst = Some(dec_inst);

        self.m_trace.m_step = step;

        self.m_trace.m_priv = self.m_priv;
        self.m_trace.m_vmmode = self.get_vm_mode();

        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let csr_addr = CsrAddr::from_u64(((inst >> 20) & 0x0fff) as u64);

        self.set_update_pc(false);

        match dec_inst {
            RiscvInstId::CSRRW => {
                let rs1_data = self.read_reg(rs1);
                let reg_data: Xlen64T = self.m_csr.csrrw(csr_addr, rs1_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::CSRRS => {
                let rs1_data = self.read_reg(rs1);
                let reg_data: Xlen64T = self.m_csr.csrrs(csr_addr, rs1_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::CSRRC => {
                let rs1_data = self.read_reg(rs1);
                let reg_data: Xlen64T = self.m_csr.csrrc(csr_addr, rs1_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::CSRRWI => {
                let zimm: Xlen64T = ((inst >> 15) & 0x1f) as Xlen64T;
                let reg_data: Xlen64T = self.m_csr.csrrw(csr_addr, zimm);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::CSRRSI => {
                let zimm: Xlen64T = ((inst >> 15) & 0x1f) as Xlen64T;
                let reg_data: Xlen64T = self.m_csr.csrrs(csr_addr, zimm);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::CSRRCI => {
                let zimm: Xlen64T = ((inst >> 15) & 0x1f) as Xlen64T;
                let reg_data: Xlen64T = self.m_csr.csrrc(csr_addr, zimm);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::LB =>  self.execute_lb  (inst),
            RiscvInstId::LH =>  self.execute_lh  (inst),
            RiscvInstId::LW =>  self.execute_lw  (inst),
            RiscvInstId::LD =>  self.execute_ld  (inst),
            RiscvInstId::LBU => self.execute_lbu (inst),
            RiscvInstId::LHU => self.execute_lhu (inst),
            RiscvInstId::LWU => self.execute_lwu (inst),

            RiscvInstId::SB =>  self.execute_sb  (inst),
            RiscvInstId::SH =>  self.execute_sh  (inst),
            RiscvInstId::SW =>  self.execute_sw  (inst),
            RiscvInstId::SD =>  self.execute_sd  (inst),

            RiscvInstId::AUIPC => self.execute_auipc (inst),
            RiscvInstId::LUI   => self.execute_lui   (inst),
            RiscvInstId::ADDI  => self.execute_addi  (inst),
            RiscvInstId::SLTI  => self.execute_slti  (inst),
            RiscvInstId::SLTIU => self.execute_sltiu (inst),
            RiscvInstId::XORI  => self.execute_xori  (inst),
            RiscvInstId::ORI   => self.execute_ori   (inst),
            RiscvInstId::ANDI  => self.execute_andi  (inst),
            RiscvInstId::SLLI  => self.execute_slli  (inst),
            RiscvInstId::SRLI  => self.execute_srli  (inst),
            RiscvInstId::SRAI  => self.execute_srai  (inst),
            RiscvInstId::ADD   => self.execute_add   (inst),
            RiscvInstId::SUB   => self.execute_sub   (inst),
            RiscvInstId::SLL   => self.execute_sll   (inst),
            RiscvInstId::SLT   => self.execute_slt   (inst),
            RiscvInstId::SLTU  => self.execute_sltu  (inst),
            RiscvInstId::XOR   => self.execute_xor   (inst),
            RiscvInstId::SRL   => self.execute_srl   (inst),
            RiscvInstId::SRA   => self.execute_sra   (inst),

            RiscvInstId::MUL => {
                let rs1_data = self.read_reg(rs1);
                let rs2_data = self.read_reg(rs2);
                let data = rs1_data.wrapping_mul(rs2_data) as Xlen64T;
                let reg_data: Xlen64T = self.sext_xlen(data);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::MULH => {
                if self.m_xlen == 32 {
                    let rs1_data: i64 = (self.read_reg(rs1) as i32) as i64;
                    let rs2_data: i64 = (self.read_reg(rs2) as i32) as i64;
                    let mut reg_data: i64 = rs1_data.wrapping_mul(rs2_data);
                    reg_data = reg_data >> 32;
                    self.write_reg(rd, reg_data as Xlen64T);
                } else {
                    let rs1_data: i128 = (self.read_reg(rs1) as i64) as i128;
                    let rs2_data: i128 = (self.read_reg(rs2) as i64) as i128;
                    let mut reg_data: i128 = rs1_data.wrapping_mul(rs2_data);
                    reg_data = reg_data >> 64;
                    self.write_reg(rd, reg_data as Xlen64T);
                }
            }
            RiscvInstId::MULHSU => {
                if self.m_xlen == 32 {
                    let rs1_data: i64 = (self.read_reg(rs1) as i32) as i64;
                    let rs2_data: i64 = (self.read_reg(rs2) as u32) as i64;
                    let mut reg_data: i64 = rs1_data.wrapping_mul(rs2_data);
                    reg_data = reg_data >> 32;
                    self.write_reg(rd, reg_data as Xlen64T);
                } else {
                    let rs1_data: i128 = (self.read_reg(rs1) as i64) as i128;
                    let rs2_data: i128 = (self.read_reg(rs2) as u64) as i128;
                    let mut reg_data: i128 = rs1_data.wrapping_mul(rs2_data);
                    reg_data = reg_data >> 64;
                    self.write_reg(rd, reg_data as Xlen64T);
                }
            }
            RiscvInstId::MULHU => {
                if self.m_xlen == 32 {
                    let rs1_data: u64 = (self.read_reg(rs1) as u32) as u64;
                    let rs2_data: u64 = (self.read_reg(rs2) as u32) as u64;
                    let mut reg_data: u64 = rs1_data.wrapping_mul(rs2_data);
                    reg_data = reg_data >> 32;
                    self.write_reg(rd, reg_data as Xlen64T);
                } else {
                    let rs1_data: u128 = (self.read_reg(rs1) as u64) as u128;
                    let rs2_data: u128 = (self.read_reg(rs2) as u64) as u128;
                    let mut reg_data: u128 = rs1_data.wrapping_mul(rs2_data);
                    reg_data = reg_data >> 64;
                    self.write_reg(rd, reg_data as Xlen64T);
                }
            }

            RiscvInstId::REM => {
                let rs1_data = self.read_reg(rs1);
                let rs2_data = self.read_reg(rs2);
                let reg_data: Xlen64T;
                if rs2_data == 0 {
                    reg_data = rs1_data;
                } else if rs2_data == -1 {
                    reg_data = 0;
                } else {
                    reg_data = self.sext_xlen(rs1_data.wrapping_rem(rs2_data) as Xlen64T);
                }
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::REMU => {
                let rs1_data: UXlen64T = self.read_reg(rs1) as UXlen64T;
                let rs2_data: UXlen64T = self.read_reg(rs2) as UXlen64T;
                let reg_data: UXlen64T;
                if rs2_data == 0 {
                    reg_data = rs1_data;
                } else {
                    reg_data = rs1_data.wrapping_rem(rs2_data);
                }
                self.write_reg(rd, reg_data as Xlen64T);
            }

            RiscvInstId::DIV => {
                let rs1_data = self.read_reg(rs1);
                let rs2_data = self.read_reg(rs2);
                let reg_data: Xlen64T;
                if rs2_data == 0 {
                    reg_data = -1;
                } else {
                    reg_data = rs1_data.wrapping_div(rs2_data);
                }
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::DIVU => {
                let rs1_data_64: UXlen64T = self.read_reg(rs1) as UXlen64T;
                let rs2_data_64: UXlen64T = self.read_reg(rs2) as UXlen64T;
                let rs1_data: UXlen64T = self.uext_xlen(rs1_data_64 as Xlen64T) as UXlen64T;
                let rs2_data: UXlen64T = self.uext_xlen(rs2_data_64 as Xlen64T) as UXlen64T;
                let reg_data: UXlen64T;
                if rs2_data == 0 {
                    reg_data = 0xffffffffffffffff as UXlen64T;
                } else {
                    reg_data = rs1_data.wrapping_div(rs2_data);
                }
                self.write_reg(rd, reg_data as Xlen64T);
            }

            RiscvInstId::OR => {
                let data = self.read_reg(rs1) | self.read_reg(rs2);
                let reg_data: Xlen64T = self.sext_xlen(data);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::AND => {
                let data = self.read_reg(rs1) & self.read_reg(rs2);
                let reg_data: Xlen64T = self.sext_xlen(data);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::JAL => {
                let addr: Addr64T = Self::extract_uj_field(inst) as Addr64T;
                self.write_reg(rd, (self.m_pc + 4) as Xlen64T);
                self.m_pc = self.m_pc.wrapping_add(addr);
                self.set_update_pc(true);
            }
            RiscvInstId::BEQ
            | RiscvInstId::BNE
            | RiscvInstId::BLT
            | RiscvInstId::BGE
            | RiscvInstId::BLTU
            | RiscvInstId::BGEU => {
                let rs1_data_64: Xlen64T = self.read_reg(rs1);
                let rs2_data_64: Xlen64T = self.read_reg(rs2);
                let rs1_data: Xlen64T = self.sext_xlen(rs1_data_64);
                let rs2_data: Xlen64T = self.sext_xlen(rs2_data_64);
                let addr: Addr64T = Self::extract_sb_field(inst) as Addr64T;
                let jump_en: bool;
                match dec_inst {
                    RiscvInstId::BEQ => jump_en = rs1_data == rs2_data,
                    RiscvInstId::BNE => jump_en = rs1_data != rs2_data,
                    RiscvInstId::BLT => jump_en = rs1_data < rs2_data,
                    RiscvInstId::BGE => jump_en = rs1_data >= rs2_data,
                    RiscvInstId::BLTU => jump_en = (rs1_data as UXlen64T) < (rs2_data as UXlen64T),
                    RiscvInstId::BGEU => jump_en = (rs1_data as UXlen64T) >= (rs2_data as UXlen64T),
                    _ => panic!("Unknown value Branch"),
                }
                if jump_en {
                    self.m_pc = self.m_pc.wrapping_add(addr);
                    self.set_update_pc(true);
                }
            }
            RiscvInstId::JALR => {
                let mut addr: Addr64T = Self::extract_ifield(inst) as Addr64T;
                let rs1_data: Addr64T = self.read_reg(rs1) as Addr64T;
                addr = rs1_data.wrapping_add(addr);
                addr = addr & (!0x01);

                self.write_reg(rd, (self.m_pc + 4) as Xlen64T);
                self.m_pc = addr;
                self.set_update_pc(true);
            }
            RiscvInstId::FENCE => {}
            RiscvInstId::FENCE_I => {}
            RiscvInstId::SFENCE_VMA => {}
            RiscvInstId::ECALL => {
                self.m_csr.csrrw(CsrAddr::Mepc, self.m_pc as Xlen64T); // MEPC

                let current_priv: PrivMode = self.m_priv;

                match current_priv {
                    PrivMode::User => self.generate_exception(ExceptCode::EcallFromUMode, 0),
                    PrivMode::Supervisor => self.generate_exception(ExceptCode::EcallFromSMode, 0),
                    PrivMode::Hypervisor => self.generate_exception(ExceptCode::EcallFromHMode, 0),
                    PrivMode::Machine => self.generate_exception(ExceptCode::EcallFromMMode, 0),
                }
                self.set_update_pc(true);
            }
            RiscvInstId::EBREAK => {}
            RiscvInstId::URET => {}
            RiscvInstId::SRET => {
                let mstatus: Xlen64T = self
                    .m_csr
                    .csrrs(CsrAddr::Mstatus, PrivMode::Machine as Xlen64T);
                let next_priv_uint: Xlen64T = Self::extract_bit_field(
                    mstatus,
                    SYSREG_MSTATUS_SPP_MSB,
                    SYSREG_MSTATUS_SPP_LSB,
                );
                let next_priv: PrivMode = PrivMode::from_u8(next_priv_uint as u8);
                let mut next_mstatus: Xlen64T = mstatus;
                next_mstatus = Self::set_bit_field(
                    next_mstatus,
                    Self::extract_bit_field(
                        mstatus,
                        SYSREG_MSTATUS_SPIE_MSB,
                        SYSREG_MSTATUS_SPIE_LSB,
                    ),
                    SYSREG_MSTATUS_SIE_MSB,
                    SYSREG_MSTATUS_SIE_LSB,
                );
                next_mstatus = Self::set_bit_field(
                    next_mstatus,
                    1,
                    SYSREG_MSTATUS_SPIE_MSB,
                    SYSREG_MSTATUS_SPIE_LSB,
                );
                next_mstatus = Self::set_bit_field(
                    next_mstatus,
                    PrivMode::User as Xlen64T,
                    SYSREG_MSTATUS_SPP_MSB,
                    SYSREG_MSTATUS_SPP_LSB,
                );

                self.m_csr.csrrw(CsrAddr::Mstatus, next_mstatus);
                let ret_pc = self.m_csr.csrrs(CsrAddr::Sepc, 0);
                self.set_priv_mode(next_priv);

                self.set_pc(ret_pc as Addr64T);
                self.set_update_pc(true);
            }
            RiscvInstId::MRET => {
                let mepc: Xlen64T = self.m_csr.csrrs(CsrAddr::Mepc, 0); // MEPC
                self.m_pc = mepc as Addr64T;
                self.set_update_pc(true);
            }
            RiscvInstId::ADDIW => self.execute_addiw(inst),
            RiscvInstId::SLLIW => self.execute_slliw(inst),
            RiscvInstId::SRLIW => self.execute_srliw(inst),
            RiscvInstId::SRAIW => self.execute_sraiw(inst),
            RiscvInstId::ADDW  => self.execute_addw (inst),
            RiscvInstId::SUBW  => self.execute_subw (inst),
            RiscvInstId::SLLW  => self.execute_sllw (inst),
            RiscvInstId::SRLW  => self.execute_srlw (inst),
            RiscvInstId::SRAW  => self.execute_sraw (inst),

            RiscvInstId::MULW => {
                let rs1_data = self.read_reg(rs1) as XlenT;
                let rs2_data = self.read_reg(rs2) as XlenT;
                let reg_data: XlenT = rs1_data.wrapping_mul(rs2_data);
                self.write_reg(rd, Self::extend_sign(reg_data as Xlen64T, 31));
            }

            RiscvInstId::DIVW => {
                let rs1_data = self.read_reg(rs1) as XlenT;
                let rs2_data = self.read_reg(rs2) as XlenT;
                let reg_data: XlenT;
                if rs2_data == 0 {
                    reg_data = -1;
                } else {
                    reg_data = rs1_data.wrapping_div(rs2_data);
                }
                self.write_reg(rd, Self::extend_sign(reg_data as Xlen64T, 31));
            }
            RiscvInstId::DIVUW => {
                let rs1_data: UXlenT = self.read_reg(rs1) as UXlenT;
                let rs2_data: UXlenT = self.read_reg(rs2) as UXlenT;
                let reg_data: UXlenT;
                if rs2_data == 0 {
                    reg_data = 0xffffffff;
                } else {
                    reg_data = rs1_data.wrapping_div(rs2_data);
                }
                self.write_reg(rd, Self::extend_sign(reg_data as Xlen64T, 31));
            }

            RiscvInstId::REMW => {
                let rs1_data = self.read_reg(rs1) as XlenT;
                let rs2_data = self.read_reg(rs2) as XlenT;
                let reg_data: XlenT;
                if rs2_data == 0 {
                    reg_data = rs1_data;
                } else if rs2_data == -1 {
                    reg_data = 0;
                } else {
                    reg_data = rs1_data.wrapping_rem(rs2_data);
                }
                self.write_reg(rd, Self::extend_sign(reg_data as Xlen64T, 31));
            }
            RiscvInstId::REMUW => {
                let rs1_data: UXlenT = self.read_reg(rs1) as UXlenT;
                let rs2_data: UXlenT = self.read_reg(rs2) as UXlenT;
                let reg_data: UXlenT;
                if rs2_data == 0 {
                    reg_data = rs1_data;
                } else {
                    reg_data = rs1_data.wrapping_rem(rs2_data);
                }
                self.write_reg(rd, Self::extend_sign(reg_data as Xlen64T, 31));
            }

            RiscvInstId::AMOSWAP_W => self.execute_amoswap_w (inst),
            RiscvInstId::AMOADD_W  => self.execute_amoadd_w  (inst),
            RiscvInstId::AMOXOR_W  => self.execute_amoxor_w  (inst),
            RiscvInstId::AMOAND_W  => self.execute_amoand_w  (inst),
            RiscvInstId::AMOOR_W   => self.execute_amoor_w   (inst),
            RiscvInstId::AMOMIN_W  => self.execute_amomin_w  (inst),
            RiscvInstId::AMOMAX_W  => self.execute_amomax_w  (inst),
            RiscvInstId::AMOMINU_W => self.execute_amominu_w (inst),
            RiscvInstId::AMOMAXU_W => self.execute_amomaxu_w (inst),

            RiscvInstId::AMOSWAP_D => self.execute_amoswap_d (inst),
            RiscvInstId::AMOADD_D  => self.execute_amoadd_d  (inst),
            RiscvInstId::AMOXOR_D  => self.execute_amoxor_d  (inst),
            RiscvInstId::AMOAND_D  => self.execute_amoand_d  (inst),
            RiscvInstId::AMOOR_D   => self.execute_amoor_d   (inst),
            RiscvInstId::AMOMIN_D  => self.execute_amomin_d  (inst),
            RiscvInstId::AMOMAX_D  => self.execute_amomax_d  (inst),
            RiscvInstId::AMOMINU_D => self.execute_amominu_d (inst),
            RiscvInstId::AMOMAXU_D => self.execute_amomaxu_d (inst),

            RiscvInstId::FLW => self.execute_flw(inst),
            RiscvInstId::FSW => self.execute_fsw(inst),
            RiscvInstId::FADD_S  => self.execute_fadd_s(inst),
            RiscvInstId::FSUB_S  => self.execute_fsub_s(inst),
            RiscvInstId::FMUL_S  => self.execute_fmul_s(inst),
            RiscvInstId::FMV_X_W => self.execute_fmv_x_w(inst),

            RiscvInstId::FLD => self.execute_fld(inst),
            RiscvInstId::FSD => self.execute_fsd(inst),
            RiscvInstId::FADD_D => self.execute_fadd_d(inst),
            RiscvInstId::FSUB_D => self.execute_fsub_d(inst),
            RiscvInstId::FMUL_D => self.execute_fmul_d(inst),
            RiscvInstId::FMV_X_D => self.execute_fmv_x_d(inst),

            RiscvInstId::FMADD_S  => self.execute_fmadd_s(inst),
            RiscvInstId::FMSUB_S  => self.execute_fmsub_s(inst),
            RiscvInstId::FNMSUB_S => self.execute_fnmsub_s(inst),
            RiscvInstId::FNMADD_S => self.execute_fnmadd_s(inst),

            RiscvInstId::FMADD_D  => self.execute_fmadd_d(inst),
            RiscvInstId::FMSUB_D  => self.execute_fmsub_d(inst),
            RiscvInstId::FNMSUB_D => self.execute_fnmsub_d(inst),
            RiscvInstId::FNMADD_D => self.execute_fnmadd_d(inst),

            _ => { panic!("Unimplemneted instruction. Stop."); }
        }

        if self.is_update_pc() == false {
            self.m_pc += 4;
        }

        self.m_trace.print_trace();
        self.m_trace.clear();
    }
}
