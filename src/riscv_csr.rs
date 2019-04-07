use crate::riscv32_core::XlenT;
use crate::riscv64_core::Xlen64T;

pub enum CsrAddr {
    None = 0x000,
    // Cycle          = 0xc00,
    // Instret        = 0xc02,
    // Hpmcounter3    = 0xc03,
    // Hpmcounter4    = 0xc04,
    // Hpmcounter5    = 0xc05,
    // Hpmcounter6    = 0xc06,
    // Hpmcounter7    = 0xc07,
    // Hpmcounter8    = 0xc08,
    // Hpmcounter9    = 0xc09,
    // Hpmcounter10   = 0xc0a,
    // Hpmcounter11   = 0xc0b,
    // Hpmcounter12   = 0xc0c,
    // Hpmcounter13   = 0xc0d,
    // Hpmcounter14   = 0xc0e,
    // Hpmcounter15   = 0xc0f,
    // Hpmcounter16   = 0xc10,
    // Hpmcounter17   = 0xc11,
    // Hpmcounter18   = 0xc12,
    // Hpmcounter19   = 0xc13,
    // Hpmcounter20   = 0xc14,
    // Hpmcounter21   = 0xc15,
    // Hpmcounter22   = 0xc16,
    // Hpmcounter23   = 0xc17,
    // Hpmcounter24   = 0xc18,
    // Hpmcounter25   = 0xc19,
    // Hpmcounter26   = 0xc1a,
    // Hpmcounter27   = 0xc1b,
    // Hpmcounter28   = 0xc1c,
    // Hpmcounter29   = 0xc1d,
    // Hpmcounter30   = 0xc1e,
    // Hpmcounter31   = 0xc1f,
    Mstatus = 0x300,
    Misa = 0x301,
    Medeleg = 0x302,
    Mideleg = 0x303,
    Mie = 0x304,
    Mtvec = 0x305,
    Mscratch = 0x340,
    Mcounteren = 0x306,
    Mepc = 0x341,
    Mcause = 0x342,
    Mtval = 0x343,
    Mip = 0x344,
    // Tselect        = 0x7a0,
    // Tdata1         = 0x7a1,
    // Tdata2         = 0x7a2,
    // Tdata3         = 0x7a3,
    Dcsr = 0x7b0,
    Dpc = 0x7b1,
    Dscratch = 0x7b2,
    Mcycle = 0xb00,
    Minstret = 0xb02,
    // Mhpmcounter3   = 0xb03,
    // Mhpmcounter4   = 0xb04,
    // Mhpmcounter5   = 0xb05,
    // Mhpmcounter6   = 0xb06,
    // Mhpmcounter7   = 0xb07,
    // Mhpmcounter8   = 0xb08,
    // Mhpmcounter9   = 0xb09,
    // Mhpmcounter10  = 0xb0a,
    // Mhpmcounter11  = 0xb0b,
    // Mhpmcounter12  = 0xb0c,
    // Mhpmcounter13  = 0xb0d,
    // Mhpmcounter14  = 0xb0e,
    // Mhpmcounter15  = 0xb0f,
    // Mhpmcounter16  = 0xb10,
    // Mhpmcounter17  = 0xb11,
    // Mhpmcounter18  = 0xb12,
    // Mhpmcounter19  = 0xb13,
    // Mhpmcounter20  = 0xb14,
    // Mhpmcounter21  = 0xb15,
    // Mhpmcounter22  = 0xb16,
    // Mhpmcounter23  = 0xb17,
    // Mhpmcounter24  = 0xb18,
    // Mhpmcounter25  = 0xb19,
    // Mhpmcounter26  = 0xb1a,
    // Mhpmcounter27  = 0xb1b,
    // Mhpmcounter28  = 0xb1c,
    // Mhpmcounter29  = 0xb1d,
    // Mhpmcounter30  = 0xb1e,
    // Mhpmcounter31  = 0xb1f,
    // Mucounteren    = 0x320,
    // Mhpmevent3     = 0x323,
    // Mhpmevent4     = 0x324,
    // Mhpmevent5     = 0x325,
    // Mhpmevent6     = 0x326,
    // Mhpmevent7     = 0x327,
    // Mhpmevent8     = 0x328,
    // Mhpmevent9     = 0x329,
    // Mhpmevent10    = 0x32a,
    // Mhpmevent11    = 0x32b,
    // Mhpmevent12    = 0x32c,
    // Mhpmevent13    = 0x32d,
    // Mhpmevent14    = 0x32e,
    // Mhpmevent15    = 0x32f,
    // Mhpmevent16    = 0x330,
    // Mhpmevent17    = 0x331,
    // Mhpmevent18    = 0x332,
    // Mhpmevent19    = 0x333,
    // Mhpmevent20    = 0x334,
    // Mhpmevent21    = 0x335,
    // Mhpmevent22    = 0x336,
    // Mhpmevent23    = 0x337,
    // Mhpmevent24    = 0x338,
    // Mhpmevent25    = 0x339,
    // Mhpmevent26    = 0x33a,
    // Mhpmevent27    = 0x33b,
    // Mhpmevent28    = 0x33c,
    // Mhpmevent29    = 0x33d,
    // Mhpmevent30    = 0x33e,
    // Mhpmevent31    = 0x33f,
    Mvendorid = 0xf11,
    Marchid = 0xf12,
    Mimpid = 0xf13,
    Mhartid = 0xf14,
    // Cycleh         = 0xc80,
    // Instreth       = 0xc82,
    // Hpmcounter3h   = 0xc83,
    // Hpmcounter4h   = 0xc84,
    // Hpmcounter5h   = 0xc85,
    // Hpmcounter6h   = 0xc86,
    // Hpmcounter7h   = 0xc87,
    // Hpmcounter8h   = 0xc88,
    // Hpmcounter9h   = 0xc89,
    // Hpmcounter10h  = 0xc8a,
    // Hpmcounter11h  = 0xc8b,
    // Hpmcounter12h  = 0xc8c,
    // Hpmcounter13h  = 0xc8d,
    // Hpmcounter14h  = 0xc8e,
    // Hpmcounter15h  = 0xc8f,
    // Hpmcounter16h  = 0xc90,
    // Hpmcounter17h  = 0xc91,
    // Hpmcounter18h  = 0xc92,
    // Hpmcounter19h  = 0xc93,
    // Hpmcounter20h  = 0xc94,
    // Hpmcounter21h  = 0xc95,
    // Hpmcounter22h  = 0xc96,
    // Hpmcounter23h  = 0xc97,
    // Hpmcounter24h  = 0xc98,
    // Hpmcounter25h  = 0xc99,
    // Hpmcounter26h  = 0xc9a,
    // Hpmcounter27h  = 0xc9b,
    // Hpmcounter28h  = 0xc9c,
    // Hpmcounter29h  = 0xc9d,
    // Hpmcounter30h  = 0xc9e,
    // Hpmcounter31h  = 0xc9f,
    // Mcycleh        = 0xb80,
    // Minstreth      = 0xb82,
    // Mhpmcounter3h  = 0xb83,
    // Mhpmcounter4h  = 0xb84,
    // Mhpmcounter5h  = 0xb85,
    // Mhpmcounter6h  = 0xb86,
    // Mhpmcounter7h  = 0xb87,
    // Mhpmcounter8h  = 0xb88,
    // Mhpmcounter9h  = 0xb89,
    // Mhpmcounter10h = 0xb8a,
    // Mhpmcounter11h = 0xb8b,
    // Mhpmcounter12h = 0xb8c,
    // Mhpmcounter13h = 0xb8d,
    // Mhpmcounter14h = 0xb8e,
    // Mhpmcounter15h = 0xb8f,
    // Mhpmcounter16h = 0xb90,
    // Mhpmcounter17h = 0xb91,
    // Mhpmcounter18h = 0xb92,
    // Mhpmcounter19h = 0xb93,
    // Mhpmcounter20h = 0xb94,
    // Mhpmcounter21h = 0xb95,
    // Mhpmcounter22h = 0xb96,
    // Mhpmcounter23h = 0xb97,
    // Mhpmcounter24h = 0xb98,
    // Mhpmcounter25h = 0xb99,
    // Mhpmcounter26h = 0xb9a,
    // Mhpmcounter27h = 0xb9b,
    // Mhpmcounter28h = 0xb9c,
    // Mhpmcounter29h = 0xb9d,
    // Mhpmcounter30h = 0xb9e,
    // Mhpmcounter31h = 0xb9f,
    Sstatus = 0x100,
    Sedeleg = 0x102,
    Sideleg = 0x103,
    Sie = 0x104,
    Stvec = 0x105,
    Scounteren = 0x106,
    Sscratch = 0x140,
    Sepc = 0x141,
    Scause = 0x142,
    Stval = 0x143,
    Sip = 0x144,
    Satp = 0x180,
}

