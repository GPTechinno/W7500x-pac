#[doc = "Register `RUN` reader"]
pub struct R(crate::R<RUN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RUN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RUN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RUN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RUN` writer"]
pub struct W(crate::W<RUN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RUN_SPEC>;
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
impl From<crate::W<RUN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RUN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RUN RNG shift flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUN_A {
    #[doc = "0: `0`"]
    STOP = 0,
    #[doc = "1: `1`"]
    RUN = 1,
}
impl From<RUN_A> for bool {
    #[inline(always)]
    fn from(variant: RUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUN` reader - RUN RNG shift flag"]
pub struct RUN_R(crate::FieldReader<bool>);
impl RUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RUN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUN_A {
        match self.bits {
            false => RUN_A::STOP,
            true => RUN_A::RUN,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == RUN_A::STOP
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        **self == RUN_A::RUN
    }
}
impl core::ops::Deref for RUN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUN` writer - RUN RNG shift flag"]
pub struct RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RUN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(RUN_A::STOP)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(RUN_A::RUN)
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
    #[doc = "Bit 0 - RUN RNG shift flag"]
    #[inline(always)]
    pub fn run(&self) -> RUN_R {
        RUN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RUN RNG shift flag"]
    #[inline(always)]
    pub fn run(&mut self) -> RUN_W {
        RUN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RNG run register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [run](index.html) module"]
pub struct RUN_SPEC;
impl crate::RegisterSpec for RUN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [run::R](R) reader structure"]
impl crate::Readable for RUN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [run::W](W) writer structure"]
impl crate::Writable for RUN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RUN to value 0"]
impl crate::Resettable for RUN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
