#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - status register"]
    pub sr: SR,
    #[doc = "0x04 - control register 1"]
    pub cr1: CR1,
    #[doc = "0x08 - control register 2"]
    pub cr2: CR2,
    #[doc = "0x0c - sample time register 1"]
    pub smpr1: SMPR1,
    #[doc = "0x10 - sample time register 2"]
    pub smpr2: SMPR2,
    #[doc = "0x14 - injected channel data offset register x"]
    pub jofr1: JOFR1,
    #[doc = "0x18 - injected channel data offset register x"]
    pub jofr2: JOFR2,
    #[doc = "0x1c - injected channel data offset register x"]
    pub jofr3: JOFR3,
    #[doc = "0x20 - injected channel data offset register x"]
    pub jofr4: JOFR4,
    #[doc = "0x24 - watchdog higher threshold register"]
    pub htr: HTR,
    #[doc = "0x28 - watchdog lower threshold register"]
    pub ltr: LTR,
    #[doc = "0x2c - regular sequence register 1"]
    pub sqr1: SQR1,
    #[doc = "0x30 - regular sequence register 2"]
    pub sqr2: SQR2,
    #[doc = "0x34 - regular sequence register 3"]
    pub sqr3: SQR3,
    #[doc = "0x38 - injected sequence register"]
    pub jsqr: JSQR,
    #[doc = "0x3c - injected data register x"]
    pub jdr1: JDR1,
    #[doc = "0x40 - injected data register x"]
    pub jdr2: JDR2,
    #[doc = "0x44 - injected data register x"]
    pub jdr3: JDR3,
    #[doc = "0x48 - injected data register x"]
    pub jdr4: JDR4,
    #[doc = "0x4c - regular data register"]
    pub dr: DR,
}
#[doc = "status register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "status register"]
pub mod sr;
#[doc = "control register 1"]
pub struct CR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register 1"]
pub mod cr1;
#[doc = "control register 2"]
pub struct CR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register 2"]
pub mod cr2;
#[doc = "sample time register 1"]
pub struct SMPR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "sample time register 1"]
pub mod smpr1;
#[doc = "sample time register 2"]
pub struct SMPR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "sample time register 2"]
pub mod smpr2;
#[doc = "injected channel data offset register x"]
pub struct JOFR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "injected channel data offset register x"]
pub mod jofr1;
#[doc = "injected channel data offset register x"]
pub struct JOFR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "injected channel data offset register x"]
pub mod jofr2;
#[doc = "injected channel data offset register x"]
pub struct JOFR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "injected channel data offset register x"]
pub mod jofr3;
#[doc = "injected channel data offset register x"]
pub struct JOFR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "injected channel data offset register x"]
pub mod jofr4;
#[doc = "watchdog higher threshold register"]
pub struct HTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "watchdog higher threshold register"]
pub mod htr;
#[doc = "watchdog lower threshold register"]
pub struct LTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "watchdog lower threshold register"]
pub mod ltr;
#[doc = "regular sequence register 1"]
pub struct SQR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "regular sequence register 1"]
pub mod sqr1;
#[doc = "regular sequence register 2"]
pub struct SQR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "regular sequence register 2"]
pub mod sqr2;
#[doc = "regular sequence register 3"]
pub struct SQR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "regular sequence register 3"]
pub mod sqr3;
#[doc = "injected sequence register"]
pub struct JSQR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "injected sequence register"]
pub mod jsqr;
#[doc = "injected data register x"]
pub struct JDR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "injected data register x"]
pub mod jdr1;
#[doc = "injected data register x"]
pub struct JDR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "injected data register x"]
pub mod jdr2;
#[doc = "injected data register x"]
pub struct JDR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "injected data register x"]
pub mod jdr3;
#[doc = "injected data register x"]
pub struct JDR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "injected data register x"]
pub mod jdr4;
#[doc = "regular data register"]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "regular data register"]
pub mod dr;
