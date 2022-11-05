#[doc = "Register `RTC_HS_PVSR` reader"]
pub struct R(crate::R<RTC_HS_PVSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_HS_PVSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_HS_PVSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_HS_PVSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_HS_PVSR` writer"]
pub struct W(crate::W<RTC_HS_PVSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_HS_PVSR_SPEC>;
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
impl From<crate::W<RTC_HS_PVSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_HS_PVSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RTC High Speed prescale value select bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCPRE_A {
    #[doc = "0: 1/1"]
    BYPASS = 0,
    #[doc = "1: 1/2"]
    HALF = 1,
    #[doc = "2: 1/4"]
    BY4 = 2,
    #[doc = "3: 1/8"]
    BY8 = 3,
    #[doc = "4: 1/16"]
    BY16 = 4,
    #[doc = "5: 1/32"]
    BY32 = 5,
    #[doc = "6: 1/64"]
    BY64 = 6,
    #[doc = "7: 1/128"]
    BY128 = 7,
}
impl From<RTCPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCPRE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RTCPRE` reader - RTC High Speed prescale value select bits"]
pub struct RTCPRE_R(crate::FieldReader<u8>);
impl RTCPRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTCPRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCPRE_A {
        match self.bits {
            0 => RTCPRE_A::BYPASS,
            1 => RTCPRE_A::HALF,
            2 => RTCPRE_A::BY4,
            3 => RTCPRE_A::BY8,
            4 => RTCPRE_A::BY16,
            5 => RTCPRE_A::BY32,
            6 => RTCPRE_A::BY64,
            7 => RTCPRE_A::BY128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        **self == RTCPRE_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        **self == RTCPRE_A::HALF
    }
    #[doc = "Checks if the value of the field is `BY4`"]
    #[inline(always)]
    pub fn is_by4(&self) -> bool {
        **self == RTCPRE_A::BY4
    }
    #[doc = "Checks if the value of the field is `BY8`"]
    #[inline(always)]
    pub fn is_by8(&self) -> bool {
        **self == RTCPRE_A::BY8
    }
    #[doc = "Checks if the value of the field is `BY16`"]
    #[inline(always)]
    pub fn is_by16(&self) -> bool {
        **self == RTCPRE_A::BY16
    }
    #[doc = "Checks if the value of the field is `BY32`"]
    #[inline(always)]
    pub fn is_by32(&self) -> bool {
        **self == RTCPRE_A::BY32
    }
    #[doc = "Checks if the value of the field is `BY64`"]
    #[inline(always)]
    pub fn is_by64(&self) -> bool {
        **self == RTCPRE_A::BY64
    }
    #[doc = "Checks if the value of the field is `BY128`"]
    #[inline(always)]
    pub fn is_by128(&self) -> bool {
        **self == RTCPRE_A::BY128
    }
}
impl core::ops::Deref for RTCPRE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCPRE` writer - RTC High Speed prescale value select bits"]
pub struct RTCPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCPRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCPRE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1/1"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(RTCPRE_A::BYPASS)
    }
    #[doc = "1/2"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(RTCPRE_A::HALF)
    }
    #[doc = "1/4"]
    #[inline(always)]
    pub fn by4(self) -> &'a mut W {
        self.variant(RTCPRE_A::BY4)
    }
    #[doc = "1/8"]
    #[inline(always)]
    pub fn by8(self) -> &'a mut W {
        self.variant(RTCPRE_A::BY8)
    }
    #[doc = "1/16"]
    #[inline(always)]
    pub fn by16(self) -> &'a mut W {
        self.variant(RTCPRE_A::BY16)
    }
    #[doc = "1/32"]
    #[inline(always)]
    pub fn by32(self) -> &'a mut W {
        self.variant(RTCPRE_A::BY32)
    }
    #[doc = "1/64"]
    #[inline(always)]
    pub fn by64(self) -> &'a mut W {
        self.variant(RTCPRE_A::BY64)
    }
    #[doc = "1/128"]
    #[inline(always)]
    pub fn by128(self) -> &'a mut W {
        self.variant(RTCPRE_A::BY128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u32 & 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - RTC High Speed prescale value select bits"]
    #[inline(always)]
    pub fn rtcpre(&self) -> RTCPRE_R {
        RTCPRE_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - RTC High Speed prescale value select bits"]
    #[inline(always)]
    pub fn rtcpre(&mut self) -> RTCPRE_W {
        RTCPRE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTCCLK prescale value select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_hs_pvsr](index.html) module"]
pub struct RTC_HS_PVSR_SPEC;
impl crate::RegisterSpec for RTC_HS_PVSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_hs_pvsr::R](R) reader structure"]
impl crate::Readable for RTC_HS_PVSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_hs_pvsr::W](W) writer structure"]
impl crate::Writable for RTC_HS_PVSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_HS_PVSR to value 0"]
impl crate::Resettable for RTC_HS_PVSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
