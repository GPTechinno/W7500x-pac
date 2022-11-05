#[doc = "Register `DATA` reader"]
pub struct R(crate::R<DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADC_DATA` reader - ADC conversion result data"]
pub struct ADC_DATA_R(crate::FieldReader<u16>);
impl ADC_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ADC_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_DATA_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11 - ADC conversion result data"]
    #[inline(always)]
    pub fn adc_data(&self) -> ADC_DATA_R {
        ADC_DATA_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "ADC conversion data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](index.html) module"]
pub struct DATA_SPEC;
impl crate::RegisterSpec for DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data::R](R) reader structure"]
impl crate::Readable for DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DATA to value 0"]
impl crate::Resettable for DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
