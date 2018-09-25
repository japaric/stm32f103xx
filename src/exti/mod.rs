#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt mask register"]
    pub imr: IMR,
    #[doc = "0x04 - Event mask register"]
    pub emr: EMR,
    #[doc = "0x08 - Rising Trigger selection register"]
    pub rtsr: RTSR,
    #[doc = "0x0c - Falling Trigger selection register"]
    pub ftsr: FTSR,
    #[doc = "0x10 - Software interrupt event register"]
    pub swier: SWIER,
    #[doc = "0x14 - Pending register"]
    pub pr: PR,
}
#[doc = "Interrupt mask register"]
pub struct IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt mask register"]
pub mod imr;
#[doc = "Event mask register"]
pub struct EMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event mask register"]
pub mod emr;
#[doc = "Rising Trigger selection register"]
pub struct RTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rising Trigger selection register"]
pub mod rtsr;
#[doc = "Falling Trigger selection register"]
pub struct FTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Falling Trigger selection register"]
pub mod ftsr;
#[doc = "Software interrupt event register"]
pub struct SWIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software interrupt event register"]
pub mod swier;
#[doc = "Pending register"]
pub struct PR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pending register"]
pub mod pr;
