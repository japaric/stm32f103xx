#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Event Control Register (AFIO_EVCR)"]
    pub evcr: EVCR,
    #[doc = "0x04 - AF remap and debug I/O configuration register (AFIO_MAPR)"]
    pub mapr: MAPR,
    #[doc = "0x08 - External interrupt configuration register 1 (AFIO_EXTICR1)"]
    pub exticr1: EXTICR1,
    #[doc = "0x0c - External interrupt configuration register 2 (AFIO_EXTICR2)"]
    pub exticr2: EXTICR2,
    #[doc = "0x10 - External interrupt configuration register 3 (AFIO_EXTICR3)"]
    pub exticr3: EXTICR3,
    #[doc = "0x14 - External interrupt configuration register 4 (AFIO_EXTICR4)"]
    pub exticr4: EXTICR4,
    _reserved0: [u8; 4usize],
    #[doc = "0x1c - AF remap and debug I/O configuration register"]
    pub mapr2: MAPR2,
}
#[doc = "Event Control Register (AFIO_EVCR)"]
pub struct EVCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Control Register (AFIO_EVCR)"]
pub mod evcr;
#[doc = "AF remap and debug I/O configuration register (AFIO_MAPR)"]
pub struct MAPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AF remap and debug I/O configuration register (AFIO_MAPR)"]
pub mod mapr;
#[doc = "External interrupt configuration register 1 (AFIO_EXTICR1)"]
pub struct EXTICR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External interrupt configuration register 1 (AFIO_EXTICR1)"]
pub mod exticr1;
#[doc = "External interrupt configuration register 2 (AFIO_EXTICR2)"]
pub struct EXTICR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External interrupt configuration register 2 (AFIO_EXTICR2)"]
pub mod exticr2;
#[doc = "External interrupt configuration register 3 (AFIO_EXTICR3)"]
pub struct EXTICR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External interrupt configuration register 3 (AFIO_EXTICR3)"]
pub mod exticr3;
#[doc = "External interrupt configuration register 4 (AFIO_EXTICR4)"]
pub struct EXTICR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External interrupt configuration register 4 (AFIO_EXTICR4)"]
pub mod exticr4;
#[doc = "AF remap and debug I/O configuration register"]
pub struct MAPR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AF remap and debug I/O configuration register"]
pub mod mapr2;
