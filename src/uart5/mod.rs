#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART4_SR"]
    pub sr: SR,
    #[doc = "0x04 - UART4_DR"]
    pub dr: DR,
    #[doc = "0x08 - UART4_BRR"]
    pub brr: BRR,
    #[doc = "0x0c - UART4_CR1"]
    pub cr1: CR1,
    #[doc = "0x10 - UART4_CR2"]
    pub cr2: CR2,
    #[doc = "0x14 - UART4_CR3"]
    pub cr3: CR3,
}
#[doc = "UART4_SR"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART4_SR"]
pub mod sr;
#[doc = "UART4_DR"]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART4_DR"]
pub mod dr;
#[doc = "UART4_BRR"]
pub struct BRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART4_BRR"]
pub mod brr;
#[doc = "UART4_CR1"]
pub struct CR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART4_CR1"]
pub mod cr1;
#[doc = "UART4_CR2"]
pub struct CR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART4_CR2"]
pub mod cr2;
#[doc = "UART4_CR3"]
pub struct CR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART4_CR3"]
pub mod cr3;
