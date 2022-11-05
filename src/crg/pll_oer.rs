#[doc = "Register `PLL_OER` reader"]
pub struct R(crate::R<PLL_OER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_OER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_OER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_OER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_OER` writer"]
pub struct W(crate::W<PLL_OER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_OER_SPEC>;
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
impl From<crate::W<PLL_OER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_OER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PLL output flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLOEN_A {
    #[doc = "0: VCO is working but FOUT is low only"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<PLLOEN_A> for bool {
    #[inline(always)]
    fn from(variant: PLLOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLOEN` reader - PLL output flag"]
pub struct PLLOEN_R(crate::FieldReader<bool>);
impl PLLOEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLLOEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLOEN_A {
        match self.bits {
            false => PLLOEN_A::DISABLE,
            true => PLLOEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == PLLOEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == PLLOEN_A::ENABLE
    }
}
impl core::ops::Deref for PLLOEN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLOEN` writer - PLL output flag"]
pub struct PLLOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLOEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLOEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "VCO is working but FOUT is low only"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLLOEN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLLOEN_A::ENABLE)
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
    #[doc = "Bit 0 - PLL output flag"]
    #[inline(always)]
    pub fn plloen(&self) -> PLLOEN_R {
        PLLOEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL output flag"]
    #[inline(always)]
    pub fn plloen(&mut self) -> PLLOEN_W {
        PLLOEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL output enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_oer](index.html) module"]
pub struct PLL_OER_SPEC;
impl crate::RegisterSpec for PLL_OER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_oer::R](R) reader structure"]
impl crate::Readable for PLL_OER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_oer::W](W) writer structure"]
impl crate::Writable for PLL_OER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL_OER to value 0x01"]
impl crate::Resettable for PLL_OER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
