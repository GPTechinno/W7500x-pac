#[doc = "Register `ERR_CLR` reader"]
pub struct R(crate::R<ERR_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERR_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERR_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERR_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERR_CLR` writer"]
pub struct W(crate::W<ERR_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERR_CLR_SPEC>;
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
impl From<crate::W<ERR_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERR_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Returns the status of DMA_ERR, or set the signal LOW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR_CLR_A {
    #[doc = "0: `0`"]
    LOW = 0,
    #[doc = "1: `1`"]
    HIGH = 1,
}
impl From<ERR_CLR_A> for bool {
    #[inline(always)]
    fn from(variant: ERR_CLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR_CLR` reader - Returns the status of DMA_ERR, or set the signal LOW"]
pub struct ERR_CLR_R(crate::FieldReader<bool, ERR_CLR_A>);
impl ERR_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERR_CLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR_CLR_A {
        match self.bits {
            false => ERR_CLR_A::LOW,
            true => ERR_CLR_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == ERR_CLR_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == ERR_CLR_A::HIGH
    }
}
impl core::ops::Deref for ERR_CLR_R {
    type Target = crate::FieldReader<bool, ERR_CLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR_CLR` writer - Returns the status of DMA_ERR, or set the signal LOW"]
pub struct ERR_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR_CLR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ERR_CLR_A::LOW)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ERR_CLR_A::HIGH)
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
    #[doc = "Bit 0 - Returns the status of DMA_ERR, or set the signal LOW"]
    #[inline(always)]
    pub fn err_clr(&self) -> ERR_CLR_R {
        ERR_CLR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Returns the status of DMA_ERR, or set the signal LOW"]
    #[inline(always)]
    pub fn err_clr(&mut self) -> ERR_CLR_W {
        ERR_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus Error Clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [err_clr](index.html) module"]
pub struct ERR_CLR_SPEC;
impl crate::RegisterSpec for ERR_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [err_clr::R](R) reader structure"]
impl crate::Readable for ERR_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [err_clr::W](W) writer structure"]
impl crate::Writable for ERR_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERR_CLR to value 0"]
impl crate::Resettable for ERR_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
