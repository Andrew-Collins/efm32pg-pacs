#[doc = "Register `CH0_EVALTHRES` reader"]
pub struct R(crate::R<CH0_EVALTHRES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH0_EVALTHRES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH0_EVALTHRES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH0_EVALTHRES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH0_EVALTHRES` writer"]
pub struct W(crate::W<CH0_EVALTHRES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH0_EVALTHRES_SPEC>;
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
impl From<crate::W<CH0_EVALTHRES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH0_EVALTHRES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVALTHRES` reader - Threshold"]
pub type EVALTHRES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EVALTHRES` writer - Threshold"]
pub type EVALTHRES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CH0_EVALTHRES_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Threshold"]
    #[inline(always)]
    pub fn evalthres(&self) -> EVALTHRES_R {
        EVALTHRES_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn evalthres(&mut self) -> EVALTHRES_W<0> {
        EVALTHRES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_evalthres](index.html) module"]
pub struct CH0_EVALTHRES_SPEC;
impl crate::RegisterSpec for CH0_EVALTHRES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch0_evalthres::R](R) reader structure"]
impl crate::Readable for CH0_EVALTHRES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch0_evalthres::W](W) writer structure"]
impl crate::Writable for CH0_EVALTHRES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH0_EVALTHRES to value 0"]
impl crate::Resettable for CH0_EVALTHRES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
