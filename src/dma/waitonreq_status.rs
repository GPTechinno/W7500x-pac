#[doc = "Register `WAITONREQ_STATUS` reader"]
pub struct R(crate::R<WAITONREQ_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAITONREQ_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAITONREQ_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAITONREQ_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA_WAITONREQ` reader - Channel wait on request status"]
pub struct DMA_WAITONREQ_R(crate::FieldReader<u8, u8>);
impl DMA_WAITONREQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA_WAITONREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_WAITONREQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:5 - Channel wait on request status"]
    #[inline(always)]
    pub fn dma_waitonreq(&self) -> DMA_WAITONREQ_R {
        DMA_WAITONREQ_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Channel Wait On Request Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [waitonreq_status](index.html) module"]
pub struct WAITONREQ_STATUS_SPEC;
impl crate::RegisterSpec for WAITONREQ_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [waitonreq_status::R](R) reader structure"]
impl crate::Readable for WAITONREQ_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WAITONREQ_STATUS to value 0"]
impl crate::Resettable for WAITONREQ_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
