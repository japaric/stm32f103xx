#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Key register (IWDG_KR)"]
    pub kr: KR,
    #[doc = "0x04 - Prescaler register (IWDG_PR)"]
    pub pr: PR,
    #[doc = "0x08 - Reload register (IWDG_RLR)"]
    pub rlr: RLR,
    #[doc = "0x0c - Status register (IWDG_SR)"]
    pub sr: SR,
}
#[doc = "Key register (IWDG_KR)"]
pub struct KR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Key register (IWDG_KR)"]
pub mod kr;
#[doc = "Prescaler register (IWDG_PR)"]
pub struct PR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Prescaler register (IWDG_PR)"]
pub mod pr;
#[doc = "Reload register (IWDG_RLR)"]
pub struct RLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reload register (IWDG_RLR)"]
pub mod rlr;
#[doc = "Status register (IWDG_SR)"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register (IWDG_SR)"]
pub mod sr;
