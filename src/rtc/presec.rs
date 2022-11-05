#[doc = "Register `PRESEC` reader"]
pub struct R(crate::R<PRESEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRESEC` writer"]
pub struct W(crate::W<PRESEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESEC_SPEC>;
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
impl From<crate::W<PRESEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESEC` reader - PRESEC\\[6:0\\]
bits (RTC PREDETERMINING Seconds value (0 to 59))"]
pub struct PRESEC_R(crate::FieldReader<u8>);
impl PRESEC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRESEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRESEC_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRESEC` writer - PRESEC\\[6:0\\]
bits (RTC PREDETERMINING Seconds value (0 to 59))"]
pub struct PRESEC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - PRESEC\\[6:0\\]
bits (RTC PREDETERMINING Seconds value (0 to 59))"]
    #[inline(always)]
    pub fn presec(&self) -> PRESEC_R {
        PRESEC_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - PRESEC\\[6:0\\]
bits (RTC PREDETERMINING Seconds value (0 to 59))"]
    #[inline(always)]
    pub fn presec(&mut self) -> PRESEC_W {
        PRESEC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Predetermining Second register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presec](index.html) module"]
pub struct PRESEC_SPEC;
impl crate::RegisterSpec for PRESEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [presec::R](R) reader structure"]
impl crate::Readable for PRESEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [presec::W](W) writer structure"]
impl crate::Writable for PRESEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRESEC to value 0"]
impl crate::Resettable for PRESEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
