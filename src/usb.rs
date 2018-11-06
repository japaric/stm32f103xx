#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - endpoint register array"]
    pub epr: [EPR; 8],
    _reserved0: [u8; 32usize],
    #[doc = "0x40 - control register"]
    pub cntr: CNTR,
    #[doc = "0x44 - interrupt status register"]
    pub istr: ISTR,
    #[doc = "0x48 - frame number register"]
    pub fnr: FNR,
    #[doc = "0x4c - device address"]
    pub daddr: DADDR,
    #[doc = "0x50 - Buffer table address"]
    pub btable: BTABLE,
}
#[doc = "endpoint register array"]
pub struct EPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "endpoint register array"]
pub mod epr;
#[doc = "control register"]
pub struct CNTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register"]
pub mod cntr;
#[doc = "interrupt status register"]
pub struct ISTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt status register"]
pub mod istr;
#[doc = "frame number register"]
pub struct FNR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "frame number register"]
pub mod fnr;
#[doc = "device address"]
pub struct DADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device address"]
pub mod daddr;
#[doc = "Buffer table address"]
pub struct BTABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Buffer table address"]
pub mod btable;
