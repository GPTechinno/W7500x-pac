#[doc = "Register `RTCAMR` reader"]
pub struct R(crate::R<RTCAMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCAMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCAMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCAMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCAMR` writer"]
pub struct W(crate::W<RTCAMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCAMR_SPEC>;
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
impl From<crate::W<RTCAMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCAMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RTC Alarm Mask for Year\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AMRYEAR_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<AMRYEAR_A> for bool {
    #[inline(always)]
    fn from(variant: AMRYEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMRYEAR` reader - RTC Alarm Mask for Year"]
pub struct AMRYEAR_R(crate::FieldReader<bool>);
impl AMRYEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AMRYEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AMRYEAR_A> {
        match self.bits {
            true => Some(AMRYEAR_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == AMRYEAR_A::ENABLE
    }
}
impl core::ops::Deref for AMRYEAR_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMRYEAR` writer - RTC Alarm Mask for Year"]
pub struct AMRYEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> AMRYEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AMRYEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AMRYEAR_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "RTC Alarm Mask for Month\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AMRMON_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<AMRMON_A> for bool {
    #[inline(always)]
    fn from(variant: AMRMON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMRMON` reader - RTC Alarm Mask for Month"]
pub struct AMRMON_R(crate::FieldReader<bool>);
impl AMRMON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AMRMON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AMRMON_A> {
        match self.bits {
            true => Some(AMRMON_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == AMRMON_A::ENABLE
    }
}
impl core::ops::Deref for AMRMON_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMRMON` writer - RTC Alarm Mask for Month"]
pub struct AMRMON_W<'a> {
    w: &'a mut W,
}
impl<'a> AMRMON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AMRMON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AMRMON_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "RTC Alarm Mask for Date\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AMRDATE_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<AMRDATE_A> for bool {
    #[inline(always)]
    fn from(variant: AMRDATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMRDATE` reader - RTC Alarm Mask for Date"]
pub struct AMRDATE_R(crate::FieldReader<bool>);
impl AMRDATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AMRDATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AMRDATE_A> {
        match self.bits {
            true => Some(AMRDATE_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == AMRDATE_A::ENABLE
    }
}
impl core::ops::Deref for AMRDATE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMRDATE` writer - RTC Alarm Mask for Date"]
pub struct AMRDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> AMRDATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AMRDATE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AMRDATE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "RTC Alarm Mask for Day\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AMRDAY_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<AMRDAY_A> for bool {
    #[inline(always)]
    fn from(variant: AMRDAY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMRDAY` reader - RTC Alarm Mask for Day"]
pub struct AMRDAY_R(crate::FieldReader<bool>);
impl AMRDAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AMRDAY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AMRDAY_A> {
        match self.bits {
            true => Some(AMRDAY_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == AMRDAY_A::ENABLE
    }
}
impl core::ops::Deref for AMRDAY_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMRDAY` writer - RTC Alarm Mask for Day"]
pub struct AMRDAY_W<'a> {
    w: &'a mut W,
}
impl<'a> AMRDAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AMRDAY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AMRDAY_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "RTC Alarm Mask for Hour\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AMRHOUR_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<AMRHOUR_A> for bool {
    #[inline(always)]
    fn from(variant: AMRHOUR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMRHOUR` reader - RTC Alarm Mask for Hour"]
pub struct AMRHOUR_R(crate::FieldReader<bool>);
impl AMRHOUR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AMRHOUR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AMRHOUR_A> {
        match self.bits {
            true => Some(AMRHOUR_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == AMRHOUR_A::ENABLE
    }
}
impl core::ops::Deref for AMRHOUR_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMRHOUR` writer - RTC Alarm Mask for Hour"]
pub struct AMRHOUR_W<'a> {
    w: &'a mut W,
}
impl<'a> AMRHOUR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AMRHOUR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AMRHOUR_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "RTC Alarm Mask for Minute\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AMRMIN_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<AMRMIN_A> for bool {
    #[inline(always)]
    fn from(variant: AMRMIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMRMIN` reader - RTC Alarm Mask for Minute"]
pub struct AMRMIN_R(crate::FieldReader<bool>);
impl AMRMIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AMRMIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AMRMIN_A> {
        match self.bits {
            true => Some(AMRMIN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == AMRMIN_A::ENABLE
    }
}
impl core::ops::Deref for AMRMIN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMRMIN` writer - RTC Alarm Mask for Minute"]
pub struct AMRMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> AMRMIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AMRMIN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AMRMIN_A::ENABLE)
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
#[doc = "RTC Alarm Mask for Second\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AMRSEC_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<AMRSEC_A> for bool {
    #[inline(always)]
    fn from(variant: AMRSEC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMRSEC` reader - RTC Alarm Mask for Second"]
pub struct AMRSEC_R(crate::FieldReader<bool>);
impl AMRSEC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AMRSEC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AMRSEC_A> {
        match self.bits {
            true => Some(AMRSEC_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == AMRSEC_A::ENABLE
    }
}
impl core::ops::Deref for AMRSEC_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMRSEC` writer - RTC Alarm Mask for Second"]
pub struct AMRSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> AMRSEC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AMRSEC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AMRSEC_A::ENABLE)
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
    #[doc = "Bit 6 - RTC Alarm Mask for Year"]
    #[inline(always)]
    pub fn amryear(&self) -> AMRYEAR_R {
        AMRYEAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC Alarm Mask for Month"]
    #[inline(always)]
    pub fn amrmon(&self) -> AMRMON_R {
        AMRMON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC Alarm Mask for Date"]
    #[inline(always)]
    pub fn amrdate(&self) -> AMRDATE_R {
        AMRDATE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC Alarm Mask for Day"]
    #[inline(always)]
    pub fn amrday(&self) -> AMRDAY_R {
        AMRDAY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC Alarm Mask for Hour"]
    #[inline(always)]
    pub fn amrhour(&self) -> AMRHOUR_R {
        AMRHOUR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - RTC Alarm Mask for Minute"]
    #[inline(always)]
    pub fn amrmin(&self) -> AMRMIN_R {
        AMRMIN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - RTC Alarm Mask for Second"]
    #[inline(always)]
    pub fn amrsec(&self) -> AMRSEC_R {
        AMRSEC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - RTC Alarm Mask for Year"]
    #[inline(always)]
    pub fn amryear(&mut self) -> AMRYEAR_W {
        AMRYEAR_W { w: self }
    }
    #[doc = "Bit 5 - RTC Alarm Mask for Month"]
    #[inline(always)]
    pub fn amrmon(&mut self) -> AMRMON_W {
        AMRMON_W { w: self }
    }
    #[doc = "Bit 4 - RTC Alarm Mask for Date"]
    #[inline(always)]
    pub fn amrdate(&mut self) -> AMRDATE_W {
        AMRDATE_W { w: self }
    }
    #[doc = "Bit 3 - RTC Alarm Mask for Day"]
    #[inline(always)]
    pub fn amrday(&mut self) -> AMRDAY_W {
        AMRDAY_W { w: self }
    }
    #[doc = "Bit 2 - RTC Alarm Mask for Hour"]
    #[inline(always)]
    pub fn amrhour(&mut self) -> AMRHOUR_W {
        AMRHOUR_W { w: self }
    }
    #[doc = "Bit 1 - RTC Alarm Mask for Minute"]
    #[inline(always)]
    pub fn amrmin(&mut self) -> AMRMIN_W {
        AMRMIN_W { w: self }
    }
    #[doc = "Bit 0 - RTC Alarm Mask for Second"]
    #[inline(always)]
    pub fn amrsec(&mut self) -> AMRSEC_W {
        AMRSEC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alarm Mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcamr](index.html) module"]
pub struct RTCAMR_SPEC;
impl crate::RegisterSpec for RTCAMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtcamr::R](R) reader structure"]
impl crate::Readable for RTCAMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcamr::W](W) writer structure"]
impl crate::Writable for RTCAMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCAMR to value 0"]
impl crate::Resettable for RTCAMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
