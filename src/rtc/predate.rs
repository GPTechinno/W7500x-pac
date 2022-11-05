#[doc = "Register `PREDATE` reader"]
pub struct R(crate::R<PREDATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PREDATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PREDATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PREDATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PREDATE` writer"]
pub struct W(crate::W<PREDATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PREDATE_SPEC>;
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
impl From<crate::W<PREDATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PREDATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PREDATE` reader - PREDATE\\[3:0\\]
bits (RTC Predetermining Day of Month value (1 to 28, 29, 30, or 31))"]
pub struct PREDATE_R(crate::FieldReader<u8>);
impl PREDATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PREDATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREDATE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREDATE` writer - PREDATE\\[3:0\\]
bits (RTC Predetermining Day of Month value (1 to 28, 29, 30, or 31))"]
pub struct PREDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> PREDATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - PREDATE\\[3:0\\]
bits (RTC Predetermining Day of Month value (1 to 28, 29, 30, or 31))"]
    #[inline(always)]
    pub fn predate(&self) -> PREDATE_R {
        PREDATE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - PREDATE\\[3:0\\]
bits (RTC Predetermining Day of Month value (1 to 28, 29, 30, or 31))"]
    #[inline(always)]
    pub fn predate(&mut self) -> PREDATE_W {
        PREDATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Predetermining Date register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [predate](index.html) module"]
pub struct PREDATE_SPEC;
impl crate::RegisterSpec for PREDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [predate::R](R) reader structure"]
impl crate::Readable for PREDATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [predate::W](W) writer structure"]
impl crate::Writable for PREDATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PREDATE to value 0"]
impl crate::Resettable for PREDATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
