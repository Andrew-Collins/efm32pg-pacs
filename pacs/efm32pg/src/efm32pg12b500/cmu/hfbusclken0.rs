#[doc = "Register `HFBUSCLKEN0` reader"]
pub struct R(crate::R<HFBUSCLKEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFBUSCLKEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFBUSCLKEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFBUSCLKEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFBUSCLKEN0` writer"]
pub struct W(crate::W<HFBUSCLKEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFBUSCLKEN0_SPEC>;
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
impl From<crate::W<HFBUSCLKEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFBUSCLKEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRYPTO0` reader - Advanced Encryption Standard Accelerator 0 Clock Enable"]
pub type CRYPTO0_R = crate::BitReader<bool>;
#[doc = "Field `CRYPTO0` writer - Advanced Encryption Standard Accelerator 0 Clock Enable"]
pub type CRYPTO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFBUSCLKEN0_SPEC, bool, O>;
#[doc = "Field `CRYPTO1` reader - Advanced Encryption Standard Accelerator 1 Clock Enable"]
pub type CRYPTO1_R = crate::BitReader<bool>;
#[doc = "Field `CRYPTO1` writer - Advanced Encryption Standard Accelerator 1 Clock Enable"]
pub type CRYPTO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFBUSCLKEN0_SPEC, bool, O>;
#[doc = "Field `LE` reader - Low Energy Peripheral Interface Clock Enable"]
pub type LE_R = crate::BitReader<bool>;
#[doc = "Field `LE` writer - Low Energy Peripheral Interface Clock Enable"]
pub type LE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFBUSCLKEN0_SPEC, bool, O>;
#[doc = "Field `GPIO` reader - General purpose Input/Output Clock Enable"]
pub type GPIO_R = crate::BitReader<bool>;
#[doc = "Field `GPIO` writer - General purpose Input/Output Clock Enable"]
pub type GPIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFBUSCLKEN0_SPEC, bool, O>;
#[doc = "Field `PRS` reader - Peripheral Reflex System Clock Enable"]
pub type PRS_R = crate::BitReader<bool>;
#[doc = "Field `PRS` writer - Peripheral Reflex System Clock Enable"]
pub type PRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFBUSCLKEN0_SPEC, bool, O>;
#[doc = "Field `LDMA` reader - Linked Direct Memory Access Controller Clock Enable"]
pub type LDMA_R = crate::BitReader<bool>;
#[doc = "Field `LDMA` writer - Linked Direct Memory Access Controller Clock Enable"]
pub type LDMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFBUSCLKEN0_SPEC, bool, O>;
#[doc = "Field `GPCRC` reader - General Purpose CRC Clock Enable"]
pub type GPCRC_R = crate::BitReader<bool>;
#[doc = "Field `GPCRC` writer - General Purpose CRC Clock Enable"]
pub type GPCRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFBUSCLKEN0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Advanced Encryption Standard Accelerator 0 Clock Enable"]
    #[inline(always)]
    pub fn crypto0(&self) -> CRYPTO0_R {
        CRYPTO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Advanced Encryption Standard Accelerator 1 Clock Enable"]
    #[inline(always)]
    pub fn crypto1(&self) -> CRYPTO1_R {
        CRYPTO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low Energy Peripheral Interface Clock Enable"]
    #[inline(always)]
    pub fn le(&self) -> LE_R {
        LE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - General purpose Input/Output Clock Enable"]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Peripheral Reflex System Clock Enable"]
    #[inline(always)]
    pub fn prs(&self) -> PRS_R {
        PRS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Linked Direct Memory Access Controller Clock Enable"]
    #[inline(always)]
    pub fn ldma(&self) -> LDMA_R {
        LDMA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - General Purpose CRC Clock Enable"]
    #[inline(always)]
    pub fn gpcrc(&self) -> GPCRC_R {
        GPCRC_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Advanced Encryption Standard Accelerator 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn crypto0(&mut self) -> CRYPTO0_W<0> {
        CRYPTO0_W::new(self)
    }
    #[doc = "Bit 1 - Advanced Encryption Standard Accelerator 1 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn crypto1(&mut self) -> CRYPTO1_W<1> {
        CRYPTO1_W::new(self)
    }
    #[doc = "Bit 2 - Low Energy Peripheral Interface Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn le(&mut self) -> LE_W<2> {
        LE_W::new(self)
    }
    #[doc = "Bit 3 - General purpose Input/Output Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio(&mut self) -> GPIO_W<3> {
        GPIO_W::new(self)
    }
    #[doc = "Bit 4 - Peripheral Reflex System Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn prs(&mut self) -> PRS_W<4> {
        PRS_W::new(self)
    }
    #[doc = "Bit 5 - Linked Direct Memory Access Controller Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ldma(&mut self) -> LDMA_W<5> {
        LDMA_W::new(self)
    }
    #[doc = "Bit 6 - General Purpose CRC Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpcrc(&mut self) -> GPCRC_W<6> {
        GPCRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High Frequency Bus Clock Enable Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfbusclken0](index.html) module"]
pub struct HFBUSCLKEN0_SPEC;
impl crate::RegisterSpec for HFBUSCLKEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfbusclken0::R](R) reader structure"]
impl crate::Readable for HFBUSCLKEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfbusclken0::W](W) writer structure"]
impl crate::Writable for HFBUSCLKEN0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HFBUSCLKEN0 to value 0"]
impl crate::Resettable for HFBUSCLKEN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
