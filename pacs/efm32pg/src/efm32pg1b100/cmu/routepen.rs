#[doc = "Register `ROUTEPEN` reader"]
pub struct R(crate::R<ROUTEPEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROUTEPEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROUTEPEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROUTEPEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROUTEPEN` writer"]
pub struct W(crate::W<ROUTEPEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROUTEPEN_SPEC>;
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
impl From<crate::W<ROUTEPEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROUTEPEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKOUT0PEN` reader - CLKOUT0 Pin Enable"]
pub type CLKOUT0PEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKOUT0PEN` writer - CLKOUT0 Pin Enable"]
pub type CLKOUT0PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `CLKOUT1PEN` reader - CLKOUT1 Pin Enable"]
pub type CLKOUT1PEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKOUT1PEN` writer - CLKOUT1 Pin Enable"]
pub type CLKOUT1PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CLKOUT0 Pin Enable"]
    #[inline(always)]
    pub fn clkout0pen(&self) -> CLKOUT0PEN_R {
        CLKOUT0PEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CLKOUT1 Pin Enable"]
    #[inline(always)]
    pub fn clkout1pen(&self) -> CLKOUT1PEN_R {
        CLKOUT1PEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CLKOUT0 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clkout0pen(&mut self) -> CLKOUT0PEN_W<0> {
        CLKOUT0PEN_W::new(self)
    }
    #[doc = "Bit 1 - CLKOUT1 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clkout1pen(&mut self) -> CLKOUT1PEN_W<1> {
        CLKOUT1PEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Routing Pin Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routepen](index.html) module"]
pub struct ROUTEPEN_SPEC;
impl crate::RegisterSpec for ROUTEPEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [routepen::R](R) reader structure"]
impl crate::Readable for ROUTEPEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [routepen::W](W) writer structure"]
impl crate::Writable for ROUTEPEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROUTEPEN to value 0"]
impl crate::Resettable for ROUTEPEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
