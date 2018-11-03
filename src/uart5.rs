#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status register"]
    pub sr: SR,
    #[doc = "0x04 - Data register"]
    pub dr: DR,
    #[doc = "0x08 - Baud rate register"]
    pub brr: BRR,
    #[doc = "0x0c - Control register 1"]
    pub cr1: CR1,
    #[doc = "0x10 - Control register 2"]
    pub cr2: CR2,
    #[doc = "0x14 - Control register 3"]
    pub cr3: CR3,
}
#[doc = "Status register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register"]
pub mod sr;
#[doc = "Data register"]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data register"]
pub mod dr;
#[doc = "Baud rate register"]
pub struct BRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Baud rate register"]
pub mod brr;
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
#[doc = "Control register 3"]
pub struct CR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register 3"]
pub mod cr3;
