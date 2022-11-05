#[doc = "Register `RTC_SSR` reader"]
pub struct R(crate::R<RTC_SSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_SSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_SSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_SSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_SSR` writer"]
pub struct W(crate::W<RTC_SSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_SSR_SPEC>;
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
impl From<crate::W<RTC_SSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_SSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RTC source select bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCSEL_A {
    #[doc = "0: `0`"]
    RTCCLK_HS = 0,
    #[doc = "1: Low speed external oscillator clock"]
    _32K_OSC_CLK = 1,
}
impl From<RTCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: RTCSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCSEL` reader - RTC source select bits"]
pub struct RTCSEL_R(crate::FieldReader<bool>);
impl RTCSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCSEL_A {
        match self.bits {
            false => RTCSEL_A::RTCCLK_HS,
            true => RTCSEL_A::_32K_OSC_CLK,
        }
    }
    #[doc = "Checks if the value of the field is `RTCCLK_HS`"]
    #[inline(always)]
    pub fn is_rtcclk_hs(&self) -> bool {
        **self == RTCSEL_A::RTCCLK_HS
    }
    #[doc = "Checks if the value of the field is `_32K_OSC_CLK`"]
    #[inline(always)]
    pub fn is_32k_osc_clk(&self) -> bool {
        **self == RTCSEL_A::_32K_OSC_CLK
    }
}
impl core::ops::Deref for RTCSEL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCSEL` writer - RTC source select bits"]
pub struct RTCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rtcclk_hs(self) -> &'a mut W {
        self.variant(RTCSEL_A::RTCCLK_HS)
    }
    #[doc = "Low speed external oscillator clock"]
    #[inline(always)]
    pub fn _32k_osc_clk(self) -> &'a mut W {
        self.variant(RTCSEL_A::_32K_OSC_CLK)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RTC source select bits"]
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC source select bits"]
    #[inline(always)]
    pub fn rtcsel(&mut self) -> RTCSEL_W {
        RTCSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTCCLK 32K select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_ssr](index.html) module"]
pub struct RTC_SSR_SPEC;
impl crate::RegisterSpec for RTC_SSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_ssr::R](R) reader structure"]
impl crate::Readable for RTC_SSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_ssr::W](W) writer structure"]
impl crate::Writable for RTC_SSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_SSR to value 0"]
impl crate::Resettable for RTC_SSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
