#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power control register (PWR_CR)"]
    pub cr: CR,
    #[doc = "0x04 - Power control register (PWR_CR)"]
    pub csr: CSR,
}
#[doc = "Power control register (PWR_CR)"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power control register (PWR_CR)"]
pub mod cr;
#[doc = "Power control register (PWR_CR)"]
pub struct CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power control register (PWR_CR)"]
pub mod csr;
