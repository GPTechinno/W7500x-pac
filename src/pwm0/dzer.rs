#[doc = "Register `DZER` reader"]
pub struct R(crate::R<DZER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DZER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DZER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DZER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DZER` writer"]
pub struct W(crate::W<DZER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DZER_SPEC>;
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
impl From<crate::W<DZER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DZER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Dead Zone Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DZE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<DZE_A> for bool {
    #[inline(always)]
    fn from(variant: DZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DZE` reader - Dead Zone Enable"]
pub struct DZE_R(crate::FieldReader<bool>);
impl DZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DZE_A {
        match self.bits {
            false => DZE_A::DISABLE,
            true => DZE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == DZE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == DZE_A::ENABLE
    }
}
impl core::ops::Deref for DZE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DZE` writer - Dead Zone Enable"]
pub struct DZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DZE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DZE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DZE_A::ENABLE)
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
    #[doc = "Bit 0 - Dead Zone Enable"]
    #[inline(always)]
    pub fn dze(&self) -> DZE_R {
        DZE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Dead Zone Enable"]
    #[inline(always)]
    pub fn dze(&mut self) -> DZE_W {
        DZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dead-zone enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dzer](index.html) module"]
pub struct DZER_SPEC;
impl crate::RegisterSpec for DZER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dzer::R](R) reader structure"]
impl crate::Readable for DZER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dzer::W](W) writer structure"]
impl crate::Writable for DZER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DZER to value 0"]
impl crate::Resettable for DZER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
