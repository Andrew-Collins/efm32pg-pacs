#[doc = "Register `EXTIPSELL` reader"]
pub struct R(crate::R<EXTIPSELL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTIPSELL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTIPSELL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTIPSELL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTIPSELL` writer"]
pub struct W(crate::W<EXTIPSELL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTIPSELL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<EXTIPSELL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTIPSELL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTIPSEL0` reader - External Interrupt 0 Port Select"]
pub type EXTIPSEL0_R = crate::FieldReader<u8, EXTIPSEL0_A>;
#[doc = "External Interrupt 0 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL0_A {
    #[doc = "0: Port A group selected for external interrupt 0"]
    PORTA = 0,
    #[doc = "1: Port B group selected for external interrupt 0"]
    PORTB = 1,
    #[doc = "2: Port C group selected for external interrupt 0"]
    PORTC = 2,
    #[doc = "3: Port D group selected for external interrupt 0"]
    PORTD = 3,
    #[doc = "5: Port F group selected for external interrupt 0"]
    PORTF = 5,
    #[doc = "8: Port I group selected for external interrupt 0"]
    PORTI = 8,
    #[doc = "9: Port J group selected for external interrupt 0"]
    PORTJ = 9,
    #[doc = "10: Port K group selected for external interrupt 0"]
    PORTK = 10,
}
impl From<EXTIPSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL0_A) -> Self {
        variant as _
    }
}
impl EXTIPSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTIPSEL0_A> {
        match self.bits {
            0 => Some(EXTIPSEL0_A::PORTA),
            1 => Some(EXTIPSEL0_A::PORTB),
            2 => Some(EXTIPSEL0_A::PORTC),
            3 => Some(EXTIPSEL0_A::PORTD),
            5 => Some(EXTIPSEL0_A::PORTF),
            8 => Some(EXTIPSEL0_A::PORTI),
            9 => Some(EXTIPSEL0_A::PORTJ),
            10 => Some(EXTIPSEL0_A::PORTK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL0_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL0_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL0_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL0_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL0_A::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline(always)]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL0_A::PORTI
    }
    #[doc = "Checks if the value of the field is `PORTJ`"]
    #[inline(always)]
    pub fn is_portj(&self) -> bool {
        *self == EXTIPSEL0_A::PORTJ
    }
    #[doc = "Checks if the value of the field is `PORTK`"]
    #[inline(always)]
    pub fn is_portk(&self) -> bool {
        *self == EXTIPSEL0_A::PORTK
    }
}
#[doc = "Field `EXTIPSEL0` writer - External Interrupt 0 Port Select"]
pub type EXTIPSEL0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXTIPSELL_SPEC, u8, EXTIPSEL0_A, 4, O>;
impl<'a, const O: u8> EXTIPSEL0_W<'a, O> {
    #[doc = "Port A group selected for external interrupt 0"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL0_A::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 0"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL0_A::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 0"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL0_A::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 0"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL0_A::PORTD)
    }
    #[doc = "Port F group selected for external interrupt 0"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL0_A::PORTF)
    }
    #[doc = "Port I group selected for external interrupt 0"]
    #[inline(always)]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL0_A::PORTI)
    }
    #[doc = "Port J group selected for external interrupt 0"]
    #[inline(always)]
    pub fn portj(self) -> &'a mut W {
        self.variant(EXTIPSEL0_A::PORTJ)
    }
    #[doc = "Port K group selected for external interrupt 0"]
    #[inline(always)]
    pub fn portk(self) -> &'a mut W {
        self.variant(EXTIPSEL0_A::PORTK)
    }
}
#[doc = "Field `EXTIPSEL1` reader - External Interrupt 1 Port Select"]
pub type EXTIPSEL1_R = crate::FieldReader<u8, EXTIPSEL1_A>;
#[doc = "External Interrupt 1 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL1_A {
    #[doc = "0: Port A group selected for external interrupt 1"]
    PORTA = 0,
    #[doc = "1: Port B group selected for external interrupt 1"]
    PORTB = 1,
    #[doc = "2: Port C group selected for external interrupt 1"]
    PORTC = 2,
    #[doc = "3: Port D group selected for external interrupt 1"]
    PORTD = 3,
    #[doc = "5: Port F group selected for external interrupt 1"]
    PORTF = 5,
    #[doc = "8: Port I group selected for external interrupt 1"]
    PORTI = 8,
    #[doc = "9: Port J group selected for external interrupt 1"]
    PORTJ = 9,
    #[doc = "10: Port K group selected for external interrupt 1"]
    PORTK = 10,
}
impl From<EXTIPSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL1_A) -> Self {
        variant as _
    }
}
impl EXTIPSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTIPSEL1_A> {
        match self.bits {
            0 => Some(EXTIPSEL1_A::PORTA),
            1 => Some(EXTIPSEL1_A::PORTB),
            2 => Some(EXTIPSEL1_A::PORTC),
            3 => Some(EXTIPSEL1_A::PORTD),
            5 => Some(EXTIPSEL1_A::PORTF),
            8 => Some(EXTIPSEL1_A::PORTI),
            9 => Some(EXTIPSEL1_A::PORTJ),
            10 => Some(EXTIPSEL1_A::PORTK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL1_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL1_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL1_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL1_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL1_A::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline(always)]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL1_A::PORTI
    }
    #[doc = "Checks if the value of the field is `PORTJ`"]
    #[inline(always)]
    pub fn is_portj(&self) -> bool {
        *self == EXTIPSEL1_A::PORTJ
    }
    #[doc = "Checks if the value of the field is `PORTK`"]
    #[inline(always)]
    pub fn is_portk(&self) -> bool {
        *self == EXTIPSEL1_A::PORTK
    }
}
#[doc = "Field `EXTIPSEL1` writer - External Interrupt 1 Port Select"]
pub type EXTIPSEL1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXTIPSELL_SPEC, u8, EXTIPSEL1_A, 4, O>;
impl<'a, const O: u8> EXTIPSEL1_W<'a, O> {
    #[doc = "Port A group selected for external interrupt 1"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL1_A::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 1"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL1_A::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 1"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL1_A::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 1"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL1_A::PORTD)
    }
    #[doc = "Port F group selected for external interrupt 1"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL1_A::PORTF)
    }
    #[doc = "Port I group selected for external interrupt 1"]
    #[inline(always)]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL1_A::PORTI)
    }
    #[doc = "Port J group selected for external interrupt 1"]
    #[inline(always)]
    pub fn portj(self) -> &'a mut W {
        self.variant(EXTIPSEL1_A::PORTJ)
    }
    #[doc = "Port K group selected for external interrupt 1"]
    #[inline(always)]
    pub fn portk(self) -> &'a mut W {
        self.variant(EXTIPSEL1_A::PORTK)
    }
}
#[doc = "Field `EXTIPSEL2` reader - External Interrupt 2 Port Select"]
pub type EXTIPSEL2_R = crate::FieldReader<u8, EXTIPSEL2_A>;
#[doc = "External Interrupt 2 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL2_A {
    #[doc = "0: Port A group selected for external interrupt 2"]
    PORTA = 0,
    #[doc = "1: Port B group selected for external interrupt 2"]
    PORTB = 1,
    #[doc = "2: Port C group selected for external interrupt 2"]
    PORTC = 2,
    #[doc = "3: Port D group selected for external interrupt 2"]
    PORTD = 3,
    #[doc = "5: Port F group selected for external interrupt 2"]
    PORTF = 5,
    #[doc = "8: Port I group selected for external interrupt 2"]
    PORTI = 8,
    #[doc = "9: Port J group selected for external interrupt 2"]
    PORTJ = 9,
    #[doc = "10: Port K group selected for external interrupt 2"]
    PORTK = 10,
}
impl From<EXTIPSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL2_A) -> Self {
        variant as _
    }
}
impl EXTIPSEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTIPSEL2_A> {
        match self.bits {
            0 => Some(EXTIPSEL2_A::PORTA),
            1 => Some(EXTIPSEL2_A::PORTB),
            2 => Some(EXTIPSEL2_A::PORTC),
            3 => Some(EXTIPSEL2_A::PORTD),
            5 => Some(EXTIPSEL2_A::PORTF),
            8 => Some(EXTIPSEL2_A::PORTI),
            9 => Some(EXTIPSEL2_A::PORTJ),
            10 => Some(EXTIPSEL2_A::PORTK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL2_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL2_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL2_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL2_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL2_A::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline(always)]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL2_A::PORTI
    }
    #[doc = "Checks if the value of the field is `PORTJ`"]
    #[inline(always)]
    pub fn is_portj(&self) -> bool {
        *self == EXTIPSEL2_A::PORTJ
    }
    #[doc = "Checks if the value of the field is `PORTK`"]
    #[inline(always)]
    pub fn is_portk(&self) -> bool {
        *self == EXTIPSEL2_A::PORTK
    }
}
#[doc = "Field `EXTIPSEL2` writer - External Interrupt 2 Port Select"]
pub type EXTIPSEL2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXTIPSELL_SPEC, u8, EXTIPSEL2_A, 4, O>;
impl<'a, const O: u8> EXTIPSEL2_W<'a, O> {
    #[doc = "Port A group selected for external interrupt 2"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL2_A::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 2"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL2_A::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 2"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL2_A::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 2"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL2_A::PORTD)
    }
    #[doc = "Port F group selected for external interrupt 2"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL2_A::PORTF)
    }
    #[doc = "Port I group selected for external interrupt 2"]
    #[inline(always)]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL2_A::PORTI)
    }
    #[doc = "Port J group selected for external interrupt 2"]
    #[inline(always)]
    pub fn portj(self) -> &'a mut W {
        self.variant(EXTIPSEL2_A::PORTJ)
    }
    #[doc = "Port K group selected for external interrupt 2"]
    #[inline(always)]
    pub fn portk(self) -> &'a mut W {
        self.variant(EXTIPSEL2_A::PORTK)
    }
}
#[doc = "Field `EXTIPSEL3` reader - External Interrupt 3 Port Select"]
pub type EXTIPSEL3_R = crate::FieldReader<u8, EXTIPSEL3_A>;
#[doc = "External Interrupt 3 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL3_A {
    #[doc = "0: Port A group selected for external interrupt 3"]
    PORTA = 0,
    #[doc = "1: Port B group selected for external interrupt 3"]
    PORTB = 1,
    #[doc = "2: Port C group selected for external interrupt 3"]
    PORTC = 2,
    #[doc = "3: Port D group selected for external interrupt 3"]
    PORTD = 3,
    #[doc = "5: Port F group selected for external interrupt 3"]
    PORTF = 5,
    #[doc = "8: Port I group selected for external interrupt 3"]
    PORTI = 8,
    #[doc = "9: Port J group selected for external interrupt 3"]
    PORTJ = 9,
    #[doc = "10: Port K group selected for external interrupt 3"]
    PORTK = 10,
}
impl From<EXTIPSEL3_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL3_A) -> Self {
        variant as _
    }
}
impl EXTIPSEL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTIPSEL3_A> {
        match self.bits {
            0 => Some(EXTIPSEL3_A::PORTA),
            1 => Some(EXTIPSEL3_A::PORTB),
            2 => Some(EXTIPSEL3_A::PORTC),
            3 => Some(EXTIPSEL3_A::PORTD),
            5 => Some(EXTIPSEL3_A::PORTF),
            8 => Some(EXTIPSEL3_A::PORTI),
            9 => Some(EXTIPSEL3_A::PORTJ),
            10 => Some(EXTIPSEL3_A::PORTK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL3_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL3_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL3_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL3_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL3_A::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline(always)]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL3_A::PORTI
    }
    #[doc = "Checks if the value of the field is `PORTJ`"]
    #[inline(always)]
    pub fn is_portj(&self) -> bool {
        *self == EXTIPSEL3_A::PORTJ
    }
    #[doc = "Checks if the value of the field is `PORTK`"]
    #[inline(always)]
    pub fn is_portk(&self) -> bool {
        *self == EXTIPSEL3_A::PORTK
    }
}
#[doc = "Field `EXTIPSEL3` writer - External Interrupt 3 Port Select"]
pub type EXTIPSEL3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXTIPSELL_SPEC, u8, EXTIPSEL3_A, 4, O>;
impl<'a, const O: u8> EXTIPSEL3_W<'a, O> {
    #[doc = "Port A group selected for external interrupt 3"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL3_A::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 3"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL3_A::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 3"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL3_A::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 3"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL3_A::PORTD)
    }
    #[doc = "Port F group selected for external interrupt 3"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL3_A::PORTF)
    }
    #[doc = "Port I group selected for external interrupt 3"]
    #[inline(always)]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL3_A::PORTI)
    }
    #[doc = "Port J group selected for external interrupt 3"]
    #[inline(always)]
    pub fn portj(self) -> &'a mut W {
        self.variant(EXTIPSEL3_A::PORTJ)
    }
    #[doc = "Port K group selected for external interrupt 3"]
    #[inline(always)]
    pub fn portk(self) -> &'a mut W {
        self.variant(EXTIPSEL3_A::PORTK)
    }
}
#[doc = "Field `EXTIPSEL4` reader - External Interrupt 4 Port Select"]
pub type EXTIPSEL4_R = crate::FieldReader<u8, EXTIPSEL4_A>;
#[doc = "External Interrupt 4 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL4_A {
    #[doc = "0: Port A group selected for external interrupt 4"]
    PORTA = 0,
    #[doc = "1: Port B group selected for external interrupt 4"]
    PORTB = 1,
    #[doc = "2: Port C group selected for external interrupt 4"]
    PORTC = 2,
    #[doc = "3: Port D group selected for external interrupt 4"]
    PORTD = 3,
    #[doc = "5: Port F group selected for external interrupt 4"]
    PORTF = 5,
    #[doc = "8: Port I group selected for external interrupt 4"]
    PORTI = 8,
    #[doc = "9: Port J group selected for external interrupt 4"]
    PORTJ = 9,
    #[doc = "10: Port K group selected for external interrupt 4"]
    PORTK = 10,
}
impl From<EXTIPSEL4_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL4_A) -> Self {
        variant as _
    }
}
impl EXTIPSEL4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTIPSEL4_A> {
        match self.bits {
            0 => Some(EXTIPSEL4_A::PORTA),
            1 => Some(EXTIPSEL4_A::PORTB),
            2 => Some(EXTIPSEL4_A::PORTC),
            3 => Some(EXTIPSEL4_A::PORTD),
            5 => Some(EXTIPSEL4_A::PORTF),
            8 => Some(EXTIPSEL4_A::PORTI),
            9 => Some(EXTIPSEL4_A::PORTJ),
            10 => Some(EXTIPSEL4_A::PORTK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL4_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL4_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL4_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL4_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL4_A::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline(always)]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL4_A::PORTI
    }
    #[doc = "Checks if the value of the field is `PORTJ`"]
    #[inline(always)]
    pub fn is_portj(&self) -> bool {
        *self == EXTIPSEL4_A::PORTJ
    }
    #[doc = "Checks if the value of the field is `PORTK`"]
    #[inline(always)]
    pub fn is_portk(&self) -> bool {
        *self == EXTIPSEL4_A::PORTK
    }
}
#[doc = "Field `EXTIPSEL4` writer - External Interrupt 4 Port Select"]
pub type EXTIPSEL4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXTIPSELL_SPEC, u8, EXTIPSEL4_A, 4, O>;
impl<'a, const O: u8> EXTIPSEL4_W<'a, O> {
    #[doc = "Port A group selected for external interrupt 4"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL4_A::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 4"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL4_A::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 4"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL4_A::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 4"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL4_A::PORTD)
    }
    #[doc = "Port F group selected for external interrupt 4"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL4_A::PORTF)
    }
    #[doc = "Port I group selected for external interrupt 4"]
    #[inline(always)]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL4_A::PORTI)
    }
    #[doc = "Port J group selected for external interrupt 4"]
    #[inline(always)]
    pub fn portj(self) -> &'a mut W {
        self.variant(EXTIPSEL4_A::PORTJ)
    }
    #[doc = "Port K group selected for external interrupt 4"]
    #[inline(always)]
    pub fn portk(self) -> &'a mut W {
        self.variant(EXTIPSEL4_A::PORTK)
    }
}
#[doc = "Field `EXTIPSEL5` reader - External Interrupt 5 Port Select"]
pub type EXTIPSEL5_R = crate::FieldReader<u8, EXTIPSEL5_A>;
#[doc = "External Interrupt 5 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL5_A {
    #[doc = "0: Port A group selected for external interrupt 5"]
    PORTA = 0,
    #[doc = "1: Port B group selected for external interrupt 5"]
    PORTB = 1,
    #[doc = "2: Port C group selected for external interrupt 5"]
    PORTC = 2,
    #[doc = "3: Port D group selected for external interrupt 5"]
    PORTD = 3,
    #[doc = "5: Port F group selected for external interrupt 5"]
    PORTF = 5,
    #[doc = "8: Port I group selected for external interrupt 5"]
    PORTI = 8,
    #[doc = "9: Port J group selected for external interrupt 5"]
    PORTJ = 9,
    #[doc = "10: Port K group selected for external interrupt 5"]
    PORTK = 10,
}
impl From<EXTIPSEL5_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL5_A) -> Self {
        variant as _
    }
}
impl EXTIPSEL5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTIPSEL5_A> {
        match self.bits {
            0 => Some(EXTIPSEL5_A::PORTA),
            1 => Some(EXTIPSEL5_A::PORTB),
            2 => Some(EXTIPSEL5_A::PORTC),
            3 => Some(EXTIPSEL5_A::PORTD),
            5 => Some(EXTIPSEL5_A::PORTF),
            8 => Some(EXTIPSEL5_A::PORTI),
            9 => Some(EXTIPSEL5_A::PORTJ),
            10 => Some(EXTIPSEL5_A::PORTK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL5_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL5_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL5_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL5_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL5_A::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline(always)]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL5_A::PORTI
    }
    #[doc = "Checks if the value of the field is `PORTJ`"]
    #[inline(always)]
    pub fn is_portj(&self) -> bool {
        *self == EXTIPSEL5_A::PORTJ
    }
    #[doc = "Checks if the value of the field is `PORTK`"]
    #[inline(always)]
    pub fn is_portk(&self) -> bool {
        *self == EXTIPSEL5_A::PORTK
    }
}
#[doc = "Field `EXTIPSEL5` writer - External Interrupt 5 Port Select"]
pub type EXTIPSEL5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXTIPSELL_SPEC, u8, EXTIPSEL5_A, 4, O>;
impl<'a, const O: u8> EXTIPSEL5_W<'a, O> {
    #[doc = "Port A group selected for external interrupt 5"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL5_A::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 5"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL5_A::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 5"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL5_A::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 5"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL5_A::PORTD)
    }
    #[doc = "Port F group selected for external interrupt 5"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL5_A::PORTF)
    }
    #[doc = "Port I group selected for external interrupt 5"]
    #[inline(always)]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL5_A::PORTI)
    }
    #[doc = "Port J group selected for external interrupt 5"]
    #[inline(always)]
    pub fn portj(self) -> &'a mut W {
        self.variant(EXTIPSEL5_A::PORTJ)
    }
    #[doc = "Port K group selected for external interrupt 5"]
    #[inline(always)]
    pub fn portk(self) -> &'a mut W {
        self.variant(EXTIPSEL5_A::PORTK)
    }
}
#[doc = "Field `EXTIPSEL6` reader - External Interrupt 6 Port Select"]
pub type EXTIPSEL6_R = crate::FieldReader<u8, EXTIPSEL6_A>;
#[doc = "External Interrupt 6 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL6_A {
    #[doc = "0: Port A group selected for external interrupt 6"]
    PORTA = 0,
    #[doc = "1: Port B group selected for external interrupt 6"]
    PORTB = 1,
    #[doc = "2: Port C group selected for external interrupt 6"]
    PORTC = 2,
    #[doc = "3: Port D group selected for external interrupt 6"]
    PORTD = 3,
    #[doc = "5: Port F group selected for external interrupt 6"]
    PORTF = 5,
    #[doc = "8: Port I group selected for external interrupt 6"]
    PORTI = 8,
    #[doc = "9: Port J group selected for external interrupt 6"]
    PORTJ = 9,
    #[doc = "10: Port K group selected for external interrupt 6"]
    PORTK = 10,
}
impl From<EXTIPSEL6_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL6_A) -> Self {
        variant as _
    }
}
impl EXTIPSEL6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTIPSEL6_A> {
        match self.bits {
            0 => Some(EXTIPSEL6_A::PORTA),
            1 => Some(EXTIPSEL6_A::PORTB),
            2 => Some(EXTIPSEL6_A::PORTC),
            3 => Some(EXTIPSEL6_A::PORTD),
            5 => Some(EXTIPSEL6_A::PORTF),
            8 => Some(EXTIPSEL6_A::PORTI),
            9 => Some(EXTIPSEL6_A::PORTJ),
            10 => Some(EXTIPSEL6_A::PORTK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL6_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL6_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL6_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL6_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL6_A::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline(always)]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL6_A::PORTI
    }
    #[doc = "Checks if the value of the field is `PORTJ`"]
    #[inline(always)]
    pub fn is_portj(&self) -> bool {
        *self == EXTIPSEL6_A::PORTJ
    }
    #[doc = "Checks if the value of the field is `PORTK`"]
    #[inline(always)]
    pub fn is_portk(&self) -> bool {
        *self == EXTIPSEL6_A::PORTK
    }
}
#[doc = "Field `EXTIPSEL6` writer - External Interrupt 6 Port Select"]
pub type EXTIPSEL6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXTIPSELL_SPEC, u8, EXTIPSEL6_A, 4, O>;
impl<'a, const O: u8> EXTIPSEL6_W<'a, O> {
    #[doc = "Port A group selected for external interrupt 6"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL6_A::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 6"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL6_A::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 6"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL6_A::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 6"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL6_A::PORTD)
    }
    #[doc = "Port F group selected for external interrupt 6"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL6_A::PORTF)
    }
    #[doc = "Port I group selected for external interrupt 6"]
    #[inline(always)]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL6_A::PORTI)
    }
    #[doc = "Port J group selected for external interrupt 6"]
    #[inline(always)]
    pub fn portj(self) -> &'a mut W {
        self.variant(EXTIPSEL6_A::PORTJ)
    }
    #[doc = "Port K group selected for external interrupt 6"]
    #[inline(always)]
    pub fn portk(self) -> &'a mut W {
        self.variant(EXTIPSEL6_A::PORTK)
    }
}
#[doc = "Field `EXTIPSEL7` reader - External Interrupt 7 Port Select"]
pub type EXTIPSEL7_R = crate::FieldReader<u8, EXTIPSEL7_A>;
#[doc = "External Interrupt 7 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL7_A {
    #[doc = "0: Port A group selected for external interrupt 7"]
    PORTA = 0,
    #[doc = "1: Port B group selected for external interrupt 7"]
    PORTB = 1,
    #[doc = "2: Port C group selected for external interrupt 7"]
    PORTC = 2,
    #[doc = "3: Port D group selected for external interrupt 7"]
    PORTD = 3,
    #[doc = "5: Port F group selected for external interrupt 7"]
    PORTF = 5,
    #[doc = "8: Port I group selected for external interrupt 7"]
    PORTI = 8,
    #[doc = "9: Port J group selected for external interrupt 7"]
    PORTJ = 9,
    #[doc = "10: Port K group selected for external interrupt 7"]
    PORTK = 10,
}
impl From<EXTIPSEL7_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL7_A) -> Self {
        variant as _
    }
}
impl EXTIPSEL7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTIPSEL7_A> {
        match self.bits {
            0 => Some(EXTIPSEL7_A::PORTA),
            1 => Some(EXTIPSEL7_A::PORTB),
            2 => Some(EXTIPSEL7_A::PORTC),
            3 => Some(EXTIPSEL7_A::PORTD),
            5 => Some(EXTIPSEL7_A::PORTF),
            8 => Some(EXTIPSEL7_A::PORTI),
            9 => Some(EXTIPSEL7_A::PORTJ),
            10 => Some(EXTIPSEL7_A::PORTK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL7_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL7_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL7_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL7_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL7_A::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline(always)]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL7_A::PORTI
    }
    #[doc = "Checks if the value of the field is `PORTJ`"]
    #[inline(always)]
    pub fn is_portj(&self) -> bool {
        *self == EXTIPSEL7_A::PORTJ
    }
    #[doc = "Checks if the value of the field is `PORTK`"]
    #[inline(always)]
    pub fn is_portk(&self) -> bool {
        *self == EXTIPSEL7_A::PORTK
    }
}
#[doc = "Field `EXTIPSEL7` writer - External Interrupt 7 Port Select"]
pub type EXTIPSEL7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXTIPSELL_SPEC, u8, EXTIPSEL7_A, 4, O>;
impl<'a, const O: u8> EXTIPSEL7_W<'a, O> {
    #[doc = "Port A group selected for external interrupt 7"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL7_A::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 7"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL7_A::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 7"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL7_A::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 7"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL7_A::PORTD)
    }
    #[doc = "Port F group selected for external interrupt 7"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL7_A::PORTF)
    }
    #[doc = "Port I group selected for external interrupt 7"]
    #[inline(always)]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL7_A::PORTI)
    }
    #[doc = "Port J group selected for external interrupt 7"]
    #[inline(always)]
    pub fn portj(self) -> &'a mut W {
        self.variant(EXTIPSEL7_A::PORTJ)
    }
    #[doc = "Port K group selected for external interrupt 7"]
    #[inline(always)]
    pub fn portk(self) -> &'a mut W {
        self.variant(EXTIPSEL7_A::PORTK)
    }
}
impl R {
    #[doc = "Bits 0:3 - External Interrupt 0 Port Select"]
    #[inline(always)]
    pub fn extipsel0(&self) -> EXTIPSEL0_R {
        EXTIPSEL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - External Interrupt 1 Port Select"]
    #[inline(always)]
    pub fn extipsel1(&self) -> EXTIPSEL1_R {
        EXTIPSEL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - External Interrupt 2 Port Select"]
    #[inline(always)]
    pub fn extipsel2(&self) -> EXTIPSEL2_R {
        EXTIPSEL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - External Interrupt 3 Port Select"]
    #[inline(always)]
    pub fn extipsel3(&self) -> EXTIPSEL3_R {
        EXTIPSEL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - External Interrupt 4 Port Select"]
    #[inline(always)]
    pub fn extipsel4(&self) -> EXTIPSEL4_R {
        EXTIPSEL4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - External Interrupt 5 Port Select"]
    #[inline(always)]
    pub fn extipsel5(&self) -> EXTIPSEL5_R {
        EXTIPSEL5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - External Interrupt 6 Port Select"]
    #[inline(always)]
    pub fn extipsel6(&self) -> EXTIPSEL6_R {
        EXTIPSEL6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - External Interrupt 7 Port Select"]
    #[inline(always)]
    pub fn extipsel7(&self) -> EXTIPSEL7_R {
        EXTIPSEL7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - External Interrupt 0 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel0(&mut self) -> EXTIPSEL0_W<0> {
        EXTIPSEL0_W::new(self)
    }
    #[doc = "Bits 4:7 - External Interrupt 1 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel1(&mut self) -> EXTIPSEL1_W<4> {
        EXTIPSEL1_W::new(self)
    }
    #[doc = "Bits 8:11 - External Interrupt 2 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel2(&mut self) -> EXTIPSEL2_W<8> {
        EXTIPSEL2_W::new(self)
    }
    #[doc = "Bits 12:15 - External Interrupt 3 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel3(&mut self) -> EXTIPSEL3_W<12> {
        EXTIPSEL3_W::new(self)
    }
    #[doc = "Bits 16:19 - External Interrupt 4 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel4(&mut self) -> EXTIPSEL4_W<16> {
        EXTIPSEL4_W::new(self)
    }
    #[doc = "Bits 20:23 - External Interrupt 5 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel5(&mut self) -> EXTIPSEL5_W<20> {
        EXTIPSEL5_W::new(self)
    }
    #[doc = "Bits 24:27 - External Interrupt 6 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel6(&mut self) -> EXTIPSEL6_W<24> {
        EXTIPSEL6_W::new(self)
    }
    #[doc = "Bits 28:31 - External Interrupt 7 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel7(&mut self) -> EXTIPSEL7_W<28> {
        EXTIPSEL7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Interrupt Port Select Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extipsell](index.html) module"]
pub struct EXTIPSELL_SPEC;
impl crate::RegisterSpec for EXTIPSELL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extipsell::R](R) reader structure"]
impl crate::Readable for EXTIPSELL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extipsell::W](W) writer structure"]
impl crate::Writable for EXTIPSELL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTIPSELL to value 0"]
impl crate::Resettable for EXTIPSELL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
