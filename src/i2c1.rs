#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register 1"]
    pub cr1: CR1,
    #[doc = "0x04 - Control register 2"]
    pub cr2: CR2,
    #[doc = "0x08 - Own address register 1"]
    pub oar1: OAR1,
    #[doc = "0x0c - Own address register 2"]
    pub oar2: OAR2,
    #[doc = "0x10 - Data register"]
    pub dr: DR,
    #[doc = "0x14 - Status register 1"]
    pub sr1: SR1,
    #[doc = "0x18 - Status register 2"]
    pub sr2: SR2,
    #[doc = "0x1c - Clock control register"]
    pub ccr: CCR,
    #[doc = "0x20 - TRISE register"]
    pub trise: TRISE,
}
#[doc = "Control register 1"]
pub struct CR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register 1"]
pub mod cr1;
#[doc = "Control register 2"]
pub struct CR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register 2"]
pub mod cr2;
#[doc = "Own address register 1"]
pub struct OAR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Own address register 1"]
pub mod oar1;
#[doc = "Own address register 2"]
pub struct OAR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Own address register 2"]
pub mod oar2;
#[doc = "Data register"]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data register"]
pub mod dr;
#[doc = "Status register 1"]
pub struct SR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register 1"]
pub mod sr1;
#[doc = "Status register 2"]
pub struct SR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register 2"]
pub mod sr2;
#[doc = "Clock control register"]
pub struct CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock control register"]
pub mod ccr;
#[doc = "TRISE register"]
pub struct TRISE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRISE register"]
pub mod trise;
