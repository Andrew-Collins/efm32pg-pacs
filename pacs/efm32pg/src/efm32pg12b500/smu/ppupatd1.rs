#[doc = "Register `PPUPATD1` reader"]
pub struct R(crate::R<PPUPATD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPUPATD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPUPATD1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPUPATD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPUPATD1` writer"]
pub struct W(crate::W<PPUPATD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPUPATD1_SPEC>;
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
impl From<crate::W<PPUPATD1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPUPATD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RMU` reader - Reset Management Unit access control bit"]
pub type RMU_R = crate::BitReader<bool>;
#[doc = "Field `RMU` writer - Reset Management Unit access control bit"]
pub type RMU_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `RTCC` reader - Real-Time Counter and Calendar access control bit"]
pub type RTCC_R = crate::BitReader<bool>;
#[doc = "Field `RTCC` writer - Real-Time Counter and Calendar access control bit"]
pub type RTCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `SMU` reader - Security Management Unit access control bit"]
pub type SMU_R = crate::BitReader<bool>;
#[doc = "Field `SMU` writer - Security Management Unit access control bit"]
pub type SMU_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `TIMER0` reader - Timer 0 access control bit"]
pub type TIMER0_R = crate::BitReader<bool>;
#[doc = "Field `TIMER0` writer - Timer 0 access control bit"]
pub type TIMER0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `TIMER1` reader - Timer 1 access control bit"]
pub type TIMER1_R = crate::BitReader<bool>;
#[doc = "Field `TIMER1` writer - Timer 1 access control bit"]
pub type TIMER1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `TRNG0` reader - True Random Number Generator 0 access control bit"]
pub type TRNG0_R = crate::BitReader<bool>;
#[doc = "Field `TRNG0` writer - True Random Number Generator 0 access control bit"]
pub type TRNG0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `USART0` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 0 access control bit"]
pub type USART0_R = crate::BitReader<bool>;
#[doc = "Field `USART0` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 0 access control bit"]
pub type USART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `USART1` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 1 access control bit"]
pub type USART1_R = crate::BitReader<bool>;
#[doc = "Field `USART1` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 1 access control bit"]
pub type USART1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `USART2` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 2 access control bit"]
pub type USART2_R = crate::BitReader<bool>;
#[doc = "Field `USART2` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 2 access control bit"]
pub type USART2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `USART3` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 3 access control bit"]
pub type USART3_R = crate::BitReader<bool>;
#[doc = "Field `USART3` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 3 access control bit"]
pub type USART3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `WDOG0` reader - Watchdog 0 access control bit"]
pub type WDOG0_R = crate::BitReader<bool>;
#[doc = "Field `WDOG0` writer - Watchdog 0 access control bit"]
pub type WDOG0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `WDOG1` reader - Watchdog 1 access control bit"]
pub type WDOG1_R = crate::BitReader<bool>;
#[doc = "Field `WDOG1` writer - Watchdog 1 access control bit"]
pub type WDOG1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `WTIMER0` reader - Wide Timer 0 access control bit"]
pub type WTIMER0_R = crate::BitReader<bool>;
#[doc = "Field `WTIMER0` writer - Wide Timer 0 access control bit"]
pub type WTIMER0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `WTIMER1` reader - Wide Timer 1 access control bit"]
pub type WTIMER1_R = crate::BitReader<bool>;
#[doc = "Field `WTIMER1` writer - Wide Timer 1 access control bit"]
pub type WTIMER1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Reset Management Unit access control bit"]
    #[inline(always)]
    pub fn rmu(&self) -> RMU_R {
        RMU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Real-Time Counter and Calendar access control bit"]
    #[inline(always)]
    pub fn rtcc(&self) -> RTCC_R {
        RTCC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Security Management Unit access control bit"]
    #[inline(always)]
    pub fn smu(&self) -> SMU_R {
        SMU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer 0 access control bit"]
    #[inline(always)]
    pub fn timer0(&self) -> TIMER0_R {
        TIMER0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer 1 access control bit"]
    #[inline(always)]
    pub fn timer1(&self) -> TIMER1_R {
        TIMER1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - True Random Number Generator 0 access control bit"]
    #[inline(always)]
    pub fn trng0(&self) -> TRNG0_R {
        TRNG0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Universal Synchronous/Asynchronous Receiver/Transmitter 0 access control bit"]
    #[inline(always)]
    pub fn usart0(&self) -> USART0_R {
        USART0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Universal Synchronous/Asynchronous Receiver/Transmitter 1 access control bit"]
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Universal Synchronous/Asynchronous Receiver/Transmitter 2 access control bit"]
    #[inline(always)]
    pub fn usart2(&self) -> USART2_R {
        USART2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Universal Synchronous/Asynchronous Receiver/Transmitter 3 access control bit"]
    #[inline(always)]
    pub fn usart3(&self) -> USART3_R {
        USART3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Watchdog 0 access control bit"]
    #[inline(always)]
    pub fn wdog0(&self) -> WDOG0_R {
        WDOG0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Watchdog 1 access control bit"]
    #[inline(always)]
    pub fn wdog1(&self) -> WDOG1_R {
        WDOG1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Wide Timer 0 access control bit"]
    #[inline(always)]
    pub fn wtimer0(&self) -> WTIMER0_R {
        WTIMER0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Wide Timer 1 access control bit"]
    #[inline(always)]
    pub fn wtimer1(&self) -> WTIMER1_R {
        WTIMER1_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Reset Management Unit access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn rmu(&mut self) -> RMU_W<1> {
        RMU_W::new(self)
    }
    #[doc = "Bit 2 - Real-Time Counter and Calendar access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn rtcc(&mut self) -> RTCC_W<2> {
        RTCC_W::new(self)
    }
    #[doc = "Bit 3 - Security Management Unit access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn smu(&mut self) -> SMU_W<3> {
        SMU_W::new(self)
    }
    #[doc = "Bit 5 - Timer 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer0(&mut self) -> TIMER0_W<5> {
        TIMER0_W::new(self)
    }
    #[doc = "Bit 6 - Timer 1 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer1(&mut self) -> TIMER1_W<6> {
        TIMER1_W::new(self)
    }
    #[doc = "Bit 7 - True Random Number Generator 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn trng0(&mut self) -> TRNG0_W<7> {
        TRNG0_W::new(self)
    }
    #[doc = "Bit 8 - Universal Synchronous/Asynchronous Receiver/Transmitter 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn usart0(&mut self) -> USART0_W<8> {
        USART0_W::new(self)
    }
    #[doc = "Bit 9 - Universal Synchronous/Asynchronous Receiver/Transmitter 1 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn usart1(&mut self) -> USART1_W<9> {
        USART1_W::new(self)
    }
    #[doc = "Bit 10 - Universal Synchronous/Asynchronous Receiver/Transmitter 2 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn usart2(&mut self) -> USART2_W<10> {
        USART2_W::new(self)
    }
    #[doc = "Bit 11 - Universal Synchronous/Asynchronous Receiver/Transmitter 3 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn usart3(&mut self) -> USART3_W<11> {
        USART3_W::new(self)
    }
    #[doc = "Bit 12 - Watchdog 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn wdog0(&mut self) -> WDOG0_W<12> {
        WDOG0_W::new(self)
    }
    #[doc = "Bit 13 - Watchdog 1 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn wdog1(&mut self) -> WDOG1_W<13> {
        WDOG1_W::new(self)
    }
    #[doc = "Bit 14 - Wide Timer 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn wtimer0(&mut self) -> WTIMER0_W<14> {
        WTIMER0_W::new(self)
    }
    #[doc = "Bit 15 - Wide Timer 1 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn wtimer1(&mut self) -> WTIMER1_W<15> {
        WTIMER1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PPU Privilege Access Type Descriptor 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppupatd1](index.html) module"]
pub struct PPUPATD1_SPEC;
impl crate::RegisterSpec for PPUPATD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppupatd1::R](R) reader structure"]
impl crate::Readable for PPUPATD1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppupatd1::W](W) writer structure"]
impl crate::Writable for PPUPATD1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PPUPATD1 to value 0"]
impl crate::Resettable for PPUPATD1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
