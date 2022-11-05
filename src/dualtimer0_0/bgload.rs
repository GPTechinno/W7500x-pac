#[doc = "Register `BGLOAD` reader"]
pub struct R(crate::R<BGLOAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BGLOAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BGLOAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BGLOAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BGLOAD` writer"]
pub struct W(crate::W<BGLOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BGLOAD_SPEC>;
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
impl From<crate::W<BGLOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BGLOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BGL` reader - Background Load"]
pub struct BGL_R(crate::FieldReader<u32>);
impl BGL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BGL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BGL_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BGL` writer - Background Load"]
pub struct BGL_W<'a> {
    w: &'a mut W,
}
impl<'a> BGL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Background Load"]
    #[inline(always)]
    pub fn bgl(&self) -> BGL_R {
        BGL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Background Load"]
    #[inline(always)]
    pub fn bgl(&mut self) -> BGL_W {
        BGL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Background Load register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgload](index.html) module"]
pub struct BGLOAD_SPEC;
impl crate::RegisterSpec for BGLOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bgload::R](R) reader structure"]
impl crate::Readable for BGLOAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bgload::W](W) writer structure"]
impl crate::Writable for BGLOAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BGLOAD to value 0"]
impl crate::Resettable for BGLOAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
