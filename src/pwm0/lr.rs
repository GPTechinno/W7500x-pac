#[doc = "Register `LR` reader"]
pub struct R(crate::R<LR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LR` writer"]
pub struct W(crate::W<LR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LR_SPEC>;
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
impl From<crate::W<LR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LR` reader - Limit"]
pub struct LR_R(crate::FieldReader<u32>);
impl LR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        LR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LR_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LR` writer - Limit"]
pub struct LR_W<'a> {
    w: &'a mut W,
}
impl<'a> LR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Limit"]
    #[inline(always)]
    pub fn lr(&self) -> LR_R {
        LR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Limit"]
    #[inline(always)]
    pub fn lr(&mut self) -> LR_W {
        LR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Limit register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lr](index.html) module"]
pub struct LR_SPEC;
impl crate::RegisterSpec for LR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lr::R](R) reader structure"]
impl crate::Readable for LR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lr::W](W) writer structure"]
impl crate::Writable for LR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LR to value 0"]
impl crate::Resettable for LR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
