#[doc = "Register `TRACEROUTEPEN` reader"]
pub struct R(crate::R<TRACEROUTEPEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRACEROUTEPEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRACEROUTEPEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRACEROUTEPEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRACEROUTEPEN` writer"]
pub struct W(crate::W<TRACEROUTEPEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRACEROUTEPEN_SPEC>;
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
impl From<crate::W<TRACEROUTEPEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRACEROUTEPEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWVPEN` reader - Serial Wire Viewer Output Pin Enable"]
pub type SWVPEN_R = crate::BitReader<bool>;
#[doc = "Field `SWVPEN` writer - Serial Wire Viewer Output Pin Enable"]
pub type SWVPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRACEROUTEPEN_SPEC, bool, O>;
#[doc = "Field `TRACECLKPEN` reader - Trace Clk Pin Enable"]
pub type TRACECLKPEN_R = crate::BitReader<bool>;
#[doc = "Field `TRACECLKPEN` writer - Trace Clk Pin Enable"]
pub type TRACECLKPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRACEROUTEPEN_SPEC, bool, O>;
#[doc = "Field `TRACEDATA0PEN` reader - Trace Data0 Pin Enable"]
pub type TRACEDATA0PEN_R = crate::BitReader<bool>;
#[doc = "Field `TRACEDATA0PEN` writer - Trace Data0 Pin Enable"]
pub type TRACEDATA0PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRACEROUTEPEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Serial Wire Viewer Output Pin Enable"]
    #[inline(always)]
    pub fn swvpen(&self) -> SWVPEN_R {
        SWVPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Trace Clk Pin Enable"]
    #[inline(always)]
    pub fn traceclkpen(&self) -> TRACECLKPEN_R {
        TRACECLKPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Trace Data0 Pin Enable"]
    #[inline(always)]
    pub fn tracedata0pen(&self) -> TRACEDATA0PEN_R {
        TRACEDATA0PEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Serial Wire Viewer Output Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn swvpen(&mut self) -> SWVPEN_W<0> {
        SWVPEN_W::new(self)
    }
    #[doc = "Bit 1 - Trace Clk Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn traceclkpen(&mut self) -> TRACECLKPEN_W<1> {
        TRACECLKPEN_W::new(self)
    }
    #[doc = "Bit 2 - Trace Data0 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tracedata0pen(&mut self) -> TRACEDATA0PEN_W<2> {
        TRACEDATA0PEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [traceroutepen](index.html) module"]
pub struct TRACEROUTEPEN_SPEC;
impl crate::RegisterSpec for TRACEROUTEPEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [traceroutepen::R](R) reader structure"]
impl crate::Readable for TRACEROUTEPEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [traceroutepen::W](W) writer structure"]
impl crate::Writable for TRACEROUTEPEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRACEROUTEPEN to value 0"]
impl crate::Resettable for TRACEROUTEPEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
