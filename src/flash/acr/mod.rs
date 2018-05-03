#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ACR {
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
#[doc = "Possible values of the field `LATENCY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LATENCYR {
    #[doc = "Zero wait state, if 0hz  SYSCLK to 24 MHz"]
    ZERO,
    #[doc = "One wait state, if 24 MHz SYSCLK to 48 MHz"]
    ONE,
    #[doc = "Two wait states, if 48 MHz SYSCLK to 72 MHz"]
    TWO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LATENCYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LATENCYR::ZERO => 0,
            LATENCYR::ONE => 1,
            LATENCYR::TWO => 2,
            LATENCYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LATENCYR {
        match value {
            0 => LATENCYR::ZERO,
            1 => LATENCYR::ONE,
            2 => LATENCYR::TWO,
            i => LATENCYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == LATENCYR::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == LATENCYR::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline]
    pub fn is_two(&self) -> bool {
        *self == LATENCYR::TWO
    }
}
#[doc = r" Value of the field"]
pub struct HLFCYAR {
    bits: bool,
}
impl HLFCYAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `PRFTBE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRFTBER {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Enabled."]
    ENABLED,
}
impl PRFTBER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PRFTBER::DISABLED => false,
            PRFTBER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRFTBER {
        match value {
            false => PRFTBER::DISABLED,
            true => PRFTBER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PRFTBER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PRFTBER::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct PRFTBSR {
    bits: bool,
}
impl PRFTBSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Values that can be written to the field `LATENCY`"]
pub enum LATENCYW {
    #[doc = "Zero wait state, if 0hz  SYSCLK to 24 MHz"]
    ZERO,
    #[doc = "One wait state, if 24 MHz SYSCLK to 48 MHz"]
    ONE,
    #[doc = "Two wait states, if 48 MHz SYSCLK to 72 MHz"]
    TWO,
}
impl LATENCYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LATENCYW::ZERO => 0,
            LATENCYW::ONE => 1,
            LATENCYW::TWO => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LATENCYW<'a> {
    w: &'a mut W,
}
impl<'a> _LATENCYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LATENCYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Zero wait state, if 0hz SYSCLK to 24 MHz"]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(LATENCYW::ZERO)
    }
    #[doc = "One wait state, if 24 MHz SYSCLK to 48 MHz"]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(LATENCYW::ONE)
    }
    #[doc = "Two wait states, if 48 MHz SYSCLK to 72 MHz"]
    #[inline]
    pub fn two(self) -> &'a mut W {
        self.variant(LATENCYW::TWO)
    }
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
pub struct _HLFCYAW<'a> {
    w: &'a mut W,
}
impl<'a> _HLFCYAW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRFTBE`"]
pub enum PRFTBEW {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Enabled."]
    ENABLED,
}
impl PRFTBEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRFTBEW::DISABLED => false,
            PRFTBEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRFTBEW<'a> {
    w: &'a mut W,
}
impl<'a> _PRFTBEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRFTBEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PRFTBEW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PRFTBEW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:2 - Latency"]
    #[inline]
    pub fn latency(&self) -> LATENCYR {
        LATENCYR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Flash half cycle access enable"]
    #[inline]
    pub fn hlfcya(&self) -> HLFCYAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HLFCYAR { bits }
    }
    #[doc = "Bit 4 - Prefetch buffer enable"]
    #[inline]
    pub fn prftbe(&self) -> PRFTBER {
        PRFTBER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Prefetch buffer status"]
    #[inline]
    pub fn prftbs(&self) -> PRFTBSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRFTBSR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 48 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Latency"]
    #[inline]
    pub fn latency(&mut self) -> _LATENCYW {
        _LATENCYW { w: self }
    }
    #[doc = "Bit 3 - Flash half cycle access enable"]
    #[inline]
    pub fn hlfcya(&mut self) -> _HLFCYAW {
        _HLFCYAW { w: self }
    }
    #[doc = "Bit 4 - Prefetch buffer enable"]
    #[inline]
    pub fn prftbe(&mut self) -> _PRFTBEW {
        _PRFTBEW { w: self }
    }
}
