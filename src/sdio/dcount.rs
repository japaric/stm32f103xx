#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DCOUNT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DATACOUNTR {
    bits: u32,
}
impl DATACOUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:24 - Data count value"]
    #[inline]
    pub fn datacount(&self) -> DATACOUNTR {
        let bits = {
            const MASK: u32 = 33554431;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        DATACOUNTR { bits }
    }
}
