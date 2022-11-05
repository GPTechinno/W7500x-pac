#[doc = "Register `UDMR` reader"]
pub struct R(crate::R<UDMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UDMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UDMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UDMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UDMR` writer"]
pub struct W(crate::W<UDMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UDMR_SPEC>;
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
impl From<crate::W<UDMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UDMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Up-Down mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UDM_A {
    #[doc = "0: `0`"]
    UP = 0,
    #[doc = "1: `1`"]
    DOWN = 1,
}
impl From<UDM_A> for bool {
    #[inline(always)]
    fn from(variant: UDM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDM` reader - Up-Down mode"]
pub struct UDM_R(crate::FieldReader<bool>);
impl UDM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UDM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UDM_A {
        match self.bits {
            false => UDM_A::UP,
            true => UDM_A::DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        **self == UDM_A::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        **self == UDM_A::DOWN
    }
}
impl core::ops::Deref for UDM_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UDM` writer - Up-Down mode"]
pub struct UDM_W<'a> {
    w: &'a mut W,
}
impl<'a> UDM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UDM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(UDM_A::UP)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(UDM_A::DOWN)
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
    #[doc = "Bit 0 - Up-Down mode"]
    #[inline(always)]
    pub fn udm(&self) -> UDM_R {
        UDM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Up-Down mode"]
    #[inline(always)]
    pub fn udm(&mut self) -> UDM_W {
        UDM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Up-Down mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmr](index.html) module"]
pub struct UDMR_SPEC;
impl crate::RegisterSpec for UDMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [udmr::R](R) reader structure"]
impl crate::Readable for UDMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [udmr::W](W) writer structure"]
impl crate::Writable for UDMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UDMR to value 0"]
impl crate::Resettable for UDMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
