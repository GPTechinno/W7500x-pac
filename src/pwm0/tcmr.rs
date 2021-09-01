#[doc = "Register `TCMR` reader"]
pub struct R(crate::R<TCMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCMR` writer"]
pub struct W(crate::W<TCMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCMR_SPEC>;
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
impl From<crate::W<TCMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "TCM\\[1:0\\]
bits (Timer/Counter mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TCM_A {
    #[doc = "0: `0`"]
    TIMER = 0,
    #[doc = "1: `1`"]
    COUNTERRISING = 1,
    #[doc = "2: `10`"]
    COUNTERFALLING = 2,
    #[doc = "3: `11`"]
    COUNTERTOGGLE = 3,
}
impl From<TCM_A> for u8 {
    #[inline(always)]
    fn from(variant: TCM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TCM` reader - TCM\\[1:0\\]
bits (Timer/Counter mode)"]
pub struct TCM_R(crate::FieldReader<u8, TCM_A>);
impl TCM_R {
    pub(crate) fn new(bits: u8) -> Self {
        TCM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCM_A {
        match self.bits {
            0 => TCM_A::TIMER,
            1 => TCM_A::COUNTERRISING,
            2 => TCM_A::COUNTERFALLING,
            3 => TCM_A::COUNTERTOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        **self == TCM_A::TIMER
    }
    #[doc = "Checks if the value of the field is `COUNTERRISING`"]
    #[inline(always)]
    pub fn is_counter_rising(&self) -> bool {
        **self == TCM_A::COUNTERRISING
    }
    #[doc = "Checks if the value of the field is `COUNTERFALLING`"]
    #[inline(always)]
    pub fn is_counter_falling(&self) -> bool {
        **self == TCM_A::COUNTERFALLING
    }
    #[doc = "Checks if the value of the field is `COUNTERTOGGLE`"]
    #[inline(always)]
    pub fn is_counter_toggle(&self) -> bool {
        **self == TCM_A::COUNTERTOGGLE
    }
}
impl core::ops::Deref for TCM_R {
    type Target = crate::FieldReader<u8, TCM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCM` writer - TCM\\[1:0\\]
bits (Timer/Counter mode)"]
pub struct TCM_W<'a> {
    w: &'a mut W,
}
impl<'a> TCM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCM_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut W {
        self.variant(TCM_A::TIMER)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn counter_rising(self) -> &'a mut W {
        self.variant(TCM_A::COUNTERRISING)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn counter_falling(self) -> &'a mut W {
        self.variant(TCM_A::COUNTERFALLING)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn counter_toggle(self) -> &'a mut W {
        self.variant(TCM_A::COUNTERTOGGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - TCM\\[1:0\\]
bits (Timer/Counter mode)"]
    #[inline(always)]
    pub fn tcm(&self) -> TCM_R {
        TCM_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - TCM\\[1:0\\]
bits (Timer/Counter mode)"]
    #[inline(always)]
    pub fn tcm(&mut self) -> TCM_W {
        TCM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcmr](index.html) module"]
pub struct TCMR_SPEC;
impl crate::RegisterSpec for TCMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcmr::R](R) reader structure"]
impl crate::Readable for TCMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcmr::W](W) writer structure"]
impl crate::Writable for TCMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCMR to value 0"]
impl crate::Resettable for TCMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
