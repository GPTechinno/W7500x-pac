#[doc = "Register `BCDDAY` reader"]
pub struct R(crate::R<BCDDAY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCDDAY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCDDAY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCDDAY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCDDAY` writer"]
pub struct W(crate::W<BCDDAY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCDDAY_SPEC>;
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
impl From<crate::W<BCDDAY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCDDAY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCDDAY` reader - BCDDAY\\[3:0\\]
bits (RTC Day of Week value (1 to 7))"]
pub struct BCDDAY_R(crate::FieldReader<u8>);
impl BCDDAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BCDDAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCDDAY_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCDDAY` writer - BCDDAY\\[3:0\\]
bits (RTC Day of Week value (1 to 7))"]
pub struct BCDDAY_W<'a> {
    w: &'a mut W,
}
impl<'a> BCDDAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - BCDDAY\\[3:0\\]
bits (RTC Day of Week value (1 to 7))"]
    #[inline(always)]
    pub fn bcdday(&self) -> BCDDAY_R {
        BCDDAY_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - BCDDAY\\[3:0\\]
bits (RTC Day of Week value (1 to 7))"]
    #[inline(always)]
    pub fn bcdday(&mut self) -> BCDDAY_W {
        BCDDAY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BCD Day register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcdday](index.html) module"]
pub struct BCDDAY_SPEC;
impl crate::RegisterSpec for BCDDAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bcdday::R](R) reader structure"]
impl crate::Readable for BCDDAY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcdday::W](W) writer structure"]
impl crate::Writable for BCDDAY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BCDDAY to value 0"]
impl crate::Resettable for BCDDAY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
