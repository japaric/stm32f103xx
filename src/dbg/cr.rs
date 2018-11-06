#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR {
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
pub struct DBG_SLEEPR {
    bits: bool,
}
impl DBG_SLEEPR {
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
pub struct DBG_STOPR {
    bits: bool,
}
impl DBG_STOPR {
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
pub struct DBG_STANDBYR {
    bits: bool,
}
impl DBG_STANDBYR {
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
pub struct TRACE_IOENR {
    bits: bool,
}
impl TRACE_IOENR {
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
pub struct TRACE_MODER {
    bits: u8,
}
impl TRACE_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DBG_IWDG_STOPR {
    bits: bool,
}
impl DBG_IWDG_STOPR {
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
pub struct DBG_WWDG_STOPR {
    bits: bool,
}
impl DBG_WWDG_STOPR {
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
pub struct DBG_TIM1_STOPR {
    bits: bool,
}
impl DBG_TIM1_STOPR {
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
pub struct DBG_TIM2_STOPR {
    bits: bool,
}
impl DBG_TIM2_STOPR {
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
pub struct DBG_TIM3_STOPR {
    bits: bool,
}
impl DBG_TIM3_STOPR {
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
pub struct DBG_TIM4_STOPR {
    bits: bool,
}
impl DBG_TIM4_STOPR {
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
pub struct DBG_CAN1_STOPR {
    bits: bool,
}
impl DBG_CAN1_STOPR {
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
pub struct DBG_I2C1_SMBUS_TIMEOUTR {
    bits: bool,
}
impl DBG_I2C1_SMBUS_TIMEOUTR {
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
pub struct DBG_I2C2_SMBUS_TIMEOUTR {
    bits: bool,
}
impl DBG_I2C2_SMBUS_TIMEOUTR {
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
pub struct DBG_TIM8_STOPR {
    bits: bool,
}
impl DBG_TIM8_STOPR {
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
pub struct DBG_TIM5_STOPR {
    bits: bool,
}
impl DBG_TIM5_STOPR {
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
pub struct DBG_TIM6_STOPR {
    bits: bool,
}
impl DBG_TIM6_STOPR {
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
pub struct DBG_TIM7_STOPR {
    bits: bool,
}
impl DBG_TIM7_STOPR {
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
pub struct DBG_CAN2_STOPR {
    bits: bool,
}
impl DBG_CAN2_STOPR {
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
pub struct _DBG_SLEEPW<'a> {
    w: &'a mut W,
}
impl<'a> _DBG_SLEEPW<'a> {
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
pub struct _DBG_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _DBG_STOPW<'a> {
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
pub struct _DBG_STANDBYW<'a> {
    w: &'a mut W,
}
impl<'a> _DBG_STANDBYW<'a> {
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
pub struct _TRACE_IOENW<'a> {
    w: &'a mut W,
}
impl<'a> _TRACE_IOENW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRACE_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRACE_MODEW<'a> {
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
pub struct _DBG_IWDG_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _DBG_IWDG_STOPW<'a> {
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
pub struct _DBG_WWDG_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _DBG_WWDG_STOPW<'a> {
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
pub struct _DBG_TIM1_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _DBG_TIM1_STOPW<'a> {
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
pub struct _DBG_TIM2_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _DBG_TIM2_STOPW<'a> {
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
pub struct _DBG_TIM3_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _DBG_TIM3_STOPW<'a> {
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
pub struct _DBG_TIM4_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _DBG_TIM4_STOPW<'a> {
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
pub struct _DBG_CAN1_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _DBG_CAN1_STOPW<'a> {
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
pub struct _DBG_I2C1_SMBUS_TIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _DBG_I2C1_SMBUS_TIMEOUTW<'a> {
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
pub struct _DBG_I2C2_SMBUS_TIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _DBG_I2C2_SMBUS_TIMEOUTW<'a> {
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
pub struct _DBG_TIM8_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _DBG_TIM8_STOPW<'a> {
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
pub struct _DBG_TIM5_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _DBG_TIM5_STOPW<'a> {
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
pub struct _DBG_TIM6_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _DBG_TIM6_STOPW<'a> {
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
pub struct _DBG_TIM7_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _DBG_TIM7_STOPW<'a> {
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
pub struct _DBG_CAN2_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _DBG_CAN2_STOPW<'a> {
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
    #[doc = "Bit 0 - DBG_SLEEP"]
    #[inline]
    pub fn dbg_sleep(&self) -> DBG_SLEEPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DBG_SLEEPR { bits }
    }
    #[doc = "Bit 1 - DBG_STOP"]
    #[inline]
    pub fn dbg_stop(&self) -> DBG_STOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DBG_STOPR { bits }
    }
    #[doc = "Bit 2 - DBG_STANDBY"]
    #[inline]
    pub fn dbg_standby(&self) -> DBG_STANDBYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DBG_STANDBYR { bits }
    }
    #[doc = "Bit 5 - TRACE_IOEN"]
    #[inline]
    pub fn trace_ioen(&self) -> TRACE_IOENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TRACE_IOENR { bits }
    }
    #[doc = "Bits 6:7 - TRACE_MODE"]
    #[inline]
    pub fn trace_mode(&self) -> TRACE_MODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRACE_MODER { bits }
    }
    #[doc = "Bit 8 - DBG_IWDG_STOP"]
    #[inline]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DBG_IWDG_STOPR { bits }
    }
    #[doc = "Bit 9 - DBG_WWDG_STOP"]
    #[inline]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DBG_WWDG_STOPR { bits }
    }
    #[doc = "Bit 10 - DBG_TIM1_STOP"]
    #[inline]
    pub fn dbg_tim1_stop(&self) -> DBG_TIM1_STOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DBG_TIM1_STOPR { bits }
    }
    #[doc = "Bit 11 - DBG_TIM2_STOP"]
    #[inline]
    pub fn dbg_tim2_stop(&self) -> DBG_TIM2_STOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DBG_TIM2_STOPR { bits }
    }
    #[doc = "Bit 12 - DBG_TIM3_STOP"]
    #[inline]
    pub fn dbg_tim3_stop(&self) -> DBG_TIM3_STOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DBG_TIM3_STOPR { bits }
    }
    #[doc = "Bit 13 - DBG_TIM4_STOP"]
    #[inline]
    pub fn dbg_tim4_stop(&self) -> DBG_TIM4_STOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DBG_TIM4_STOPR { bits }
    }
    #[doc = "Bit 14 - DBG_CAN1_STOP"]
    #[inline]
    pub fn dbg_can1_stop(&self) -> DBG_CAN1_STOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DBG_CAN1_STOPR { bits }
    }
    #[doc = "Bit 15 - DBG_I2C1_SMBUS_TIMEOUT"]
    #[inline]
    pub fn dbg_i2c1_smbus_timeout(&self) -> DBG_I2C1_SMBUS_TIMEOUTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DBG_I2C1_SMBUS_TIMEOUTR { bits }
    }
    #[doc = "Bit 16 - DBG_I2C2_SMBUS_TIMEOUT"]
    #[inline]
    pub fn dbg_i2c2_smbus_timeout(&self) -> DBG_I2C2_SMBUS_TIMEOUTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DBG_I2C2_SMBUS_TIMEOUTR { bits }
    }
    #[doc = "Bit 17 - DBG_TIM8_STOP"]
    #[inline]
    pub fn dbg_tim8_stop(&self) -> DBG_TIM8_STOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DBG_TIM8_STOPR { bits }
    }
    #[doc = "Bit 18 - DBG_TIM5_STOP"]
    #[inline]
    pub fn dbg_tim5_stop(&self) -> DBG_TIM5_STOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DBG_TIM5_STOPR { bits }
    }
    #[doc = "Bit 19 - DBG_TIM6_STOP"]
    #[inline]
    pub fn dbg_tim6_stop(&self) -> DBG_TIM6_STOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DBG_TIM6_STOPR { bits }
    }
    #[doc = "Bit 20 - DBG_TIM7_STOP"]
    #[inline]
    pub fn dbg_tim7_stop(&self) -> DBG_TIM7_STOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DBG_TIM7_STOPR { bits }
    }
    #[doc = "Bit 21 - DBG_CAN2_STOP"]
    #[inline]
    pub fn dbg_can2_stop(&self) -> DBG_CAN2_STOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DBG_CAN2_STOPR { bits }
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
    #[doc = "Bit 0 - DBG_SLEEP"]
    #[inline]
    pub fn dbg_sleep(&mut self) -> _DBG_SLEEPW {
        _DBG_SLEEPW { w: self }
    }
    #[doc = "Bit 1 - DBG_STOP"]
    #[inline]
    pub fn dbg_stop(&mut self) -> _DBG_STOPW {
        _DBG_STOPW { w: self }
    }
    #[doc = "Bit 2 - DBG_STANDBY"]
    #[inline]
    pub fn dbg_standby(&mut self) -> _DBG_STANDBYW {
        _DBG_STANDBYW { w: self }
    }
    #[doc = "Bit 5 - TRACE_IOEN"]
    #[inline]
    pub fn trace_ioen(&mut self) -> _TRACE_IOENW {
        _TRACE_IOENW { w: self }
    }
    #[doc = "Bits 6:7 - TRACE_MODE"]
    #[inline]
    pub fn trace_mode(&mut self) -> _TRACE_MODEW {
        _TRACE_MODEW { w: self }
    }
    #[doc = "Bit 8 - DBG_IWDG_STOP"]
    #[inline]
    pub fn dbg_iwdg_stop(&mut self) -> _DBG_IWDG_STOPW {
        _DBG_IWDG_STOPW { w: self }
    }
    #[doc = "Bit 9 - DBG_WWDG_STOP"]
    #[inline]
    pub fn dbg_wwdg_stop(&mut self) -> _DBG_WWDG_STOPW {
        _DBG_WWDG_STOPW { w: self }
    }
    #[doc = "Bit 10 - DBG_TIM1_STOP"]
    #[inline]
    pub fn dbg_tim1_stop(&mut self) -> _DBG_TIM1_STOPW {
        _DBG_TIM1_STOPW { w: self }
    }
    #[doc = "Bit 11 - DBG_TIM2_STOP"]
    #[inline]
    pub fn dbg_tim2_stop(&mut self) -> _DBG_TIM2_STOPW {
        _DBG_TIM2_STOPW { w: self }
    }
    #[doc = "Bit 12 - DBG_TIM3_STOP"]
    #[inline]
    pub fn dbg_tim3_stop(&mut self) -> _DBG_TIM3_STOPW {
        _DBG_TIM3_STOPW { w: self }
    }
    #[doc = "Bit 13 - DBG_TIM4_STOP"]
    #[inline]
    pub fn dbg_tim4_stop(&mut self) -> _DBG_TIM4_STOPW {
        _DBG_TIM4_STOPW { w: self }
    }
    #[doc = "Bit 14 - DBG_CAN1_STOP"]
    #[inline]
    pub fn dbg_can1_stop(&mut self) -> _DBG_CAN1_STOPW {
        _DBG_CAN1_STOPW { w: self }
    }
    #[doc = "Bit 15 - DBG_I2C1_SMBUS_TIMEOUT"]
    #[inline]
    pub fn dbg_i2c1_smbus_timeout(&mut self) -> _DBG_I2C1_SMBUS_TIMEOUTW {
        _DBG_I2C1_SMBUS_TIMEOUTW { w: self }
    }
    #[doc = "Bit 16 - DBG_I2C2_SMBUS_TIMEOUT"]
    #[inline]
    pub fn dbg_i2c2_smbus_timeout(&mut self) -> _DBG_I2C2_SMBUS_TIMEOUTW {
        _DBG_I2C2_SMBUS_TIMEOUTW { w: self }
    }
    #[doc = "Bit 17 - DBG_TIM8_STOP"]
    #[inline]
    pub fn dbg_tim8_stop(&mut self) -> _DBG_TIM8_STOPW {
        _DBG_TIM8_STOPW { w: self }
    }
    #[doc = "Bit 18 - DBG_TIM5_STOP"]
    #[inline]
    pub fn dbg_tim5_stop(&mut self) -> _DBG_TIM5_STOPW {
        _DBG_TIM5_STOPW { w: self }
    }
    #[doc = "Bit 19 - DBG_TIM6_STOP"]
    #[inline]
    pub fn dbg_tim6_stop(&mut self) -> _DBG_TIM6_STOPW {
        _DBG_TIM6_STOPW { w: self }
    }
    #[doc = "Bit 20 - DBG_TIM7_STOP"]
    #[inline]
    pub fn dbg_tim7_stop(&mut self) -> _DBG_TIM7_STOPW {
        _DBG_TIM7_STOPW { w: self }
    }
    #[doc = "Bit 21 - DBG_CAN2_STOP"]
    #[inline]
    pub fn dbg_can2_stop(&mut self) -> _DBG_CAN2_STOPW {
        _DBG_CAN2_STOPW { w: self }
    }
}
