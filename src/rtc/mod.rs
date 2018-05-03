#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC Control Register High"]
    pub crh: CRH,
    #[doc = "0x04 - RTC Control Register Low"]
    pub crl: CRL,
    #[doc = "0x08 - RTC Prescaler Load Register High"]
    pub prlh: PRLH,
    #[doc = "0x0c - RTC Prescaler Load Register Low"]
    pub prll: PRLL,
    #[doc = "0x10 - RTC Prescaler Divider Register High"]
    pub divh: DIVH,
    #[doc = "0x14 - RTC Prescaler Divider Register Low"]
    pub divl: DIVL,
    #[doc = "0x18 - RTC Counter Register High"]
    pub cnth: CNTH,
    #[doc = "0x1c - RTC Counter Register Low"]
    pub cntl: CNTL,
    #[doc = "0x20 - RTC Alarm Register High"]
    pub alrh: ALRH,
    #[doc = "0x24 - RTC Alarm Register Low"]
    pub alrl: ALRL,
}
#[doc = "RTC Control Register High"]
pub struct CRH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Control Register High"]
pub mod crh;
#[doc = "RTC Control Register Low"]
pub struct CRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Control Register Low"]
pub mod crl;
#[doc = "RTC Prescaler Load Register High"]
pub struct PRLH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Prescaler Load Register High"]
pub mod prlh;
#[doc = "RTC Prescaler Load Register Low"]
pub struct PRLL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Prescaler Load Register Low"]
pub mod prll;
#[doc = "RTC Prescaler Divider Register High"]
pub struct DIVH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Prescaler Divider Register High"]
pub mod divh;
#[doc = "RTC Prescaler Divider Register Low"]
pub struct DIVL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Prescaler Divider Register Low"]
pub mod divl;
#[doc = "RTC Counter Register High"]
pub struct CNTH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Counter Register High"]
pub mod cnth;
#[doc = "RTC Counter Register Low"]
pub struct CNTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Counter Register Low"]
pub mod cntl;
#[doc = "RTC Alarm Register High"]
pub struct ALRH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Alarm Register High"]
pub mod alrh;
#[doc = "RTC Alarm Register Low"]
pub struct ALRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Alarm Register Low"]
pub mod alrl;
