#[doc = "Register `BCDYEAR` reader"]
pub struct R(crate::R<BCDYEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCDYEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCDYEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCDYEAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCDYEAR` writer"]
pub struct W(crate::W<BCDYEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCDYEAR_SPEC>;
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
impl From<crate::W<BCDYEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCDYEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCDYEAR` reader - BCDYEAR\\[15:0\\]
bits (RTC Year value (0 to 4095))"]
pub struct BCDYEAR_R(crate::FieldReader<u16>);
impl BCDYEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BCDYEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCDYEAR_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCDYEAR` writer - BCDYEAR\\[15:0\\]
bits (RTC Year value (0 to 4095))"]
pub struct BCDYEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> BCDYEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - BCDYEAR\\[15:0\\]
bits (RTC Year value (0 to 4095))"]
    #[inline(always)]
    pub fn bcdyear(&self) -> BCDYEAR_R {
        BCDYEAR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BCDYEAR\\[15:0\\]
bits (RTC Year value (0 to 4095))"]
    #[inline(always)]
    pub fn bcdyear(&mut self) -> BCDYEAR_W {
        BCDYEAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BCD Year register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcdyear](index.html) module"]
pub struct BCDYEAR_SPEC;
impl crate::RegisterSpec for BCDYEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bcdyear::R](R) reader structure"]
impl crate::Readable for BCDYEAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcdyear::W](W) writer structure"]
impl crate::Writable for BCDYEAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BCDYEAR to value 0"]
impl crate::Resettable for BCDYEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