impl CsrAddr {
    pub fn from_i64(n: i64) -> Option<CsrAddr> {
        match n {
            _ => None,
        }
    }
    pub fn from_u64(n: u64) -> CsrAddr {
        match n {
            // 0xc00 => CsrAddr::Cycle        ,
            // 0xc02 => CsrAddr::Instret      ,
            // 0xc03 => CsrAddr::Hpmcounter3  ,
            // 0xc04 => CsrAddr::Hpmcounter4  ,
            // 0xc05 => CsrAddr::Hpmcounter5  ,
            // 0xc06 => CsrAddr::Hpmcounter6  ,
            // 0xc07 => CsrAddr::Hpmcounter7  ,
            // 0xc08 => CsrAddr::Hpmcounter8  ,
            // 0xc09 => CsrAddr::Hpmcounter9  ,
            // 0xc0a => CsrAddr::Hpmcounter10 ,
            // 0xc0b => CsrAddr::Hpmcounter11 ,
            // 0xc0c => CsrAddr::Hpmcounter12 ,
            // 0xc0d => CsrAddr::Hpmcounter13 ,
            // 0xc0e => CsrAddr::Hpmcounter14 ,
            // 0xc0f => CsrAddr::Hpmcounter15 ,
            // 0xc10 => CsrAddr::Hpmcounter16 ,
            // 0xc11 => CsrAddr::Hpmcounter17 ,
            // 0xc12 => CsrAddr::Hpmcounter18 ,
            // 0xc13 => CsrAddr::Hpmcounter19 ,
            // 0xc14 => CsrAddr::Hpmcounter20 ,
            // 0xc15 => CsrAddr::Hpmcounter21 ,
            // 0xc16 => CsrAddr::Hpmcounter22 ,
            // 0xc17 => CsrAddr::Hpmcounter23 ,
            // 0xc18 => CsrAddr::Hpmcounter24 ,
            // 0xc19 => CsrAddr::Hpmcounter25 ,
            // 0xc1a => CsrAddr::Hpmcounter26 ,
            // 0xc1b => CsrAddr::Hpmcounter27 ,
            // 0xc1c => CsrAddr::Hpmcounter28 ,
            // 0xc1d => CsrAddr::Hpmcounter29 ,
            // 0xc1e => CsrAddr::Hpmcounter30 ,
            // 0xc1f => CsrAddr::Hpmcounter31 ,
            0x300 => CsrAddr::Mstatus,
            0x301 => CsrAddr::Misa,
            0x302 => CsrAddr::Medeleg,
            0x303 => CsrAddr::Mideleg,
            0x304 => CsrAddr::Mie,
            0x305 => CsrAddr::Mtvec,
            0x340 => CsrAddr::Mscratch,
            0x306 => CsrAddr::Mcounteren,
            0x341 => CsrAddr::Mepc,
            0x342 => CsrAddr::Mcause,
            0x343 => CsrAddr::Mtval,
            0x344 => CsrAddr::Mip,
            // 0x7a0 => CsrAddr::Tselect      ,
            // 0x7a1 => CsrAddr::Tdata1       ,
            // 0x7a2 => CsrAddr::Tdata2       ,
            // 0x7a3 => CsrAddr::Tdata3       ,
            0x7b0 => CsrAddr::Dcsr,
            0x7b1 => CsrAddr::Dpc,
            0x7b2 => CsrAddr::Dscratch,
            0xb00 => CsrAddr::Mcycle,
            0xb02 => CsrAddr::Minstret,
            // 0xb03 => CsrAddr::Mhpmcounter3 ,
            // 0xb04 => CsrAddr::Mhpmcounter4 ,
            // 0xb05 => CsrAddr::Mhpmcounter5 ,
            // 0xb06 => CsrAddr::Mhpmcounter6 ,
            // 0xb07 => CsrAddr::Mhpmcounter7 ,
            // 0xb08 => CsrAddr::Mhpmcounter8 ,
            // 0xb09 => CsrAddr::Mhpmcounter9 ,
            // 0xb0a => CsrAddr::Mhpmcounter10,
            // 0xb0b => CsrAddr::Mhpmcounter11,
            // 0xb0c => CsrAddr::Mhpmcounter12,
            // 0xb0d => CsrAddr::Mhpmcounter13,
            // 0xb0e => CsrAddr::Mhpmcounter14,
            // 0xb0f => CsrAddr::Mhpmcounter15,
            // 0xb10 => CsrAddr::Mhpmcounter16,
            // 0xb11 => CsrAddr::Mhpmcounter17,
            // 0xb12 => CsrAddr::Mhpmcounter18,
            // 0xb13 => CsrAddr::Mhpmcounter19,
            // 0xb14 => CsrAddr::Mhpmcounter20,
            // 0xb15 => CsrAddr::Mhpmcounter21,
            // 0xb16 => CsrAddr::Mhpmcounter22,
            // 0xb17 => CsrAddr::Mhpmcounter23,
            // 0xb18 => CsrAddr::Mhpmcounter24,
            // 0xb19 => CsrAddr::Mhpmcounter25,
            // 0xb1a => CsrAddr::Mhpmcounter26,
            // 0xb1b => CsrAddr::Mhpmcounter27,
            // 0xb1c => CsrAddr::Mhpmcounter28,
            // 0xb1d => CsrAddr::Mhpmcounter29,
            // 0xb1e => CsrAddr::Mhpmcounter30,
            // 0xb1f => CsrAddr::Mhpmcounter31,
            // 0x320 => CsrAddr::Mucounteren  ,
            // 0x323 => CsrAddr::Mhpmevent3   ,
            // 0x324 => CsrAddr::Mhpmevent4   ,
            // 0x325 => CsrAddr::Mhpmevent5   ,
            // 0x326 => CsrAddr::Mhpmevent6   ,
            // 0x327 => CsrAddr::Mhpmevent7   ,
            // 0x328 => CsrAddr::Mhpmevent8   ,
            // 0x329 => CsrAddr::Mhpmevent9   ,
            // 0x32a => CsrAddr::Mhpmevent10  ,
            // 0x32b => CsrAddr::Mhpmevent11  ,
            // 0x32c => CsrAddr::Mhpmevent12  ,
            // 0x32d => CsrAddr::Mhpmevent13  ,
            // 0x32e => CsrAddr::Mhpmevent14  ,
            // 0x32f => CsrAddr::Mhpmevent15  ,
            // 0x330 => CsrAddr::Mhpmevent16  ,
            // 0x331 => CsrAddr::Mhpmevent17  ,
            // 0x332 => CsrAddr::Mhpmevent18  ,
            // 0x333 => CsrAddr::Mhpmevent19  ,
            // 0x334 => CsrAddr::Mhpmevent20  ,
            // 0x335 => CsrAddr::Mhpmevent21  ,
            // 0x336 => CsrAddr::Mhpmevent22  ,
            // 0x337 => CsrAddr::Mhpmevent23  ,
            // 0x338 => CsrAddr::Mhpmevent24  ,
            // 0x339 => CsrAddr::Mhpmevent25  ,
            // 0x33a => CsrAddr::Mhpmevent26  ,
            // 0x33b => CsrAddr::Mhpmevent27  ,
            // 0x33c => CsrAddr::Mhpmevent28  ,
            // 0x33d => CsrAddr::Mhpmevent29  ,
            // 0x33e => CsrAddr::Mhpmevent30  ,
            // 0x33f => CsrAddr::Mhpmevent31  ,
            0xf11 => CsrAddr::Mvendorid,
            0xf12 => CsrAddr::Marchid,
            0xf13 => CsrAddr::Mimpid,
            0xf14 => CsrAddr::Mhartid,
            // 0xc80 => CsrAddr::Cycleh       ,
            // 0xc82 => CsrAddr::Instreth     ,
            // 0xc83 => CsrAddr::Hpmcounter3h ,
            // 0xc84 => CsrAddr::Hpmcounter4h ,
            // 0xc85 => CsrAddr::Hpmcounter5h ,
            // 0xc86 => CsrAddr::Hpmcounter6h ,
            // 0xc87 => CsrAddr::Hpmcounter7h ,
            // 0xc88 => CsrAddr::Hpmcounter8h ,
            // 0xc89 => CsrAddr::Hpmcounter9h ,
            // 0xc8a => CsrAddr::Hpmcounter10h,
            // 0xc8b => CsrAddr::Hpmcounter11h,
            // 0xc8c => CsrAddr::Hpmcounter12h,
            // 0xc8d => CsrAddr::Hpmcounter13h,
            // 0xc8e => CsrAddr::Hpmcounter14h,
            // 0xc8f => CsrAddr::Hpmcounter15h,
            // 0xc90 => CsrAddr::Hpmcounter16h,
            // 0xc91 => CsrAddr::Hpmcounter17h,
            // 0xc92 => CsrAddr::Hpmcounter18h,
            // 0xc93 => CsrAddr::Hpmcounter19h,
            // 0xc94 => CsrAddr::Hpmcounter20h,
            // 0xc95 => CsrAddr::Hpmcounter21h,
            // 0xc96 => CsrAddr::Hpmcounter22h,
            // 0xc97 => CsrAddr::Hpmcounter23h,
            // 0xc98 => CsrAddr::Hpmcounter24h,
            // 0xc99 => CsrAddr::Hpmcounter25h,
            // 0xc9a => CsrAddr::Hpmcounter26h,
            // 0xc9b => CsrAddr::Hpmcounter27h,
            // 0xc9c => CsrAddr::Hpmcounter28h,
            // 0xc9d => CsrAddr::Hpmcounter29h,
            // 0xc9e => CsrAddr::Hpmcounter30h,
            // 0xc9f => CsrAddr::Hpmcounter31h,
            // 0xb80 => CsrAddr::Mcycleh      ,
            // 0xb82 => CsrAddr::Minstreth    ,
            // 0xb83 => CsrAddr::Mhpmcounter3h,
            // 0xb84 => CsrAddr::Mhpmcounter4h,
            // 0xb85 => CsrAddr::Mhpmcounter5h,
            // 0xb86 => CsrAddr::Mhpmcounter6h,
            // 0xb87 => CsrAddr::Mhpmcounter7h,
            // 0xb88 => CsrAddr::Mhpmcounter8h,
            // 0xb89 => CsrAddr::Mhpmcounter9h,
            // 0xb8a => CsrAddr::Mhpmcounter10h,
            // 0xb8b => CsrAddr::Mhpmcounter11h,
            // 0xb8c => CsrAddr::Mhpmcounter12h,
            // 0xb8d => CsrAddr::Mhpmcounter13h,
            // 0xb8e => CsrAddr::Mhpmcounter14h,
            // 0xb8f => CsrAddr::Mhpmcounter15h,
            // 0xb90 => CsrAddr::Mhpmcounter16h,
            // 0xb91 => CsrAddr::Mhpmcounter17h,
            // 0xb92 => CsrAddr::Mhpmcounter18h,
            // 0xb93 => CsrAddr::Mhpmcounter19h,
            // 0xb94 => CsrAddr::Mhpmcounter20h,
            // 0xb95 => CsrAddr::Mhpmcounter21h,
            // 0xb96 => CsrAddr::Mhpmcounter22h,
            // 0xb97 => CsrAddr::Mhpmcounter23h,
            // 0xb98 => CsrAddr::Mhpmcounter24h,
            // 0xb99 => CsrAddr::Mhpmcounter25h,
            // 0xb9a => CsrAddr::Mhpmcounter26h,
            // 0xb9b => CsrAddr::Mhpmcounter27h,
            // 0xb9c => CsrAddr::Mhpmcounter28h,
            // 0xb9d => CsrAddr::Mhpmcounter29h,
            // 0xb9e => CsrAddr::Mhpmcounter30h,
            // 0xb9f => CsrAddr::Mhpmcounter31h,
            0x100 => CsrAddr::Sstatus,
            0x102 => CsrAddr::Sedeleg,
            0x103 => CsrAddr::Sideleg,
            0x104 => CsrAddr::Sie,
            0x105 => CsrAddr::Stvec,
            0x106 => CsrAddr::Scounteren,
            0x140 => CsrAddr::Sscratch,
            0x141 => CsrAddr::Sepc,
            0x142 => CsrAddr::Scause,
            0x143 => CsrAddr::Stval,
            0x144 => CsrAddr::Sip,
            0x180 => CsrAddr::Satp,
            _ => CsrAddr::None,
        }
    }
}

