#[doc = "Register `LOAD` reader"]
pub struct R(crate::R<LOAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOAD` writer"]
pub struct W(crate::W<LOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOAD_SPEC>;
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
impl From<crate::W<LOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TLR` reader - Timer Load"]
pub struct TLR_R(crate::FieldReader<u32>);
impl TLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TLR_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TLR` writer - Timer Load"]
pub struct TLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Timer Load"]
    #[inline(always)]
    pub fn tlr(&self) -> TLR_R {
        TLR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer Load"]
    #[inline(always)]
    pub fn tlr(&mut self) -> TLR_W {
        TLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Load register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [load](index.html) module"]
pub struct LOAD_SPEC;
impl crate::RegisterSpec for LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [load::R](R) reader structure"]
impl crate::Readable for LOAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [load::W](W) writer structure"]
impl crate::Writable for LOAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOAD to value 0"]
impl crate::Resettable for LOAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
