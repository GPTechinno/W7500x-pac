#[doc = "Register `POLY` reader"]
pub struct R(crate::R<POLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POLY` writer"]
pub struct W(crate::W<POLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POLY_SPEC>;
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
impl From<crate::W<POLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POLY` reader - 32bit polynomial of random number generator"]
pub struct POLY_R(crate::FieldReader<u32>);
impl POLY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        POLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POLY_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POLY` writer - 32bit polynomial of random number generator"]
pub struct POLY_W<'a> {
    w: &'a mut W,
}
impl<'a> POLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 32bit polynomial of random number generator"]
    #[inline(always)]
    pub fn poly(&self) -> POLY_R {
        POLY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32bit polynomial of random number generator"]
    #[inline(always)]
    pub fn poly(&mut self) -> POLY_W {
        POLY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RNG polynomial register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [poly](index.html) module"]
pub struct POLY_SPEC;
impl crate::RegisterSpec for POLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [poly::R](R) reader structure"]
impl crate::Readable for POLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [poly::W](W) writer structure"]
impl crate::Writable for POLY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POLY to value 0xe000_0202"]
impl crate::Resettable for POLY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xe000_0202
    }
}
