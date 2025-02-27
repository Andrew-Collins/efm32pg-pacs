#[doc = "Register `TXDATA` writer"]
pub struct W(crate::W<TXDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXDATA_SPEC>;
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
impl From<crate::W<TXDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDATA` writer - TX Data"]
pub type TXDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXDATA_SPEC, u16, u16, 9, O>;
#[doc = "Field `UBRXAT` writer - Unblock RX After Transmission"]
pub type UBRXAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXDATA_SPEC, bool, O>;
#[doc = "Field `TXTRIAT` writer - Set TXTRI After Transmisssion"]
pub type TXTRIAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXDATA_SPEC, bool, O>;
#[doc = "Field `TXBREAK` writer - Transit Data as Break"]
pub type TXBREAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXDATA_SPEC, bool, O>;
#[doc = "Field `TXDISAT` writer - Clear TXEN After Transmission"]
pub type TXDISAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXDATA_SPEC, bool, O>;
#[doc = "Field `RXENAT` writer - Enable RXEN After Transmission"]
pub type RXENAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXDATA_SPEC, bool, O>;
impl W {
    #[doc = "Bits 0:8 - TX Data"]
    #[inline(always)]
    #[must_use]
    pub fn txdata(&mut self) -> TXDATA_W<0> {
        TXDATA_W::new(self)
    }
    #[doc = "Bit 9 - Unblock RX After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn ubrxat(&mut self) -> UBRXAT_W<9> {
        UBRXAT_W::new(self)
    }
    #[doc = "Bit 10 - Set TXTRI After Transmisssion"]
    #[inline(always)]
    #[must_use]
    pub fn txtriat(&mut self) -> TXTRIAT_W<10> {
        TXTRIAT_W::new(self)
    }
    #[doc = "Bit 11 - Transit Data as Break"]
    #[inline(always)]
    #[must_use]
    pub fn txbreak(&mut self) -> TXBREAK_W<11> {
        TXBREAK_W::new(self)
    }
    #[doc = "Bit 12 - Clear TXEN After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn txdisat(&mut self) -> TXDISAT_W<12> {
        TXDISAT_W::new(self)
    }
    #[doc = "Bit 13 - Enable RXEN After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn rxenat(&mut self) -> RXENAT_W<13> {
        RXENAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdata](index.html) module"]
pub struct TXDATA_SPEC;
impl crate::RegisterSpec for TXDATA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [txdata::W](W) writer structure"]
impl crate::Writable for TXDATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXDATA to value 0"]
impl crate::Resettable for TXDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