pub struct RiscvCsrBase<W> {
    pub m_csr: W,
}

impl RiscvCsrBase<i32> {
    pub fn new() -> RiscvCsrBase<i32> {
        RiscvCsrBase { m_csr: 0x0 }
    }

    fn csrrw(&mut self, imm: i32) -> i32 {
        let ret_val: i32 = self.m_csr;
        self.m_csr = imm;
        return ret_val;
    }

    fn csrrs(&mut self, imm: i32) -> i32 {
        let ret_val: i32 = self.m_csr;
        self.m_csr = self.m_csr | imm;
        return ret_val;
    }

    fn csrrc(&mut self, imm: i32) -> i32 {
        let ret_val: i32 = self.m_csr;
        self.m_csr = self.m_csr & !imm;
        return ret_val;
    }
}

impl RiscvCsrBase<i64> {
    pub fn new() -> RiscvCsrBase<i64> {
        RiscvCsrBase { m_csr: 0x0 }
    }

    fn csrrw(&mut self, imm: i64) -> i64 {
        let ret_val: i64 = self.m_csr;
        self.m_csr = imm;
        return ret_val;
    }

    fn csrrs(&mut self, imm: i64) -> i64 {
        let ret_val: i64 = self.m_csr;
        self.m_csr = self.m_csr | imm;
        return ret_val;
    }

