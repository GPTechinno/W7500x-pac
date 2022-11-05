#[doc = "Register `DZCR` reader"]
pub struct R(crate::R<DZCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DZCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DZCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DZCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DZCR` writer"]
pub struct W(crate::W<DZCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DZCR_SPEC>;
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
impl From<crate::W<DZCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DZCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DZC` reader - Dead Zone Counter value"]
pub struct DZC_R(crate::FieldReader<u16>);
impl DZC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DZC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DZC_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DZC` writer - Dead Zone Counter value"]
pub struct DZC_W<'a> {
    w: &'a mut W,
}
impl<'a> DZC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Dead Zone Counter value"]
    #[inline(always)]
    pub fn dzc(&self) -> DZC_R {
        DZC_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Dead Zone Counter value"]
    #[inline(always)]
    pub fn dzc(&mut self) -> DZC_W {
        DZC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dead-zone counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dzcr](index.html) module"]
pub struct DZCR_SPEC;
impl crate::RegisterSpec for DZCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dzcr::R](R) reader structure"]
impl crate::Readable for DZCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dzcr::W](W) writer structure"]
impl crate::Writable for DZCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DZCR to value 0"]
impl crate::Resettable for DZCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
