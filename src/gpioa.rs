#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port configuration register low"]
    pub crl: CRL,
    #[doc = "0x04 - Port configuration register high"]
    pub crh: CRH,
    #[doc = "0x08 - GPIO port Input data register"]
    pub idr: IDR,
    #[doc = "0x0c - GPIO port Output data register"]
    pub odr: ODR,
    #[doc = "0x10 - GPIO port bit set/reset register"]
    pub bsrr: BSRR,
    #[doc = "0x14 - GPIO port bit reset register"]
    pub brr: BRR,
    #[doc = "0x18 - GPIO port configuration lock register"]
    pub lckr: LCKR,
}
#[doc = "Port configuration register low"]
pub struct CRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port configuration register low"]
pub mod crl;
#[doc = "Port configuration register high"]
pub struct CRH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port configuration register high"]
pub mod crh;
#[doc = "GPIO port Input data register"]
pub struct IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO port Input data register"]
pub mod idr;
#[doc = "GPIO port Output data register"]
pub struct ODR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO port Output data register"]
pub mod odr;
#[doc = "GPIO port bit set/reset register"]
pub struct BSRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO port bit set/reset register"]
pub mod bsrr;
#[doc = "GPIO port bit reset register"]
pub struct BRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO port bit reset register"]
pub mod brr;
#[doc = "GPIO port configuration lock register"]
pub struct LCKR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO port configuration lock register"]
pub mod lckr;