    fn csrrc(&mut self, imm: i64) -> i64 {
        let ret_val: i64 = self.m_csr;
        self.m_csr = self.m_csr & !imm;
        return ret_val;
    }
}

pub struct RiscvCsr<W> {
    pub m_mcycle: RiscvCsrBase<W>,
    pub m_minstret: RiscvCsrBase<W>,
    pub m_mimpid: RiscvCsrBase<W>,
    pub m_marchid: RiscvCsrBase<W>,
    pub m_mvendorid: RiscvCsrBase<W>,
    pub m_misa: RiscvCsrBase<W>,
    pub m_mstatus: RiscvCsrBase<W>,
    pub m_mtvec: RiscvCsrBase<W>,
    pub m_mip: RiscvCsrBase<W>,
    pub m_mie: RiscvCsrBase<W>,
    pub m_mscratch: RiscvCsrBase<W>,
    pub m_mepc: RiscvCsrBase<W>,
    pub m_mtval: RiscvCsrBase<W>,
    pub m_mcause: RiscvCsrBase<W>,
    pub m_mhartid: RiscvCsrBase<W>,
    pub m_dcsr: RiscvCsrBase<W>,
    pub m_dpc: RiscvCsrBase<W>,
    pub m_dscratch: RiscvCsrBase<W>,
    pub m_medeleg: RiscvCsrBase<W>,

