#[doc = "Register `TRIM` reader"]
pub struct R(crate::R<TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIM` writer"]
pub struct W(crate::W<TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIM_SPEC>;
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
impl From<crate::W<TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WARMUPTIME` reader - Warm up time"]
pub type WARMUPTIME_R = crate::FieldReader<u8, WARMUPTIME_A>;
#[doc = "Warm up time\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WARMUPTIME_A {
    #[doc = "0: Warm up cycle = 72; 3.6us @20 MHz"]
    WUCYCLES72 = 0,
    #[doc = "1: Warm up cycle = 96; 4.8us @ 20 MHz"]
    WUCYCLES96 = 1,
    #[doc = "2: Warm up cycle = 128; 6.4us @ 20 MHz"]
    WUCYCLES128 = 2,
    #[doc = "3: Warm up cycle = 160; 8.0us @ 20 MHz"]
    WUCYCLES160 = 3,
}
impl From<WARMUPTIME_A> for u8 {
    #[inline(always)]
    fn from(variant: WARMUPTIME_A) -> Self {
        variant as _
    }
}
impl WARMUPTIME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WARMUPTIME_A {
        match self.bits {
            0 => WARMUPTIME_A::WUCYCLES72,
            1 => WARMUPTIME_A::WUCYCLES96,
            2 => WARMUPTIME_A::WUCYCLES128,
            3 => WARMUPTIME_A::WUCYCLES160,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `WUCYCLES72`"]
    #[inline(always)]
    pub fn is_wucycles72(&self) -> bool {
        *self == WARMUPTIME_A::WUCYCLES72
    }
    #[doc = "Checks if the value of the field is `WUCYCLES96`"]
    #[inline(always)]
    pub fn is_wucycles96(&self) -> bool {
        *self == WARMUPTIME_A::WUCYCLES96
    }
    #[doc = "Checks if the value of the field is `WUCYCLES128`"]
    #[inline(always)]
    pub fn is_wucycles128(&self) -> bool {
        *self == WARMUPTIME_A::WUCYCLES128
    }
    #[doc = "Checks if the value of the field is `WUCYCLES160`"]
    #[inline(always)]
    pub fn is_wucycles160(&self) -> bool {
        *self == WARMUPTIME_A::WUCYCLES160
    }
}
#[doc = "Field `WARMUPTIME` writer - Warm up time"]
pub type WARMUPTIME_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TRIM_SPEC, u8, WARMUPTIME_A, 2, O>;
impl<'a, const O: u8> WARMUPTIME_W<'a, O> {
    #[doc = "Warm up cycle = 72; 3.6us @20 MHz"]
    #[inline(always)]
    pub fn wucycles72(self) -> &'a mut W {
        self.variant(WARMUPTIME_A::WUCYCLES72)
    }
    #[doc = "Warm up cycle = 96; 4.8us @ 20 MHz"]
    #[inline(always)]
    pub fn wucycles96(self) -> &'a mut W {
        self.variant(WARMUPTIME_A::WUCYCLES96)
    }
    #[doc = "Warm up cycle = 128; 6.4us @ 20 MHz"]
    #[inline(always)]
    pub fn wucycles128(self) -> &'a mut W {
        self.variant(WARMUPTIME_A::WUCYCLES128)
    }
    #[doc = "Warm up cycle = 160; 8.0us @ 20 MHz"]
    #[inline(always)]
    pub fn wucycles160(self) -> &'a mut W {
        self.variant(WARMUPTIME_A::WUCYCLES160)
    }
}
#[doc = "Field `FLOATVDDCPLO` reader - Float VDDCP Low Power"]
pub type FLOATVDDCPLO_R = crate::BitReader<bool>;
#[doc = "Field `FLOATVDDCPLO` writer - Float VDDCP Low Power"]
pub type FLOATVDDCPLO_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIM_SPEC, bool, O>;
#[doc = "Field `FLOATVDDCPHI` reader - Float VDDCP High Power"]
pub type FLOATVDDCPHI_R = crate::BitReader<bool>;
#[doc = "Field `FLOATVDDCPHI` writer - Float VDDCP High Power"]
pub type FLOATVDDCPHI_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIM_SPEC, bool, O>;
#[doc = "Field `BYPASSDIV2LO` reader - Bypass Div2 Low Power"]
pub type BYPASSDIV2LO_R = crate::BitReader<bool>;
#[doc = "Field `BYPASSDIV2LO` writer - Bypass Div2 Low Power"]
pub type BYPASSDIV2LO_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIM_SPEC, bool, O>;
#[doc = "Field `BYPASSDIV2HI` reader - Bypass Div2 High Power"]
pub type BYPASSDIV2HI_R = crate::BitReader<bool>;
#[doc = "Field `BYPASSDIV2HI` writer - Bypass Div2 High Power"]
pub type BYPASSDIV2HI_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIM_SPEC, bool, O>;
#[doc = "Field `BUMP0P5XLO` reader - Bump 0.5X Low Power"]
pub type BUMP0P5XLO_R = crate::BitReader<bool>;
#[doc = "Field `BUMP0P5XLO` writer - Bump 0.5X Low Power"]
pub type BUMP0P5XLO_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIM_SPEC, bool, O>;
#[doc = "Field `BUMP0P5XHI` reader - Bump 0.5X High Power"]
pub type BUMP0P5XHI_R = crate::BitReader<bool>;
#[doc = "Field `BUMP0P5XHI` writer - Bump 0.5X High Power"]
pub type BUMP0P5XHI_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIM_SPEC, bool, O>;
#[doc = "Field `BIAS2XLO` reader - Bias 2x Low Power"]
pub type BIAS2XLO_R = crate::BitReader<bool>;
#[doc = "Field `BIAS2XLO` writer - Bias 2x Low Power"]
pub type BIAS2XLO_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIM_SPEC, bool, O>;
#[doc = "Field `BIAS2XHI` reader - Bias 2x High Power"]
pub type BIAS2XHI_R = crate::BitReader<bool>;
#[doc = "Field `BIAS2XHI` writer - Bias 2x High Power"]
pub type BIAS2XHI_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIM_SPEC, bool, O>;
#[doc = "Field `VOLTAGECTRLLO` reader - Charge Pump Voltage Control Low Power"]
pub type VOLTAGECTRLLO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VOLTAGECTRLLO` writer - Charge Pump Voltage Control Low Power"]
pub type VOLTAGECTRLLO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRIM_SPEC, u8, u8, 2, O>;
#[doc = "Field `VOLTAGECTRLHI` reader - Charge Pump Voltage Control High Power"]
pub type VOLTAGECTRLHI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VOLTAGECTRLHI` writer - Charge Pump Voltage Control High Power"]
pub type VOLTAGECTRLHI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRIM_SPEC, u8, u8, 2, O>;
#[doc = "Field `BIASCTRLLO` reader - Bias Control Low Power"]
pub type BIASCTRLLO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BIASCTRLLO` writer - Bias Control Low Power"]
pub type BIASCTRLLO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRIM_SPEC, u8, u8, 3, O>;
#[doc = "Field `BIASCTRLLOCONT` reader - Bias Control Low Power Continuous"]
pub type BIASCTRLLOCONT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BIASCTRLLOCONT` writer - Bias Control Low Power Continuous"]
pub type BIASCTRLLOCONT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRIM_SPEC, u8, u8, 3, O>;
#[doc = "Field `BIASCTRLHI` reader - Bias Control High Power"]
pub type BIASCTRLHI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BIASCTRLHI` writer - Bias Control High Power"]
pub type BIASCTRLHI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRIM_SPEC, u8, u8, 3, O>;
#[doc = "Field `PUMPCAPLO` reader - Pump Cap Low Power"]
pub type PUMPCAPLO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUMPCAPLO` writer - Pump Cap Low Power"]
pub type PUMPCAPLO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRIM_SPEC, u8, u8, 3, O>;
#[doc = "Field `PUMPCAPHI` reader - Pump Cap High Power"]
pub type PUMPCAPHI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUMPCAPHI` writer - Pump Cap High Power"]
pub type PUMPCAPHI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRIM_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:1 - Warm up time"]
    #[inline(always)]
    pub fn warmuptime(&self) -> WARMUPTIME_R {
        WARMUPTIME_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Float VDDCP Low Power"]
    #[inline(always)]
    pub fn floatvddcplo(&self) -> FLOATVDDCPLO_R {
        FLOATVDDCPLO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Float VDDCP High Power"]
    #[inline(always)]
    pub fn floatvddcphi(&self) -> FLOATVDDCPHI_R {
        FLOATVDDCPHI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bypass Div2 Low Power"]
    #[inline(always)]
    pub fn bypassdiv2lo(&self) -> BYPASSDIV2LO_R {
        BYPASSDIV2LO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bypass Div2 High Power"]
    #[inline(always)]
    pub fn bypassdiv2hi(&self) -> BYPASSDIV2HI_R {
        BYPASSDIV2HI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bump 0.5X Low Power"]
    #[inline(always)]
    pub fn bump0p5xlo(&self) -> BUMP0P5XLO_R {
        BUMP0P5XLO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bump 0.5X High Power"]
    #[inline(always)]
    pub fn bump0p5xhi(&self) -> BUMP0P5XHI_R {
        BUMP0P5XHI_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bias 2x Low Power"]
    #[inline(always)]
    pub fn bias2xlo(&self) -> BIAS2XLO_R {
        BIAS2XLO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Bias 2x High Power"]
    #[inline(always)]
    pub fn bias2xhi(&self) -> BIAS2XHI_R {
        BIAS2XHI_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Charge Pump Voltage Control Low Power"]
    #[inline(always)]
    pub fn voltagectrllo(&self) -> VOLTAGECTRLLO_R {
        VOLTAGECTRLLO_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 13:14 - Charge Pump Voltage Control High Power"]
    #[inline(always)]
    pub fn voltagectrlhi(&self) -> VOLTAGECTRLHI_R {
        VOLTAGECTRLHI_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:17 - Bias Control Low Power"]
    #[inline(always)]
    pub fn biasctrllo(&self) -> BIASCTRLLO_R {
        BIASCTRLLO_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Bias Control Low Power Continuous"]
    #[inline(always)]
    pub fn biasctrllocont(&self) -> BIASCTRLLOCONT_R {
        BIASCTRLLOCONT_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Bias Control High Power"]
    #[inline(always)]
    pub fn biasctrlhi(&self) -> BIASCTRLHI_R {
        BIASCTRLHI_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Pump Cap Low Power"]
    #[inline(always)]
    pub fn pumpcaplo(&self) -> PUMPCAPLO_R {
        PUMPCAPLO_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Pump Cap High Power"]
    #[inline(always)]
    pub fn pumpcaphi(&self) -> PUMPCAPHI_R {
        PUMPCAPHI_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Warm up time"]
    #[inline(always)]
    #[must_use]
    pub fn warmuptime(&mut self) -> WARMUPTIME_W<0> {
        WARMUPTIME_W::new(self)
    }
    #[doc = "Bit 2 - Float VDDCP Low Power"]
    #[inline(always)]
    #[must_use]
    pub fn floatvddcplo(&mut self) -> FLOATVDDCPLO_W<2> {
        FLOATVDDCPLO_W::new(self)
    }
    #[doc = "Bit 3 - Float VDDCP High Power"]
    #[inline(always)]
    #[must_use]
    pub fn floatvddcphi(&mut self) -> FLOATVDDCPHI_W<3> {
        FLOATVDDCPHI_W::new(self)
    }
    #[doc = "Bit 4 - Bypass Div2 Low Power"]
    #[inline(always)]
    #[must_use]
    pub fn bypassdiv2lo(&mut self) -> BYPASSDIV2LO_W<4> {
        BYPASSDIV2LO_W::new(self)
    }
    #[doc = "Bit 5 - Bypass Div2 High Power"]
    #[inline(always)]
    #[must_use]
    pub fn bypassdiv2hi(&mut self) -> BYPASSDIV2HI_W<5> {
        BYPASSDIV2HI_W::new(self)
    }
    #[doc = "Bit 6 - Bump 0.5X Low Power"]
    #[inline(always)]
    #[must_use]
    pub fn bump0p5xlo(&mut self) -> BUMP0P5XLO_W<6> {
        BUMP0P5XLO_W::new(self)
    }
    #[doc = "Bit 7 - Bump 0.5X High Power"]
    #[inline(always)]
    #[must_use]
    pub fn bump0p5xhi(&mut self) -> BUMP0P5XHI_W<7> {
        BUMP0P5XHI_W::new(self)
    }
    #[doc = "Bit 8 - Bias 2x Low Power"]
    #[inline(always)]
    #[must_use]
    pub fn bias2xlo(&mut self) -> BIAS2XLO_W<8> {
        BIAS2XLO_W::new(self)
    }
    #[doc = "Bit 9 - Bias 2x High Power"]
    #[inline(always)]
    #[must_use]
    pub fn bias2xhi(&mut self) -> BIAS2XHI_W<9> {
        BIAS2XHI_W::new(self)
    }
    #[doc = "Bits 10:11 - Charge Pump Voltage Control Low Power"]
    #[inline(always)]
    #[must_use]
    pub fn voltagectrllo(&mut self) -> VOLTAGECTRLLO_W<10> {
        VOLTAGECTRLLO_W::new(self)
    }
    #[doc = "Bits 13:14 - Charge Pump Voltage Control High Power"]
    #[inline(always)]
    #[must_use]
    pub fn voltagectrlhi(&mut self) -> VOLTAGECTRLHI_W<13> {
        VOLTAGECTRLHI_W::new(self)
    }
    #[doc = "Bits 15:17 - Bias Control Low Power"]
    #[inline(always)]
    #[must_use]
    pub fn biasctrllo(&mut self) -> BIASCTRLLO_W<15> {
        BIASCTRLLO_W::new(self)
    }
    #[doc = "Bits 18:20 - Bias Control Low Power Continuous"]
    #[inline(always)]
    #[must_use]
    pub fn biasctrllocont(&mut self) -> BIASCTRLLOCONT_W<18> {
        BIASCTRLLOCONT_W::new(self)
    }
    #[doc = "Bits 21:23 - Bias Control High Power"]
    #[inline(always)]
    #[must_use]
    pub fn biasctrlhi(&mut self) -> BIASCTRLHI_W<21> {
        BIASCTRLHI_W::new(self)
    }
    #[doc = "Bits 24:26 - Pump Cap Low Power"]
    #[inline(always)]
    #[must_use]
    pub fn pumpcaplo(&mut self) -> PUMPCAPLO_W<24> {
        PUMPCAPLO_W::new(self)
    }
    #[doc = "Bits 28:30 - Pump Cap High Power"]
    #[inline(always)]
    #[must_use]
    pub fn pumpcaphi(&mut self) -> PUMPCAPHI_W<28> {
        PUMPCAPHI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trim\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim](index.html) module"]
pub struct TRIM_SPEC;
impl crate::RegisterSpec for TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trim::R](R) reader structure"]
impl crate::Readable for TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trim::W](W) writer structure"]
impl crate::Writable for TRIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIM to value 0x77e4_4aa1"]
impl crate::Resettable for TRIM_SPEC {
    const RESET_VALUE: Self::Ux = 0x77e4_4aa1;
}
