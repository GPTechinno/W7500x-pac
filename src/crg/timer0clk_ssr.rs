#[doc = "Register `TIMER0CLK_SSR` reader"]
pub struct R(crate::R<TIMER0CLK_SSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER0CLK_SSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER0CLK_SSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER0CLK_SSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER0CLK_SSR` writer"]
pub struct W(crate::W<TIMER0CLK_SSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER0CLK_SSR_SPEC>;
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
impl From<crate::W<TIMER0CLK_SSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER0CLK_SSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "TIMERnCLK source select bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TCSS_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: Output clock of PLL (MCLK)"]
    PLL = 1,
    #[doc = "2: Internal 8MHz RC oscillator clock (RCLK)"]
    INTERNAL = 2,
    #[doc = "3: External oscillator clock (OCLK, 8MHz-24MHz)"]
    EXTERNAL = 3,
}
impl From<TCSS_A> for u8 {
    #[inline(always)]
    fn from(variant: TCSS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TCSS` reader - TIMERnCLK source select bits"]
pub struct TCSS_R(crate::FieldReader<u8>);
impl TCSS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TCSS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCSS_A {
        match self.bits {
            0 => TCSS_A::DISABLE,
            1 => TCSS_A::PLL,
            2 => TCSS_A::INTERNAL,
            3 => TCSS_A::EXTERNAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TCSS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        **self == TCSS_A::PLL
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        **self == TCSS_A::INTERNAL
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        **self == TCSS_A::EXTERNAL
    }
}
impl core::ops::Deref for TCSS_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCSS` writer - TIMERnCLK source select bits"]
pub struct TCSS_W<'a> {
    w: &'a mut W,
}
impl<'a> TCSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCSS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TCSS_A::DISABLE)
    }
    #[doc = "Output clock of PLL (MCLK)"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(TCSS_A::PLL)
    }
    #[doc = "Internal 8MHz RC oscillator clock (RCLK)"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(TCSS_A::INTERNAL)
    }
    #[doc = "External oscillator clock (OCLK, 8MHz-24MHz)"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(TCSS_A::EXTERNAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - TIMERnCLK source select bits"]
    #[inline(always)]
    pub fn tcss(&self) -> TCSS_R {
        TCSS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - TIMERnCLK source select bits"]
    #[inline(always)]
    pub fn tcss(&mut self) -> TCSS_W {
        TCSS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMER0CLK source select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer0clk_ssr](index.html) module"]
pub struct TIMER0CLK_SSR_SPEC;
impl crate::RegisterSpec for TIMER0CLK_SSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer0clk_ssr::R](R) reader structure"]
impl crate::Readable for TIMER0CLK_SSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer0clk_ssr::W](W) writer structure"]
impl crate::Writable for TIMER0CLK_SSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER0CLK_SSR to value 0x01"]
impl crate::Resettable for TIMER0CLK_SSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