    pub m_sstatus: RiscvCsrBase<W>,
    pub m_sedeleg: RiscvCsrBase<W>,
    pub m_sideleg: RiscvCsrBase<W>,
    pub m_sie: RiscvCsrBase<W>,
    pub m_stvec: RiscvCsrBase<W>,
    pub m_scounteren: RiscvCsrBase<W>,
    pub m_sscratch: RiscvCsrBase<W>,
    pub m_sepc: RiscvCsrBase<W>,
    pub m_scause: RiscvCsrBase<W>,
    pub m_stval: RiscvCsrBase<W>,
    pub m_sip: RiscvCsrBase<W>,
    pub m_satp: RiscvCsrBase<W>,
}

impl RiscvCsr<i32> {
    pub fn new() -> RiscvCsr<i32> {
        RiscvCsr {
            m_mcycle: RiscvCsrBase::<i32>::new(),
            m_minstret: RiscvCsrBase::<i32>::new(),
            m_mimpid: RiscvCsrBase::<i32>::new(),
            m_marchid: RiscvCsrBase::<i32>::new(),
            m_mvendorid: RiscvCsrBase::<i32>::new(),
            m_misa: RiscvCsrBase::<i32>::new(),
            m_mstatus: RiscvCsrBase::<i32>::new(),
            m_mtvec: RiscvCsrBase::<i32>::new(),
            m_mip: RiscvCsrBase::<i32>::new(),
            m_mie: RiscvCsrBase::<i32>::new(),
            m_mscratch: RiscvCsrBase::<i32>::new(),
            m_mepc: RiscvCsrBase::<i32>::new(),
            m_mtval: RiscvCsrBase::<i32>::new(),
            m_mcause: RiscvCsrBase::<i32>::new(),
            m_mhartid: RiscvCsrBase::<i32>::new(),
            m_dcsr: RiscvCsrBase::<i32>::new(),
            m_dpc: RiscvCsrBase::<i32>::new(),
            m_dscratch: RiscvCsrBase::<i32>::new(),
            m_medeleg: RiscvCsrBase::<i32>::new(),

            m_sstatus: RiscvCsrBase::<i32>::new(),
            m_sedeleg: RiscvCsrBase::<i32>::new(),
            m_sideleg: RiscvCsrBase::<i32>::new(),
            m_sie: RiscvCsrBase::<i32>::new(),
            m_stvec: RiscvCsrBase::<i32>::new(),
            m_scounteren: RiscvCsrBase::<i32>::new(),
            m_sscratch: RiscvCsrBase::<i32>::new(),
            m_sepc: RiscvCsrBase::<i32>::new(),
            m_scause: RiscvCsrBase::<i32>::new(),
            m_stval: RiscvCsrBase::<i32>::new(),
            m_sip: RiscvCsrBase::<i32>::new(),
            m_satp: RiscvCsrBase::<i32>::new(),
        }
    }

