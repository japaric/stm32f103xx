#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA interrupt status register (DMA_ISR)"]
    pub isr: ISR,
    #[doc = "0x04 - DMA interrupt flag clear register (DMA_IFCR)"]
    pub ifcr: IFCR,
    #[doc = "0x08 - DMA channel x: CCRx, CNDTRx, CPARx, CMARx"]
    pub ch1: CH,
    _reserved0: [u8; 4usize],
    #[doc = "0x1c - DMA channel x: CCRx, CNDTRx, CPARx, CMARx"]
    pub ch2: CH,
    _reserved1: [u8; 4usize],
    #[doc = "0x30 - DMA channel x: CCRx, CNDTRx, CPARx, CMARx"]
    pub ch3: CH,
    _reserved2: [u8; 4usize],
    #[doc = "0x44 - DMA channel x: CCRx, CNDTRx, CPARx, CMARx"]
    pub ch4: CH,
    _reserved3: [u8; 4usize],
    #[doc = "0x58 - DMA channel x: CCRx, CNDTRx, CPARx, CMARx"]
    pub ch5: CH,
    _reserved4: [u8; 4usize],
    #[doc = "0x6c - DMA channel x: CCRx, CNDTRx, CPARx, CMARx"]
    pub ch6: CH,
    _reserved5: [u8; 4usize],
    #[doc = "0x80 - DMA channel x: CCRx, CNDTRx, CPARx, CMARx"]
    pub ch7: CH,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - DMA channel x configuration register (DMA_CCR)"]
    pub ccr: self::ch::CCR,
    #[doc = "0x04 - DMA channel x number of data register"]
    pub cndtr: self::ch::CNDTR,
    #[doc = "0x08 - DMA channel x peripheral address register"]
    pub cpar: self::ch::CPAR,
    #[doc = "0x0c - DMA channel x memory address register"]
    pub cmar: self::ch::CMAR,
}
#[doc = r" Register block"]
#[doc = "DMA channel x: CCRx, CNDTRx, CPARx, CMARx"]
pub mod ch;
#[doc = "DMA interrupt status register (DMA_ISR)"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA interrupt status register (DMA_ISR)"]
pub mod isr;
#[doc = "DMA interrupt flag clear register (DMA_IFCR)"]
pub struct IFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA interrupt flag clear register (DMA_IFCR)"]
pub mod ifcr;
