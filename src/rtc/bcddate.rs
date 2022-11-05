#[doc = "Register `BCDDATE` reader"]
pub struct R(crate::R<BCDDATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCDDATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCDDATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCDDATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCDDATE` writer"]
pub struct W(crate::W<BCDDATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCDDATE_SPEC>;
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
impl From<crate::W<BCDDATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCDDATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCDDATE` reader - BCDDATE\\[5:0\\]
bits (RTC Day of Month value (1 to 28, 29, 30, or 31))"]
pub struct BCDDATE_R(crate::FieldReader<u8>);
impl BCDDATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BCDDATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCDDATE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCDDATE` writer - BCDDATE\\[5:0\\]
bits (RTC Day of Month value (1 to 28, 29, 30, or 31))"]
pub struct BCDDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> BCDDATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - BCDDATE\\[5:0\\]
bits (RTC Day of Month value (1 to 28, 29, 30, or 31))"]
    #[inline(always)]
    pub fn bcddate(&self) -> BCDDATE_R {
        BCDDATE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - BCDDATE\\[5:0\\]
bits (RTC Day of Month value (1 to 28, 29, 30, or 31))"]
    #[inline(always)]
    pub fn bcddate(&mut self) -> BCDDATE_W {
        BCDDATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BCD Date register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcddate](index.html) module"]
pub struct BCDDATE_SPEC;
impl crate::RegisterSpec for BCDDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bcddate::R](R) reader structure"]
impl crate::Readable for BCDDATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcddate::W](W) writer structure"]
impl crate::Writable for BCDDATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BCDDATE to value 0"]
impl crate::Resettable for BCDDATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
