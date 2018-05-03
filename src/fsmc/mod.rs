#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM/NOR-Flash chip-select control register 1"]
    pub bcr1: BCR1,
    #[doc = "0x04 - SRAM/NOR-Flash chip-select timing register 1"]
    pub btr1: BTR1,
    #[doc = "0x08 - SRAM/NOR-Flash chip-select control register 2"]
    pub bcr2: BCR2,
    #[doc = "0x0c - SRAM/NOR-Flash chip-select timing register 2"]
    pub btr2: BTR2,
    #[doc = "0x10 - SRAM/NOR-Flash chip-select control register 3"]
    pub bcr3: BCR3,
    #[doc = "0x14 - SRAM/NOR-Flash chip-select timing register 3"]
    pub btr3: BTR3,
    #[doc = "0x18 - SRAM/NOR-Flash chip-select control register 4"]
    pub bcr4: BCR4,
    #[doc = "0x1c - SRAM/NOR-Flash chip-select timing register 4"]
    pub btr4: BTR4,
    _reserved0: [u8; 64usize],
    #[doc = "0x60 - PC Card/NAND Flash control register 2"]
    pub pcr2: PCR2,
    #[doc = "0x64 - FIFO status and interrupt register 2"]
    pub sr2: SR2,
    #[doc = "0x68 - Common memory space timing register 2"]
    pub pmem2: PMEM2,
    #[doc = "0x6c - Attribute memory space timing register 2"]
    pub patt2: PATT2,
    _reserved1: [u8; 4usize],
    #[doc = "0x74 - ECC result register 2"]
    pub eccr2: ECCR2,
    _reserved2: [u8; 8usize],
    #[doc = "0x80 - PC Card/NAND Flash control register 3"]
    pub pcr3: PCR3,
    #[doc = "0x84 - FIFO status and interrupt register 3"]
    pub sr3: SR3,
    #[doc = "0x88 - Common memory space timing register 3"]
    pub pmem3: PMEM3,
    #[doc = "0x8c - Attribute memory space timing register 3"]
    pub patt3: PATT3,
    _reserved3: [u8; 4usize],
    #[doc = "0x94 - ECC result register 3"]
    pub eccr3: ECCR3,
    _reserved4: [u8; 8usize],
    #[doc = "0xa0 - PC Card/NAND Flash control register 4"]
    pub pcr4: PCR4,
    #[doc = "0xa4 - FIFO status and interrupt register 4"]
    pub sr4: SR4,
    #[doc = "0xa8 - Common memory space timing register 4"]
    pub pmem4: PMEM4,
    #[doc = "0xac - Attribute memory space timing register 4"]
    pub patt4: PATT4,
    #[doc = "0xb0 - I/O space timing register 4"]
    pub pio4: PIO4,
    _reserved5: [u8; 80usize],
    #[doc = "0x104 - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr1: BWTR1,
    _reserved6: [u8; 4usize],
    #[doc = "0x10c - SRAM/NOR-Flash write timing registers 2"]
    pub bwtr2: BWTR2,
    _reserved7: [u8; 4usize],
    #[doc = "0x114 - SRAM/NOR-Flash write timing registers 3"]
    pub bwtr3: BWTR3,
    _reserved8: [u8; 4usize],
    #[doc = "0x11c - SRAM/NOR-Flash write timing registers 4"]
    pub bwtr4: BWTR4,
}
#[doc = "SRAM/NOR-Flash chip-select control register 1"]
pub struct BCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select control register 1"]
pub mod bcr1;
#[doc = "SRAM/NOR-Flash chip-select timing register 1"]
pub struct BTR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select timing register 1"]
pub mod btr1;
#[doc = "SRAM/NOR-Flash chip-select control register 2"]
pub struct BCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select control register 2"]
pub mod bcr2;
#[doc = "SRAM/NOR-Flash chip-select timing register 2"]
pub struct BTR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select timing register 2"]
pub mod btr2;
#[doc = "SRAM/NOR-Flash chip-select control register 3"]
pub struct BCR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select control register 3"]
pub mod bcr3;
#[doc = "SRAM/NOR-Flash chip-select timing register 3"]
pub struct BTR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select timing register 3"]
pub mod btr3;
#[doc = "SRAM/NOR-Flash chip-select control register 4"]
pub struct BCR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select control register 4"]
pub mod bcr4;
#[doc = "SRAM/NOR-Flash chip-select timing register 4"]
pub struct BTR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select timing register 4"]
pub mod btr4;
#[doc = "PC Card/NAND Flash control register 2"]
pub struct PCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PC Card/NAND Flash control register 2"]
pub mod pcr2;
#[doc = "FIFO status and interrupt register 2"]
pub struct SR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO status and interrupt register 2"]
pub mod sr2;
#[doc = "Common memory space timing register 2"]
pub struct PMEM2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Common memory space timing register 2"]
pub mod pmem2;
#[doc = "Attribute memory space timing register 2"]
pub struct PATT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Attribute memory space timing register 2"]
pub mod patt2;
#[doc = "ECC result register 2"]
pub struct ECCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ECC result register 2"]
pub mod eccr2;
#[doc = "PC Card/NAND Flash control register 3"]
pub struct PCR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PC Card/NAND Flash control register 3"]
pub mod pcr3;
#[doc = "FIFO status and interrupt register 3"]
pub struct SR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO status and interrupt register 3"]
pub mod sr3;
#[doc = "Common memory space timing register 3"]
pub struct PMEM3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Common memory space timing register 3"]
pub mod pmem3;
#[doc = "Attribute memory space timing register 3"]
pub struct PATT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Attribute memory space timing register 3"]
pub mod patt3;
#[doc = "ECC result register 3"]
pub struct ECCR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ECC result register 3"]
pub mod eccr3;
#[doc = "PC Card/NAND Flash control register 4"]
pub struct PCR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PC Card/NAND Flash control register 4"]
pub mod pcr4;
#[doc = "FIFO status and interrupt register 4"]
pub struct SR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO status and interrupt register 4"]
pub mod sr4;
#[doc = "Common memory space timing register 4"]
pub struct PMEM4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Common memory space timing register 4"]
pub mod pmem4;
#[doc = "Attribute memory space timing register 4"]
pub struct PATT4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Attribute memory space timing register 4"]
pub mod patt4;
#[doc = "I/O space timing register 4"]
pub struct PIO4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O space timing register 4"]
pub mod pio4;
#[doc = "SRAM/NOR-Flash write timing registers 1"]
pub struct BWTR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash write timing registers 1"]
pub mod bwtr1;
#[doc = "SRAM/NOR-Flash write timing registers 2"]
pub struct BWTR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash write timing registers 2"]
pub mod bwtr2;
#[doc = "SRAM/NOR-Flash write timing registers 3"]
pub struct BWTR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash write timing registers 3"]
pub mod bwtr3;
#[doc = "SRAM/NOR-Flash write timing registers 4"]
pub struct BWTR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash write timing registers 4"]
pub mod bwtr4;
