#[doc = "Register `SEED` reader"]
pub struct R(crate::R<SEED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEED` writer"]
pub struct W(crate::W<SEED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEED_SPEC>;
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
impl From<crate::W<SEED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEED` reader - seed value of random number generator shift register"]
pub struct SEED_R(crate::FieldReader<u32>);
impl SEED_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SEED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEED_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEED` writer - seed value of random number generator shift register"]
pub struct SEED_W<'a> {
    w: &'a mut W,
}
impl<'a> SEED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - seed value of random number generator shift register"]
    #[inline(always)]
    pub fn seed(&self) -> SEED_R {
        SEED_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - seed value of random number generator shift register"]
    #[inline(always)]
    pub fn seed(&mut self) -> SEED_W {
        SEED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RNG seed value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seed](index.html) module"]
pub struct SEED_SPEC;
impl crate::RegisterSpec for SEED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seed::R](R) reader structure"]
impl crate::Readable for SEED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seed::W](W) writer structure"]
impl crate::Writable for SEED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEED to value 0"]
impl crate::Resettable for SEED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
