#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port configuration register low (GPIOn_CRL)"]
    pub crl: CRL,
    #[doc = "0x04 - Port configuration register high (GPIOn_CRL)"]
    pub crh: CRH,
    #[doc = "0x08 - Port input data register (GPIOn_IDR)"]
    pub idr: IDR,
    #[doc = "0x0c - Port output data register (GPIOn_ODR)"]
    pub odr: ODR,
    #[doc = "0x10 - Port bit set/reset register (GPIOn_BSRR)"]
    pub bsrr: BSRR,
    #[doc = "0x14 - Port bit reset register (GPIOn_BRR)"]
    pub brr: BRR,
    #[doc = "0x18 - Port configuration lock register"]
    pub lckr: LCKR,
}
#[doc = "Port configuration register low (GPIOn_CRL)"]
pub struct CRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port configuration register low (GPIOn_CRL)"]
pub mod crl;
#[doc = "Port configuration register high (GPIOn_CRL)"]
pub struct CRH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port configuration register high (GPIOn_CRL)"]
pub mod crh;
#[doc = "Port input data register (GPIOn_IDR)"]
pub struct IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port input data register (GPIOn_IDR)"]
pub mod idr;
#[doc = "Port output data register (GPIOn_ODR)"]
pub struct ODR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port output data register (GPIOn_ODR)"]
pub mod odr;
#[doc = "Port bit set/reset register (GPIOn_BSRR)"]
pub struct BSRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port bit set/reset register (GPIOn_BSRR)"]
pub mod bsrr;
#[doc = "Port bit reset register (GPIOn_BRR)"]
pub struct BRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port bit reset register (GPIOn_BRR)"]
pub mod brr;
#[doc = "Port configuration lock register"]
pub struct LCKR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port configuration lock register"]
pub mod lckr;
