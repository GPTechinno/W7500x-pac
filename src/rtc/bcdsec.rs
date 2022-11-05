#[doc = "Register `BCDSEC` reader"]
pub struct R(crate::R<BCDSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCDSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCDSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCDSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCDSEC` writer"]
pub struct W(crate::W<BCDSEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCDSEC_SPEC>;
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
impl From<crate::W<BCDSEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCDSEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCDSEC` reader - BCDSEC\\[6:0\\]
bits (RTC Seconds value (0 to 59))"]
pub struct BCDSEC_R(crate::FieldReader<u8>);
impl BCDSEC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BCDSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCDSEC_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCDSEC` writer - BCDSEC\\[6:0\\]
bits (RTC Seconds value (0 to 59))"]
pub struct BCDSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> BCDSEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - BCDSEC\\[6:0\\]
bits (RTC Seconds value (0 to 59))"]
    #[inline(always)]
    pub fn bcdsec(&self) -> BCDSEC_R {
        BCDSEC_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - BCDSEC\\[6:0\\]
bits (RTC Seconds value (0 to 59))"]
    #[inline(always)]
    pub fn bcdsec(&mut self) -> BCDSEC_W {
        BCDSEC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BCD Second register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcdsec](index.html) module"]
pub struct BCDSEC_SPEC;
impl crate::RegisterSpec for BCDSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bcdsec::R](R) reader structure"]
impl crate::Readable for BCDSEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcdsec::W](W) writer structure"]
impl crate::Writable for BCDSEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BCDSEC to value 0"]
impl crate::Resettable for BCDSEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
