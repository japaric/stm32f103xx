#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - master control register"]
    pub mcr: MCR,
    #[doc = "0x04 - master status register"]
    pub msr: MSR,
    #[doc = "0x08 - transmit status register"]
    pub tsr: TSR,
    #[doc = "0x0c - receive FIFO X register"]
    pub rfr: [RFR; 2],
    #[doc = "0x14 - interrupt enable register"]
    pub ier: IER,
    #[doc = "0x18 - error status register"]
    pub esr: ESR,
    #[doc = "0x1c - bit timing register"]
    pub btr: BTR,
    _reserved0: [u8; 352usize],
    #[doc = "0x180 - TIxR, TDTxR, TDLxR, TDHxR"]
    pub tx: [TX; 3],
    #[doc = "0x1b0 - RIxR, RDTxR, RDLxR, TDHxR"]
    pub rx: [RX; 2],
    _reserved1: [u8; 48usize],
    #[doc = "0x200 - filter master register"]
    pub fmr: FMR,
    #[doc = "0x204 - filter mode register"]
    pub fm1r: FM1R,
    _reserved2: [u8; 4usize],
    #[doc = "0x20c - filter scale register"]
    pub fs1r: FS1R,
    _reserved3: [u8; 4usize],
    #[doc = "0x214 - filter FIFO assignment register"]
    pub ffa1r: FFA1R,
    _reserved4: [u8; 4usize],
    #[doc = "0x21c - CAN filter activation register"]
    pub fa1r: FA1R,
    _reserved5: [u8; 32usize],
    #[doc = "0x240 - FxR1, FxR2"]
    pub fb: [FB; 14],
}
#[doc = r" Register block"]
#[repr(C)]
pub struct TX {
    #[doc = "0x00 - TX mailbox identifier register"]
    pub tir: self::tx::TIR,
    #[doc = "0x04 - mailbox data length control and time stamp register"]
    pub tdtr: self::tx::TDTR,
    #[doc = "0x08 - mailbox data low register"]
    pub tdlr: self::tx::TDLR,
    #[doc = "0x0c - mailbox data high register"]
    pub tdhr: self::tx::TDHR,
}
#[doc = r" Register block"]
#[doc = "TIxR, TDTxR, TDLxR, TDHxR"]
pub mod tx;
#[doc = r" Register block"]
#[repr(C)]
pub struct RX {
    #[doc = "0x00 - receive FIFO mailbox identifier register"]
    pub rir: self::rx::RIR,
    #[doc = "0x04 - mailbox data high register"]
    pub rdtr: self::rx::RDTR,
    #[doc = "0x08 - mailbox data low register"]
    pub rdlr: self::rx::RDLR,
    #[doc = "0x0c - mailbox data high register"]
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
#[doc = "master control register"]
pub struct MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "master control register"]
pub mod mcr;
#[doc = "master status register"]
pub struct MSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "master status register"]
pub mod msr;
#[doc = "transmit status register"]
pub struct TSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "transmit status register"]
pub mod tsr;
#[doc = "receive FIFO X register"]
pub struct RFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "receive FIFO X register"]
pub mod rfr;
#[doc = "interrupt enable register"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt enable register"]
pub mod ier;
#[doc = "error status register"]
pub struct ESR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "error status register"]
pub mod esr;
#[doc = "bit timing register"]
pub struct BTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "bit timing register"]
pub mod btr;
#[doc = "filter master register"]
pub struct FMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "filter master register"]
pub mod fmr;
#[doc = "filter mode register"]
pub struct FM1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "filter mode register"]
pub mod fm1r;
#[doc = "filter scale register"]
pub struct FS1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "filter scale register"]
pub mod fs1r;
#[doc = "filter FIFO assignment register"]
pub struct FFA1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "filter FIFO assignment register"]
pub mod ffa1r;
#[doc = "CAN filter activation register"]
pub struct FA1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN filter activation register"]
pub mod fa1r;
