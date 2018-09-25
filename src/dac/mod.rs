#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub cr: CR,
    #[doc = "0x04 - DAC software trigger register"]
    pub swtrigr: SWTRIGR,
    #[doc = "0x08 - DAC channelX 12-bit right-aligned data holding register"]
    pub dhr12r1: DHR12R,
    #[doc = "0x0c - DAC channelX 12-bit left-aligned data holding register"]
    pub dhr12l1: DHR12L,
    #[doc = "0x10 - DAC channelX 8-bit right-aligned data holding register"]
    pub dhr8r1: DHR8R,
    #[doc = "0x14 - DAC channelX 12-bit right-aligned data holding register"]
    pub dhr12r2: DHR12R,
    #[doc = "0x18 - DAC channelX 12-bit left-aligned data holding register"]
    pub dhr12l2: DHR12L,
    #[doc = "0x1c - DAC channelX 8-bit right-aligned data holding register"]
    pub dhr8r2: DHR8R,
    #[doc = "0x20 - Dual DAC 12-bit right-aligned data holding register"]
    pub dhr12rd: DHR12RD,
    #[doc = "0x24 - Dual DAC 12-bit left-aligned data holding register"]
    pub dhr12ld: DHR12LD,
    #[doc = "0x28 - Dual DAC 8-bit right-aligned data holding register"]
    pub dhr8rd: DHR8RD,
    #[doc = "0x2c - DAC channelX data output register"]
    pub dor1: DOR,
    #[doc = "0x30 - DAC channelX data output register"]
    pub dor2: DOR,
}
#[doc = "Control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register"]
pub mod cr;
#[doc = "DAC software trigger register"]
pub struct SWTRIGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC software trigger register"]
pub mod swtrigr;
#[doc = "DAC channelX 12-bit right-aligned data holding register"]
pub struct DHR12R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC channelX 12-bit right-aligned data holding register"]
pub mod dhr12r;
#[doc = "DAC channelX 12-bit left-aligned data holding register"]
pub struct DHR12L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC channelX 12-bit left-aligned data holding register"]
pub mod dhr12l;
#[doc = "DAC channelX 8-bit right-aligned data holding register"]
pub struct DHR8R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC channelX 8-bit right-aligned data holding register"]
pub mod dhr8r;
#[doc = "Dual DAC 12-bit right-aligned data holding register"]
pub struct DHR12RD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Dual DAC 12-bit right-aligned data holding register"]
pub mod dhr12rd;
#[doc = "Dual DAC 12-bit left-aligned data holding register"]
pub struct DHR12LD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Dual DAC 12-bit left-aligned data holding register"]
pub mod dhr12ld;
#[doc = "Dual DAC 8-bit right-aligned data holding register"]
pub struct DHR8RD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Dual DAC 8-bit right-aligned data holding register"]
pub mod dhr8rd;
#[doc = "DAC channelX data output register"]
pub struct DOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC channelX data output register"]
pub mod dor;
