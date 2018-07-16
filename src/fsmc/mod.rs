#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM/NOR-Flash chip-select control register x"]
    pub bcr1: BCR,
    #[doc = "0x04 - SRAM/NOR-Flash chip-select timing register x"]
    pub btr1: BTR,
    #[doc = "0x08 - SRAM/NOR-Flash chip-select control register x"]
    pub bcr2: BCR,
    #[doc = "0x0c - SRAM/NOR-Flash chip-select timing register x"]
    pub btr2: BTR,
    #[doc = "0x10 - SRAM/NOR-Flash chip-select control register x"]
    pub bcr3: BCR,
    #[doc = "0x14 - SRAM/NOR-Flash chip-select timing register x"]
    pub btr3: BTR,
    #[doc = "0x18 - SRAM/NOR-Flash chip-select control register x"]
    pub bcr4: BCR,
    #[doc = "0x1c - SRAM/NOR-Flash chip-select timing register x"]
    pub btr4: BTR,
    _reserved0: [u8; 64usize],
    #[doc = "0x60 - PC Card/NAND Flash control register x"]
    pub pcr2: PCR,
    #[doc = "0x64 - FIFO status and interrupt register x"]
    pub sr2: SR,
    #[doc = "0x68 - Common memory space timing register x"]
    pub pmem2: PMEM,
    #[doc = "0x6c - Attribute memory space timing register x"]
    pub patt2: PATT,
    _reserved1: [u8; 4usize],
    #[doc = "0x74 - ECC result register x"]
    pub eccr2: ECCR,
    _reserved2: [u8; 8usize],
    #[doc = "0x80 - PC Card/NAND Flash control register x"]
    pub pcr3: PCR,
    #[doc = "0x84 - FIFO status and interrupt register x"]
    pub sr3: SR,
    #[doc = "0x88 - Common memory space timing register x"]
    pub pmem3: PMEM,
    #[doc = "0x8c - Attribute memory space timing register x"]
    pub patt3: PATT,
    _reserved3: [u8; 4usize],
    #[doc = "0x94 - ECC result register x"]
    pub eccr3: ECCR,
    _reserved4: [u8; 8usize],
    #[doc = "0xa0 - PC Card/NAND Flash control register x"]
    pub pcr4: PCR,
    #[doc = "0xa4 - FIFO status and interrupt register x"]
    pub sr4: SR,
    #[doc = "0xa8 - Common memory space timing register x"]
    pub pmem4: PMEM,
    #[doc = "0xac - Attribute memory space timing register x"]
    pub patt4: PATT,
    #[doc = "0xb0 - I/O space timing register 4"]
    pub pio4: PIO4,
    _reserved5: [u8; 80usize],
    #[doc = "0x104 - SRAM/NOR-Flash write timing registers x"]
    pub bwtr1: BWTR,
    _reserved6: [u8; 4usize],
    #[doc = "0x10c - SRAM/NOR-Flash write timing registers x"]
    pub bwtr2: BWTR,
    _reserved7: [u8; 4usize],
    #[doc = "0x114 - SRAM/NOR-Flash write timing registers x"]
    pub bwtr3: BWTR,
    _reserved8: [u8; 4usize],
    #[doc = "0x11c - SRAM/NOR-Flash write timing registers x"]
    pub bwtr4: BWTR,
}
#[doc = "SRAM/NOR-Flash chip-select control register x"]
pub struct BCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select control register x"]
pub mod bcr;
#[doc = "SRAM/NOR-Flash chip-select timing register x"]
pub struct BTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select timing register x"]
pub mod btr;
#[doc = "PC Card/NAND Flash control register x"]
pub struct PCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PC Card/NAND Flash control register x"]
pub mod pcr;
#[doc = "FIFO status and interrupt register x"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO status and interrupt register x"]
pub mod sr;
#[doc = "Common memory space timing register x"]
pub struct PMEM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Common memory space timing register x"]
pub mod pmem;
#[doc = "Attribute memory space timing register x"]
pub struct PATT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Attribute memory space timing register x"]
pub mod patt;
#[doc = "ECC result register x"]
pub struct ECCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ECC result register x"]
pub mod eccr;
#[doc = "I/O space timing register 4"]
pub struct PIO4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O space timing register 4"]
pub mod pio4;
#[doc = "SRAM/NOR-Flash write timing registers x"]
pub struct BWTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash write timing registers x"]
pub mod bwtr;
