#[doc = "Register `IEN` reader"]
pub struct R(crate::R<IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEN` writer"]
pub struct W(crate::W<IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEN_SPEC>;
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
impl From<crate::W<IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDY` reader - Ready Enable"]
pub type RDY_R = crate::BitReader<bool>;
#[doc = "Field `RDY` writer - Ready Enable"]
pub type RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `POSEDGE` reader - Rising Edge Enable"]
pub type POSEDGE_R = crate::BitReader<bool>;
#[doc = "Field `POSEDGE` writer - Rising Edge Enable"]
pub type POSEDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `NEGEDGE` reader - Falling Edge Enable"]
pub type NEGEDGE_R = crate::BitReader<bool>;
#[doc = "Field `NEGEDGE` writer - Falling Edge Enable"]
pub type NEGEDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `TCDONE` reader - Temperature Check Done Enable"]
pub type TCDONE_R = crate::BitReader<bool>;
#[doc = "Field `TCDONE` writer - Temperature Check Done Enable"]
pub type TCDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CALDONE` reader - Calibration Done Enable"]
pub type CALDONE_R = crate::BitReader<bool>;
#[doc = "Field `CALDONE` writer - Calibration Done Enable"]
pub type CALDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `TEMPCHANGE` reader - Temperature Change Enable"]
pub type TEMPCHANGE_R = crate::BitReader<bool>;
#[doc = "Field `TEMPCHANGE` writer - Temperature Change Enable"]
pub type TEMPCHANGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `SCHEDERR` reader - Scheduling Error Enable"]
pub type SCHEDERR_R = crate::BitReader<bool>;
#[doc = "Field `SCHEDERR` writer - Scheduling Error Enable"]
pub type SCHEDERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `TCOOR` reader - Temperature Check Out Of Range Enable"]
pub type TCOOR_R = crate::BitReader<bool>;
#[doc = "Field `TCOOR` writer - Temperature Check Out Of Range Enable"]
pub type TCOOR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CALOOR` reader - Calibration Out Of Range Enable"]
pub type CALOOR_R = crate::BitReader<bool>;
#[doc = "Field `CALOOR` writer - Calibration Out Of Range Enable"]
pub type CALOOR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Ready Enable"]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising Edge Enable"]
    #[inline(always)]
    pub fn posedge(&self) -> POSEDGE_R {
        POSEDGE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Falling Edge Enable"]
    #[inline(always)]
    pub fn negedge(&self) -> NEGEDGE_R {
        NEGEDGE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Temperature Check Done Enable"]
    #[inline(always)]
    pub fn tcdone(&self) -> TCDONE_R {
        TCDONE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Calibration Done Enable"]
    #[inline(always)]
    pub fn caldone(&self) -> CALDONE_R {
        CALDONE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Temperature Change Enable"]
    #[inline(always)]
    pub fn tempchange(&self) -> TEMPCHANGE_R {
        TEMPCHANGE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Scheduling Error Enable"]
    #[inline(always)]
    pub fn schederr(&self) -> SCHEDERR_R {
        SCHEDERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Temperature Check Out Of Range Enable"]
    #[inline(always)]
    pub fn tcoor(&self) -> TCOOR_R {
        TCOOR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Calibration Out Of Range Enable"]
    #[inline(always)]
    pub fn caloor(&self) -> CALOOR_R {
        CALOOR_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ready Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdy(&mut self) -> RDY_W<0> {
        RDY_W::new(self)
    }
    #[doc = "Bit 1 - Rising Edge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn posedge(&mut self) -> POSEDGE_W<1> {
        POSEDGE_W::new(self)
    }
    #[doc = "Bit 2 - Falling Edge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn negedge(&mut self) -> NEGEDGE_W<2> {
        NEGEDGE_W::new(self)
    }
    #[doc = "Bit 8 - Temperature Check Done Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcdone(&mut self) -> TCDONE_W<8> {
        TCDONE_W::new(self)
    }
    #[doc = "Bit 9 - Calibration Done Enable"]
    #[inline(always)]
    #[must_use]
    pub fn caldone(&mut self) -> CALDONE_W<9> {
        CALDONE_W::new(self)
    }
    #[doc = "Bit 10 - Temperature Change Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tempchange(&mut self) -> TEMPCHANGE_W<10> {
        TEMPCHANGE_W::new(self)
    }
    #[doc = "Bit 16 - Scheduling Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn schederr(&mut self) -> SCHEDERR_W<16> {
        SCHEDERR_W::new(self)
    }
    #[doc = "Bit 17 - Temperature Check Out Of Range Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcoor(&mut self) -> TCOOR_W<17> {
        TCOOR_W::new(self)
    }
    #[doc = "Bit 18 - Calibration Out Of Range Enable"]
    #[inline(always)]
    #[must_use]
    pub fn caloor(&mut self) -> CALOOR_W<18> {
        CALOOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ien::R](R) reader structure"]
impl crate::Readable for IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ien::W](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
