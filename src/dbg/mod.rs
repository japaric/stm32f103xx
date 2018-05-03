#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DBGMCU_IDCODE"]
    pub idcode: IDCODE,
    #[doc = "0x04 - DBGMCU_CR"]
    pub cr: CR,
}
#[doc = "DBGMCU_IDCODE"]
pub struct IDCODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DBGMCU_IDCODE"]
pub mod idcode;
#[doc = "DBGMCU_CR"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DBGMCU_CR"]
pub mod cr;