    pub fn csrrw(&mut self, addr: CsrAddr, data: XlenT) -> XlenT {
        match addr {
            CsrAddr::Mcycle => return self.m_mcycle.csrrw(data),
            CsrAddr::Minstret => return self.m_minstret.csrrw(data),
            CsrAddr::Mimpid => return self.m_mimpid.csrrw(data),
            CsrAddr::Marchid => return self.m_marchid.csrrw(data),
            CsrAddr::Mvendorid => return self.m_mvendorid.csrrw(data),
            CsrAddr::Misa => return self.m_misa.csrrw(data),
            CsrAddr::Mstatus => return self.m_mstatus.csrrw(data),
            CsrAddr::Mtvec => return self.m_mtvec.csrrw(data),
            CsrAddr::Mip => return self.m_mip.csrrw(data),
            CsrAddr::Mie => return self.m_mie.csrrw(data),
            CsrAddr::Mscratch => return self.m_mscratch.csrrw(data),
            CsrAddr::Mepc => return self.m_mepc.csrrw(data),
            CsrAddr::Mtval => return self.m_mtval.csrrw(data),
            CsrAddr::Mcause => return self.m_mcause.csrrw(data),
            CsrAddr::Mhartid => return self.m_mhartid.csrrw(data),
            CsrAddr::Dcsr => return self.m_dcsr.csrrw(data),
            CsrAddr::Dpc => return self.m_dpc.csrrw(data),
            CsrAddr::Dscratch => return self.m_dscratch.csrrw(data),
            CsrAddr::Medeleg => return self.m_medeleg.csrrw(data),

            // CsrAddr::Sstatus,
            CsrAddr::Sedeleg => return self.m_sedeleg.csrrw(data),
            CsrAddr::Sideleg => return self.m_sideleg.csrrw(data),
            CsrAddr::Sie => return self.m_sie.csrrw(data),
            CsrAddr::Stvec => return self.m_stvec.csrrw(data),
            CsrAddr::Scounteren => return self.m_scounteren.csrrw(data),
            CsrAddr::Sscratch => return self.m_sscratch.csrrw(data),
            CsrAddr::Sepc => return self.m_sepc.csrrw(data),
            CsrAddr::Scause => return self.m_scause.csrrw(data),
            CsrAddr::Stval => return self.m_stval.csrrw(data),
            CsrAddr::Sip => return self.m_sip.csrrw(data),
            CsrAddr::Satp => return self.m_satp.csrrw(data),
            _ => return 0x0,
        }
    }

    pub fn csrrs(&mut self, addr: CsrAddr, data: XlenT) -> XlenT {
        match addr {
            CsrAddr::Mcycle => return self.m_mcycle.csrrs(data),
            CsrAddr::Minstret => return self.m_minstret.csrrs(data),
            CsrAddr::Mimpid => return self.m_mimpid.csrrs(data),
            CsrAddr::Marchid => return self.m_marchid.csrrs(data),
            CsrAddr::Mvendorid => return self.m_mvendorid.csrrs(data),
            CsrAddr::Misa => return self.m_misa.csrrs(data),
            CsrAddr::Mstatus => return self.m_mstatus.csrrs(data),
            CsrAddr::Mtvec => return self.m_mtvec.csrrs(data),
            CsrAddr::Mip => return self.m_mip.csrrs(data),
            CsrAddr::Mie => return self.m_mie.csrrs(data),
            CsrAddr::Mscratch => return self.m_mscratch.csrrs(data),
            CsrAddr::Mepc => return self.m_mepc.csrrs(data),
            CsrAddr::Mtval => return self.m_mtval.csrrs(data),
            CsrAddr::Mcause => return self.m_mcause.csrrs(data),
            CsrAddr::Mhartid => return self.m_mhartid.csrrs(data),
            CsrAddr::Dcsr => return self.m_dcsr.csrrs(data),
            CsrAddr::Dpc => return self.m_dpc.csrrs(data),
            CsrAddr::Dscratch => return self.m_dscratch.csrrs(data),
            CsrAddr::Medeleg => return self.m_medeleg.csrrs(data),

            // CsrAddr::Sstatus,
            CsrAddr::Sedeleg => return self.m_sedeleg.csrrs(data),
            CsrAddr::Sideleg => return self.m_sideleg.csrrs(data),
            CsrAddr::Sie => return self.m_sie.csrrs(data),
            CsrAddr::Stvec => return self.m_stvec.csrrs(data),
            CsrAddr::Scounteren => return self.m_scounteren.csrrs(data),
            CsrAddr::Sscratch => return self.m_sscratch.csrrs(data),
            CsrAddr::Sepc => return self.m_sepc.csrrs(data),
            CsrAddr::Scause => return self.m_scause.csrrs(data),
            CsrAddr::Stval => return self.m_stval.csrrs(data),
            CsrAddr::Sip => return self.m_sip.csrrs(data),
            CsrAddr::Satp => return self.m_satp.csrrs(data),
            _ => return 0x0,
        }
    }

