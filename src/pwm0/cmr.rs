#[doc = "Register `CMR` reader"]
pub struct R(crate::R<CMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMR` writer"]
pub struct W(crate::W<CMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMR_SPEC>;
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
impl From<crate::W<CMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Capture mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CM_A {
    #[doc = "0: `0`"]
    RISINGEDGE = 0,
    #[doc = "1: `1`"]
    FALLINGEDGE = 1,
}
impl From<CM_A> for bool {
    #[inline(always)]
    fn from(variant: CM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CM` reader - Capture mode"]
pub struct CM_R(crate::FieldReader<bool>);
impl CM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CM_A {
        match self.bits {
            false => CM_A::RISINGEDGE,
            true => CM_A::FALLINGEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `RISINGEDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        **self == CM_A::RISINGEDGE
    }
    #[doc = "Checks if the value of the field is `FALLINGEDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        **self == CM_A::FALLINGEDGE
    }
}
impl core::ops::Deref for CM_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CM` writer - Capture mode"]
pub struct CM_W<'a> {
    w: &'a mut W,
}
impl<'a> CM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(CM_A::RISINGEDGE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(CM_A::FALLINGEDGE)
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
    #[doc = "Bit 0 - Capture mode"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture mode"]
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W {
        CM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmr](index.html) module"]
pub struct CMR_SPEC;
impl crate::RegisterSpec for CMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmr::R](R) reader structure"]
impl crate::Readable for CMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmr::W](W) writer structure"]
impl crate::Writable for CMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMR to value 0"]
impl crate::Resettable for CMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
