#[doc = "Register `RTCINTE` reader"]
pub struct R(crate::R<RTCINTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCINTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCINTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCINTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCINTE` writer"]
pub struct W(crate::W<RTCINTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCINTE_SPEC>;
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
impl From<crate::W<RTCINTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCINTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RTC Alarm Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AINT_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<AINT_A> for bool {
    #[inline(always)]
    fn from(variant: AINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AINT` reader - RTC Alarm Interrupt Enable"]
pub struct AINT_R(crate::FieldReader<bool>);
impl AINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AINT_A {
        match self.bits {
            false => AINT_A::DISABLE,
            true => AINT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == AINT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == AINT_A::ENABLE
    }
}
impl core::ops::Deref for AINT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AINT` writer - RTC Alarm Interrupt Enable"]
pub struct AINT_W<'a> {
    w: &'a mut W,
}
impl<'a> AINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AINT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AINT_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AINT_A::ENABLE)
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
#[doc = "RTC Month Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMMON_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<IMMON_A> for bool {
    #[inline(always)]
    fn from(variant: IMMON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMMON` reader - RTC Month Interrupt Enable"]
pub struct IMMON_R(crate::FieldReader<bool>);
impl IMMON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IMMON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMMON_A {
        match self.bits {
            false => IMMON_A::DISABLE,
            true => IMMON_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == IMMON_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == IMMON_A::ENABLE
    }
}
impl core::ops::Deref for IMMON_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMMON` writer - RTC Month Interrupt Enable"]
pub struct IMMON_W<'a> {
    w: &'a mut W,
}
impl<'a> IMMON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IMMON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IMMON_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IMMON_A::ENABLE)
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
#[doc = "RTC Day Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMDAY_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<IMDAY_A> for bool {
    #[inline(always)]
    fn from(variant: IMDAY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMDAY` reader - RTC Day Interrupt Enable"]
pub struct IMDAY_R(crate::FieldReader<bool>);
impl IMDAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IMDAY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMDAY_A {
        match self.bits {
            false => IMDAY_A::DISABLE,
            true => IMDAY_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == IMDAY_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == IMDAY_A::ENABLE
    }
}
impl core::ops::Deref for IMDAY_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMDAY` writer - RTC Day Interrupt Enable"]
pub struct IMDAY_W<'a> {
    w: &'a mut W,
}
impl<'a> IMDAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IMDAY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IMDAY_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IMDAY_A::ENABLE)
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
#[doc = "RTC Date Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMDATE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<IMDATE_A> for bool {
    #[inline(always)]
    fn from(variant: IMDATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMDATE` reader - RTC Date Interrupt Enable"]
pub struct IMDATE_R(crate::FieldReader<bool>);
impl IMDATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IMDATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMDATE_A {
        match self.bits {
            false => IMDATE_A::DISABLE,
            true => IMDATE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == IMDATE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == IMDATE_A::ENABLE
    }
}
impl core::ops::Deref for IMDATE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMDATE` writer - RTC Date Interrupt Enable"]
pub struct IMDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> IMDATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IMDATE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IMDATE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IMDATE_A::ENABLE)
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
#[doc = "RTC Hour Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMHOUR_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<IMHOUR_A> for bool {
    #[inline(always)]
    fn from(variant: IMHOUR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMHOUR` reader - RTC Hour Interrupt Enable"]
pub struct IMHOUR_R(crate::FieldReader<bool>);
impl IMHOUR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IMHOUR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMHOUR_A {
        match self.bits {
            false => IMHOUR_A::DISABLE,
            true => IMHOUR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == IMHOUR_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == IMHOUR_A::ENABLE
    }
}
impl core::ops::Deref for IMHOUR_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMHOUR` writer - RTC Hour Interrupt Enable"]
pub struct IMHOUR_W<'a> {
    w: &'a mut W,
}
impl<'a> IMHOUR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IMHOUR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IMHOUR_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IMHOUR_A::ENABLE)
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
#[doc = "RTC Minute Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMMIN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<IMMIN_A> for bool {
    #[inline(always)]
    fn from(variant: IMMIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMMIN` reader - RTC Minute Interrupt Enable"]
pub struct IMMIN_R(crate::FieldReader<bool>);
impl IMMIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IMMIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMMIN_A {
        match self.bits {
            false => IMMIN_A::DISABLE,
            true => IMMIN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == IMMIN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == IMMIN_A::ENABLE
    }
}
impl core::ops::Deref for IMMIN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMMIN` writer - RTC Minute Interrupt Enable"]
pub struct IMMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> IMMIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IMMIN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IMMIN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IMMIN_A::ENABLE)
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
#[doc = "RTC Second Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMSEC_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<IMSEC_A> for bool {
    #[inline(always)]
    fn from(variant: IMSEC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMSEC` reader - RTC Second Interrupt Enable"]
pub struct IMSEC_R(crate::FieldReader<bool>);
impl IMSEC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IMSEC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMSEC_A {
        match self.bits {
            false => IMSEC_A::DISABLE,
            true => IMSEC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == IMSEC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == IMSEC_A::ENABLE
    }
}
impl core::ops::Deref for IMSEC_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMSEC` writer - RTC Second Interrupt Enable"]
pub struct IMSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> IMSEC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IMSEC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IMSEC_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IMSEC_A::ENABLE)
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
    #[doc = "Bit 6 - RTC Alarm Interrupt Enable"]
    #[inline(always)]
    pub fn aint(&self) -> AINT_R {
        AINT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC Month Interrupt Enable"]
    #[inline(always)]
    pub fn immon(&self) -> IMMON_R {
        IMMON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC Day Interrupt Enable"]
    #[inline(always)]
    pub fn imday(&self) -> IMDAY_R {
        IMDAY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC Date Interrupt Enable"]
    #[inline(always)]
    pub fn imdate(&self) -> IMDATE_R {
        IMDATE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC Hour Interrupt Enable"]
    #[inline(always)]
    pub fn imhour(&self) -> IMHOUR_R {
        IMHOUR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - RTC Minute Interrupt Enable"]
    #[inline(always)]
    pub fn immin(&self) -> IMMIN_R {
        IMMIN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - RTC Second Interrupt Enable"]
    #[inline(always)]
    pub fn imsec(&self) -> IMSEC_R {
        IMSEC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - RTC Alarm Interrupt Enable"]
    #[inline(always)]
    pub fn aint(&mut self) -> AINT_W {
        AINT_W { w: self }
    }
    #[doc = "Bit 5 - RTC Month Interrupt Enable"]
    #[inline(always)]
    pub fn immon(&mut self) -> IMMON_W {
        IMMON_W { w: self }
    }
    #[doc = "Bit 4 - RTC Day Interrupt Enable"]
    #[inline(always)]
    pub fn imday(&mut self) -> IMDAY_W {
        IMDAY_W { w: self }
    }
    #[doc = "Bit 3 - RTC Date Interrupt Enable"]
    #[inline(always)]
    pub fn imdate(&mut self) -> IMDATE_W {
        IMDATE_W { w: self }
    }
    #[doc = "Bit 2 - RTC Hour Interrupt Enable"]
    #[inline(always)]
    pub fn imhour(&mut self) -> IMHOUR_W {
        IMHOUR_W { w: self }
    }
    #[doc = "Bit 1 - RTC Minute Interrupt Enable"]
    #[inline(always)]
    pub fn immin(&mut self) -> IMMIN_W {
        IMMIN_W { w: self }
    }
    #[doc = "Bit 0 - RTC Second Interrupt Enable"]
    #[inline(always)]
    pub fn imsec(&mut self) -> IMSEC_W {
        IMSEC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcinte](index.html) module"]
pub struct RTCINTE_SPEC;
impl crate::RegisterSpec for RTCINTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtcinte::R](R) reader structure"]
impl crate::Readable for RTCINTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcinte::W](W) writer structure"]
impl crate::Writable for RTCINTE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCINTE to value 0"]
impl crate::Resettable for RTCINTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