    pub fn csrrc(&mut self, addr: CsrAddr, data: XlenT) -> XlenT {
        match addr {
            CsrAddr::Mcycle => return self.m_mcycle.csrrc(data),
            CsrAddr::Minstret => return self.m_minstret.csrrc(data),
            CsrAddr::Mimpid => return self.m_mimpid.csrrc(data),
            CsrAddr::Marchid => return self.m_marchid.csrrc(data),
            CsrAddr::Mvendorid => return self.m_mvendorid.csrrc(data),
            CsrAddr::Misa => return self.m_misa.csrrc(data),
            CsrAddr::Mstatus => return self.m_mstatus.csrrc(data),
            CsrAddr::Mtvec => return self.m_mtvec.csrrc(data),
            CsrAddr::Mip => return self.m_mip.csrrc(data),
            CsrAddr::Mie => return self.m_mie.csrrc(data),
            CsrAddr::Mscratch => return self.m_mscratch.csrrc(data),
            CsrAddr::Mepc => return self.m_mepc.csrrc(data),
            CsrAddr::Mtval => return self.m_mtval.csrrc(data),
            CsrAddr::Mcause => return self.m_mcause.csrrc(data),
            CsrAddr::Mhartid => return self.m_mhartid.csrrc(data),
            CsrAddr::Dcsr => return self.m_dcsr.csrrc(data),
            CsrAddr::Dpc => return self.m_dpc.csrrc(data),
            CsrAddr::Dscratch => return self.m_dscratch.csrrc(data),
            CsrAddr::Medeleg => return self.m_medeleg.csrrc(data),

            // CsrAddr::Sstatus,
            CsrAddr::Sedeleg => return self.m_sedeleg.csrrc(data),
            CsrAddr::Sideleg => return self.m_sideleg.csrrc(data),
            CsrAddr::Sie => return self.m_sie.csrrc(data),
            CsrAddr::Stvec => return self.m_stvec.csrrc(data),
            CsrAddr::Scounteren => return self.m_scounteren.csrrc(data),
            CsrAddr::Sscratch => return self.m_sscratch.csrrc(data),
            CsrAddr::Sepc => return self.m_sepc.csrrc(data),
            CsrAddr::Scause => return self.m_scause.csrrc(data),
            CsrAddr::Stval => return self.m_stval.csrrc(data),
            CsrAddr::Sip => return self.m_sip.csrrc(data),
            CsrAddr::Satp => return self.m_satp.csrrc(data),
            _ => return 0x0,
        }
    }
}

impl RiscvCsr<i64> {
    pub fn new() -> RiscvCsr<i64> {
        RiscvCsr {
            m_mcycle: RiscvCsrBase::<i64>::new(),
            m_minstret: RiscvCsrBase::<i64>::new(),
            m_mimpid: RiscvCsrBase::<i64>::new(),
            m_marchid: RiscvCsrBase::<i64>::new(),
            m_mvendorid: RiscvCsrBase::<i64>::new(),
            m_misa: RiscvCsrBase::<i64>::new(),
            m_mstatus: RiscvCsrBase::<i64>::new(),
            m_mtvec: RiscvCsrBase::<i64>::new(),
            m_mip: RiscvCsrBase::<i64>::new(),
            m_mie: RiscvCsrBase::<i64>::new(),
            m_mscratch: RiscvCsrBase::<i64>::new(),
            m_mepc: RiscvCsrBase::<i64>::new(),
            m_mtval: RiscvCsrBase::<i64>::new(),
            m_mcause: RiscvCsrBase::<i64>::new(),
            m_mhartid: RiscvCsrBase::<i64>::new(),
            m_dcsr: RiscvCsrBase::<i64>::new(),
            m_dpc: RiscvCsrBase::<i64>::new(),
            m_dscratch: RiscvCsrBase::<i64>::new(),
            m_medeleg: RiscvCsrBase::<i64>::new(),

            m_sstatus: RiscvCsrBase::<i64>::new(),
            m_sedeleg: RiscvCsrBase::<i64>::new(),
            m_sideleg: RiscvCsrBase::<i64>::new(),
            m_sie: RiscvCsrBase::<i64>::new(),
            m_stvec: RiscvCsrBase::<i64>::new(),
            m_scounteren: RiscvCsrBase::<i64>::new(),
            m_sscratch: RiscvCsrBase::<i64>::new(),
            m_sepc: RiscvCsrBase::<i64>::new(),
            m_scause: RiscvCsrBase::<i64>::new(),
            m_stval: RiscvCsrBase::<i64>::new(),
            m_sip: RiscvCsrBase::<i64>::new(),
            m_satp: RiscvCsrBase::<i64>::new(),
        }
    }

    pub fn csrrw(&mut self, addr: CsrAddr, data: Xlen64T) -> Xlen64T {
        match addr {
            CsrAddr::Mcycle => return self.m_mcycle.csrrw(data),
            CsrAddr::Minstret => return self.m_minstret.csrrw(data),
            CsrAddr::Mimpid => return self.m_mimpid.csrrw(data),
            CsrAddr::Marchid => return self.m_marchid.csrrw(data),
            CsrAddr::Mvendorid => return self.m_mvendorid.csrrw(data),
            CsrAddr::Misa => return self.m_misa.csrrw(data),
            CsrAddr::Mstatus => return self.m_mstatus.csrrw(data),
            CsrAddr::Mtvec => return self.m_mtvec.csrrw(data),
            CsrAddr::Mip => return self.m_mip.csrrw(data),
            CsrAddr::Mie => return self.m_mie.csrrw(data),
            CsrAddr::Mscratch => return self.m_mscratch.csrrw(data),
            CsrAddr::Mepc => return self.m_mepc.csrrw(data),
            CsrAddr::Mtval => return self.m_mtval.csrrw(data),
            CsrAddr::Mcause => return self.m_mcause.csrrw(data),
            CsrAddr::Mhartid => return self.m_mhartid.csrrw(data),
            CsrAddr::Dcsr => return self.m_dcsr.csrrw(data),
            CsrAddr::Dpc => return self.m_dpc.csrrw(data),
            CsrAddr::Dscratch => return self.m_dscratch.csrrw(data),
            CsrAddr::Medeleg => return self.m_medeleg.csrrw(data),

            // CsrAddr::Sstatus,
            CsrAddr::Sedeleg => return self.m_sedeleg.csrrw(data),
            CsrAddr::Sideleg => return self.m_sideleg.csrrw(data),
            CsrAddr::Sie => return self.m_sie.csrrw(data),
            CsrAddr::Stvec => return self.m_stvec.csrrw(data),
            CsrAddr::Scounteren => return self.m_scounteren.csrrw(data),
            CsrAddr::Sscratch => return self.m_sscratch.csrrw(data),
            CsrAddr::Sepc => return self.m_sepc.csrrw(data),
            CsrAddr::Scause => return self.m_scause.csrrw(data),
            CsrAddr::Stval => return self.m_stval.csrrw(data),
            CsrAddr::Sip => return self.m_sip.csrrw(data),
            CsrAddr::Satp => return self.m_satp.csrrw(data),
            _ => return 0x0,
        }
    }

