#[doc = "Register `RTCTIME0` reader"]
pub struct R(crate::R<RTCTIME0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCTIME0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCTIME0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCTIME0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CBCDDAY` reader - CBCDDAY\\[27:24\\]
bits ( RTC Consolidated Day of Week value (1 to 7))"]
pub struct CBCDDAY_R(crate::FieldReader<u8>);
impl CBCDDAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CBCDDAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBCDDAY_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBCDHOUR` reader - CBCDHOUR\\[21:16\\]
bits (RTC Consolidated Hour value (0 to 23))"]
pub struct CBCDHOUR_R(crate::FieldReader<u8>);
impl CBCDHOUR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CBCDHOUR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBCDHOUR_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBCDMIN` reader - CBCDMIN\\[14:8\\]
bits (RTC Consolidated Minute value (0 to 59))"]
pub struct CBCDMIN_R(crate::FieldReader<u8>);
impl CBCDMIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CBCDMIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBCDMIN_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBCDSEC` reader - CBCDSEC\\[6:0\\]
bits (RTC Consolidated Second value (0 to 59))"]
pub struct CBCDSEC_R(crate::FieldReader<u8>);
impl CBCDSEC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CBCDSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBCDSEC_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 24:27 - CBCDDAY\\[27:24\\]
bits ( RTC Consolidated Day of Week value (1 to 7))"]
    #[inline(always)]
    pub fn cbcdday(&self) -> CBCDDAY_R {
        CBCDDAY_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - CBCDHOUR\\[21:16\\]
bits (RTC Consolidated Hour value (0 to 23))"]
    #[inline(always)]
    pub fn cbcdhour(&self) -> CBCDHOUR_R {
        CBCDHOUR_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 8:14 - CBCDMIN\\[14:8\\]
bits (RTC Consolidated Minute value (0 to 59))"]
    #[inline(always)]
    pub fn cbcdmin(&self) -> CBCDMIN_R {
        CBCDMIN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 0:6 - CBCDSEC\\[6:0\\]
bits (RTC Consolidated Second value (0 to 59))"]
    #[inline(always)]
    pub fn cbcdsec(&self) -> CBCDSEC_R {
        CBCDSEC_R::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "Consolidated Time0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtctime0](index.html) module"]
pub struct RTCTIME0_SPEC;
impl crate::RegisterSpec for RTCTIME0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtctime0::R](R) reader structure"]
impl crate::Readable for RTCTIME0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RTCTIME0 to value 0"]
impl crate::Resettable for RTCTIME0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
