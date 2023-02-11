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
#[doc = "Field `DCDC` reader - DCDC Privileged Access"]
pub type DCDC_R = crate::BitReader<bool>;
#[doc = "Field `DCDC` writer - DCDC Privileged Access"]
pub type DCDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `PDM` reader - PDM Privileged Access"]
pub type PDM_R = crate::BitReader<bool>;
#[doc = "Field `PDM` writer - PDM Privileged Access"]
pub type PDM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `SMU` reader - SMU Privileged Access"]
pub type SMU_R = crate::BitReader<bool>;
#[doc = "Field `SMU` writer - SMU Privileged Access"]
pub type SMU_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `SMUCFGNS` reader - SMUCFGNS Privileged Access"]
pub type SMUCFGNS_R = crate::BitReader<bool>;
#[doc = "Field `SMUCFGNS` writer - SMUCFGNS Privileged Access"]
pub type SMUCFGNS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `RTCC` reader - RTCC Privileged Access"]
pub type RTCC_R = crate::BitReader<bool>;
#[doc = "Field `RTCC` writer - RTCC Privileged Access"]
pub type RTCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `LETIMER0` reader - LETIMER0 Privileged Access"]
pub type LETIMER0_R = crate::BitReader<bool>;
#[doc = "Field `LETIMER0` writer - LETIMER0 Privileged Access"]
pub type LETIMER0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `IADC0` reader - IADC0 Privileged Access"]
pub type IADC0_R = crate::BitReader<bool>;
#[doc = "Field `IADC0` writer - IADC0 Privileged Access"]
pub type IADC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `I2C0` reader - I2C0 Privileged Access"]
pub type I2C0_R = crate::BitReader<bool>;
#[doc = "Field `I2C0` writer - I2C0 Privileged Access"]
pub type I2C0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `WDOG0` reader - WDOG0 Privileged Access"]
pub type WDOG0_R = crate::BitReader<bool>;
#[doc = "Field `WDOG0` writer - WDOG0 Privileged Access"]
pub type WDOG0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `AMUXCP0` reader - AMUXCP0 Privileged Access"]
pub type AMUXCP0_R = crate::BitReader<bool>;
#[doc = "Field `AMUXCP0` writer - AMUXCP0 Privileged Access"]
pub type AMUXCP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `EUART0` reader - EUART0 Privileged Access"]
pub type EUART0_R = crate::BitReader<bool>;
#[doc = "Field `EUART0` writer - EUART0 Privileged Access"]
pub type EUART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
#[doc = "Field `CRYPTOACC` reader - CRYPTOACC Privileged Access"]
pub type CRYPTOACC_R = crate::BitReader<bool>;
#[doc = "Field `CRYPTOACC` writer - CRYPTOACC Privileged Access"]
pub type CRYPTOACC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - DCDC Privileged Access"]
    #[inline(always)]
    pub fn dcdc(&self) -> DCDC_R {
        DCDC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDM Privileged Access"]
    #[inline(always)]
    pub fn pdm(&self) -> PDM_R {
        PDM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - SMU Privileged Access"]
    #[inline(always)]
    pub fn smu(&self) -> SMU_R {
        SMU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SMUCFGNS Privileged Access"]
    #[inline(always)]
    pub fn smucfgns(&self) -> SMUCFGNS_R {
        SMUCFGNS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RTCC Privileged Access"]
    #[inline(always)]
    pub fn rtcc(&self) -> RTCC_R {
        RTCC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LETIMER0 Privileged Access"]
    #[inline(always)]
    pub fn letimer0(&self) -> LETIMER0_R {
        LETIMER0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IADC0 Privileged Access"]
    #[inline(always)]
    pub fn iadc0(&self) -> IADC0_R {
        IADC0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - I2C0 Privileged Access"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WDOG0 Privileged Access"]
    #[inline(always)]
    pub fn wdog0(&self) -> WDOG0_R {
        WDOG0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AMUXCP0 Privileged Access"]
    #[inline(always)]
    pub fn amuxcp0(&self) -> AMUXCP0_R {
        AMUXCP0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EUART0 Privileged Access"]
    #[inline(always)]
    pub fn euart0(&self) -> EUART0_R {
        EUART0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CRYPTOACC Privileged Access"]
    #[inline(always)]
    pub fn cryptoacc(&self) -> CRYPTOACC_R {
        CRYPTOACC_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DCDC Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc(&mut self) -> DCDC_W<1> {
        DCDC_W::new(self)
    }
    #[doc = "Bit 2 - PDM Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn pdm(&mut self) -> PDM_W<2> {
        PDM_W::new(self)
    }
    #[doc = "Bit 5 - SMU Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn smu(&mut self) -> SMU_W<5> {
        SMU_W::new(self)
    }
    #[doc = "Bit 6 - SMUCFGNS Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn smucfgns(&mut self) -> SMUCFGNS_W<6> {
        SMUCFGNS_W::new(self)
    }
    #[doc = "Bit 7 - RTCC Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn rtcc(&mut self) -> RTCC_W<7> {
        RTCC_W::new(self)
    }
    #[doc = "Bit 8 - LETIMER0 Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn letimer0(&mut self) -> LETIMER0_W<8> {
        LETIMER0_W::new(self)
    }
    #[doc = "Bit 9 - IADC0 Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn iadc0(&mut self) -> IADC0_W<9> {
        IADC0_W::new(self)
    }
    #[doc = "Bit 10 - I2C0 Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0(&mut self) -> I2C0_W<10> {
        I2C0_W::new(self)
    }
    #[doc = "Bit 11 - WDOG0 Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn wdog0(&mut self) -> WDOG0_W<11> {
        WDOG0_W::new(self)
    }
    #[doc = "Bit 12 - AMUXCP0 Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn amuxcp0(&mut self) -> AMUXCP0_W<12> {
        AMUXCP0_W::new(self)
    }
    #[doc = "Bit 13 - EUART0 Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn euart0(&mut self) -> EUART0_W<13> {
        EUART0_W::new(self)
    }
    #[doc = "Bit 14 - CRYPTOACC Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn cryptoacc(&mut self) -> CRYPTOACC_W<14> {
        CRYPTOACC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set peripheral bits to 1 to mark as privileged access only.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppupatd1](index.html) module"]
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
#[doc = "`reset()` method sets PPUPATD1 to value 0xffff"]
impl crate::Resettable for PPUPATD1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
