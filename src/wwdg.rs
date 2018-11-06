#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register (WWDG_CR)"]
    pub cr: CR,
    #[doc = "0x04 - Configuration register (WWDG_CFR)"]
    pub cfr: CFR,
    #[doc = "0x08 - Status register (WWDG_SR)"]
    pub sr: SR,
}
#[doc = "Control register (WWDG_CR)"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register (WWDG_CR)"]
pub mod cr;
#[doc = "Configuration register (WWDG_CFR)"]
pub struct CFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration register (WWDG_CFR)"]
pub mod cfr;
#[doc = "Status register (WWDG_SR)"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register (WWDG_SR)"]
pub mod sr;
