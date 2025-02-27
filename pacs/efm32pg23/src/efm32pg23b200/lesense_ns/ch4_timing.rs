#[doc = "Register `CH4_TIMING` reader"]
pub struct R(crate::R<CH4_TIMING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH4_TIMING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH4_TIMING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH4_TIMING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH4_TIMING` writer"]
pub struct W(crate::W<CH4_TIMING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH4_TIMING_SPEC>;
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
impl From<crate::W<CH4_TIMING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH4_TIMING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTIME` reader - Set excitation time"]
pub type EXTIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTIME` writer - Set excitation time"]
pub type EXTIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH4_TIMING_SPEC, u8, u8, 6, O>;
#[doc = "Field `SAMPLEDLY` reader - Set sample delay"]
pub type SAMPLEDLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMPLEDLY` writer - Set sample delay"]
pub type SAMPLEDLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH4_TIMING_SPEC, u8, u8, 8, O>;
#[doc = "Field `MEASUREDLY` reader - Set measure delay"]
pub type MEASUREDLY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MEASUREDLY` writer - Set measure delay"]
pub type MEASUREDLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CH4_TIMING_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:5 - Set excitation time"]
    #[inline(always)]
    pub fn extime(&self) -> EXTIME_R {
        EXTIME_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:13 - Set sample delay"]
    #[inline(always)]
    pub fn sampledly(&self) -> SAMPLEDLY_R {
        SAMPLEDLY_R::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bits 14:23 - Set measure delay"]
    #[inline(always)]
    pub fn measuredly(&self) -> MEASUREDLY_R {
        MEASUREDLY_R::new(((self.bits >> 14) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - Set excitation time"]
    #[inline(always)]
    #[must_use]
    pub fn extime(&mut self) -> EXTIME_W<0> {
        EXTIME_W::new(self)
    }
    #[doc = "Bits 6:13 - Set sample delay"]
    #[inline(always)]
    #[must_use]
    pub fn sampledly(&mut self) -> SAMPLEDLY_W<6> {
        SAMPLEDLY_W::new(self)
    }
    #[doc = "Bits 14:23 - Set measure delay"]
    #[inline(always)]
    #[must_use]
    pub fn measuredly(&mut self) -> MEASUREDLY_W<14> {
        MEASUREDLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_timing](index.html) module"]
pub struct CH4_TIMING_SPEC;
impl crate::RegisterSpec for CH4_TIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch4_timing::R](R) reader structure"]
impl crate::Readable for CH4_TIMING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch4_timing::W](W) writer structure"]
impl crate::Writable for CH4_TIMING_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH4_TIMING to value 0"]
impl crate::Resettable for CH4_TIMING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
