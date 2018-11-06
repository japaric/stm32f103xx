#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Backup data register (BKP_DR)"]
    pub dr1: DR1,
    #[doc = "0x04 - Backup data register (BKP_DR)"]
    pub dr2: DR2,
    #[doc = "0x08 - Backup data register (BKP_DR)"]
    pub dr3: DR3,
    #[doc = "0x0c - Backup data register (BKP_DR)"]
    pub dr4: DR4,
    #[doc = "0x10 - Backup data register (BKP_DR)"]
    pub dr5: DR5,
    #[doc = "0x14 - Backup data register (BKP_DR)"]
    pub dr6: DR6,
    #[doc = "0x18 - Backup data register (BKP_DR)"]
    pub dr7: DR7,
    #[doc = "0x1c - Backup data register (BKP_DR)"]
    pub dr8: DR8,
    #[doc = "0x20 - Backup data register (BKP_DR)"]
    pub dr9: DR9,
    #[doc = "0x24 - Backup data register (BKP_DR)"]
    pub dr10: DR10,
    #[doc = "0x28 - RTC clock calibration register (BKP_RTCCR)"]
    pub rtccr: RTCCR,
    #[doc = "0x2c - Backup control register (BKP_CR)"]
    pub cr: CR,
    #[doc = "0x30 - BKP_CSR control/status register (BKP_CSR)"]
    pub csr: CSR,
    _reserved0: [u8; 8usize],
    #[doc = "0x3c - Backup data register (BKP_DR)"]
    pub dr11: DR11,
    #[doc = "0x40 - Backup data register (BKP_DR)"]
    pub dr12: DR12,
    #[doc = "0x44 - Backup data register (BKP_DR)"]
    pub dr13: DR13,
    #[doc = "0x48 - Backup data register (BKP_DR)"]
    pub dr14: DR14,
    #[doc = "0x4c - Backup data register (BKP_DR)"]
    pub dr15: DR15,
    #[doc = "0x50 - Backup data register (BKP_DR)"]
    pub dr16: DR16,
    #[doc = "0x54 - Backup data register (BKP_DR)"]
    pub dr17: DR17,
    #[doc = "0x58 - Backup data register (BKP_DR)"]
    pub dr18: DR18,
    #[doc = "0x5c - Backup data register (BKP_DR)"]
    pub dr19: DR19,
    #[doc = "0x60 - Backup data register (BKP_DR)"]
    pub dr20: DR20,
    #[doc = "0x64 - Backup data register (BKP_DR)"]
    pub dr21: DR21,
    #[doc = "0x68 - Backup data register (BKP_DR)"]
    pub dr22: DR22,
    #[doc = "0x6c - Backup data register (BKP_DR)"]
    pub dr23: DR23,
    #[doc = "0x70 - Backup data register (BKP_DR)"]
    pub dr24: DR24,
    #[doc = "0x74 - Backup data register (BKP_DR)"]
    pub dr25: DR25,
    #[doc = "0x78 - Backup data register (BKP_DR)"]
    pub dr26: DR26,
    #[doc = "0x7c - Backup data register (BKP_DR)"]
    pub dr27: DR27,
    #[doc = "0x80 - Backup data register (BKP_DR)"]
    pub dr28: DR28,
    #[doc = "0x84 - Backup data register (BKP_DR)"]
    pub dr29: DR29,
    #[doc = "0x88 - Backup data register (BKP_DR)"]
    pub dr30: DR30,
    #[doc = "0x8c - Backup data register (BKP_DR)"]
    pub dr31: DR31,
    #[doc = "0x90 - Backup data register (BKP_DR)"]
    pub dr32: DR32,
    #[doc = "0x94 - Backup data register (BKP_DR)"]
    pub dr33: DR33,
    #[doc = "0x98 - Backup data register (BKP_DR)"]
    pub dr34: DR34,
    #[doc = "0x9c - Backup data register (BKP_DR)"]
    pub dr35: DR35,
    #[doc = "0xa0 - Backup data register (BKP_DR)"]
    pub dr36: DR36,
    #[doc = "0xa4 - Backup data register (BKP_DR)"]
    pub dr37: DR37,
    #[doc = "0xa8 - Backup data register (BKP_DR)"]
    pub dr38: DR38,
    #[doc = "0xac - Backup data register (BKP_DR)"]
    pub dr39: DR39,
    #[doc = "0xb0 - Backup data register (BKP_DR)"]
    pub dr40: DR40,
    #[doc = "0xb4 - Backup data register (BKP_DR)"]
    pub dr41: DR41,
    #[doc = "0xb8 - Backup data register (BKP_DR)"]
    pub dr42: DR42,
}
#[doc = "Backup data register (BKP_DR)"]
pub struct DR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr1;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr2;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr3;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr4;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr5;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr6;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr7;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr8;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr9;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr10;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr11;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr12;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr13;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr14;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr15;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr16;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr17;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr18;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr19;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr20;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr21;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr22;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr23;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr24;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr25;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr26;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr27;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr28;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr29;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr30;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr31;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr32;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR33 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr33;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR34 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr34;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR35 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr35;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR36 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr36;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR37 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr37;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR38 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr38;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR39 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr39;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR40 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr40;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR41 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr41;
#[doc = "Backup data register (BKP_DR)"]
pub struct DR42 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr42;
#[doc = "RTC clock calibration register (BKP_RTCCR)"]
pub struct RTCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC clock calibration register (BKP_RTCCR)"]
pub mod rtccr;
#[doc = "Backup control register (BKP_CR)"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup control register (BKP_CR)"]
pub mod cr;
#[doc = "BKP_CSR control/status register (BKP_CSR)"]
pub struct CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BKP_CSR control/status register (BKP_CSR)"]
pub mod csr;
