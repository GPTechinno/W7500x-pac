#[doc = "Register `UB_MASKED[%s]` reader"]
pub struct R(crate::R<UB_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UB_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UB_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UB_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UB_MASKED[%s]` writer"]
pub struct W(crate::W<UB_MASKED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UB_MASKED_SPEC>;
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
impl From<crate::W<UB_MASKED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UB_MASKED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HBM` reader - Data for higher byte access"]
pub struct HBM_R(crate::FieldReader<u8>);
impl HBM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HBM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HBM_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HBM` writer - Data for higher byte access"]
pub struct HBM_W<'a> {
    w: &'a mut W,
}
impl<'a> HBM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Data for higher byte access"]
    #[inline(always)]
    pub fn hbm(&self) -> HBM_R {
        HBM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Data for higher byte access"]
    #[inline(always)]
    pub fn hbm(&mut self) -> HBM_W {
        HBM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Upper byte Masked Access register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ub_masked](index.html) module"]
pub struct UB_MASKED_SPEC;
impl crate::RegisterSpec for UB_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ub_masked::R](R) reader structure"]
impl crate::Readable for UB_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ub_masked::W](W) writer structure"]
impl crate::Writable for UB_MASKED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UB_MASKED[%s]
to value 0"]
impl crate::Resettable for UB_MASKED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