    pub fn csrrs(&mut self, addr: CsrAddr, data: Xlen64T) -> Xlen64T {
        match addr {
            CsrAddr::Mcycle => return self.m_mcycle.csrrs(data),
            CsrAddr::Minstret => return self.m_minstret.csrrs(data),
            CsrAddr::Mimpid => return self.m_mimpid.csrrs(data),
            CsrAddr::Marchid => return self.m_marchid.csrrs(data),
            CsrAddr::Mvendorid => return self.m_mvendorid.csrrs(data),
            CsrAddr::Misa => return self.m_misa.csrrs(data),
            CsrAddr::Mstatus => return self.m_mstatus.csrrs(data),
            CsrAddr::Mtvec => return self.m_mtvec.csrrs(data),
            CsrAddr::Mip => return self.m_mip.csrrs(data),
            CsrAddr::Mie => return self.m_mie.csrrs(data),
            CsrAddr::Mscratch => return self.m_mscratch.csrrs(data),
            CsrAddr::Mepc => return self.m_mepc.csrrs(data),
            CsrAddr::Mtval => return self.m_mtval.csrrs(data),
            CsrAddr::Mcause => return self.m_mcause.csrrs(data),
            CsrAddr::Mhartid => return self.m_mhartid.csrrs(data),
            CsrAddr::Dcsr => return self.m_dcsr.csrrs(data),
            CsrAddr::Dpc => return self.m_dpc.csrrs(data),
            CsrAddr::Dscratch => return self.m_dscratch.csrrs(data),
            CsrAddr::Medeleg => return self.m_medeleg.csrrs(data),

            // CsrAddr::Sstatus,
            CsrAddr::Sedeleg => return self.m_sedeleg.csrrs(data),
            CsrAddr::Sideleg => return self.m_sideleg.csrrs(data),
            CsrAddr::Sie => return self.m_sie.csrrs(data),
            CsrAddr::Stvec => return self.m_stvec.csrrs(data),
            CsrAddr::Scounteren => return self.m_scounteren.csrrs(data),
            CsrAddr::Sscratch => return self.m_sscratch.csrrs(data),
            CsrAddr::Sepc => return self.m_sepc.csrrs(data),
            CsrAddr::Scause => return self.m_scause.csrrs(data),
            CsrAddr::Stval => return self.m_stval.csrrs(data),
            CsrAddr::Sip => return self.m_sip.csrrs(data),
            CsrAddr::Satp => return self.m_satp.csrrs(data),
            _ => return 0x0,
        }
    }

    pub fn csrrc(&mut self, addr: CsrAddr, data: Xlen64T) -> Xlen64T {
        match addr {
            CsrAddr::Mcycle => return self.m_mcycle.csrrc(data),
            CsrAddr::Minstret => return self.m_minstret.csrrc(data),
            CsrAddr::Mimpid => return self.m_mimpid.csrrc(data),
            CsrAddr::Marchid => return self.m_marchid.csrrc(data),
            CsrAddr::Mvendorid => return self.m_mvendorid.csrrc(data),
            CsrAddr::Misa => return self.m_misa.csrrc(data),
            CsrAddr::Mstatus => return self.m_mstatus.csrrc(data),
            CsrAddr::Mtvec => return self.m_mtvec.csrrc(data),
            CsrAddr::Mip => return self.m_mip.csrrc(data),
            CsrAddr::Mie => return self.m_mie.csrrc(data),
            CsrAddr::Mscratch => return self.m_mscratch.csrrc(data),
            CsrAddr::Mepc => return self.m_mepc.csrrc(data),
            CsrAddr::Mtval => return self.m_mtval.csrrc(data),
            CsrAddr::Mcause => return self.m_mcause.csrrc(data),
            CsrAddr::Mhartid => return self.m_mhartid.csrrc(data),
            CsrAddr::Dcsr => return self.m_dcsr.csrrc(data),
            CsrAddr::Dpc => return self.m_dpc.csrrc(data),
            CsrAddr::Dscratch => return self.m_dscratch.csrrc(data),
            CsrAddr::Medeleg => return self.m_medeleg.csrrc(data),

            // CsrAddr::Sstatus,
            CsrAddr::Sedeleg => return self.m_sedeleg.csrrc(data),
            CsrAddr::Sideleg => return self.m_sideleg.csrrc(data),
            CsrAddr::Sie => return self.m_sie.csrrc(data),
            CsrAddr::Stvec => return self.m_stvec.csrrc(data),
            CsrAddr::Scounteren => return self.m_scounteren.csrrc(data),
            CsrAddr::Sscratch => return self.m_sscratch.csrrc(data),
            CsrAddr::Sepc => return self.m_sepc.csrrc(data),
            CsrAddr::Scause => return self.m_scause.csrrc(data),
            CsrAddr::Stval => return self.m_stval.csrrc(data),
            CsrAddr::Sip => return self.m_sip.csrrc(data),
            CsrAddr::Satp => return self.m_satp.csrrc(data),
            _ => return 0x0,
        }
    }
}
