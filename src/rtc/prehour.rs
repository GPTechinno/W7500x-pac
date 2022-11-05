#[doc = "Register `PREHOUR` reader"]
pub struct R(crate::R<PREHOUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PREHOUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PREHOUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PREHOUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PREHOUR` writer"]
pub struct W(crate::W<PREHOUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PREHOUR_SPEC>;
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
impl From<crate::W<PREHOUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PREHOUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PREHOUR` reader - PREHOUR\\[5:0\\]
bits (RTC Predetermining Hour value (0 to 23))"]
pub struct PREHOUR_R(crate::FieldReader<u8>);
impl PREHOUR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PREHOUR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREHOUR_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREHOUR` writer - PREHOUR\\[5:0\\]
bits (RTC Predetermining Hour value (0 to 23))"]
pub struct PREHOUR_W<'a> {
    w: &'a mut W,
}
impl<'a> PREHOUR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - PREHOUR\\[5:0\\]
bits (RTC Predetermining Hour value (0 to 23))"]
    #[inline(always)]
    pub fn prehour(&self) -> PREHOUR_R {
        PREHOUR_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - PREHOUR\\[5:0\\]
bits (RTC Predetermining Hour value (0 to 23))"]
    #[inline(always)]
    pub fn prehour(&mut self) -> PREHOUR_W {
        PREHOUR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Predetermining Hour register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prehour](index.html) module"]
pub struct PREHOUR_SPEC;
impl crate::RegisterSpec for PREHOUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prehour::R](R) reader structure"]
impl crate::Readable for PREHOUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prehour::W](W) writer structure"]
impl crate::Writable for PREHOUR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PREHOUR to value 0"]
impl crate::Resettable for PREHOUR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
