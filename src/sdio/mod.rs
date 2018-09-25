#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power control register"]
    pub power: POWER,
    #[doc = "0x04 - SDI clock control register"]
    pub clkcr: CLKCR,
    #[doc = "0x08 - argument register"]
    pub arg: ARG,
    #[doc = "0x0c - command register"]
    pub cmd: CMD,
    #[doc = "0x10 - command response register"]
    pub respcmd: RESPCMD,
    #[doc = "0x14 - response 1..4 register"]
    pub resp1: RESP,
    #[doc = "0x18 - response 1..4 register"]
    pub resp2: RESP,
    #[doc = "0x1c - response 1..4 register"]
    pub resp3: RESP,
    #[doc = "0x20 - response 1..4 register"]
    pub resp4: RESP,
    #[doc = "0x24 - data timer register"]
    pub dtimer: DTIMER,
    #[doc = "0x28 - data length register"]
    pub dlen: DLEN,
    #[doc = "0x2c - data control register"]
    pub dctrl: DCTRL,
    #[doc = "0x30 - data counter register"]
    pub dcount: DCOUNT,
    #[doc = "0x34 - status register"]
    pub sta: STA,
    #[doc = "0x38 - interrupt clear register"]
    pub icr: ICR,
    #[doc = "0x3c - mask register"]
    pub mask: MASK,
    _reserved0: [u8; 8usize],
    #[doc = "0x48 - FIFO counter register"]
    pub fifocnt: FIFOCNT,
    _reserved1: [u8; 52usize],
    #[doc = "0x80 - data FIFO register"]
    pub fifo: FIFO,
}
#[doc = "Power control register"]
pub struct POWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power control register"]
pub mod power;
#[doc = "SDI clock control register"]
pub struct CLKCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDI clock control register"]
pub mod clkcr;
#[doc = "argument register"]
pub struct ARG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "argument register"]
pub mod arg;
#[doc = "command register"]
pub struct CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "command register"]
pub mod cmd;
#[doc = "command response register"]
pub struct RESPCMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "command response register"]
pub mod respcmd;
#[doc = "response 1..4 register"]
pub struct RESP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "response 1..4 register"]
pub mod resp;
#[doc = "data timer register"]
pub struct DTIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "data timer register"]
pub mod dtimer;
#[doc = "data length register"]
pub struct DLEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "data length register"]
pub mod dlen;
#[doc = "data control register"]
pub struct DCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "data control register"]
pub mod dctrl;
#[doc = "data counter register"]
pub struct DCOUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "data counter register"]
pub mod dcount;
#[doc = "status register"]
pub struct STA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "status register"]
pub mod sta;
#[doc = "interrupt clear register"]
pub struct ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt clear register"]
pub mod icr;
#[doc = "mask register"]
pub struct MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "mask register"]
pub mod mask;
#[doc = "FIFO counter register"]
pub struct FIFOCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO counter register"]
pub mod fifocnt;
#[doc = "data FIFO register"]
pub struct FIFO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "data FIFO register"]
pub mod fifo;
