#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CRH {
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
#[doc = "Possible values of the field `MODE8`"]
pub type MODE8R = super::crl::MODE0R;
#[doc = "Possible values of the field `CNF8`"]
pub type CNF8R = super::crl::CNF0R;
#[doc = "Possible values of the field `MODE9`"]
pub type MODE9R = super::crl::MODE0R;
#[doc = "Possible values of the field `CNF9`"]
pub type CNF9R = super::crl::CNF0R;
#[doc = "Possible values of the field `MODE10`"]
pub type MODE10R = super::crl::MODE0R;
#[doc = "Possible values of the field `CNF10`"]
pub type CNF10R = super::crl::CNF0R;
#[doc = "Possible values of the field `MODE11`"]
pub type MODE11R = super::crl::MODE0R;
#[doc = "Possible values of the field `CNF11`"]
pub type CNF11R = super::crl::CNF0R;
#[doc = "Possible values of the field `MODE12`"]
pub type MODE12R = super::crl::MODE0R;
#[doc = "Possible values of the field `CNF12`"]
pub type CNF12R = super::crl::CNF0R;
#[doc = "Possible values of the field `MODE13`"]
pub type MODE13R = super::crl::MODE0R;
#[doc = "Possible values of the field `CNF13`"]
pub type CNF13R = super::crl::CNF0R;
#[doc = "Possible values of the field `MODE14`"]
pub type MODE14R = super::crl::MODE0R;
#[doc = "Possible values of the field `CNF14`"]
pub type CNF14R = super::crl::CNF0R;
#[doc = "Possible values of the field `MODE15`"]
pub type MODE15R = super::crl::MODE0R;
#[doc = "Possible values of the field `CNF15`"]
pub type CNF15R = super::crl::CNF0R;
#[doc = "Values that can be written to the field `MODE8`"]
pub type MODE8W = super::crl::MODE0W;
#[doc = r" Proxy"]
pub struct _MODE8W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE8W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline]
    pub fn output2(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline]
    pub fn output50(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::OUTPUT50)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CNF8`"]
pub type CNF8W = super::crl::CNF0W;
#[doc = r" Proxy"]
pub struct _CNF8W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF8W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Push-Pull mode"]
    #[inline]
    pub fn push(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::PUSH)
    }
    #[doc = "Open Drain-Mode"]
    #[inline]
    pub fn open(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::OPEN)
    }
    #[doc = "Alternate Function Push-Pull Mode"]
    #[inline]
    pub fn alt_push(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::ALTPUSH)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline]
    pub fn alt_open(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::ALTOPEN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE9`"]
pub type MODE9W = super::crl::MODE0W;
#[doc = r" Proxy"]
pub struct _MODE9W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE9W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline]
    pub fn output2(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline]
    pub fn output50(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::OUTPUT50)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CNF9`"]
pub type CNF9W = super::crl::CNF0W;
#[doc = r" Proxy"]
pub struct _CNF9W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF9W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Push-Pull mode"]
    #[inline]
    pub fn push(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::PUSH)
    }
    #[doc = "Open Drain-Mode"]
    #[inline]
    pub fn open(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::OPEN)
    }
    #[doc = "Alternate Function Push-Pull Mode"]
    #[inline]
    pub fn alt_push(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::ALTPUSH)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline]
    pub fn alt_open(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::ALTOPEN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE10`"]
pub type MODE10W = super::crl::MODE0W;
#[doc = r" Proxy"]
pub struct _MODE10W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE10W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline]
    pub fn output2(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline]
    pub fn output50(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::OUTPUT50)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CNF10`"]
pub type CNF10W = super::crl::CNF0W;
#[doc = r" Proxy"]
pub struct _CNF10W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF10W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Push-Pull mode"]
    #[inline]
    pub fn push(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::PUSH)
    }
    #[doc = "Open Drain-Mode"]
    #[inline]
    pub fn open(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::OPEN)
    }
    #[doc = "Alternate Function Push-Pull Mode"]
    #[inline]
    pub fn alt_push(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::ALTPUSH)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline]
    pub fn alt_open(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::ALTOPEN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE11`"]
