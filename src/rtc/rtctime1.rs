#[doc = "Register `RTCTIME1` reader"]
pub struct R(crate::R<RTCTIME1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCTIME1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCTIME1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCTIME1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CBCDYEAR` reader - CBCDYEAR\\[31:16\\]
bits (RTC Consolidated Year value (0 to 4095))"]
pub struct CBCDYEAR_R(crate::FieldReader<u16, u16>);
impl CBCDYEAR_R {
    pub(crate) fn new(bits: u16) -> Self {
        CBCDYEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBCDYEAR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBCDMON` reader - CBCDMON\\[12:8\\]
bits (RTC Consolidated Month value (1 to 12))"]
pub struct CBCDMON_R(crate::FieldReader<u8, u8>);
impl CBCDMON_R {
    pub(crate) fn new(bits: u8) -> Self {
        CBCDMON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBCDMON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBCDDATE` reader - CBCDDATE\\[5:0\\]
bits (RTC Consolidated Day of Month value (1 to 28, 29, 30, or 31))"]
pub struct CBCDDATE_R(crate::FieldReader<u8, u8>);
impl CBCDDATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CBCDDATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBCDDATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 16:31 - CBCDYEAR\\[31:16\\]
bits (RTC Consolidated Year value (0 to 4095))"]
    #[inline(always)]
    pub fn cbcdyear(&self) -> CBCDYEAR_R {
        CBCDYEAR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:12 - CBCDMON\\[12:8\\]
bits (RTC Consolidated Month value (1 to 12))"]
    #[inline(always)]
    pub fn cbcdmon(&self) -> CBCDMON_R {
        CBCDMON_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:5 - CBCDDATE\\[5:0\\]
bits (RTC Consolidated Day of Month value (1 to 28, 29, 30, or 31))"]
    #[inline(always)]
    pub fn cbcddate(&self) -> CBCDDATE_R {
        CBCDDATE_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Consolidated Time1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtctime1](index.html) module"]
pub struct RTCTIME1_SPEC;
impl crate::RegisterSpec for RTCTIME1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtctime1::R](R) reader structure"]
impl crate::Readable for RTCTIME1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RTCTIME1 to value 0"]
impl crate::Resettable for RTCTIME1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
