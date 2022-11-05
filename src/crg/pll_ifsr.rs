#[doc = "Register `PLL_IFSR` reader"]
pub struct R(crate::R<PLL_IFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_IFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_IFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_IFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_IFSR` writer"]
pub struct W(crate::W<PLL_IFSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_IFSR_SPEC>;
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
impl From<crate::W<PLL_IFSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_IFSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PLL input clock source select flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLIS_A {
    #[doc = "0: Internal 8MHz RC oscillator clock (RCLK)"]
    INTERNAL = 0,
    #[doc = "1: External oscillator clock (OCLK, 8MHz-24MHz)"]
    EXTERNAL = 1,
}
impl From<PLLIS_A> for bool {
    #[inline(always)]
    fn from(variant: PLLIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLIS` reader - PLL input clock source select flag"]
pub struct PLLIS_R(crate::FieldReader<bool>);
impl PLLIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLLIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLIS_A {
        match self.bits {
            false => PLLIS_A::INTERNAL,
            true => PLLIS_A::EXTERNAL,
        }
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        **self == PLLIS_A::INTERNAL
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        **self == PLLIS_A::EXTERNAL
    }
}
impl core::ops::Deref for PLLIS_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLIS` writer - PLL input clock source select flag"]
pub struct PLLIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal 8MHz RC oscillator clock (RCLK)"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(PLLIS_A::INTERNAL)
    }
    #[doc = "External oscillator clock (OCLK, 8MHz-24MHz)"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(PLLIS_A::EXTERNAL)
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
    #[doc = "Bit 0 - PLL input clock source select flag"]
    #[inline(always)]
    pub fn pllis(&self) -> PLLIS_R {
        PLLIS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL input clock source select flag"]
    #[inline(always)]
    pub fn pllis(&mut self) -> PLLIS_W {
        PLLIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL input frequency select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_ifsr](index.html) module"]
pub struct PLL_IFSR_SPEC;
impl crate::RegisterSpec for PLL_IFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_ifsr::R](R) reader structure"]
impl crate::Readable for PLL_IFSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_ifsr::W](W) writer structure"]
impl crate::Writable for PLL_IFSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL_IFSR to value 0"]
impl crate::Resettable for PLL_IFSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
