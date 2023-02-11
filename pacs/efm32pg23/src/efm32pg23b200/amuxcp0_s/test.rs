#[doc = "Register `TEST` reader"]
pub struct R(crate::R<TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEST` writer"]
pub struct W(crate::W<TEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEST_SPEC>;
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
impl From<crate::W<TEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNCCLK` reader - Sync Clock"]
pub type SYNCCLK_R = crate::BitReader<bool>;
#[doc = "Field `SYNCCLK` writer - Sync Clock"]
pub type SYNCCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_SPEC, bool, O>;
#[doc = "Field `SYNCMODE` reader - Sync Mode"]
pub type SYNCMODE_R = crate::BitReader<bool>;
#[doc = "Field `SYNCMODE` writer - Sync Mode"]
pub type SYNCMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_SPEC, bool, O>;
#[doc = "Field `FORCEREQUEST` reader - Force Request"]
pub type FORCEREQUEST_R = crate::BitReader<bool>;
#[doc = "Field `FORCEREQUEST` writer - Force Request"]
pub type FORCEREQUEST_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_SPEC, bool, O>;
#[doc = "Field `FORCEHICAP` reader - Force high capacitance driver"]
pub type FORCEHICAP_R = crate::BitReader<bool>;
#[doc = "Field `FORCEHICAP` writer - Force high capacitance driver"]
pub type FORCEHICAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_SPEC, bool, O>;
#[doc = "Field `FORCELOCAP` reader - Force low capacitance driver"]
pub type FORCELOCAP_R = crate::BitReader<bool>;
#[doc = "Field `FORCELOCAP` writer - Force low capacitance driver"]
pub type FORCELOCAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_SPEC, bool, O>;
#[doc = "Field `FORCEBOOSTON` reader - Force Boost On"]
pub type FORCEBOOSTON_R = crate::BitReader<bool>;
#[doc = "Field `FORCEBOOSTON` writer - Force Boost On"]
pub type FORCEBOOSTON_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_SPEC, bool, O>;
#[doc = "Field `FORCEBOOSTOFF` reader - Force Boost Off"]
pub type FORCEBOOSTOFF_R = crate::BitReader<bool>;
#[doc = "Field `FORCEBOOSTOFF` writer - Force Boost Off"]
pub type FORCEBOOSTOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_SPEC, bool, O>;
#[doc = "Field `STATUSEN` reader - Enable write to status bits"]
pub type STATUSEN_R = crate::BitReader<bool>;
#[doc = "Field `STATUSEN` writer - Enable write to status bits"]
pub type STATUSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Sync Clock"]
    #[inline(always)]
    pub fn syncclk(&self) -> SYNCCLK_R {
        SYNCCLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sync Mode"]
    #[inline(always)]
    pub fn syncmode(&self) -> SYNCMODE_R {
        SYNCMODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Force Request"]
    #[inline(always)]
    pub fn forcerequest(&self) -> FORCEREQUEST_R {
        FORCEREQUEST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Force high capacitance driver"]
    #[inline(always)]
    pub fn forcehicap(&self) -> FORCEHICAP_R {
        FORCEHICAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Force low capacitance driver"]
    #[inline(always)]
    pub fn forcelocap(&self) -> FORCELOCAP_R {
        FORCELOCAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Force Boost On"]
    #[inline(always)]
    pub fn forcebooston(&self) -> FORCEBOOSTON_R {
        FORCEBOOSTON_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Force Boost Off"]
    #[inline(always)]
    pub fn forceboostoff(&self) -> FORCEBOOSTOFF_R {
        FORCEBOOSTOFF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable write to status bits"]
    #[inline(always)]
    pub fn statusen(&self) -> STATUSEN_R {
        STATUSEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sync Clock"]
    #[inline(always)]
    #[must_use]
    pub fn syncclk(&mut self) -> SYNCCLK_W<0> {
        SYNCCLK_W::new(self)
    }
    #[doc = "Bit 1 - Sync Mode"]
    #[inline(always)]
    #[must_use]
    pub fn syncmode(&mut self) -> SYNCMODE_W<1> {
        SYNCMODE_W::new(self)
    }
    #[doc = "Bit 4 - Force Request"]
    #[inline(always)]
    #[must_use]
    pub fn forcerequest(&mut self) -> FORCEREQUEST_W<4> {
        FORCEREQUEST_W::new(self)
    }
    #[doc = "Bit 8 - Force high capacitance driver"]
    #[inline(always)]
    #[must_use]
    pub fn forcehicap(&mut self) -> FORCEHICAP_W<8> {
        FORCEHICAP_W::new(self)
    }
    #[doc = "Bit 9 - Force low capacitance driver"]
    #[inline(always)]
    #[must_use]
    pub fn forcelocap(&mut self) -> FORCELOCAP_W<9> {
        FORCELOCAP_W::new(self)
    }
    #[doc = "Bit 12 - Force Boost On"]
    #[inline(always)]
    #[must_use]
    pub fn forcebooston(&mut self) -> FORCEBOOSTON_W<12> {
        FORCEBOOSTON_W::new(self)
    }
    #[doc = "Bit 13 - Force Boost Off"]
    #[inline(always)]
    #[must_use]
    pub fn forceboostoff(&mut self) -> FORCEBOOSTOFF_W<13> {
        FORCEBOOSTOFF_W::new(self)
    }
    #[doc = "Bit 31 - Enable write to status bits"]
    #[inline(always)]
    #[must_use]
    pub fn statusen(&mut self) -> STATUSEN_W<31> {
        STATUSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test](index.html) module"]
pub struct TEST_SPEC;
impl crate::RegisterSpec for TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [test::R](R) reader structure"]
impl crate::Readable for TEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [test::W](W) writer structure"]
impl crate::Writable for TEST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
