#[doc = "Register `PKCTRL` writer"]
pub struct W(crate::W<PKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PKCTRL_SPEC>;
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
impl From<crate::W<PKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PKSTART` writer - PK Start"]
pub type PKSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, PKCTRL_SPEC, bool, O>;
#[doc = "Field `IFC` writer - ClearIRQ"]
pub type IFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PKCTRL_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - PK Start"]
    #[inline(always)]
    #[must_use]
    pub fn pkstart(&mut self) -> PKSTART_W<0> {
        PKSTART_W::new(self)
    }
    #[doc = "Bit 1 - ClearIRQ"]
    #[inline(always)]
    #[must_use]
    pub fn ifc(&mut self) -> IFC_W<1> {
        IFC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkctrl](index.html) module"]
pub struct PKCTRL_SPEC;
impl crate::RegisterSpec for PKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pkctrl::W](W) writer structure"]
impl crate::Writable for PKCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PKCTRL to value 0"]
impl crate::Resettable for PKCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
