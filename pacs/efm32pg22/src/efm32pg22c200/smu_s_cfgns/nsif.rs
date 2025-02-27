#[doc = "Register `NSIF` reader"]
pub struct R(crate::R<NSIF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NSIF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NSIF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NSIF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NSIF` writer"]
pub struct W(crate::W<NSIF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NSIF_SPEC>;
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
impl From<crate::W<NSIF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NSIF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PPUNSPRIV` reader - PPUNS Privilege Interrupt Flag"]
pub type PPUNSPRIV_R = crate::BitReader<bool>;
#[doc = "Field `PPUNSPRIV` writer - PPUNS Privilege Interrupt Flag"]
pub type PPUNSPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSIF_SPEC, bool, O>;
#[doc = "Field `PPUNSINST` reader - PPUNS Instruction Interrupt Flag"]
pub type PPUNSINST_R = crate::BitReader<bool>;
#[doc = "Field `PPUNSINST` writer - PPUNS Instruction Interrupt Flag"]
pub type PPUNSINST_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSIF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PPUNS Privilege Interrupt Flag"]
    #[inline(always)]
    pub fn ppunspriv(&self) -> PPUNSPRIV_R {
        PPUNSPRIV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - PPUNS Instruction Interrupt Flag"]
    #[inline(always)]
    pub fn ppunsinst(&self) -> PPUNSINST_R {
        PPUNSINST_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PPUNS Privilege Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ppunspriv(&mut self) -> PPUNSPRIV_W<0> {
        PPUNSPRIV_W::new(self)
    }
    #[doc = "Bit 2 - PPUNS Instruction Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ppunsinst(&mut self) -> PPUNSINST_W<2> {
        PPUNSINST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register for interrupt status flags.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nsif](index.html) module"]
pub struct NSIF_SPEC;
impl crate::RegisterSpec for NSIF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nsif::R](R) reader structure"]
impl crate::Readable for NSIF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nsif::W](W) writer structure"]
impl crate::Writable for NSIF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NSIF to value 0"]
impl crate::Resettable for NSIF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