pub type MODE11W = super::crl::MODE0W;
#[doc = r" Proxy"]
pub struct _MODE11W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE11W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline]
    pub fn output2(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline]
    pub fn output50(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::OUTPUT50)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CNF11`"]
pub type CNF11W = super::crl::CNF0W;
#[doc = r" Proxy"]
pub struct _CNF11W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF11W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Push-Pull mode"]
    #[inline]
    pub fn push(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::PUSH)
    }
    #[doc = "Open Drain-Mode"]
    #[inline]
    pub fn open(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::OPEN)
    }
    #[doc = "Alternate Function Push-Pull Mode"]
    #[inline]
    pub fn alt_push(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::ALTPUSH)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline]
    pub fn alt_open(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::ALTOPEN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE12`"]
pub type MODE12W = super::crl::MODE0W;
#[doc = r" Proxy"]
pub struct _MODE12W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE12W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline]
    pub fn output2(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline]
    pub fn output50(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::OUTPUT50)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CNF12`"]
pub type CNF12W = super::crl::CNF0W;
#[doc = r" Proxy"]
pub struct _CNF12W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF12W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Push-Pull mode"]
    #[inline]
    pub fn push(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::PUSH)
    }
    #[doc = "Open Drain-Mode"]
    #[inline]
    pub fn open(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::OPEN)
    }
    #[doc = "Alternate Function Push-Pull Mode"]
    #[inline]
    pub fn alt_push(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::ALTPUSH)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline]
    pub fn alt_open(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::ALTOPEN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE13`"]
pub type MODE13W = super::crl::MODE0W;
#[doc = r" Proxy"]
pub struct _MODE13W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE13W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline]
    pub fn output2(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline]
    pub fn output50(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::OUTPUT50)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CNF13`"]
pub type CNF13W = super::crl::CNF0W;
#[doc = r" Proxy"]
pub struct _CNF13W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF13W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Push-Pull mode"]
    #[inline]
    pub fn push(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::PUSH)
    }
    #[doc = "Open Drain-Mode"]
    #[inline]
    pub fn open(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::OPEN)
    }
    #[doc = "Alternate Function Push-Pull Mode"]
    #[inline]
    pub fn alt_push(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::ALTPUSH)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline]
    pub fn alt_open(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::ALTOPEN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE14`"]
pub type MODE14W = super::crl::MODE0W;
#[doc = r" Proxy"]
pub struct _MODE14W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE14W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline]
    pub fn output2(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline]
    pub fn output50(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::OUTPUT50)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CNF14`"]
pub type CNF14W = super::crl::CNF0W;
#[doc = r" Proxy"]
pub struct _CNF14W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF14W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Push-Pull mode"]
    #[inline]
    pub fn push(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::PUSH)
    }
    #[doc = "Open Drain-Mode"]
    #[inline]
    pub fn open(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::OPEN)
    }
    #[doc = "Alternate Function Push-Pull Mode"]
    #[inline]
    pub fn alt_push(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::ALTPUSH)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline]
    pub fn alt_open(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::ALTOPEN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE15`"]
