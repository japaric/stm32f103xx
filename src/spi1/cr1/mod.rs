#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR1 {
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
pub struct BIDIMODER {
    bits: bool,
}
impl BIDIMODER {
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
pub struct BIDIOER {
    bits: bool,
}
impl BIDIOER {
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
pub struct CRCENR {
    bits: bool,
}
impl CRCENR {
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
pub struct CRCNEXTR {
    bits: bool,
}
impl CRCNEXTR {
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
#[doc = "Possible values of the field `DFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFFR {
    #[doc = "8-bit data frame format is selected for transmission/reception"]
    BIT8,
    #[doc = "16-bit data frame format is selected for transmission/reception"]
    BIT16,
}
impl DFFR {
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
            DFFR::BIT8 => false,
            DFFR::BIT16 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFFR {
        match value {
            false => DFFR::BIT8,
            true => DFFR::BIT16,
        }
    }
    #[doc = "Checks if the value of the field is `BIT8`"]
    #[inline]
    pub fn is_bit8(&self) -> bool {
        *self == DFFR::BIT8
    }
    #[doc = "Checks if the value of the field is `BIT16`"]
    #[inline]
    pub fn is_bit16(&self) -> bool {
        *self == DFFR::BIT16
    }
}
#[doc = r" Value of the field"]
pub struct RXONLYR {
    bits: bool,
}
impl RXONLYR {
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
pub struct SSMR {
    bits: bool,
}
impl SSMR {
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
pub struct SSIR {
    bits: bool,
}
impl SSIR {
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
pub struct LSBFIRSTR {
    bits: bool,
}
impl LSBFIRSTR {
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
#[doc = "Possible values of the field `SPE`"]
pub type SPER = ::rcc::ahbenr::DMA1ENR;
#[doc = "Possible values of the field `BR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRR {
    #[doc = "f_PCLK/2"]
    DIV2,
    #[doc = "f_PCLK/4"]
    DIV4,
    #[doc = "f_PCLK/8"]
    DIV8,
    #[doc = "f_PCLK/16"]
    DIV16,
    #[doc = "f_PCLK/32"]
    DIV32,
    #[doc = "f_PCLK/64"]
    DIV64,
    #[doc = "f_PCLK/128"]
    DIV128,
    #[doc = "f_PCLK/256"]
    DIV256,
}
impl BRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BRR::DIV2 => 0,
            BRR::DIV4 => 1,
            BRR::DIV8 => 2,
            BRR::DIV16 => 3,
            BRR::DIV32 => 4,
            BRR::DIV64 => 5,
            BRR::DIV128 => 6,
            BRR::DIV256 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BRR {
        match value {
            0 => BRR::DIV2,
            1 => BRR::DIV4,
            2 => BRR::DIV8,
            3 => BRR::DIV16,
            4 => BRR::DIV32,
            5 => BRR::DIV64,
            6 => BRR::DIV128,
            7 => BRR::DIV256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == BRR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == BRR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == BRR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == BRR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == BRR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == BRR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == BRR::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline]
    pub fn is_div256(&self) -> bool {
        *self == BRR::DIV256
    }
}
#[doc = "Possible values of the field `MSTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTRR {
    #[doc = "Slave configuration"]
    SLAVE,
    #[doc = "Master configuration"]
    MASTER,
}
impl MSTRR {
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
            MSTRR::SLAVE => false,
            MSTRR::MASTER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTRR {
        match value {
            false => MSTRR::SLAVE,
            true => MSTRR::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline]
    pub fn is_slave(&self) -> bool {
        *self == MSTRR::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline]
    pub fn is_master(&self) -> bool {
        *self == MSTRR::MASTER
    }
}
#[doc = r" Value of the field"]
pub struct CPOLR {
    bits: bool,
}
impl CPOLR {
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
pub struct CPHAR {
    bits: bool,
}
impl CPHAR {
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
#[doc = r" Proxy"]
pub struct _BIDIMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _BIDIMODEW<'a> {
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
pub struct _BIDIOEW<'a> {
    w: &'a mut W,
}
impl<'a> _BIDIOEW<'a> {
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
pub struct _CRCENW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCENW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CRCNEXTW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCNEXTW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DFF`"]
pub enum DFFW {
    #[doc = "8-bit data frame format is selected for transmission/reception"]
    BIT8,
    #[doc = "16-bit data frame format is selected for transmission/reception"]
    BIT16,
}
impl DFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFFW::BIT8 => false,
            DFFW::BIT16 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFFW<'a> {
    w: &'a mut W,
}
impl<'a> _DFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "8-bit data frame format is selected for transmission/reception"]
    #[inline]
    pub fn bit8(self) -> &'a mut W {
        self.variant(DFFW::BIT8)
    }
    #[doc = "16-bit data frame format is selected for transmission/reception"]
    #[inline]
    pub fn bit16(self) -> &'a mut W {
        self.variant(DFFW::BIT16)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXONLYW<'a> {
    w: &'a mut W,
}
impl<'a> _RXONLYW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SSMW<'a> {
    w: &'a mut W,
}
impl<'a> _SSMW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SSIW<'a> {
    w: &'a mut W,
}
impl<'a> _SSIW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LSBFIRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _LSBFIRSTW<'a> {
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
#[doc = "Values that can be written to the field `SPE`"]
pub type SPEW = ::rcc::ahbenr::DMA1ENW;
#[doc = r" Proxy"]
pub struct _SPEW<'a> {
    w: &'a mut W,
}
impl<'a> _SPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(::rcc::ahbenr::DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(::rcc::ahbenr::DMA1ENW::ENABLED)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BR`"]
pub enum BRW {
    #[doc = "f_PCLK/2"]
    DIV2,
    #[doc = "f_PCLK/4"]
    DIV4,
    #[doc = "f_PCLK/8"]
    DIV8,
    #[doc = "f_PCLK/16"]
    DIV16,
    #[doc = "f_PCLK/32"]
    DIV32,
    #[doc = "f_PCLK/64"]
    DIV64,
    #[doc = "f_PCLK/128"]
    DIV128,
    #[doc = "f_PCLK/256"]
    DIV256,
}
impl BRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BRW::DIV2 => 0,
            BRW::DIV4 => 1,
            BRW::DIV8 => 2,
            BRW::DIV16 => 3,
            BRW::DIV32 => 4,
            BRW::DIV64 => 5,
            BRW::DIV128 => 6,
            BRW::DIV256 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BRW<'a> {
    w: &'a mut W,
}
impl<'a> _BRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "f_PCLK/2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(BRW::DIV2)
    }
    #[doc = "f_PCLK/4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(BRW::DIV4)
    }
    #[doc = "f_PCLK/8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(BRW::DIV8)
    }
    #[doc = "f_PCLK/16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(BRW::DIV16)
    }
    #[doc = "f_PCLK/32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(BRW::DIV32)
    }
    #[doc = "f_PCLK/64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(BRW::DIV64)
    }
    #[doc = "f_PCLK/128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(BRW::DIV128)
    }
    #[doc = "f_PCLK/256"]
    #[inline]
    pub fn div256(self) -> &'a mut W {
        self.variant(BRW::DIV256)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MSTR`"]
pub enum MSTRW {
    #[doc = "Slave configuration"]
    SLAVE,
    #[doc = "Master configuration"]
    MASTER,
}
impl MSTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTRW::SLAVE => false,
            MSTRW::MASTER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTRW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slave configuration"]
    #[inline]
    pub fn slave(self) -> &'a mut W {
        self.variant(MSTRW::SLAVE)
    }
    #[doc = "Master configuration"]
    #[inline]
    pub fn master(self) -> &'a mut W {
        self.variant(MSTRW::MASTER)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _CPOLW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _CPHAW<'a> {
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
    #[doc = "Bit 15 - Bidirectional data mode enable"]
    #[inline]
    pub fn bidimode(&self) -> BIDIMODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BIDIMODER { bits }
    }
    #[doc = "Bit 14 - Output enable in bidirectional mode"]
    #[inline]
    pub fn bidioe(&self) -> BIDIOER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BIDIOER { bits }
    }
    #[doc = "Bit 13 - Hardware CRC calculation enable"]
    #[inline]
    pub fn crcen(&self) -> CRCENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CRCENR { bits }
    }
    #[doc = "Bit 12 - CRC transfer next"]
    #[inline]
    pub fn crcnext(&self) -> CRCNEXTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CRCNEXTR { bits }
    }
    #[doc = "Bit 11 - Data frame format"]
    #[inline]
    pub fn dff(&self) -> DFFR {
        DFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Receive only"]
    #[inline]
    pub fn rxonly(&self) -> RXONLYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXONLYR { bits }
    }
    #[doc = "Bit 9 - Software slave management"]
    #[inline]
    pub fn ssm(&self) -> SSMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SSMR { bits }
    }
    #[doc = "Bit 8 - Internal slave select"]
    #[inline]
    pub fn ssi(&self) -> SSIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SSIR { bits }
    }
    #[doc = "Bit 7 - Frame format"]
    #[inline]
    pub fn lsbfirst(&self) -> LSBFIRSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LSBFIRSTR { bits }
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline]
    pub fn spe(&self) -> SPER {
        SPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:5 - Baud rate control"]
    #[inline]
    pub fn br(&self) -> BRR {
        BRR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Master selection"]
    #[inline]
    pub fn mstr(&self) -> MSTRR {
        MSTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Clock polarity"]
    #[inline]
    pub fn cpol(&self) -> CPOLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CPOLR { bits }
    }
    #[doc = "Bit 0 - Clock phase"]
    #[inline]
    pub fn cpha(&self) -> CPHAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CPHAR { bits }
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
    #[doc = "Bit 15 - Bidirectional data mode enable"]
    #[inline]
    pub fn bidimode(&mut self) -> _BIDIMODEW {
        _BIDIMODEW { w: self }
    }
    #[doc = "Bit 14 - Output enable in bidirectional mode"]
    #[inline]
    pub fn bidioe(&mut self) -> _BIDIOEW {
        _BIDIOEW { w: self }
    }
    #[doc = "Bit 13 - Hardware CRC calculation enable"]
    #[inline]
    pub fn crcen(&mut self) -> _CRCENW {
        _CRCENW { w: self }
    }
    #[doc = "Bit 12 - CRC transfer next"]
    #[inline]
    pub fn crcnext(&mut self) -> _CRCNEXTW {
        _CRCNEXTW { w: self }
    }
    #[doc = "Bit 11 - Data frame format"]
    #[inline]
    pub fn dff(&mut self) -> _DFFW {
        _DFFW { w: self }
    }
    #[doc = "Bit 10 - Receive only"]
    #[inline]
    pub fn rxonly(&mut self) -> _RXONLYW {
        _RXONLYW { w: self }
    }
    #[doc = "Bit 9 - Software slave management"]
    #[inline]
    pub fn ssm(&mut self) -> _SSMW {
        _SSMW { w: self }
    }
    #[doc = "Bit 8 - Internal slave select"]
    #[inline]
    pub fn ssi(&mut self) -> _SSIW {
        _SSIW { w: self }
    }
    #[doc = "Bit 7 - Frame format"]
    #[inline]
    pub fn lsbfirst(&mut self) -> _LSBFIRSTW {
        _LSBFIRSTW { w: self }
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline]
    pub fn spe(&mut self) -> _SPEW {
        _SPEW { w: self }
    }
    #[doc = "Bits 3:5 - Baud rate control"]
    #[inline]
    pub fn br(&mut self) -> _BRW {
        _BRW { w: self }
    }
    #[doc = "Bit 2 - Master selection"]
    #[inline]
    pub fn mstr(&mut self) -> _MSTRW {
        _MSTRW { w: self }
    }
    #[doc = "Bit 1 - Clock polarity"]
    #[inline]
    pub fn cpol(&mut self) -> _CPOLW {
        _CPOLW { w: self }
    }
    #[doc = "Bit 0 - Clock phase"]
    #[inline]
    pub fn cpha(&mut self) -> _CPHAW {
        _CPHAW { w: self }
    }
}
