#[doc = "Register `ADCCLK_SSR` reader"]
pub struct R(crate::R<ADCCLK_SSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCCLK_SSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCCLK_SSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCCLK_SSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCCLK_SSR` writer"]
pub struct W(crate::W<ADCCLK_SSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCCLK_SSR_SPEC>;
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
impl From<crate::W<ADCCLK_SSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCCLK_SSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ADCCLK source select bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCSS_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: Output clock of PLL (MCLK)"]
    PLL = 1,
    #[doc = "2: Internal 8MHz RC oscillator clock (RCLK)"]
    INTERNAL = 2,
    #[doc = "3: External oscillator clock (OCLK, 8MHz-24MHz)"]
    EXTERNAL = 3,
}
impl From<ADCSS_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCSS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADCSS` reader - ADCCLK source select bits"]
pub struct ADCSS_R(crate::FieldReader<u8, ADCSS_A>);
impl ADCSS_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCSS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCSS_A {
        match self.bits {
            0 => ADCSS_A::DISABLE,
            1 => ADCSS_A::PLL,
            2 => ADCSS_A::INTERNAL,
            3 => ADCSS_A::EXTERNAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ADCSS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        **self == ADCSS_A::PLL
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        **self == ADCSS_A::INTERNAL
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        **self == ADCSS_A::EXTERNAL
    }
}
impl core::ops::Deref for ADCSS_R {
    type Target = crate::FieldReader<u8, ADCSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCSS` writer - ADCCLK source select bits"]
pub struct ADCSS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCSS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADCSS_A::DISABLE)
    }
    #[doc = "Output clock of PLL (MCLK)"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(ADCSS_A::PLL)
    }
    #[doc = "Internal 8MHz RC oscillator clock (RCLK)"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(ADCSS_A::INTERNAL)
    }
    #[doc = "External oscillator clock (OCLK, 8MHz-24MHz)"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(ADCSS_A::EXTERNAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - ADCCLK source select bits"]
    #[inline(always)]
    pub fn adcss(&self) -> ADCSS_R {
        ADCSS_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADCCLK source select bits"]
    #[inline(always)]
    pub fn adcss(&mut self) -> ADCSS_W {
        ADCSS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADCCLK source select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcclk_ssr](index.html) module"]
pub struct ADCCLK_SSR_SPEC;
impl crate::RegisterSpec for ADCCLK_SSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcclk_ssr::R](R) reader structure"]
impl crate::Readable for ADCCLK_SSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcclk_ssr::W](W) writer structure"]
impl crate::Writable for ADCCLK_SSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCCLK_SSR to value 0x01"]
impl crate::Resettable for ADCCLK_SSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
