#[doc = "Register `PDMR` reader"]
pub struct R(crate::R<PDMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMR` writer"]
pub struct W(crate::W<PDMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMR_SPEC>;
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
impl From<crate::W<PDMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Periodic Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDM_A {
    #[doc = "0: `0`"]
    PERIODIC = 0,
    #[doc = "1: `1`"]
    ONESHOT = 1,
}
impl From<PDM_A> for bool {
    #[inline(always)]
    fn from(variant: PDM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDM` reader - Periodic Mode"]
pub struct PDM_R(crate::FieldReader<bool, PDM_A>);
impl PDM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDM_A {
        match self.bits {
            false => PDM_A::PERIODIC,
            true => PDM_A::ONESHOT,
        }
    }
    #[doc = "Checks if the value of the field is `PERIODIC`"]
    #[inline(always)]
    pub fn is_periodic(&self) -> bool {
        **self == PDM_A::PERIODIC
    }
    #[doc = "Checks if the value of the field is `ONESHOT`"]
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        **self == PDM_A::ONESHOT
    }
}
impl core::ops::Deref for PDM_R {
    type Target = crate::FieldReader<bool, PDM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDM` writer - Periodic Mode"]
pub struct PDM_W<'a> {
    w: &'a mut W,
}
impl<'a> PDM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn periodic(self) -> &'a mut W {
        self.variant(PDM_A::PERIODIC)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut W {
        self.variant(PDM_A::ONESHOT)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Periodic Mode"]
    #[inline(always)]
    pub fn pdm(&self) -> PDM_R {
        PDM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Periodic Mode"]
    #[inline(always)]
    pub fn pdm(&mut self) -> PDM_W {
        PDM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Periodic mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdmr](index.html) module"]
pub struct PDMR_SPEC;
impl crate::RegisterSpec for PDMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdmr::R](R) reader structure"]
impl crate::Readable for PDMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdmr::W](W) writer structure"]
impl crate::Writable for PDMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMR to value 0"]
impl crate::Resettable for PDMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
