#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Backup data register"]
    pub dr1: DR,
    #[doc = "0x04 - Backup data register"]
    pub dr2: DR,
    #[doc = "0x08 - Backup data register"]
    pub dr3: DR,
    #[doc = "0x0c - Backup data register"]
    pub dr4: DR,
    #[doc = "0x10 - Backup data register"]
    pub dr5: DR,
    #[doc = "0x14 - Backup data register"]
    pub dr6: DR,
    #[doc = "0x18 - Backup data register"]
    pub dr7: DR,
    #[doc = "0x1c - Backup data register"]
    pub dr8: DR,
    #[doc = "0x20 - Backup data register"]
    pub dr9: DR,
    #[doc = "0x24 - Backup data register"]
    pub dr10: DR,
    #[doc = "0x28 - RTC clock calibration register (BKP_RTCCR)"]
    pub rtccr: RTCCR,
    #[doc = "0x2c - Backup control register (BKP_CR)"]
    pub cr: CR,
    #[doc = "0x30 - BKP_CSR control/status register (BKP_CSR)"]
    pub csr: CSR,
    _reserved0: [u8; 8usize],
    #[doc = "0x3c - Backup data register"]
    pub bkp_dr11: BKP_DR,
    #[doc = "0x40 - Backup data register"]
    pub bkp_dr12: BKP_DR,
    #[doc = "0x44 - Backup data register"]
    pub bkp_dr13: BKP_DR,
    #[doc = "0x48 - Backup data register"]
    pub bkp_dr14: BKP_DR,
    #[doc = "0x4c - Backup data register"]
    pub bkp_dr15: BKP_DR,
    #[doc = "0x50 - Backup data register"]
    pub bkp_dr16: BKP_DR,
    #[doc = "0x54 - Backup data register"]
    pub bkp_dr17: BKP_DR,
    #[doc = "0x58 - Backup data register"]
    pub bkp_dr18: BKP_DR,
    #[doc = "0x5c - Backup data register"]
    pub bkp_dr19: BKP_DR,
    #[doc = "0x60 - Backup data register"]
    pub bkp_dr20: BKP_DR,
    #[doc = "0x64 - Backup data register"]
    pub bkp_dr21: BKP_DR,
    #[doc = "0x68 - Backup data register"]
    pub bkp_dr22: BKP_DR,
    #[doc = "0x6c - Backup data register"]
    pub bkp_dr23: BKP_DR,
    #[doc = "0x70 - Backup data register"]
    pub bkp_dr24: BKP_DR,
    #[doc = "0x74 - Backup data register"]
    pub bkp_dr25: BKP_DR,
    #[doc = "0x78 - Backup data register"]
    pub bkp_dr26: BKP_DR,
    #[doc = "0x7c - Backup data register"]
    pub bkp_dr27: BKP_DR,
    #[doc = "0x80 - Backup data register"]
    pub bkp_dr28: BKP_DR,
    #[doc = "0x84 - Backup data register"]
    pub bkp_dr29: BKP_DR,
    #[doc = "0x88 - Backup data register"]
    pub bkp_dr30: BKP_DR,
    #[doc = "0x8c - Backup data register"]
    pub bkp_dr31: BKP_DR,
    #[doc = "0x90 - Backup data register"]
    pub bkp_dr32: BKP_DR,
    #[doc = "0x94 - Backup data register"]
    pub bkp_dr33: BKP_DR,
    #[doc = "0x98 - Backup data register"]
    pub bkp_dr34: BKP_DR,
    #[doc = "0x9c - Backup data register"]
    pub bkp_dr35: BKP_DR,
    #[doc = "0xa0 - Backup data register"]
    pub bkp_dr36: BKP_DR,
    #[doc = "0xa4 - Backup data register"]
    pub bkp_dr37: BKP_DR,
    #[doc = "0xa8 - Backup data register"]
    pub bkp_dr38: BKP_DR,
    #[doc = "0xac - Backup data register"]
    pub bkp_dr39: BKP_DR,
    #[doc = "0xb0 - Backup data register"]
    pub bkp_dr40: BKP_DR,
    #[doc = "0xb4 - Backup data register"]
    pub bkp_dr41: BKP_DR,
    #[doc = "0xb8 - Backup data register"]
    pub bkp_dr42: BKP_DR,
}
#[doc = "Backup data register"]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register"]
pub mod dr;
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
#[doc = "Backup data register"]
pub struct BKP_DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup data register"]
pub mod bkp_dr;
