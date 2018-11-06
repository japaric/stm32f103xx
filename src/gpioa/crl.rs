#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CRL {
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
#[doc = "Possible values of the field `MODE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE0R {
    #[doc = "Input mode"]
    INPUT,
    #[doc = "Output mode 10 MHz"]
    OUTPUT,
    #[doc = "Output mode 2 MHz"]
    OUTPUT2,
    #[doc = "Output mode 50 MHz"]
    OUTPUT50,
}
impl MODE0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODE0R::INPUT => 0,
            MODE0R::OUTPUT => 1,
            MODE0R::OUTPUT2 => 2,
            MODE0R::OUTPUT50 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODE0R {
        match value {
            0 => MODE0R::INPUT,
            1 => MODE0R::OUTPUT,
            2 => MODE0R::OUTPUT2,
            3 => MODE0R::OUTPUT50,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == MODE0R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == MODE0R::OUTPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT2`"]
    #[inline]
    pub fn is_output2(&self) -> bool {
        *self == MODE0R::OUTPUT2
    }
    #[doc = "Checks if the value of the field is `OUTPUT50`"]
    #[inline]
    pub fn is_output50(&self) -> bool {
        *self == MODE0R::OUTPUT50
    }
}
#[doc = "Possible values of the field `CNF0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNF0R {
    #[doc = "Push-Pull mode"]
    PUSH,
    #[doc = "Open Drain-Mode"]
    OPEN,
    #[doc = "Alternate Function Push-Pull Mode"]
    ALTPUSH,
    #[doc = "Alternate Function Open-Drain Mode"]
    ALTOPEN,
}
impl CNF0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CNF0R::PUSH => 0,
            CNF0R::OPEN => 1,
            CNF0R::ALTPUSH => 2,
            CNF0R::ALTOPEN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CNF0R {
        match value {
            0 => CNF0R::PUSH,
            1 => CNF0R::OPEN,
            2 => CNF0R::ALTPUSH,
            3 => CNF0R::ALTOPEN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PUSH`"]
    #[inline]
    pub fn is_push(&self) -> bool {
        *self == CNF0R::PUSH
    }
    #[doc = "Checks if the value of the field is `OPEN`"]
    #[inline]
    pub fn is_open(&self) -> bool {
        *self == CNF0R::OPEN
    }
    #[doc = "Checks if the value of the field is `ALTPUSH`"]
    #[inline]
    pub fn is_alt_push(&self) -> bool {
        *self == CNF0R::ALTPUSH
    }
    #[doc = "Checks if the value of the field is `ALTOPEN`"]
    #[inline]
    pub fn is_alt_open(&self) -> bool {
        *self == CNF0R::ALTOPEN
    }
}
#[doc = "Possible values of the field `MODE1`"]
pub type MODE1R = MODE0R;
#[doc = "Possible values of the field `CNF1`"]
pub type CNF1R = CNF0R;
#[doc = "Possible values of the field `MODE2`"]
pub type MODE2R = MODE0R;
#[doc = "Possible values of the field `CNF2`"]
pub type CNF2R = CNF0R;
#[doc = "Possible values of the field `MODE3`"]
pub type MODE3R = MODE0R;
#[doc = "Possible values of the field `CNF3`"]
pub type CNF3R = CNF0R;
#[doc = "Possible values of the field `MODE4`"]
pub type MODE4R = MODE0R;
#[doc = "Possible values of the field `CNF4`"]
pub type CNF4R = CNF0R;
#[doc = "Possible values of the field `MODE5`"]
pub type MODE5R = MODE0R;
#[doc = "Possible values of the field `CNF5`"]
pub type CNF5R = CNF0R;
#[doc = "Possible values of the field `MODE6`"]
pub type MODE6R = MODE0R;
#[doc = "Possible values of the field `CNF6`"]
pub type CNF6R = CNF0R;
#[doc = "Possible values of the field `MODE7`"]
pub type MODE7R = MODE0R;
#[doc = "Possible values of the field `CNF7`"]
pub type CNF7R = CNF0R;
#[doc = "Values that can be written to the field `MODE0`"]
pub enum MODE0W {
    #[doc = "Input mode"]
    INPUT,
    #[doc = "Output mode 10 MHz"]
    OUTPUT,
    #[doc = "Output mode 2 MHz"]
    OUTPUT2,
    #[doc = "Output mode 50 MHz"]
    OUTPUT50,
}
impl MODE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODE0W::INPUT => 0,
            MODE0W::OUTPUT => 1,
            MODE0W::OUTPUT2 => 2,
            MODE0W::OUTPUT50 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODE0W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE0W::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE0W::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline]
    pub fn output2(self) -> &'a mut W {
        self.variant(MODE0W::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline]
    pub fn output50(self) -> &'a mut W {
        self.variant(MODE0W::OUTPUT50)
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
#[doc = "Values that can be written to the field `CNF0`"]
pub enum CNF0W {
    #[doc = "Push-Pull mode"]
    PUSH,
    #[doc = "Open Drain-Mode"]
    OPEN,
    #[doc = "Alternate Function Push-Pull Mode"]
    ALTPUSH,
    #[doc = "Alternate Function Open-Drain Mode"]
    ALTOPEN,
}
impl CNF0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CNF0W::PUSH => 0,
            CNF0W::OPEN => 1,
            CNF0W::ALTPUSH => 2,
            CNF0W::ALTOPEN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNF0W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Push-Pull mode"]
    #[inline]
    pub fn push(self) -> &'a mut W {
        self.variant(CNF0W::PUSH)
    }
    #[doc = "Open Drain-Mode"]
    #[inline]
    pub fn open(self) -> &'a mut W {
        self.variant(CNF0W::OPEN)
    }
    #[doc = "Alternate Function Push-Pull Mode"]
    #[inline]
    pub fn alt_push(self) -> &'a mut W {
        self.variant(CNF0W::ALTPUSH)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline]
    pub fn alt_open(self) -> &'a mut W {
        self.variant(CNF0W::ALTOPEN)
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
#[doc = "Values that can be written to the field `MODE1`"]
pub type MODE1W = MODE0W;
#[doc = r" Proxy"]
pub struct _MODE1W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE0W::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE0W::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline]
    pub fn output2(self) -> &'a mut W {
        self.variant(MODE0W::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline]
    pub fn output50(self) -> &'a mut W {
        self.variant(MODE0W::OUTPUT50)
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
#[doc = "Values that can be written to the field `CNF1`"]
pub type CNF1W = CNF0W;
#[doc = r" Proxy"]
pub struct _CNF1W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Push-Pull mode"]
    #[inline]
    pub fn push(self) -> &'a mut W {
        self.variant(CNF0W::PUSH)
    }
    #[doc = "Open Drain-Mode"]
    #[inline]
    pub fn open(self) -> &'a mut W {
        self.variant(CNF0W::OPEN)
    }
    #[doc = "Alternate Function Push-Pull Mode"]
    #[inline]
    pub fn alt_push(self) -> &'a mut W {
        self.variant(CNF0W::ALTPUSH)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline]
    pub fn alt_open(self) -> &'a mut W {
        self.variant(CNF0W::ALTOPEN)
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
#[doc = "Values that can be written to the field `MODE2`"]
pub type MODE2W = MODE0W;
#[doc = r" Proxy"]
pub struct _MODE2W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE0W::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE0W::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline]
    pub fn output2(self) -> &'a mut W {
        self.variant(MODE0W::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline]
    pub fn output50(self) -> &'a mut W {
        self.variant(MODE0W::OUTPUT50)
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
#[doc = "Values that can be written to the field `CNF2`"]
pub type CNF2W = CNF0W;
#[doc = r" Proxy"]
pub struct _CNF2W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Push-Pull mode"]
    #[inline]
    pub fn push(self) -> &'a mut W {
        self.variant(CNF0W::PUSH)
    }
    #[doc = "Open Drain-Mode"]
    #[inline]
    pub fn open(self) -> &'a mut W {
        self.variant(CNF0W::OPEN)
    }
    #[doc = "Alternate Function Push-Pull Mode"]
    #[inline]
    pub fn alt_push(self) -> &'a mut W {
        self.variant(CNF0W::ALTPUSH)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline]
    pub fn alt_open(self) -> &'a mut W {
        self.variant(CNF0W::ALTOPEN)
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
#[doc = "Values that can be written to the field `MODE3`"]
pub type MODE3W = MODE0W;
#[doc = r" Proxy"]
pub struct _MODE3W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE0W::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE0W::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline]
    pub fn output2(self) -> &'a mut W {
        self.variant(MODE0W::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline]
    pub fn output50(self) -> &'a mut W {
        self.variant(MODE0W::OUTPUT50)
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
#[doc = "Values that can be written to the field `CNF3`"]
pub type CNF3W = CNF0W;
#[doc = r" Proxy"]
pub struct _CNF3W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Push-Pull mode"]
    #[inline]
    pub fn push(self) -> &'a mut W {
        self.variant(CNF0W::PUSH)
    }
    #[doc = "Open Drain-Mode"]
    #[inline]
    pub fn open(self) -> &'a mut W {
        self.variant(CNF0W::OPEN)
    }
    #[doc = "Alternate Function Push-Pull Mode"]
    #[inline]
    pub fn alt_push(self) -> &'a mut W {
        self.variant(CNF0W::ALTPUSH)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline]
    pub fn alt_open(self) -> &'a mut W {
        self.variant(CNF0W::ALTOPEN)
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
#[doc = "Values that can be written to the field `MODE4`"]
pub type MODE4W = MODE0W;
#[doc = r" Proxy"]
pub struct _MODE4W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE4W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE0W::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE0W::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline]
    pub fn output2(self) -> &'a mut W {
        self.variant(MODE0W::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline]
    pub fn output50(self) -> &'a mut W {
        self.variant(MODE0W::OUTPUT50)
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
#[doc = "Values that can be written to the field `CNF4`"]
pub type CNF4W = CNF0W;
#[doc = r" Proxy"]
pub struct _CNF4W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF4W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Push-Pull mode"]
    #[inline]
    pub fn push(self) -> &'a mut W {
        self.variant(CNF0W::PUSH)
    }
    #[doc = "Open Drain-Mode"]
    #[inline]
    pub fn open(self) -> &'a mut W {
        self.variant(CNF0W::OPEN)
    }
    #[doc = "Alternate Function Push-Pull Mode"]
    #[inline]
    pub fn alt_push(self) -> &'a mut W {
        self.variant(CNF0W::ALTPUSH)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline]
    pub fn alt_open(self) -> &'a mut W {
        self.variant(CNF0W::ALTOPEN)
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
#[doc = "Values that can be written to the field `MODE5`"]
pub type MODE5W = MODE0W;
#[doc = r" Proxy"]
pub struct _MODE5W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE5W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE0W::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE0W::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline]
    pub fn output2(self) -> &'a mut W {
        self.variant(MODE0W::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline]
    pub fn output50(self) -> &'a mut W {
        self.variant(MODE0W::OUTPUT50)
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
#[doc = "Values that can be written to the field `CNF5`"]
pub type CNF5W = CNF0W;
#[doc = r" Proxy"]
pub struct _CNF5W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF5W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Push-Pull mode"]
    #[inline]
    pub fn push(self) -> &'a mut W {
        self.variant(CNF0W::PUSH)
    }
    #[doc = "Open Drain-Mode"]
    #[inline]
    pub fn open(self) -> &'a mut W {
        self.variant(CNF0W::OPEN)
    }
    #[doc = "Alternate Function Push-Pull Mode"]
    #[inline]
    pub fn alt_push(self) -> &'a mut W {
        self.variant(CNF0W::ALTPUSH)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline]
    pub fn alt_open(self) -> &'a mut W {
        self.variant(CNF0W::ALTOPEN)
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
#[doc = "Values that can be written to the field `MODE6`"]
pub type MODE6W = MODE0W;
#[doc = r" Proxy"]
pub struct _MODE6W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE6W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE0W::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE0W::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline]
    pub fn output2(self) -> &'a mut W {
        self.variant(MODE0W::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline]
    pub fn output50(self) -> &'a mut W {
        self.variant(MODE0W::OUTPUT50)
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
#[doc = "Values that can be written to the field `CNF6`"]
pub type CNF6W = CNF0W;
#[doc = r" Proxy"]
pub struct _CNF6W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF6W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Push-Pull mode"]
    #[inline]
    pub fn push(self) -> &'a mut W {
        self.variant(CNF0W::PUSH)
    }
    #[doc = "Open Drain-Mode"]
    #[inline]
    pub fn open(self) -> &'a mut W {
        self.variant(CNF0W::OPEN)
    }
    #[doc = "Alternate Function Push-Pull Mode"]
    #[inline]
    pub fn alt_push(self) -> &'a mut W {
        self.variant(CNF0W::ALTPUSH)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline]
    pub fn alt_open(self) -> &'a mut W {
        self.variant(CNF0W::ALTOPEN)
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
#[doc = "Values that can be written to the field `MODE7`"]
pub type MODE7W = MODE0W;
#[doc = r" Proxy"]
pub struct _MODE7W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE7W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE0W::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE0W::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline]
    pub fn output2(self) -> &'a mut W {
        self.variant(MODE0W::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline]
    pub fn output50(self) -> &'a mut W {
        self.variant(MODE0W::OUTPUT50)
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
#[doc = "Values that can be written to the field `CNF7`"]
pub type CNF7W = CNF0W;
#[doc = r" Proxy"]
pub struct _CNF7W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF7W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Push-Pull mode"]
    #[inline]
    pub fn push(self) -> &'a mut W {
        self.variant(CNF0W::PUSH)
    }
    #[doc = "Open Drain-Mode"]
    #[inline]
    pub fn open(self) -> &'a mut W {
        self.variant(CNF0W::OPEN)
    }
    #[doc = "Alternate Function Push-Pull Mode"]
    #[inline]
    pub fn alt_push(self) -> &'a mut W {
        self.variant(CNF0W::ALTPUSH)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline]
    pub fn alt_open(self) -> &'a mut W {
        self.variant(CNF0W::ALTOPEN)
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
    #[doc = "Bits 0:1 - Port n.0 mode bits"]
    #[inline]
    pub fn mode0(&self) -> MODE0R {
        MODE0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Port n.0 configuration bits"]
    #[inline]
    pub fn cnf0(&self) -> CNF0R {
        CNF0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Port n.1 mode bits"]
    #[inline]
    pub fn mode1(&self) -> MODE1R {
        MODE1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Port n.1 configuration bits"]
    #[inline]
    pub fn cnf1(&self) -> CNF1R {
        CNF1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Port n.2 mode bits"]
    #[inline]
    pub fn mode2(&self) -> MODE2R {
        MODE2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Port n.2 configuration bits"]
    #[inline]
    pub fn cnf2(&self) -> CNF2R {
        CNF2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Port n.3 mode bits"]
    #[inline]
    pub fn mode3(&self) -> MODE3R {
        MODE3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Port n.3 configuration bits"]
    #[inline]
    pub fn cnf3(&self) -> CNF3R {
        CNF3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Port n.4 mode bits"]
    #[inline]
    pub fn mode4(&self) -> MODE4R {
        MODE4R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Port n.4 configuration bits"]
    #[inline]
    pub fn cnf4(&self) -> CNF4R {
        CNF4R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Port n.5 mode bits"]
    #[inline]
    pub fn mode5(&self) -> MODE5R {
        MODE5R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Port n.5 configuration bits"]
    #[inline]
    pub fn cnf5(&self) -> CNF5R {
        CNF5R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Port n.6 mode bits"]
    #[inline]
    pub fn mode6(&self) -> MODE6R {
        MODE6R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - Port n.6 configuration bits"]
    #[inline]
    pub fn cnf6(&self) -> CNF6R {
        CNF6R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - Port n.7 mode bits"]
    #[inline]
    pub fn mode7(&self) -> MODE7R {
        MODE7R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - Port n.7 configuration bits"]
    #[inline]
    pub fn cnf7(&self) -> CNF7R {
        CNF7R::_from({
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
    #[doc = "Bits 0:1 - Port n.0 mode bits"]
    #[inline]
    pub fn mode0(&mut self) -> _MODE0W {
        _MODE0W { w: self }
    }
    #[doc = "Bits 2:3 - Port n.0 configuration bits"]
    #[inline]
    pub fn cnf0(&mut self) -> _CNF0W {
        _CNF0W { w: self }
    }
    #[doc = "Bits 4:5 - Port n.1 mode bits"]
    #[inline]
    pub fn mode1(&mut self) -> _MODE1W {
        _MODE1W { w: self }
    }
    #[doc = "Bits 6:7 - Port n.1 configuration bits"]
    #[inline]
    pub fn cnf1(&mut self) -> _CNF1W {
        _CNF1W { w: self }
    }
    #[doc = "Bits 8:9 - Port n.2 mode bits"]
    #[inline]
    pub fn mode2(&mut self) -> _MODE2W {
        _MODE2W { w: self }
    }
    #[doc = "Bits 10:11 - Port n.2 configuration bits"]
    #[inline]
    pub fn cnf2(&mut self) -> _CNF2W {
        _CNF2W { w: self }
    }
    #[doc = "Bits 12:13 - Port n.3 mode bits"]
    #[inline]
    pub fn mode3(&mut self) -> _MODE3W {
        _MODE3W { w: self }
    }
    #[doc = "Bits 14:15 - Port n.3 configuration bits"]
    #[inline]
    pub fn cnf3(&mut self) -> _CNF3W {
        _CNF3W { w: self }
    }
    #[doc = "Bits 16:17 - Port n.4 mode bits"]
    #[inline]
    pub fn mode4(&mut self) -> _MODE4W {
        _MODE4W { w: self }
    }
    #[doc = "Bits 18:19 - Port n.4 configuration bits"]
    #[inline]
    pub fn cnf4(&mut self) -> _CNF4W {
        _CNF4W { w: self }
    }
    #[doc = "Bits 20:21 - Port n.5 mode bits"]
    #[inline]
    pub fn mode5(&mut self) -> _MODE5W {
        _MODE5W { w: self }
    }
    #[doc = "Bits 22:23 - Port n.5 configuration bits"]
    #[inline]
    pub fn cnf5(&mut self) -> _CNF5W {
        _CNF5W { w: self }
    }
    #[doc = "Bits 24:25 - Port n.6 mode bits"]
    #[inline]
    pub fn mode6(&mut self) -> _MODE6W {
        _MODE6W { w: self }
    }
    #[doc = "Bits 26:27 - Port n.6 configuration bits"]
    #[inline]
    pub fn cnf6(&mut self) -> _CNF6W {
        _CNF6W { w: self }
    }
    #[doc = "Bits 28:29 - Port n.7 mode bits"]
    #[inline]
    pub fn mode7(&mut self) -> _MODE7W {
        _MODE7W { w: self }
    }
    #[doc = "Bits 30:31 - Port n.7 configuration bits"]
    #[inline]
    pub fn cnf7(&mut self) -> _CNF7W {
        _CNF7W { w: self }
    }
}
