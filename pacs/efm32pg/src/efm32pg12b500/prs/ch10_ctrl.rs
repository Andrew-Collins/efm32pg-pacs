#[doc = "Register `CH10_CTRL` reader"]
pub struct R(crate::R<CH10_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH10_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH10_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH10_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH10_CTRL` writer"]
pub struct W(crate::W<CH10_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH10_CTRL_SPEC>;
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
impl From<crate::W<CH10_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH10_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIGSEL` reader - Signal Select"]
pub type SIGSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIGSEL` writer - Signal Select"]
pub type SIGSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH10_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `SOURCESEL` reader - Source Select"]
pub type SOURCESEL_R = crate::FieldReader<u8, SOURCESEL_A>;
#[doc = "Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SOURCESEL_A {
    #[doc = "0: No source selected"]
    NONE = 0,
    #[doc = "1: Peripheral Reflex System"]
    PRSL = 1,
    #[doc = "2: Peripheral Reflex System"]
    PRSH = 2,
    #[doc = "3: Analog Comparator 0"]
    ACMP0 = 3,
    #[doc = "4: Analog Comparator 1"]
    ACMP1 = 4,
    #[doc = "5: Analog to Digital Converter 0"]
    ADC0 = 5,
    #[doc = "7: Low Energy Sensor Interface"]
    LESENSEL = 7,
    #[doc = "8: Low Energy Sensor Interface"]
    LESENSEH = 8,
    #[doc = "9: Low Energy Sensor Interface"]
    LESENSED = 9,
    #[doc = "10: Low Energy Sensor Interface"]
    LESENSE = 10,
    #[doc = "11: Real-Time Counter and Calendar"]
    RTCC = 11,
    #[doc = "12: General purpose Input/Output"]
    GPIOL = 12,
    #[doc = "13: General purpose Input/Output"]
    GPIOH = 13,
    #[doc = "14: Low Energy Timer 0"]
    LETIMER0 = 14,
    #[doc = "15: Pulse Counter 0"]
    PCNT0 = 15,
    #[doc = "16: Pulse Counter 1"]
    PCNT1 = 16,
    #[doc = "17: Pulse Counter 2"]
    PCNT2 = 17,
    #[doc = "18: Clock Management Unit"]
    CMU = 18,
    #[doc = "24: Digital to Analog Converter 0"]
    VDAC0 = 24,
    #[doc = "26: CRYOTIMER"]
    CRYOTIMER = 26,
    #[doc = "48: Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    USART0 = 48,
    #[doc = "49: Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    USART1 = 49,
    #[doc = "50: Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    USART2 = 50,
    #[doc = "51: Universal Synchronous/Asynchronous Receiver/Transmitter 3"]
    USART3 = 51,
    #[doc = "60: Timer 0"]
    TIMER0 = 60,
    #[doc = "61: Timer 1"]
    TIMER1 = 61,
    #[doc = "62: Wide Timer 0"]
    WTIMER0 = 62,
    #[doc = "63: Wide Timer 1"]
    WTIMER1 = 63,
    #[doc = "67: `1000011`"]
    CM4 = 67,
}
impl From<SOURCESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SOURCESEL_A) -> Self {
        variant as _
    }
}
impl SOURCESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SOURCESEL_A> {
        match self.bits {
            0 => Some(SOURCESEL_A::NONE),
            1 => Some(SOURCESEL_A::PRSL),
            2 => Some(SOURCESEL_A::PRSH),
            3 => Some(SOURCESEL_A::ACMP0),
            4 => Some(SOURCESEL_A::ACMP1),
            5 => Some(SOURCESEL_A::ADC0),
            7 => Some(SOURCESEL_A::LESENSEL),
            8 => Some(SOURCESEL_A::LESENSEH),
            9 => Some(SOURCESEL_A::LESENSED),
            10 => Some(SOURCESEL_A::LESENSE),
            11 => Some(SOURCESEL_A::RTCC),
            12 => Some(SOURCESEL_A::GPIOL),
            13 => Some(SOURCESEL_A::GPIOH),
            14 => Some(SOURCESEL_A::LETIMER0),
            15 => Some(SOURCESEL_A::PCNT0),
            16 => Some(SOURCESEL_A::PCNT1),
            17 => Some(SOURCESEL_A::PCNT2),
            18 => Some(SOURCESEL_A::CMU),
            24 => Some(SOURCESEL_A::VDAC0),
            26 => Some(SOURCESEL_A::CRYOTIMER),
            48 => Some(SOURCESEL_A::USART0),
            49 => Some(SOURCESEL_A::USART1),
            50 => Some(SOURCESEL_A::USART2),
            51 => Some(SOURCESEL_A::USART3),
            60 => Some(SOURCESEL_A::TIMER0),
            61 => Some(SOURCESEL_A::TIMER1),
            62 => Some(SOURCESEL_A::WTIMER0),
            63 => Some(SOURCESEL_A::WTIMER1),
            67 => Some(SOURCESEL_A::CM4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SOURCESEL_A::NONE
    }
    #[doc = "Checks if the value of the field is `PRSL`"]
    #[inline(always)]
    pub fn is_prsl(&self) -> bool {
        *self == SOURCESEL_A::PRSL
    }
    #[doc = "Checks if the value of the field is `PRSH`"]
    #[inline(always)]
    pub fn is_prsh(&self) -> bool {
        *self == SOURCESEL_A::PRSH
    }
    #[doc = "Checks if the value of the field is `ACMP0`"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == SOURCESEL_A::ACMP0
    }
    #[doc = "Checks if the value of the field is `ACMP1`"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == SOURCESEL_A::ACMP1
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == SOURCESEL_A::ADC0
    }
    #[doc = "Checks if the value of the field is `LESENSEL`"]
    #[inline(always)]
    pub fn is_lesensel(&self) -> bool {
        *self == SOURCESEL_A::LESENSEL
    }
    #[doc = "Checks if the value of the field is `LESENSEH`"]
    #[inline(always)]
    pub fn is_lesenseh(&self) -> bool {
        *self == SOURCESEL_A::LESENSEH
    }
    #[doc = "Checks if the value of the field is `LESENSED`"]
    #[inline(always)]
    pub fn is_lesensed(&self) -> bool {
        *self == SOURCESEL_A::LESENSED
    }
    #[doc = "Checks if the value of the field is `LESENSE`"]
    #[inline(always)]
    pub fn is_lesense(&self) -> bool {
        *self == SOURCESEL_A::LESENSE
    }
    #[doc = "Checks if the value of the field is `RTCC`"]
    #[inline(always)]
    pub fn is_rtcc(&self) -> bool {
        *self == SOURCESEL_A::RTCC
    }
    #[doc = "Checks if the value of the field is `GPIOL`"]
    #[inline(always)]
    pub fn is_gpiol(&self) -> bool {
        *self == SOURCESEL_A::GPIOL
    }
    #[doc = "Checks if the value of the field is `GPIOH`"]
    #[inline(always)]
    pub fn is_gpioh(&self) -> bool {
        *self == SOURCESEL_A::GPIOH
    }
    #[doc = "Checks if the value of the field is `LETIMER0`"]
    #[inline(always)]
    pub fn is_letimer0(&self) -> bool {
        *self == SOURCESEL_A::LETIMER0
    }
    #[doc = "Checks if the value of the field is `PCNT0`"]
    #[inline(always)]
    pub fn is_pcnt0(&self) -> bool {
        *self == SOURCESEL_A::PCNT0
    }
    #[doc = "Checks if the value of the field is `PCNT1`"]
    #[inline(always)]
    pub fn is_pcnt1(&self) -> bool {
        *self == SOURCESEL_A::PCNT1
    }
    #[doc = "Checks if the value of the field is `PCNT2`"]
    #[inline(always)]
    pub fn is_pcnt2(&self) -> bool {
        *self == SOURCESEL_A::PCNT2
    }
    #[doc = "Checks if the value of the field is `CMU`"]
    #[inline(always)]
    pub fn is_cmu(&self) -> bool {
        *self == SOURCESEL_A::CMU
    }
    #[doc = "Checks if the value of the field is `VDAC0`"]
    #[inline(always)]
    pub fn is_vdac0(&self) -> bool {
        *self == SOURCESEL_A::VDAC0
    }
    #[doc = "Checks if the value of the field is `CRYOTIMER`"]
    #[inline(always)]
    pub fn is_cryotimer(&self) -> bool {
        *self == SOURCESEL_A::CRYOTIMER
    }
    #[doc = "Checks if the value of the field is `USART0`"]
    #[inline(always)]
    pub fn is_usart0(&self) -> bool {
        *self == SOURCESEL_A::USART0
    }
    #[doc = "Checks if the value of the field is `USART1`"]
    #[inline(always)]
    pub fn is_usart1(&self) -> bool {
        *self == SOURCESEL_A::USART1
    }
    #[doc = "Checks if the value of the field is `USART2`"]
    #[inline(always)]
    pub fn is_usart2(&self) -> bool {
        *self == SOURCESEL_A::USART2
    }
    #[doc = "Checks if the value of the field is `USART3`"]
    #[inline(always)]
    pub fn is_usart3(&self) -> bool {
        *self == SOURCESEL_A::USART3
    }
    #[doc = "Checks if the value of the field is `TIMER0`"]
    #[inline(always)]
    pub fn is_timer0(&self) -> bool {
        *self == SOURCESEL_A::TIMER0
    }
    #[doc = "Checks if the value of the field is `TIMER1`"]
    #[inline(always)]
    pub fn is_timer1(&self) -> bool {
        *self == SOURCESEL_A::TIMER1
    }
    #[doc = "Checks if the value of the field is `WTIMER0`"]
    #[inline(always)]
    pub fn is_wtimer0(&self) -> bool {
        *self == SOURCESEL_A::WTIMER0
    }
    #[doc = "Checks if the value of the field is `WTIMER1`"]
    #[inline(always)]
    pub fn is_wtimer1(&self) -> bool {
        *self == SOURCESEL_A::WTIMER1
    }
    #[doc = "Checks if the value of the field is `CM4`"]
    #[inline(always)]
    pub fn is_cm4(&self) -> bool {
        *self == SOURCESEL_A::CM4
    }
}
#[doc = "Field `SOURCESEL` writer - Source Select"]
pub type SOURCESEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CH10_CTRL_SPEC, u8, SOURCESEL_A, 7, O>;
impl<'a, const O: u8> SOURCESEL_W<'a, O> {
    #[doc = "No source selected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SOURCESEL_A::NONE)
    }
    #[doc = "Peripheral Reflex System"]
    #[inline(always)]
    pub fn prsl(self) -> &'a mut W {
        self.variant(SOURCESEL_A::PRSL)
    }
    #[doc = "Peripheral Reflex System"]
    #[inline(always)]
    pub fn prsh(self) -> &'a mut W {
        self.variant(SOURCESEL_A::PRSH)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::ACMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut W {
        self.variant(SOURCESEL_A::ACMP1)
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::ADC0)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn lesensel(self) -> &'a mut W {
        self.variant(SOURCESEL_A::LESENSEL)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn lesenseh(self) -> &'a mut W {
        self.variant(SOURCESEL_A::LESENSEH)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn lesensed(self) -> &'a mut W {
        self.variant(SOURCESEL_A::LESENSED)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn lesense(self) -> &'a mut W {
        self.variant(SOURCESEL_A::LESENSE)
    }
    #[doc = "Real-Time Counter and Calendar"]
    #[inline(always)]
    pub fn rtcc(self) -> &'a mut W {
        self.variant(SOURCESEL_A::RTCC)
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn gpiol(self) -> &'a mut W {
        self.variant(SOURCESEL_A::GPIOL)
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn gpioh(self) -> &'a mut W {
        self.variant(SOURCESEL_A::GPIOH)
    }
    #[doc = "Low Energy Timer 0"]
    #[inline(always)]
    pub fn letimer0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::LETIMER0)
    }
    #[doc = "Pulse Counter 0"]
    #[inline(always)]
    pub fn pcnt0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::PCNT0)
    }
    #[doc = "Pulse Counter 1"]
    #[inline(always)]
    pub fn pcnt1(self) -> &'a mut W {
        self.variant(SOURCESEL_A::PCNT1)
    }
    #[doc = "Pulse Counter 2"]
    #[inline(always)]
    pub fn pcnt2(self) -> &'a mut W {
        self.variant(SOURCESEL_A::PCNT2)
    }
    #[doc = "Clock Management Unit"]
    #[inline(always)]
    pub fn cmu(self) -> &'a mut W {
        self.variant(SOURCESEL_A::CMU)
    }
    #[doc = "Digital to Analog Converter 0"]
    #[inline(always)]
    pub fn vdac0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::VDAC0)
    }
    #[doc = "CRYOTIMER"]
    #[inline(always)]
    pub fn cryotimer(self) -> &'a mut W {
        self.variant(SOURCESEL_A::CRYOTIMER)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn usart0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::USART0)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn usart1(self) -> &'a mut W {
        self.variant(SOURCESEL_A::USART1)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    #[inline(always)]
    pub fn usart2(self) -> &'a mut W {
        self.variant(SOURCESEL_A::USART2)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3"]
    #[inline(always)]
    pub fn usart3(self) -> &'a mut W {
        self.variant(SOURCESEL_A::USART3)
    }
    #[doc = "Timer 0"]
    #[inline(always)]
    pub fn timer0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::TIMER0)
    }
    #[doc = "Timer 1"]
    #[inline(always)]
    pub fn timer1(self) -> &'a mut W {
        self.variant(SOURCESEL_A::TIMER1)
    }
    #[doc = "Wide Timer 0"]
    #[inline(always)]
    pub fn wtimer0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::WTIMER0)
    }
    #[doc = "Wide Timer 1"]
    #[inline(always)]
    pub fn wtimer1(self) -> &'a mut W {
        self.variant(SOURCESEL_A::WTIMER1)
    }
    #[doc = "`1000011`"]
    #[inline(always)]
    pub fn cm4(self) -> &'a mut W {
        self.variant(SOURCESEL_A::CM4)
    }
}
#[doc = "Field `EDSEL` reader - Edge Detect Select"]
pub type EDSEL_R = crate::FieldReader<u8, EDSEL_A>;
#[doc = "Edge Detect Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDSEL_A {
    #[doc = "0: Signal is left as it is"]
    OFF = 0,
    #[doc = "1: A one HFCLK cycle pulse is generated for every positive edge of the incoming signal"]
    POSEDGE = 1,
    #[doc = "2: A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    NEGEDGE = 2,
    #[doc = "3: A one HFCLK clock cycle pulse is generated for every edge of the incoming signal"]
    BOTHEDGES = 3,
}
impl From<EDSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EDSEL_A) -> Self {
        variant as _
    }
}
impl EDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDSEL_A {
        match self.bits {
            0 => EDSEL_A::OFF,
            1 => EDSEL_A::POSEDGE,
            2 => EDSEL_A::NEGEDGE,
            3 => EDSEL_A::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == EDSEL_A::OFF
    }
    #[doc = "Checks if the value of the field is `POSEDGE`"]
    #[inline(always)]
    pub fn is_posedge(&self) -> bool {
        *self == EDSEL_A::POSEDGE
    }
    #[doc = "Checks if the value of the field is `NEGEDGE`"]
    #[inline(always)]
    pub fn is_negedge(&self) -> bool {
        *self == EDSEL_A::NEGEDGE
    }
    #[doc = "Checks if the value of the field is `BOTHEDGES`"]
    #[inline(always)]
    pub fn is_bothedges(&self) -> bool {
        *self == EDSEL_A::BOTHEDGES
    }
}
#[doc = "Field `EDSEL` writer - Edge Detect Select"]
pub type EDSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CH10_CTRL_SPEC, u8, EDSEL_A, 2, O>;
impl<'a, const O: u8> EDSEL_W<'a, O> {
    #[doc = "Signal is left as it is"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(EDSEL_A::OFF)
    }
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal"]
    #[inline(always)]
    pub fn posedge(self) -> &'a mut W {
        self.variant(EDSEL_A::POSEDGE)
    }
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    #[inline(always)]
    pub fn negedge(self) -> &'a mut W {
        self.variant(EDSEL_A::NEGEDGE)
    }
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal"]
    #[inline(always)]
    pub fn bothedges(self) -> &'a mut W {
        self.variant(EDSEL_A::BOTHEDGES)
    }
}
#[doc = "Field `STRETCH` reader - Stretch Channel Output"]
pub type STRETCH_R = crate::BitReader<bool>;
#[doc = "Field `STRETCH` writer - Stretch Channel Output"]
pub type STRETCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH10_CTRL_SPEC, bool, O>;
#[doc = "Field `INV` reader - Invert Channel"]
pub type INV_R = crate::BitReader<bool>;
#[doc = "Field `INV` writer - Invert Channel"]
pub type INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH10_CTRL_SPEC, bool, O>;
#[doc = "Field `ORPREV` reader - Or Previous"]
pub type ORPREV_R = crate::BitReader<bool>;
#[doc = "Field `ORPREV` writer - Or Previous"]
pub type ORPREV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH10_CTRL_SPEC, bool, O>;
#[doc = "Field `ANDNEXT` reader - And Next"]
pub type ANDNEXT_R = crate::BitReader<bool>;
#[doc = "Field `ANDNEXT` writer - And Next"]
pub type ANDNEXT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH10_CTRL_SPEC, bool, O>;
#[doc = "Field `ASYNC` reader - Asynchronous Reflex"]
pub type ASYNC_R = crate::BitReader<bool>;
#[doc = "Field `ASYNC` writer - Asynchronous Reflex"]
pub type ASYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH10_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&self) -> SIGSEL_R {
        SIGSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:14 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&self) -> SOURCESEL_R {
        SOURCESEL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 20:21 - Edge Detect Select"]
    #[inline(always)]
    pub fn edsel(&self) -> EDSEL_R {
        EDSEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 25 - Stretch Channel Output"]
    #[inline(always)]
    pub fn stretch(&self) -> STRETCH_R {
        STRETCH_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Invert Channel"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Or Previous"]
    #[inline(always)]
    pub fn orprev(&self) -> ORPREV_R {
        ORPREV_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - And Next"]
    #[inline(always)]
    pub fn andnext(&self) -> ANDNEXT_R {
        ANDNEXT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Asynchronous Reflex"]
    #[inline(always)]
    pub fn async_(&self) -> ASYNC_R {
        ASYNC_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    #[must_use]
    pub fn sigsel(&mut self) -> SIGSEL_W<0> {
        SIGSEL_W::new(self)
    }
    #[doc = "Bits 8:14 - Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn sourcesel(&mut self) -> SOURCESEL_W<8> {
        SOURCESEL_W::new(self)
    }
    #[doc = "Bits 20:21 - Edge Detect Select"]
    #[inline(always)]
    #[must_use]
    pub fn edsel(&mut self) -> EDSEL_W<20> {
        EDSEL_W::new(self)
    }
    #[doc = "Bit 25 - Stretch Channel Output"]
    #[inline(always)]
    #[must_use]
    pub fn stretch(&mut self) -> STRETCH_W<25> {
        STRETCH_W::new(self)
    }
    #[doc = "Bit 26 - Invert Channel"]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> INV_W<26> {
        INV_W::new(self)
    }
    #[doc = "Bit 27 - Or Previous"]
    #[inline(always)]
    #[must_use]
    pub fn orprev(&mut self) -> ORPREV_W<27> {
        ORPREV_W::new(self)
    }
    #[doc = "Bit 28 - And Next"]
    #[inline(always)]
    #[must_use]
    pub fn andnext(&mut self) -> ANDNEXT_W<28> {
        ANDNEXT_W::new(self)
    }
    #[doc = "Bit 30 - Asynchronous Reflex"]
    #[inline(always)]
    #[must_use]
    pub fn async_(&mut self) -> ASYNC_W<30> {
        ASYNC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_ctrl](index.html) module"]
pub struct CH10_CTRL_SPEC;
impl crate::RegisterSpec for CH10_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch10_ctrl::R](R) reader structure"]
impl crate::Readable for CH10_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch10_ctrl::W](W) writer structure"]
impl crate::Writable for CH10_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH10_CTRL to value 0"]
impl crate::Resettable for CH10_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
