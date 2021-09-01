#[doc = "Register `PLL_BPR` reader"]
pub struct R(crate::R<PLL_BPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_BPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_BPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_BPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_BPR` writer"]
pub struct W(crate::W<PLL_BPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_BPR_SPEC>;
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
impl From<crate::W<PLL_BPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_BPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLBP` reader - PLL bypass flag"]
pub struct PLLBP_R(crate::FieldReader<bool, bool>);
impl PLLBP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLBP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLBP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLBP` writer - PLL bypass flag"]
pub struct PLLBP_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLBP_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PLL bypass flag"]
    #[inline(always)]
    pub fn pllbp(&self) -> PLLBP_R {
        PLLBP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL bypass flag"]
    #[inline(always)]
    pub fn pllbp(&mut self) -> PLLBP_W {
        PLLBP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL bypass register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_bpr](index.html) module"]
pub struct PLL_BPR_SPEC;
impl crate::RegisterSpec for PLL_BPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_bpr::R](R) reader structure"]
impl crate::Readable for PLL_BPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_bpr::W](W) writer structure"]
impl crate::Writable for PLL_BPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL_BPR to value 0"]
impl crate::Resettable for PLL_BPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
