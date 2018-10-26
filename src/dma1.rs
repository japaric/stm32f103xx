#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA interrupt status register"]
    pub isr: ISR,
    #[doc = "0x04 - DMA interrupt flag clear register"]
    pub ifcr: IFCR,
    #[doc = "0x08 - DMA channel X: CCRx, CNDTRx, CPARx, CMARx"]
    pub ch1: CH,
    _reserved0: [u8; 4usize],
    #[doc = "0x1c - DMA channel X: CCRx, CNDTRx, CPARx, CMARx"]
    pub ch2: CH,
    _reserved1: [u8; 4usize],
    #[doc = "0x30 - DMA channel X: CCRx, CNDTRx, CPARx, CMARx"]
    pub ch3: CH,
    _reserved2: [u8; 4usize],
    #[doc = "0x44 - DMA channel X: CCRx, CNDTRx, CPARx, CMARx"]
    pub ch4: CH,
    _reserved3: [u8; 4usize],
    #[doc = "0x58 - DMA channel X: CCRx, CNDTRx, CPARx, CMARx"]
    pub ch5: CH,
    _reserved4: [u8; 4usize],
    #[doc = "0x6c - DMA channel X: CCRx, CNDTRx, CPARx, CMARx"]
    pub ch6: CH,
    _reserved5: [u8; 4usize],
    #[doc = "0x80 - DMA channel X: CCRx, CNDTRx, CPARx, CMARx"]
    pub ch7: CH,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - DMA channel X configuration register"]
    pub ccr: self::ch::CCR,
    #[doc = "0x04 - DMA channel X number of data register"]
    pub cndtr: self::ch::CNDTR,
    #[doc = "0x08 - DMA channel X peripheral address register"]
    pub cpar: self::ch::CPAR,
    #[doc = "0x0c - DMA channel X memory address register"]
    pub cmar: self::ch::CMAR,
}
#[doc = r" Register block"]
#[doc = "DMA channel X: CCRx, CNDTRx, CPARx, CMARx"]
pub mod ch;
#[doc = "DMA interrupt status register"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA interrupt status register"]
pub mod isr;
#[doc = "DMA interrupt flag clear register"]
pub struct IFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA interrupt flag clear register"]
pub mod ifcr;
