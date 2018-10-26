#[doc = "TX mailbox identifier register"]
pub struct TIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX mailbox identifier register"]
pub mod tir;
#[doc = "mailbox data length control and time stamp register"]
pub struct TDTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "mailbox data length control and time stamp register"]
pub mod tdtr;
#[doc = "mailbox data low register"]
pub struct TDLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "mailbox data low register"]
pub mod tdlr;
#[doc = "mailbox data high register"]
pub struct TDHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "mailbox data high register"]
pub mod tdhr;
