#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MAPR {
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
pub struct SPI1_REMAPR {
    bits: bool,
}
impl SPI1_REMAPR {
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
pub struct I2C1_REMAPR {
    bits: bool,
}
impl I2C1_REMAPR {
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
pub struct USART1_REMAPR {
    bits: bool,
}
impl USART1_REMAPR {
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
pub struct USART2_REMAPR {
    bits: bool,
}
impl USART2_REMAPR {
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
pub struct USART3_REMAPR {
    bits: u8,
}
impl USART3_REMAPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TIM1_REMAPR {
    bits: u8,
}
impl TIM1_REMAPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TIM2_REMAPR {
    bits: u8,
}
impl TIM2_REMAPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TIM3_REMAPR {
    bits: u8,
}
impl TIM3_REMAPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TIM4_REMAPR {
    bits: bool,
}
impl TIM4_REMAPR {
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
pub struct CAN_REMAPR {
    bits: u8,
}
impl CAN_REMAPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PD01_REMAPR {
    bits: bool,
}
impl PD01_REMAPR {
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
pub struct TIM5CH4_IREMAPR {
    bits: bool,
}
impl TIM5CH4_IREMAPR {
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
pub struct ADC1_ETRGINJ_REMAPR {
    bits: bool,
}
impl ADC1_ETRGINJ_REMAPR {
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
pub struct ADC1_ETRGREG_REMAPR {
    bits: bool,
}
impl ADC1_ETRGREG_REMAPR {
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
pub struct ADC2_ETRGINJ_REMAPR {
    bits: bool,
}
impl ADC2_ETRGINJ_REMAPR {
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
pub struct ADC2_ETRGREG_REMAPR {
    bits: bool,
}
impl ADC2_ETRGREG_REMAPR {
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
pub struct _SPI1_REMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI1_REMAPW<'a> {
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
#[doc = r" Proxy"]
pub struct _I2C1_REMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C1_REMAPW<'a> {
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
pub struct _USART1_REMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _USART1_REMAPW<'a> {
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
pub struct _USART2_REMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _USART2_REMAPW<'a> {
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
#[doc = r" Proxy"]
pub struct _USART3_REMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _USART3_REMAPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TIM1_REMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM1_REMAPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TIM2_REMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM2_REMAPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TIM3_REMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM3_REMAPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TIM4_REMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM4_REMAPW<'a> {
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
#[doc = r" Proxy"]
pub struct _CAN_REMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_REMAPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PD01_REMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _PD01_REMAPW<'a> {
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
pub struct _TIM5CH4_IREMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM5CH4_IREMAPW<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADC1_ETRGINJ_REMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC1_ETRGINJ_REMAPW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADC1_ETRGREG_REMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC1_ETRGREG_REMAPW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADC2_ETRGINJ_REMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC2_ETRGINJ_REMAPW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADC2_ETRGREG_REMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC2_ETRGREG_REMAPW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SWJ_CFGW<'a> {
    w: &'a mut W,
}
impl<'a> _SWJ_CFGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - SPI1 remapping"]
    #[inline]
    pub fn spi1_remap(&self) -> SPI1_REMAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SPI1_REMAPR { bits }
    }
    #[doc = "Bit 1 - I2C1 remapping"]
    #[inline]
    pub fn i2c1_remap(&self) -> I2C1_REMAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        I2C1_REMAPR { bits }
    }
    #[doc = "Bit 2 - USART1 remapping"]
    #[inline]
    pub fn usart1_remap(&self) -> USART1_REMAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USART1_REMAPR { bits }
    }
    #[doc = "Bit 3 - USART2 remapping"]
    #[inline]
    pub fn usart2_remap(&self) -> USART2_REMAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USART2_REMAPR { bits }
    }
    #[doc = "Bits 4:5 - USART3 remapping"]
    #[inline]
    pub fn usart3_remap(&self) -> USART3_REMAPR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USART3_REMAPR { bits }
    }
    #[doc = "Bits 6:7 - TIM1 remapping"]
    #[inline]
    pub fn tim1_remap(&self) -> TIM1_REMAPR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TIM1_REMAPR { bits }
    }
    #[doc = "Bits 8:9 - TIM2 remapping"]
    #[inline]
    pub fn tim2_remap(&self) -> TIM2_REMAPR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TIM2_REMAPR { bits }
    }
    #[doc = "Bits 10:11 - TIM3 remapping"]
    #[inline]
    pub fn tim3_remap(&self) -> TIM3_REMAPR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TIM3_REMAPR { bits }
    }
    #[doc = "Bit 12 - TIM4 remapping"]
    #[inline]
    pub fn tim4_remap(&self) -> TIM4_REMAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TIM4_REMAPR { bits }
    }
    #[doc = "Bits 13:14 - CAN1 remapping"]
    #[inline]
    pub fn can_remap(&self) -> CAN_REMAPR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CAN_REMAPR { bits }
    }
    #[doc = "Bit 15 - Port D0/Port D1 mapping on OSCIN/OSCOUT"]
    #[inline]
    pub fn pd01_remap(&self) -> PD01_REMAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PD01_REMAPR { bits }
    }
    #[doc = "Bit 16 - Set and cleared by software"]
    #[inline]
    pub fn tim5ch4_iremap(&self) -> TIM5CH4_IREMAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TIM5CH4_IREMAPR { bits }
    }
    #[doc = "Bit 17 - ADC 1 External trigger injected conversion remapping"]
    #[inline]
    pub fn adc1_etrginj_remap(&self) -> ADC1_ETRGINJ_REMAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADC1_ETRGINJ_REMAPR { bits }
    }
    #[doc = "Bit 18 - ADC 1 external trigger regular conversion remapping"]
    #[inline]
    pub fn adc1_etrgreg_remap(&self) -> ADC1_ETRGREG_REMAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADC1_ETRGREG_REMAPR { bits }
    }
    #[doc = "Bit 19 - ADC 2 external trigger injected conversion remapping"]
    #[inline]
    pub fn adc2_etrginj_remap(&self) -> ADC2_ETRGINJ_REMAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADC2_ETRGINJ_REMAPR { bits }
    }
    #[doc = "Bit 20 - ADC 2 external trigger regular conversion remapping"]
    #[inline]
    pub fn adc2_etrgreg_remap(&self) -> ADC2_ETRGREG_REMAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADC2_ETRGREG_REMAPR { bits }
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
    #[doc = "Bit 0 - SPI1 remapping"]
    #[inline]
    pub fn spi1_remap(&mut self) -> _SPI1_REMAPW {
        _SPI1_REMAPW { w: self }
    }
    #[doc = "Bit 1 - I2C1 remapping"]
    #[inline]
    pub fn i2c1_remap(&mut self) -> _I2C1_REMAPW {
        _I2C1_REMAPW { w: self }
    }
    #[doc = "Bit 2 - USART1 remapping"]
    #[inline]
    pub fn usart1_remap(&mut self) -> _USART1_REMAPW {
        _USART1_REMAPW { w: self }
    }
    #[doc = "Bit 3 - USART2 remapping"]
    #[inline]
    pub fn usart2_remap(&mut self) -> _USART2_REMAPW {
        _USART2_REMAPW { w: self }
    }
    #[doc = "Bits 4:5 - USART3 remapping"]
    #[inline]
    pub fn usart3_remap(&mut self) -> _USART3_REMAPW {
        _USART3_REMAPW { w: self }
    }
    #[doc = "Bits 6:7 - TIM1 remapping"]
    #[inline]
    pub fn tim1_remap(&mut self) -> _TIM1_REMAPW {
        _TIM1_REMAPW { w: self }
    }
    #[doc = "Bits 8:9 - TIM2 remapping"]
    #[inline]
    pub fn tim2_remap(&mut self) -> _TIM2_REMAPW {
        _TIM2_REMAPW { w: self }
    }
    #[doc = "Bits 10:11 - TIM3 remapping"]
    #[inline]
    pub fn tim3_remap(&mut self) -> _TIM3_REMAPW {
        _TIM3_REMAPW { w: self }
    }
    #[doc = "Bit 12 - TIM4 remapping"]
    #[inline]
    pub fn tim4_remap(&mut self) -> _TIM4_REMAPW {
        _TIM4_REMAPW { w: self }
    }
    #[doc = "Bits 13:14 - CAN1 remapping"]
    #[inline]
    pub fn can_remap(&mut self) -> _CAN_REMAPW {
        _CAN_REMAPW { w: self }
    }
    #[doc = "Bit 15 - Port D0/Port D1 mapping on OSCIN/OSCOUT"]
    #[inline]
    pub fn pd01_remap(&mut self) -> _PD01_REMAPW {
        _PD01_REMAPW { w: self }
    }
    #[doc = "Bit 16 - Set and cleared by software"]
    #[inline]
    pub fn tim5ch4_iremap(&mut self) -> _TIM5CH4_IREMAPW {
        _TIM5CH4_IREMAPW { w: self }
    }
    #[doc = "Bit 17 - ADC 1 External trigger injected conversion remapping"]
    #[inline]
    pub fn adc1_etrginj_remap(&mut self) -> _ADC1_ETRGINJ_REMAPW {
        _ADC1_ETRGINJ_REMAPW { w: self }
    }
    #[doc = "Bit 18 - ADC 1 external trigger regular conversion remapping"]
    #[inline]
    pub fn adc1_etrgreg_remap(&mut self) -> _ADC1_ETRGREG_REMAPW {
        _ADC1_ETRGREG_REMAPW { w: self }
    }
    #[doc = "Bit 19 - ADC 2 external trigger injected conversion remapping"]
    #[inline]
    pub fn adc2_etrginj_remap(&mut self) -> _ADC2_ETRGINJ_REMAPW {
        _ADC2_ETRGINJ_REMAPW { w: self }
    }
    #[doc = "Bit 20 - ADC 2 external trigger regular conversion remapping"]
    #[inline]
    pub fn adc2_etrgreg_remap(&mut self) -> _ADC2_ETRGREG_REMAPW {
        _ADC2_ETRGREG_REMAPW { w: self }
    }
    #[doc = "Bits 24:26 - Serial wire JTAG configuration"]
    #[inline]
    pub fn swj_cfg(&mut self) -> _SWJ_CFGW {
        _SWJ_CFGW { w: self }
    }
}
