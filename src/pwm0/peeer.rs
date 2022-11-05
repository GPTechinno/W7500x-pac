#[doc = "Register `PEEER` reader"]
pub struct R(crate::R<PEEER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEEER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEEER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEEER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEEER` writer"]
pub struct W(crate::W<PEEER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEEER_SPEC>;
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
impl From<crate::W<PEEER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEEER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM output Enable and External input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PEEE_A {
    #[doc = "0: `0`"]
    OUTDISABLEINDISABLE = 0,
    #[doc = "1: `1`"]
    OUTDISABLEINENABLE = 1,
    #[doc = "2: `10`"]
    OUTENABLEINDISABLE = 2,
}
impl From<PEEE_A> for u8 {
    #[inline(always)]
    fn from(variant: PEEE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PEEE` reader - PWM output Enable and External input Enable"]
pub struct PEEE_R(crate::FieldReader<u8>);
impl PEEE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PEEE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PEEE_A> {
        match self.bits {
            0 => Some(PEEE_A::OUTDISABLEINDISABLE),
            1 => Some(PEEE_A::OUTDISABLEINENABLE),
            2 => Some(PEEE_A::OUTENABLEINDISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OUTDISABLEINDISABLE`"]
    #[inline(always)]
    pub fn is_out_disable_in_disable(&self) -> bool {
        **self == PEEE_A::OUTDISABLEINDISABLE
    }
    #[doc = "Checks if the value of the field is `OUTDISABLEINENABLE`"]
    #[inline(always)]
    pub fn is_out_disable_in_enable(&self) -> bool {
        **self == PEEE_A::OUTDISABLEINENABLE
    }
    #[doc = "Checks if the value of the field is `OUTENABLEINDISABLE`"]
    #[inline(always)]
    pub fn is_out_enable_in_disable(&self) -> bool {
        **self == PEEE_A::OUTENABLEINDISABLE
    }
}
impl core::ops::Deref for PEEE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEEE` writer - PWM output Enable and External input Enable"]
pub struct PEEE_W<'a> {
    w: &'a mut W,
}
impl<'a> PEEE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEEE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn out_disable_in_disable(self) -> &'a mut W {
        self.variant(PEEE_A::OUTDISABLEINDISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn out_disable_in_enable(self) -> &'a mut W {
        self.variant(PEEE_A::OUTDISABLEINENABLE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn out_enable_in_disable(self) -> &'a mut W {
        self.variant(PEEE_A::OUTENABLEINDISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - PWM output Enable and External input Enable"]
    #[inline(always)]
    pub fn peee(&self) -> PEEE_R {
        PEEE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PWM output Enable and External input Enable"]
    #[inline(always)]
    pub fn peee(&mut self) -> PEEE_W {
        PEEE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM output enable and external input enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peeer](index.html) module"]
pub struct PEEER_SPEC;
impl crate::RegisterSpec for PEEER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peeer::R](R) reader structure"]
impl crate::Readable for PEEER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peeer::W](W) writer structure"]
impl crate::Writable for PEEER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PEEER to value 0"]
impl crate::Resettable for PEEER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
