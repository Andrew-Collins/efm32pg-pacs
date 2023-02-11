#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FETCHERSCATTERGATHER` reader - Fetcher scatter/gather"]
pub type FETCHERSCATTERGATHER_R = crate::BitReader<bool>;
#[doc = "Field `FETCHERSCATTERGATHER` writer - Fetcher scatter/gather"]
pub type FETCHERSCATTERGATHER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `PUSHERSCATTERGATHER` reader - Pusher scatter/gather"]
pub type PUSHERSCATTERGATHER_R = crate::BitReader<bool>;
#[doc = "Field `PUSHERSCATTERGATHER` writer - Pusher scatter/gather"]
pub type PUSHERSCATTERGATHER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `STOPFETCHER` reader - Stop fetcher"]
pub type STOPFETCHER_R = crate::BitReader<bool>;
#[doc = "Field `STOPFETCHER` writer - Stop fetcher"]
pub type STOPFETCHER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `STOPPUSHER` reader - Stop pusher"]
pub type STOPPUSHER_R = crate::BitReader<bool>;
#[doc = "Field `STOPPUSHER` writer - Stop pusher"]
pub type STOPPUSHER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SWRESET` reader - Software reset"]
pub type SWRESET_R = crate::BitReader<bool>;
#[doc = "Field `SWRESET` writer - Software reset"]
pub type SWRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Fetcher scatter/gather"]
    #[inline(always)]
    pub fn fetcherscattergather(&self) -> FETCHERSCATTERGATHER_R {
        FETCHERSCATTERGATHER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pusher scatter/gather"]
    #[inline(always)]
    pub fn pusherscattergather(&self) -> PUSHERSCATTERGATHER_R {
        PUSHERSCATTERGATHER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop fetcher"]
    #[inline(always)]
    pub fn stopfetcher(&self) -> STOPFETCHER_R {
        STOPFETCHER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stop pusher"]
    #[inline(always)]
    pub fn stoppusher(&self) -> STOPPUSHER_R {
        STOPPUSHER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software reset"]
    #[inline(always)]
    pub fn swreset(&self) -> SWRESET_R {
        SWRESET_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fetcher scatter/gather"]
    #[inline(always)]
    #[must_use]
    pub fn fetcherscattergather(&mut self) -> FETCHERSCATTERGATHER_W<0> {
        FETCHERSCATTERGATHER_W::new(self)
    }
    #[doc = "Bit 1 - Pusher scatter/gather"]
    #[inline(always)]
    #[must_use]
    pub fn pusherscattergather(&mut self) -> PUSHERSCATTERGATHER_W<1> {
        PUSHERSCATTERGATHER_W::new(self)
    }
    #[doc = "Bit 2 - Stop fetcher"]
    #[inline(always)]
    #[must_use]
    pub fn stopfetcher(&mut self) -> STOPFETCHER_W<2> {
        STOPFETCHER_W::new(self)
    }
    #[doc = "Bit 3 - Stop pusher"]
    #[inline(always)]
    #[must_use]
    pub fn stoppusher(&mut self) -> STOPPUSHER_W<3> {
        STOPPUSHER_W::new(self)
    }
    #[doc = "Bit 4 - Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn swreset(&mut self) -> SWRESET_W<4> {
        SWRESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register, called CONFIG in Barco datasheet.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
