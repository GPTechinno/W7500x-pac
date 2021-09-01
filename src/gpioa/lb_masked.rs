#[doc = "Register `LB_MASKED[%s]` reader"]
pub struct R(crate::R<LB_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LB_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LB_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LB_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LB_MASKED[%s]` writer"]
pub struct W(crate::W<LB_MASKED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LB_MASKED_SPEC>;
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
impl From<crate::W<LB_MASKED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LB_MASKED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LBM` reader - Data for lower byte access"]
pub struct LBM_R(crate::FieldReader<u8, u8>);
impl LBM_R {
    pub(crate) fn new(bits: u8) -> Self {
        LBM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LBM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LBM` writer - Data for lower byte access"]
pub struct LBM_W<'a> {
    w: &'a mut W,
}
impl<'a> LBM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data for lower byte access"]
    #[inline(always)]
    pub fn lbm(&self) -> LBM_R {
        LBM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data for lower byte access"]
    #[inline(always)]
    pub fn lbm(&mut self) -> LBM_W {
        LBM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lower byte Masked Access register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lb_masked](index.html) module"]
pub struct LB_MASKED_SPEC;
impl crate::RegisterSpec for LB_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lb_masked::R](R) reader structure"]
impl crate::Readable for LB_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lb_masked::W](W) writer structure"]
impl crate::Writable for LB_MASKED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LB_MASKED[%s]
to value 0"]
impl crate::Resettable for LB_MASKED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