pub type MODE15W = super::crl::MODE0W;
#[doc = r" Proxy"]
pub struct _MODE15W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE15W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline]
    pub fn output2(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline]
    pub fn output50(self) -> &'a mut W {
        self.variant(super::crl::MODE0W::OUTPUT50)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CNF15`"]
pub type CNF15W = super::crl::CNF0W;
#[doc = r" Proxy"]
pub struct _CNF15W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF15W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Push-Pull mode"]
    #[inline]
    pub fn push(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::PUSH)
    }
    #[doc = "Open Drain-Mode"]
    #[inline]
    pub fn open(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::OPEN)
    }
    #[doc = "Alternate Function Push-Pull Mode"]
    #[inline]
    pub fn alt_push(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::ALTPUSH)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline]
    pub fn alt_open(self) -> &'a mut W {
        self.variant(super::crl::CNF0W::ALTOPEN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:1 - Port n.8 mode bits"]
    #[inline]
    pub fn mode8(&self) -> MODE8R {
        MODE8R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Port n.8 configuration bits"]
    #[inline]
    pub fn cnf8(&self) -> CNF8R {
        CNF8R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Port n.9 mode bits"]
    #[inline]
    pub fn mode9(&self) -> MODE9R {
        MODE9R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Port n.9 configuration bits"]
    #[inline]
    pub fn cnf9(&self) -> CNF9R {
        CNF9R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Port n.10 mode bits"]
    #[inline]
    pub fn mode10(&self) -> MODE10R {
        MODE10R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Port n.10 configuration bits"]
    #[inline]
    pub fn cnf10(&self) -> CNF10R {
        CNF10R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Port n.11 mode bits"]
    #[inline]
    pub fn mode11(&self) -> MODE11R {
        MODE11R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Port n.11 configuration bits"]
    #[inline]
    pub fn cnf11(&self) -> CNF11R {
        CNF11R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Port n.12 mode bits"]
    #[inline]
    pub fn mode12(&self) -> MODE12R {
        MODE12R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Port n.12 configuration bits"]
    #[inline]
    pub fn cnf12(&self) -> CNF12R {
        CNF12R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Port n.13 mode bits"]
    #[inline]
    pub fn mode13(&self) -> MODE13R {
        MODE13R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Port n.13 configuration bits"]
    #[inline]
    pub fn cnf13(&self) -> CNF13R {
        CNF13R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Port n.14 mode bits"]
    #[inline]
    pub fn mode14(&self) -> MODE14R {
        MODE14R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - Port n.14 configuration bits"]
    #[inline]
    pub fn cnf14(&self) -> CNF14R {
        CNF14R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - Port n.15 mode bits"]
    #[inline]
    pub fn mode15(&self) -> MODE15R {
        MODE15R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - Port n.15 configuration bits"]
    #[inline]
    pub fn cnf15(&self) -> CNF15R {
        CNF15R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1145324612 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Port n.8 mode bits"]
    #[inline]
    pub fn mode8(&mut self) -> _MODE8W {
        _MODE8W { w: self }
    }
    #[doc = "Bits 2:3 - Port n.8 configuration bits"]
    #[inline]
    pub fn cnf8(&mut self) -> _CNF8W {
        _CNF8W { w: self }
    }
    #[doc = "Bits 4:5 - Port n.9 mode bits"]
    #[inline]
    pub fn mode9(&mut self) -> _MODE9W {
        _MODE9W { w: self }
    }
    #[doc = "Bits 6:7 - Port n.9 configuration bits"]
    #[inline]
    pub fn cnf9(&mut self) -> _CNF9W {
        _CNF9W { w: self }
    }
    #[doc = "Bits 8:9 - Port n.10 mode bits"]
    #[inline]
    pub fn mode10(&mut self) -> _MODE10W {
        _MODE10W { w: self }
    }
    #[doc = "Bits 10:11 - Port n.10 configuration bits"]
    #[inline]
    pub fn cnf10(&mut self) -> _CNF10W {
        _CNF10W { w: self }
    }
    #[doc = "Bits 12:13 - Port n.11 mode bits"]
    #[inline]
    pub fn mode11(&mut self) -> _MODE11W {
        _MODE11W { w: self }
    }
    #[doc = "Bits 14:15 - Port n.11 configuration bits"]
    #[inline]
    pub fn cnf11(&mut self) -> _CNF11W {
        _CNF11W { w: self }
    }
    #[doc = "Bits 16:17 - Port n.12 mode bits"]
    #[inline]
    pub fn mode12(&mut self) -> _MODE12W {
        _MODE12W { w: self }
    }
    #[doc = "Bits 18:19 - Port n.12 configuration bits"]
    #[inline]
    pub fn cnf12(&mut self) -> _CNF12W {
        _CNF12W { w: self }
    }
    #[doc = "Bits 20:21 - Port n.13 mode bits"]
    #[inline]
    pub fn mode13(&mut self) -> _MODE13W {
        _MODE13W { w: self }
    }
    #[doc = "Bits 22:23 - Port n.13 configuration bits"]
    #[inline]
    pub fn cnf13(&mut self) -> _CNF13W {
        _CNF13W { w: self }
    }
    #[doc = "Bits 24:25 - Port n.14 mode bits"]
    #[inline]
    pub fn mode14(&mut self) -> _MODE14W {
        _MODE14W { w: self }
    }
    #[doc = "Bits 26:27 - Port n.14 configuration bits"]
    #[inline]
    pub fn cnf14(&mut self) -> _CNF14W {
        _CNF14W { w: self }
    }
    #[doc = "Bits 28:29 - Port n.15 mode bits"]
    #[inline]
    pub fn mode15(&mut self) -> _MODE15W {
        _MODE15W { w: self }
    }
    #[doc = "Bits 30:31 - Port n.15 configuration bits"]
    #[inline]
    pub fn cnf15(&mut self) -> _CNF15W {
        _CNF15W { w: self }
    }
}
