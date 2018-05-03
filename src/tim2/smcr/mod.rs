#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SMCR {
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
pub struct ETPR {
    bits: bool,
}
impl ETPR {
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
#[doc = r" Value of the field"]
pub struct ECER {
    bits: bool,
}
impl ECER {
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
#[doc = r" Value of the field"]
pub struct ETPSR {
    bits: u8,
}
impl ETPSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ETFR {
    bits: u8,
}
impl ETFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MSMR {
    bits: bool,
}
impl MSMR {
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
#[doc = "Possible values of the field `TS`"]
pub type TSR = ::tim1::smcr::TSR;
#[doc = "Possible values of the field `SMS`"]
pub type SMSR = ::tim1::smcr::SMSR;
#[doc = r" Proxy"]
pub struct _ETPW<'a> {
    w: &'a mut W,
}
impl<'a> _ETPW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ECEW<'a> {
    w: &'a mut W,
}
impl<'a> _ECEW<'a> {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ETPSW<'a> {
    w: &'a mut W,
}
impl<'a> _ETPSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ETFW<'a> {
    w: &'a mut W,
}
impl<'a> _ETFW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MSMW<'a> {
    w: &'a mut W,
}
impl<'a> _MSMW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TS`"]
pub type TSW = ::tim1::smcr::TSW;
#[doc = r" Proxy"]
pub struct _TSW<'a> {
    w: &'a mut W,
}
impl<'a> _TSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Internal Trigger 0 (ITR0)"]
    #[inline]
    pub fn itr0(self) -> &'a mut W {
        self.variant(::tim1::smcr::TSW::ITR0)
    }
    #[doc = "Internal Trigger 1 (ITR1)"]
    #[inline]
    pub fn itr1(self) -> &'a mut W {
        self.variant(::tim1::smcr::TSW::ITR1)
    }
    #[doc = "Internal Trigger 2 (ITR2)"]
    #[inline]
    pub fn itr2(self) -> &'a mut W {
        self.variant(::tim1::smcr::TSW::ITR2)
    }
    #[doc = "Internal Trigger 3 (ITR3)"]
    #[inline]
    pub fn itr3(self) -> &'a mut W {
        self.variant(::tim1::smcr::TSW::ITR3)
    }
    #[doc = "TI1 Edge Detector"]
    #[inline]
    pub fn ti1f_ed(self) -> &'a mut W {
        self.variant(::tim1::smcr::TSW::TI1F_ED)
    }
    #[doc = "Filtered Timer Input 1"]
    #[inline]
    pub fn ti1fp1(self) -> &'a mut W {
        self.variant(::tim1::smcr::TSW::TI1FP1)
    }
    #[doc = "Filtered Timer Input 2"]
    #[inline]
    pub fn ti2fp2(self) -> &'a mut W {
        self.variant(::tim1::smcr::TSW::TI2FP2)
    }
    #[doc = "External Trigger input"]
    #[inline]
    pub fn etrf(self) -> &'a mut W {
        self.variant(::tim1::smcr::TSW::ETRF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMS`"]
pub type SMSW = ::tim1::smcr::SMSW;
#[doc = r" Proxy"]
pub struct _SMSW<'a> {
    w: &'a mut W,
}
impl<'a> _SMSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Counter disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(::tim1::smcr::SMSW::DISABLED)
    }
    #[doc = "Encoder mode, count up/down on TI2FP1"]
    #[inline]
    pub fn encoder_ti2(self) -> &'a mut W {
        self.variant(::tim1::smcr::SMSW::ENCODERTI2)
    }
    #[doc = "Encoder mode, count up/down on TI1FP2"]
    #[inline]
    pub fn encoder_ti1(self) -> &'a mut W {
        self.variant(::tim1::smcr::SMSW::ENCODERTI1)
    }
    #[doc = "Encoder mode, count up/down on both TI1FP1 and TI2FP2"]
    #[inline]
    pub fn encoder_ti1ti2(self) -> &'a mut W {
        self.variant(::tim1::smcr::SMSW::ENCODERTI1TI2)
    }
    #[doc = "Rising edge of the selected trigger input (TRGI) reinitializes the counter"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(::tim1::smcr::SMSW::RESET)
    }
    #[doc = "The counter clock is enabled when the trigger input (TRGI) is high"]
    #[inline]
    pub fn gated(self) -> &'a mut W {
        self.variant(::tim1::smcr::SMSW::GATED)
    }
    #[doc = "The counter starts at a rising edge of the trigger TRGI"]
    #[inline]
    pub fn trigger(self) -> &'a mut W {
        self.variant(::tim1::smcr::SMSW::TRIGGER)
    }
    #[doc = "Rising edges of the selected trigger (TRGI) clock the counter"]
    #[inline]
    pub fn external(self) -> &'a mut W {
        self.variant(::tim1::smcr::SMSW::EXTERNAL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline]
    pub fn etp(&self) -> ETPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ETPR { bits }
    }
    #[doc = "Bit 14 - External clock enable"]
    #[inline]
    pub fn ece(&self) -> ECER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ECER { bits }
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline]
    pub fn etps(&self) -> ETPSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ETPSR { bits }
    }
    #[doc = "Bits 8:11 - External trigger filter"]
    #[inline]
    pub fn etf(&self) -> ETFR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ETFR { bits }
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline]
    pub fn msm(&self) -> MSMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MSMR { bits }
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline]
    pub fn ts(&self) -> TSR {
        TSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline]
    pub fn sms(&self) -> SMSR {
        SMSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline]
    pub fn etp(&mut self) -> _ETPW {
        _ETPW { w: self }
    }
    #[doc = "Bit 14 - External clock enable"]
    #[inline]
    pub fn ece(&mut self) -> _ECEW {
        _ECEW { w: self }
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline]
    pub fn etps(&mut self) -> _ETPSW {
        _ETPSW { w: self }
    }
    #[doc = "Bits 8:11 - External trigger filter"]
    #[inline]
    pub fn etf(&mut self) -> _ETFW {
        _ETFW { w: self }
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline]
    pub fn msm(&mut self) -> _MSMW {
        _MSMW { w: self }
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline]
    pub fn ts(&mut self) -> _TSW {
        _TSW { w: self }
    }
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline]
    pub fn sms(&mut self) -> _SMSW {
        _SMSW { w: self }
    }
}
