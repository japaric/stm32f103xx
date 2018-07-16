#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCR"]
    pub mcr: MCR,
    #[doc = "0x04 - MSR"]
    pub msr: MSR,
    #[doc = "0x08 - TSR"]
    pub tsr: TSR,
    #[doc = "0x0c - RFxR"]
    pub rfr: [RFR; 2],
    #[doc = "0x14 - IER"]
    pub ier: IER,
    #[doc = "0x18 - ESR"]
    pub esr: ESR,
    #[doc = "0x1c - BTR"]
    pub btr: BTR,
    _reserved0: [u8; 352usize],
    #[doc = "0x180 - TIxR, TDTxR, TDLxR, TDHxR"]
    pub tx: [TX; 3],
    #[doc = "0x1b0 - RIxR, RDTxR, RDLxR, TDHxR"]
    pub rx: [RX; 2],
    _reserved1: [u8; 48usize],
    #[doc = "0x200 - FMR"]
    pub fmr: FMR,
    #[doc = "0x204 - FM1R"]
    pub fm1r: FM1R,
    _reserved2: [u8; 4usize],
    #[doc = "0x20c - FS1R"]
    pub fs1r: FS1R,
    _reserved3: [u8; 4usize],
    #[doc = "0x214 - FFA1R"]
    pub ffa1r: FFA1R,
    _reserved4: [u8; 4usize],
    #[doc = "0x21c - FA1R"]
    pub fa1r: FA1R,
    _reserved5: [u8; 32usize],
    #[doc = "0x240 - FxR1, FxR2"]
    pub fb: [FB; 14],
}
#[doc = r" Register block"]
#[repr(C)]
pub struct TX {
    #[doc = "0x00 - TIxR"]
    pub tir: self::tx::TIR,
    #[doc = "0x04 - TDTxR"]
    pub tdtr: self::tx::TDTR,
    #[doc = "0x08 - TDLxR"]
    pub tdlr: self::tx::TDLR,
    #[doc = "0x0c - TDHxR"]
    pub tdhr: self::tx::TDHR,
}
#[doc = r" Register block"]
#[doc = "TIxR, TDTxR, TDLxR, TDHxR"]
pub mod tx;
#[doc = r" Register block"]
#[repr(C)]
pub struct RX {
    #[doc = "0x00 - RIxR"]
    pub rir: self::rx::RIR,
    #[doc = "0x04 - RDTxR"]
    pub rdtr: self::rx::RDTR,
    #[doc = "0x08 - RDLxR"]
    pub rdlr: self::rx::RDLR,
    #[doc = "0x0c - RDHxR"]
    pub rdhr: self::rx::RDHR,
}
#[doc = r" Register block"]
#[doc = "RIxR, RDTxR, RDLxR, TDHxR"]
pub mod rx;
#[doc = r" Register block"]
#[repr(C)]
pub struct FB {
    #[doc = "0x00 - Filter bank x register 1"]
    pub fr1: self::fb::FR1,
    #[doc = "0x04 - Filter bank x register 2"]
    pub fr2: self::fb::FR2,
}
#[doc = r" Register block"]
#[doc = "FxR1, FxR2"]
pub mod fb;
#[doc = "MCR"]
pub struct MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCR"]
pub mod mcr;
#[doc = "MSR"]
pub struct MSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MSR"]
pub mod msr;
#[doc = "TSR"]
pub struct TSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSR"]
pub mod tsr;
#[doc = "RFxR"]
pub struct RFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RFxR"]
pub mod rfr;
#[doc = "IER"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IER"]
pub mod ier;
#[doc = "ESR"]
pub struct ESR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ESR"]
pub mod esr;
#[doc = "BTR"]
pub struct BTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BTR"]
pub mod btr;
#[doc = "FMR"]
pub struct FMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FMR"]
pub mod fmr;
#[doc = "FM1R"]
pub struct FM1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FM1R"]
pub mod fm1r;
#[doc = "FS1R"]
pub struct FS1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FS1R"]
pub mod fs1r;
#[doc = "FFA1R"]
pub struct FFA1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FFA1R"]
pub mod ffa1r;
#[doc = "FA1R"]
pub struct FA1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FA1R"]
pub mod fa1r;
