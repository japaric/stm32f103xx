#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SMPR1 {
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
pub struct SMP10R {
    bits: u8,
}
impl SMP10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SMP11R {
    bits: u8,
}
impl SMP11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SMP12R {
    bits: u8,
}
impl SMP12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SMP13R {
    bits: u8,
}
impl SMP13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SMP14R {
    bits: u8,
}
impl SMP14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SMP15R {
    bits: u8,
}
impl SMP15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SMP16R {
    bits: u8,
}
impl SMP16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SMP17R {
    bits: u8,
}
impl SMP17R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SMP10W<'a> {
    w: &'a mut W,
}
impl<'a> _SMP10W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SMP11W<'a> {
    w: &'a mut W,
}
impl<'a> _SMP11W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SMP12W<'a> {
    w: &'a mut W,
}
impl<'a> _SMP12W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SMP13W<'a> {
    w: &'a mut W,
}
impl<'a> _SMP13W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SMP14W<'a> {
    w: &'a mut W,
}
impl<'a> _SMP14W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SMP15W<'a> {
    w: &'a mut W,
}
impl<'a> _SMP15W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SMP16W<'a> {
    w: &'a mut W,
}
impl<'a> _SMP16W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SMP17W<'a> {
    w: &'a mut W,
}
impl<'a> _SMP17W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 21;
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
    #[doc = "Bits 0:2 - Channel 10 sample time selection"]
    #[inline]
    pub fn smp10(&self) -> SMP10R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SMP10R { bits }
    }
    #[doc = "Bits 3:5 - Channel 11 sample time selection"]
    #[inline]
    pub fn smp11(&self) -> SMP11R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SMP11R { bits }
    }
    #[doc = "Bits 6:8 - Channel 12 sample time selection"]
    #[inline]
    pub fn smp12(&self) -> SMP12R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SMP12R { bits }
    }
    #[doc = "Bits 9:11 - Channel 13 sample time selection"]
    #[inline]
    pub fn smp13(&self) -> SMP13R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SMP13R { bits }
    }
    #[doc = "Bits 12:14 - Channel 14 sample time selection"]
    #[inline]
    pub fn smp14(&self) -> SMP14R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SMP14R { bits }
    }
    #[doc = "Bits 15:17 - Channel 15 sample time selection"]
    #[inline]
    pub fn smp15(&self) -> SMP15R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SMP15R { bits }
    }
    #[doc = "Bits 18:20 - Channel 16 sample time selection"]
    #[inline]
    pub fn smp16(&self) -> SMP16R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SMP16R { bits }
    }
    #[doc = "Bits 21:23 - Channel 17 sample time selection"]
    #[inline]
    pub fn smp17(&self) -> SMP17R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SMP17R { bits }
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
    #[doc = "Bits 0:2 - Channel 10 sample time selection"]
    #[inline]
    pub fn smp10(&mut self) -> _SMP10W {
        _SMP10W { w: self }
    }
    #[doc = "Bits 3:5 - Channel 11 sample time selection"]
    #[inline]
    pub fn smp11(&mut self) -> _SMP11W {
        _SMP11W { w: self }
    }
    #[doc = "Bits 6:8 - Channel 12 sample time selection"]
    #[inline]
    pub fn smp12(&mut self) -> _SMP12W {
        _SMP12W { w: self }
    }
    #[doc = "Bits 9:11 - Channel 13 sample time selection"]
    #[inline]
    pub fn smp13(&mut self) -> _SMP13W {
        _SMP13W { w: self }
    }
    #[doc = "Bits 12:14 - Channel 14 sample time selection"]
    #[inline]
    pub fn smp14(&mut self) -> _SMP14W {
        _SMP14W { w: self }
    }
    #[doc = "Bits 15:17 - Channel 15 sample time selection"]
    #[inline]
    pub fn smp15(&mut self) -> _SMP15W {
        _SMP15W { w: self }
    }
    #[doc = "Bits 18:20 - Channel 16 sample time selection"]
    #[inline]
    pub fn smp16(&mut self) -> _SMP16W {
        _SMP16W { w: self }
    }
    #[doc = "Bits 21:23 - Channel 17 sample time selection"]
    #[inline]
    pub fn smp17(&mut self) -> _SMP17W {
        _SMP17W { w: self }
    }
}
