#[doc = "Register `RN` reader"]
pub struct R(crate::R<RN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RN` reader - random number of RNG shift register"]
pub struct RN_R(crate::FieldReader<u32>);
impl RN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RN_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - random number of RNG shift register"]
    #[inline(always)]
    pub fn rn(&self) -> RN_R {
        RN_R::new(self.bits)
    }
}
#[doc = "RNG random number value register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rn](index.html) module"]
pub struct RN_SPEC;
impl crate::RegisterSpec for RN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rn::R](R) reader structure"]
impl crate::Readable for RN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RN to value 0"]
impl crate::Resettable for RN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
