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
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSR {
    #[doc = "Internal Trigger 0 (ITR0)"]
    ITR0,
    #[doc = "Internal Trigger 1 (ITR1)"]
    ITR1,
    #[doc = "Internal Trigger 2 (ITR2)"]
    ITR2,
    #[doc = "Internal Trigger 3 (ITR3)"]
    ITR3,
    #[doc = "TI1 Edge Detector"]
    TI1F_ED,
    #[doc = "Filtered Timer Input 1"]
    TI1FP1,
    #[doc = "Filtered Timer Input 2"]
    TI2FP2,
    #[doc = "External Trigger input"]
    ETRF,
}
impl TSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSR::ITR0 => 0,
            TSR::ITR1 => 1,
            TSR::ITR2 => 2,
            TSR::ITR3 => 3,
            TSR::TI1F_ED => 4,
            TSR::TI1FP1 => 5,
            TSR::TI2FP2 => 6,
            TSR::ETRF => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSR {
        match value {
            0 => TSR::ITR0,
            1 => TSR::ITR1,
            2 => TSR::ITR2,
            3 => TSR::ITR3,
            4 => TSR::TI1F_ED,
            5 => TSR::TI1FP1,
            6 => TSR::TI2FP2,
            7 => TSR::ETRF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ITR0`"]
    #[inline]
    pub fn is_itr0(&self) -> bool {
        *self == TSR::ITR0
    }
    #[doc = "Checks if the value of the field is `ITR1`"]
    #[inline]
    pub fn is_itr1(&self) -> bool {
        *self == TSR::ITR1
    }
    #[doc = "Checks if the value of the field is `ITR2`"]
    #[inline]
    pub fn is_itr2(&self) -> bool {
        *self == TSR::ITR2
    }
    #[doc = "Checks if the value of the field is `ITR3`"]
    #[inline]
    pub fn is_itr3(&self) -> bool {
        *self == TSR::ITR3
    }
    #[doc = "Checks if the value of the field is `TI1F_ED`"]
    #[inline]
    pub fn is_ti1f_ed(&self) -> bool {
        *self == TSR::TI1F_ED
    }
    #[doc = "Checks if the value of the field is `TI1FP1`"]
    #[inline]
    pub fn is_ti1fp1(&self) -> bool {
        *self == TSR::TI1FP1
    }
    #[doc = "Checks if the value of the field is `TI2FP2`"]
    #[inline]
    pub fn is_ti2fp2(&self) -> bool {
        *self == TSR::TI2FP2
    }
    #[doc = "Checks if the value of the field is `ETRF`"]
    #[inline]
    pub fn is_etrf(&self) -> bool {
        *self == TSR::ETRF
    }
}
#[doc = "Possible values of the field `SMS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMSR {
    #[doc = "Counter disabled"]
    DISABLED,
    #[doc = "Encoder mode, count up/down on TI2FP1"]
    ENCODERTI2,
    #[doc = "Encoder mode, count up/down on TI1FP2"]
    ENCODERTI1,
    #[doc = "Encoder mode, count up/down on both TI1FP1 and TI2FP2"]
    ENCODERTI1TI2,
    #[doc = "Rising edge of the selected trigger input (TRGI) reinitializes the counter"]
    RESET,
    #[doc = " The counter clock is enabled when the trigger input (TRGI) is high"]
    GATED,
    #[doc = "The counter starts at a rising edge of the trigger TRGI "]
    TRIGGER,
    #[doc = " Rising edges of the selected trigger (TRGI) clock the counter"]
    EXTERNAL,
}
impl SMSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SMSR::DISABLED => 0,
            SMSR::ENCODERTI2 => 1,
            SMSR::ENCODERTI1 => 2,
            SMSR::ENCODERTI1TI2 => 3,
            SMSR::RESET => 4,
            SMSR::GATED => 5,
            SMSR::TRIGGER => 6,
            SMSR::EXTERNAL => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SMSR {
        match value {
            0 => SMSR::DISABLED,
            1 => SMSR::ENCODERTI2,
            2 => SMSR::ENCODERTI1,
            3 => SMSR::ENCODERTI1TI2,
            4 => SMSR::RESET,
            5 => SMSR::GATED,
            6 => SMSR::TRIGGER,
            7 => SMSR::EXTERNAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SMSR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENCODERTI2`"]
    #[inline]
    pub fn is_encoder_ti2(&self) -> bool {
        *self == SMSR::ENCODERTI2
    }
    #[doc = "Checks if the value of the field is `ENCODERTI1`"]
    #[inline]
    pub fn is_encoder_ti1(&self) -> bool {
        *self == SMSR::ENCODERTI1
    }
    #[doc = "Checks if the value of the field is `ENCODERTI1TI2`"]
    #[inline]
    pub fn is_encoder_ti1ti2(&self) -> bool {
        *self == SMSR::ENCODERTI1TI2
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == SMSR::RESET
    }
    #[doc = "Checks if the value of the field is `GATED`"]
    #[inline]
    pub fn is_gated(&self) -> bool {
        *self == SMSR::GATED
    }
    #[doc = "Checks if the value of the field is `TRIGGER`"]
    #[inline]
    pub fn is_trigger(&self) -> bool {
        *self == SMSR::TRIGGER
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline]
    pub fn is_external(&self) -> bool {
        *self == SMSR::EXTERNAL
    }
}
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
pub enum TSW {
    #[doc = "Internal Trigger 0 (ITR0)"]
    ITR0,
    #[doc = "Internal Trigger 1 (ITR1)"]
    ITR1,
    #[doc = "Internal Trigger 2 (ITR2)"]
    ITR2,
    #[doc = "Internal Trigger 3 (ITR3)"]
    ITR3,
    #[doc = "TI1 Edge Detector"]
    TI1F_ED,
    #[doc = "Filtered Timer Input 1"]
    TI1FP1,
    #[doc = "Filtered Timer Input 2"]
    TI2FP2,
    #[doc = "External Trigger input"]
    ETRF,
}
impl TSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSW::ITR0 => 0,
            TSW::ITR1 => 1,
            TSW::ITR2 => 2,
            TSW::ITR3 => 3,
            TSW::TI1F_ED => 4,
            TSW::TI1FP1 => 5,
            TSW::TI2FP2 => 6,
            TSW::ETRF => 7,
        }
    }
}
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
        self.variant(TSW::ITR0)
    }
    #[doc = "Internal Trigger 1 (ITR1)"]
    #[inline]
    pub fn itr1(self) -> &'a mut W {
        self.variant(TSW::ITR1)
    }
    #[doc = "Internal Trigger 2 (ITR2)"]
    #[inline]
    pub fn itr2(self) -> &'a mut W {
        self.variant(TSW::ITR2)
    }
    #[doc = "Internal Trigger 3 (ITR3)"]
    #[inline]
    pub fn itr3(self) -> &'a mut W {
        self.variant(TSW::ITR3)
    }
    #[doc = "TI1 Edge Detector"]
    #[inline]
    pub fn ti1f_ed(self) -> &'a mut W {
        self.variant(TSW::TI1F_ED)
    }
    #[doc = "Filtered Timer Input 1"]
    #[inline]
    pub fn ti1fp1(self) -> &'a mut W {
        self.variant(TSW::TI1FP1)
    }
    #[doc = "Filtered Timer Input 2"]
    #[inline]
    pub fn ti2fp2(self) -> &'a mut W {
        self.variant(TSW::TI2FP2)
    }
    #[doc = "External Trigger input"]
    #[inline]
    pub fn etrf(self) -> &'a mut W {
        self.variant(TSW::ETRF)
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
pub enum SMSW {
    #[doc = "Counter disabled"]
    DISABLED,
    #[doc = "Encoder mode, count up/down on TI2FP1"]
    ENCODERTI2,
    #[doc = "Encoder mode, count up/down on TI1FP2"]
    ENCODERTI1,
    #[doc = "Encoder mode, count up/down on both TI1FP1 and TI2FP2"]
    ENCODERTI1TI2,
    #[doc = "Rising edge of the selected trigger input (TRGI) reinitializes the counter"]
    RESET,
    #[doc = " The counter clock is enabled when the trigger input (TRGI) is high"]
    GATED,
    #[doc = "The counter starts at a rising edge of the trigger TRGI "]
    TRIGGER,
    #[doc = " Rising edges of the selected trigger (TRGI) clock the counter"]
    EXTERNAL,
}
impl SMSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SMSW::DISABLED => 0,
            SMSW::ENCODERTI2 => 1,
            SMSW::ENCODERTI1 => 2,
            SMSW::ENCODERTI1TI2 => 3,
            SMSW::RESET => 4,
            SMSW::GATED => 5,
            SMSW::TRIGGER => 6,
            SMSW::EXTERNAL => 7,
        }
    }
}
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
        self.variant(SMSW::DISABLED)
    }
    #[doc = "Encoder mode, count up/down on TI2FP1"]
    #[inline]
    pub fn encoder_ti2(self) -> &'a mut W {
        self.variant(SMSW::ENCODERTI2)
    }
    #[doc = "Encoder mode, count up/down on TI1FP2"]
    #[inline]
    pub fn encoder_ti1(self) -> &'a mut W {
        self.variant(SMSW::ENCODERTI1)
    }
    #[doc = "Encoder mode, count up/down on both TI1FP1 and TI2FP2"]
    #[inline]
    pub fn encoder_ti1ti2(self) -> &'a mut W {
        self.variant(SMSW::ENCODERTI1TI2)
    }
    #[doc = "Rising edge of the selected trigger input (TRGI) reinitializes the counter"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(SMSW::RESET)
    }
    #[doc = "The counter clock is enabled when the trigger input (TRGI) is high"]
    #[inline]
    pub fn gated(self) -> &'a mut W {
        self.variant(SMSW::GATED)
    }
    #[doc = "The counter starts at a rising edge of the trigger TRGI"]
    #[inline]
    pub fn trigger(self) -> &'a mut W {
        self.variant(SMSW::TRIGGER)
    }
    #[doc = "Rising edges of the selected trigger (TRGI) clock the counter"]
    #[inline]
    pub fn external(self) -> &'a mut W {
        self.variant(SMSW::EXTERNAL)
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
