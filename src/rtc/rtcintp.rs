#[doc = "Register `RTCINTP` reader"]
pub struct R(crate::R<RTCINTP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCINTP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCINTP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCINTP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCINTP` writer"]
pub struct W(crate::W<RTCINTP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCINTP_SPEC>;
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
impl From<crate::W<RTCINTP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCINTP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RTC Alarm interrupt pending flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCALF_A {
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<RTCALF_A> for bool {
    #[inline(always)]
    fn from(variant: RTCALF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCALF` reader - RTC Alarm interrupt pending flag"]
pub struct RTCALF_R(crate::FieldReader<bool>);
impl RTCALF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTCALF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RTCALF_A> {
        match self.bits {
            true => Some(RTCALF_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == RTCALF_A::CLEAR
    }
}
impl core::ops::Deref for RTCALF_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCALF` writer - RTC Alarm interrupt pending flag"]
pub struct RTCALF_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCALF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCALF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RTCALF_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "RTC Counter Interrupt pending flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCCIF_A {
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<RTCCIF_A> for bool {
    #[inline(always)]
    fn from(variant: RTCCIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCCIF` reader - RTC Counter Interrupt pending flag"]
pub struct RTCCIF_R(crate::FieldReader<bool>);
impl RTCCIF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTCCIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RTCCIF_A> {
        match self.bits {
            true => Some(RTCCIF_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == RTCCIF_A::CLEAR
    }
}
impl core::ops::Deref for RTCCIF_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCCIF` writer - RTC Counter Interrupt pending flag"]
pub struct RTCCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCCIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RTCCIF_A::CLEAR)
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
    #[doc = "Bit 1 - RTC Alarm interrupt pending flag"]
    #[inline(always)]
    pub fn rtcalf(&self) -> RTCALF_R {
        RTCALF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - RTC Counter Interrupt pending flag"]
    #[inline(always)]
    pub fn rtccif(&self) -> RTCCIF_R {
        RTCCIF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - RTC Alarm interrupt pending flag"]
    #[inline(always)]
    pub fn rtcalf(&mut self) -> RTCALF_W {
        RTCALF_W { w: self }
    }
    #[doc = "Bit 0 - RTC Counter Interrupt pending flag"]
    #[inline(always)]
    pub fn rtccif(&mut self) -> RTCCIF_W {
        RTCCIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcintp](index.html) module"]
pub struct RTCINTP_SPEC;
impl crate::RegisterSpec for RTCINTP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtcintp::R](R) reader structure"]
impl crate::Readable for RTCINTP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcintp::W](W) writer structure"]
impl crate::Writable for RTCINTP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCINTP to value 0"]
impl crate::Resettable for RTCINTP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
