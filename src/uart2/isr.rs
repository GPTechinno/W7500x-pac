#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXOI` reader - Receive Overrun Interrupt"]
pub struct RXOI_R(crate::FieldReader<bool>);
impl RXOI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXOI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOI_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXOI` reader - Transmit Overrun Interrupt"]
pub struct TXOI_R(crate::FieldReader<bool>);
impl TXOI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXOI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXOI_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXI` reader - Receive Interrupt"]
pub struct RXI_R(crate::FieldReader<bool>);
impl RXI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXI_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXI` reader - Transmit Interrupt"]
pub struct TXI_R(crate::FieldReader<bool>);
impl TXI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXI_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 3 - Receive Overrun Interrupt"]
    #[inline(always)]
    pub fn rxoi(&self) -> RXOI_R {
        RXOI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Overrun Interrupt"]
    #[inline(always)]
    pub fn txoi(&self) -> TXOI_R {
        TXOI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Interrupt"]
    #[inline(always)]
    pub fn rxi(&self) -> RXI_R {
        RXI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    pub fn txi(&self) -> TXI_R {
        TXI_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
