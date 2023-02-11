#[doc = "Register `IF` reader"]
pub struct R(crate::R<IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IF` writer"]
pub struct W(crate::W<IF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF_SPEC>;
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
impl From<crate::W<IF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW0` reader - Software Interrupt 0"]
pub type SW0_R = crate::BitReader<bool>;
#[doc = "Field `SW0` writer - Software Interrupt 0"]
pub type SW0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SW1` reader - Software Interrupt 1"]
pub type SW1_R = crate::BitReader<bool>;
#[doc = "Field `SW1` writer - Software Interrupt 1"]
pub type SW1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SW2` reader - Software Interrupt 2"]
pub type SW2_R = crate::BitReader<bool>;
#[doc = "Field `SW2` writer - Software Interrupt 2"]
pub type SW2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SW3` reader - Software Interrupt 3"]
pub type SW3_R = crate::BitReader<bool>;
#[doc = "Field `SW3` writer - Software Interrupt 3"]
pub type SW3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `RAMERR1B` reader - RAM 1-Bit Error Interrupt Flag"]
pub type RAMERR1B_R = crate::BitReader<bool>;
#[doc = "Field `RAMERR1B` writer - RAM 1-Bit Error Interrupt Flag"]
pub type RAMERR1B_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `RAMERR2B` reader - RAM 2-Bit Error Interrupt Flag"]
pub type RAMERR2B_R = crate::BitReader<bool>;
#[doc = "Field `RAMERR2B` writer - RAM 2-Bit Error Interrupt Flag"]
pub type RAMERR2B_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Software Interrupt 0"]
    #[inline(always)]
    pub fn sw0(&self) -> SW0_R {
        SW0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Interrupt 1"]
    #[inline(always)]
    pub fn sw1(&self) -> SW1_R {
        SW1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software Interrupt 2"]
    #[inline(always)]
    pub fn sw2(&self) -> SW2_R {
        SW2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software Interrupt 3"]
    #[inline(always)]
    pub fn sw3(&self) -> SW3_R {
        SW3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - RAM 1-Bit Error Interrupt Flag"]
    #[inline(always)]
    pub fn ramerr1b(&self) -> RAMERR1B_R {
        RAMERR1B_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RAM 2-Bit Error Interrupt Flag"]
    #[inline(always)]
    pub fn ramerr2b(&self) -> RAMERR2B_R {
        RAMERR2B_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Interrupt 0"]
    #[inline(always)]
    #[must_use]
    pub fn sw0(&mut self) -> SW0_W<0> {
        SW0_W::new(self)
    }
    #[doc = "Bit 1 - Software Interrupt 1"]
    #[inline(always)]
    #[must_use]
    pub fn sw1(&mut self) -> SW1_W<1> {
        SW1_W::new(self)
    }
    #[doc = "Bit 2 - Software Interrupt 2"]
    #[inline(always)]
    #[must_use]
    pub fn sw2(&mut self) -> SW2_W<2> {
        SW2_W::new(self)
    }
    #[doc = "Bit 3 - Software Interrupt 3"]
    #[inline(always)]
    #[must_use]
    pub fn sw3(&mut self) -> SW3_W<3> {
        SW3_W::new(self)
    }
    #[doc = "Bit 16 - RAM 1-Bit Error Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ramerr1b(&mut self) -> RAMERR1B_W<16> {
        RAMERR1B_W::new(self)
    }
    #[doc = "Bit 17 - RAM 2-Bit Error Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ramerr2b(&mut self) -> RAMERR2B_W<17> {
        RAMERR2B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read to get system status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_::R](R) reader structure"]
impl crate::Readable for IF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [if_::W](W) writer structure"]
impl crate::Writable for IF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
