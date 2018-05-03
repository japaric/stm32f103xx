#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Bits 1:0 = PWRCTRL: Power supply control bits"]
    pub power: POWER,
    #[doc = "0x04 - SDI clock control register (SDIO_CLKCR)"]
    pub clkcr: CLKCR,
    #[doc = "0x08 - Bits 31:0 = : Command argument"]
    pub arg: ARG,
    #[doc = "0x0c - SDIO command register (SDIO_CMD)"]
    pub cmd: CMD,
    #[doc = "0x10 - SDIO command register"]
    pub respcmd: RESPCMD,
    #[doc = "0x14 - Bits 31:0 = CARDSTATUS1"]
    pub respi1: RESPI1,
    #[doc = "0x18 - Bits 31:0 = CARDSTATUS2"]
    pub resp2: RESP2,
    #[doc = "0x1c - Bits 31:0 = CARDSTATUS3"]
    pub resp3: RESP3,
    #[doc = "0x20 - Bits 31:0 = CARDSTATUS4"]
    pub resp4: RESP4,
    #[doc = "0x24 - Bits 31:0 = DATATIME: Data timeout period"]
    pub dtimer: DTIMER,
    #[doc = "0x28 - Bits 24:0 = DATALENGTH: Data length value"]
    pub dlen: DLEN,
    #[doc = "0x2c - SDIO data control register (SDIO_DCTRL)"]
    pub dctrl: DCTRL,
    #[doc = "0x30 - Bits 24:0 = DATACOUNT: Data count value"]
    pub dcount: DCOUNT,
    #[doc = "0x34 - SDIO status register (SDIO_STA)"]
    pub sta: STA,
    #[doc = "0x38 - SDIO interrupt clear register (SDIO_ICR)"]
    pub icr: ICR,
    #[doc = "0x3c - SDIO mask register (SDIO_MASK)"]
    pub mask: MASK,
    _reserved0: [u8; 8usize],
    #[doc = "0x48 - Bits 23:0 = FIFOCOUNT: Remaining number of words to be written to or read from the FIFO"]
    pub fifocnt: FIFOCNT,
    _reserved1: [u8; 52usize],
    #[doc = "0x80 - bits 31:0 = FIFOData: Receive and transmit FIFO data"]
    pub fifo: FIFO,
}
#[doc = "Bits 1:0 = PWRCTRL: Power supply control bits"]
pub struct POWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bits 1:0 = PWRCTRL: Power supply control bits"]
pub mod power;
#[doc = "SDI clock control register (SDIO_CLKCR)"]
pub struct CLKCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDI clock control register (SDIO_CLKCR)"]
pub mod clkcr;
#[doc = "Bits 31:0 = : Command argument"]
pub struct ARG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bits 31:0 = : Command argument"]
pub mod arg;
#[doc = "SDIO command register (SDIO_CMD)"]
pub struct CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDIO command register (SDIO_CMD)"]
pub mod cmd;
#[doc = "SDIO command register"]
pub struct RESPCMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDIO command register"]
pub mod respcmd;
#[doc = "Bits 31:0 = CARDSTATUS1"]
pub struct RESPI1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bits 31:0 = CARDSTATUS1"]
pub mod respi1;
#[doc = "Bits 31:0 = CARDSTATUS2"]
pub struct RESP2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bits 31:0 = CARDSTATUS2"]
pub mod resp2;
#[doc = "Bits 31:0 = CARDSTATUS3"]
pub struct RESP3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bits 31:0 = CARDSTATUS3"]
pub mod resp3;
#[doc = "Bits 31:0 = CARDSTATUS4"]
pub struct RESP4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bits 31:0 = CARDSTATUS4"]
pub mod resp4;
#[doc = "Bits 31:0 = DATATIME: Data timeout period"]
pub struct DTIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bits 31:0 = DATATIME: Data timeout period"]
pub mod dtimer;
#[doc = "Bits 24:0 = DATALENGTH: Data length value"]
pub struct DLEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bits 24:0 = DATALENGTH: Data length value"]
pub mod dlen;
#[doc = "SDIO data control register (SDIO_DCTRL)"]
pub struct DCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDIO data control register (SDIO_DCTRL)"]
pub mod dctrl;
#[doc = "Bits 24:0 = DATACOUNT: Data count value"]
pub struct DCOUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bits 24:0 = DATACOUNT: Data count value"]
pub mod dcount;
#[doc = "SDIO status register (SDIO_STA)"]
pub struct STA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDIO status register (SDIO_STA)"]
pub mod sta;
#[doc = "SDIO interrupt clear register (SDIO_ICR)"]
pub struct ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDIO interrupt clear register (SDIO_ICR)"]
pub mod icr;
#[doc = "SDIO mask register (SDIO_MASK)"]
pub struct MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDIO mask register (SDIO_MASK)"]
pub mod mask;
#[doc = "Bits 23:0 = FIFOCOUNT: Remaining number of words to be written to or read from the FIFO"]
pub struct FIFOCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bits 23:0 = FIFOCOUNT: Remaining number of words to be written to or read from the FIFO"]
pub mod fifocnt;
#[doc = "bits 31:0 = FIFOData: Receive and transmit FIFO data"]
pub struct FIFO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "bits 31:0 = FIFOData: Receive and transmit FIFO data"]
pub mod fifo;
