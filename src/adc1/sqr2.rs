#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SQR2 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct SQ12R {
    bits: u8,
}
impl SQ12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SQ11R {
    bits: u8,
}
impl SQ11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SQ10R {
    bits: u8,
}
impl SQ10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SQ9R {
    bits: u8,
}
impl SQ9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SQ8R {
    bits: u8,
}
impl SQ8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SQ7R {
    bits: u8,
}
impl SQ7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SQ12W<'a> {
    w: &'a mut W,
}
impl<'a> _SQ12W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SQ11W<'a> {
    w: &'a mut W,
}
impl<'a> _SQ11W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SQ10W<'a> {
    w: &'a mut W,
}
impl<'a> _SQ10W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SQ9W<'a> {
    w: &'a mut W,
}
impl<'a> _SQ9W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SQ8W<'a> {
    w: &'a mut W,
}
impl<'a> _SQ8W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SQ7W<'a> {
    w: &'a mut W,
}
impl<'a> _SQ7W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 25:29 - 12th conversion in regular sequence"]
    #[inline]
    pub fn sq12(&self) -> SQ12R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SQ12R { bits }
    }
    #[doc = "Bits 20:24 - 11th conversion in regular sequence"]
    #[inline]
    pub fn sq11(&self) -> SQ11R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SQ11R { bits }
    }
    #[doc = "Bits 15:19 - 10th conversion in regular sequence"]
    #[inline]
    pub fn sq10(&self) -> SQ10R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SQ10R { bits }
    }
    #[doc = "Bits 10:14 - 9th conversion in regular sequence"]
    #[inline]
    pub fn sq9(&self) -> SQ9R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SQ9R { bits }
    }
    #[doc = "Bits 5:9 - 8th conversion in regular sequence"]
    #[inline]
    pub fn sq8(&self) -> SQ8R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SQ8R { bits }
    }
    #[doc = "Bits 0:4 - 7th conversion in regular sequence"]
    #[inline]
    pub fn sq7(&self) -> SQ7R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SQ7R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 25:29 - 12th conversion in regular sequence"]
    #[inline]
    pub fn sq12(&mut self) -> _SQ12W {
        _SQ12W { w: self }
    }
    #[doc = "Bits 20:24 - 11th conversion in regular sequence"]
    #[inline]
    pub fn sq11(&mut self) -> _SQ11W {
        _SQ11W { w: self }
    }
    #[doc = "Bits 15:19 - 10th conversion in regular sequence"]
    #[inline]
    pub fn sq10(&mut self) -> _SQ10W {
        _SQ10W { w: self }
    }
    #[doc = "Bits 10:14 - 9th conversion in regular sequence"]
    #[inline]
    pub fn sq9(&mut self) -> _SQ9W {
        _SQ9W { w: self }
    }
    #[doc = "Bits 5:9 - 8th conversion in regular sequence"]
    #[inline]
    pub fn sq8(&mut self) -> _SQ8W {
        _SQ8W { w: self }
    }
    #[doc = "Bits 0:4 - 7th conversion in regular sequence"]
    #[inline]
    pub fn sq7(&mut self) -> _SQ7W {
        _SQ7W { w: self }
    }
}
