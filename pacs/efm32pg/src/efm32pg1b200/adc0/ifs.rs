#[doc = "Register `IFS` writer"]
pub struct W(crate::W<IFS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFS_SPEC>;
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
impl From<crate::W<IFS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SINGLEOF` writer - Set SINGLEOF Interrupt Flag"]
pub type SINGLEOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `SCANOF` writer - Set SCANOF Interrupt Flag"]
pub type SCANOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `SINGLEUF` writer - Set SINGLEUF Interrupt Flag"]
pub type SINGLEUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `SCANUF` writer - Set SCANUF Interrupt Flag"]
pub type SCANUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `SINGLECMP` writer - Set SINGLECMP Interrupt Flag"]
pub type SINGLECMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `SCANCMP` writer - Set SCANCMP Interrupt Flag"]
pub type SCANCMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `VREFOV` writer - Set VREFOV Interrupt Flag"]
pub type VREFOV_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `PROGERR` writer - Set PROGERR Interrupt Flag"]
pub type PROGERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
impl W {
    #[doc = "Bit 8 - Set SINGLEOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn singleof(&mut self) -> SINGLEOF_W<8> {
        SINGLEOF_W::new(self)
    }
    #[doc = "Bit 9 - Set SCANOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn scanof(&mut self) -> SCANOF_W<9> {
        SCANOF_W::new(self)
    }
    #[doc = "Bit 10 - Set SINGLEUF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn singleuf(&mut self) -> SINGLEUF_W<10> {
        SINGLEUF_W::new(self)
    }
    #[doc = "Bit 11 - Set SCANUF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn scanuf(&mut self) -> SCANUF_W<11> {
        SCANUF_W::new(self)
    }
    #[doc = "Bit 16 - Set SINGLECMP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn singlecmp(&mut self) -> SINGLECMP_W<16> {
        SINGLECMP_W::new(self)
    }
    #[doc = "Bit 17 - Set SCANCMP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn scancmp(&mut self) -> SCANCMP_W<17> {
        SCANCMP_W::new(self)
    }
    #[doc = "Bit 24 - Set VREFOV Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vrefov(&mut self) -> VREFOV_W<24> {
        VREFOV_W::new(self)
    }
    #[doc = "Bit 25 - Set PROGERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn progerr(&mut self) -> PROGERR_W<25> {
        PROGERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifs](index.html) module"]
pub struct IFS_SPEC;
impl crate::RegisterSpec for IFS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifs::W](W) writer structure"]
impl crate::Writable for IFS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
