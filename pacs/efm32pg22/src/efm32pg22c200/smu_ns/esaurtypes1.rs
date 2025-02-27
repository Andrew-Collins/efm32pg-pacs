#[doc = "Register `ESAURTYPES1` reader"]
pub struct R(crate::R<ESAURTYPES1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESAURTYPES1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESAURTYPES1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESAURTYPES1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESAURTYPES1` writer"]
pub struct W(crate::W<ESAURTYPES1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESAURTYPES1_SPEC>;
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
impl From<crate::W<ESAURTYPES1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESAURTYPES1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ESAUR11NS` reader - Region 11 Non-Secure Type"]
pub type ESAUR11NS_R = crate::BitReader<bool>;
#[doc = "Field `ESAUR11NS` writer - Region 11 Non-Secure Type"]
pub type ESAUR11NS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ESAURTYPES1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 12 - Region 11 Non-Secure Type"]
    #[inline(always)]
    pub fn esaur11ns(&self) -> ESAUR11NS_R {
        ESAUR11NS_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Region 11 Non-Secure Type"]
    #[inline(always)]
    #[must_use]
    pub fn esaur11ns(&mut self) -> ESAUR11NS_W<12> {
        ESAUR11NS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write to specify if a region is secure or non-secure.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esaurtypes1](index.html) module"]
pub struct ESAURTYPES1_SPEC;
impl crate::RegisterSpec for ESAURTYPES1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [esaurtypes1::R](R) reader structure"]
impl crate::Readable for ESAURTYPES1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [esaurtypes1::W](W) writer structure"]
impl crate::Writable for ESAURTYPES1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ESAURTYPES1 to value 0"]
impl crate::Resettable for ESAURTYPES1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
