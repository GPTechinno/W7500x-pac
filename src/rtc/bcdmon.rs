#[doc = "Register `BCDMON` reader"]
pub struct R(crate::R<BCDMON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCDMON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCDMON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCDMON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCDMON` writer"]
pub struct W(crate::W<BCDMON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCDMON_SPEC>;
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
impl From<crate::W<BCDMON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCDMON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCDMON` reader - BCDMON\\[4:0\\]
bits (RTC Month value (1 to 12))"]
pub struct BCDMON_R(crate::FieldReader<u8, u8>);
impl BCDMON_R {
    pub(crate) fn new(bits: u8) -> Self {
        BCDMON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCDMON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCDMON` writer - BCDMON\\[4:0\\]
bits (RTC Month value (1 to 12))"]
pub struct BCDMON_W<'a> {
    w: &'a mut W,
}
impl<'a> BCDMON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - BCDMON\\[4:0\\]
bits (RTC Month value (1 to 12))"]
    #[inline(always)]
    pub fn bcdmon(&self) -> BCDMON_R {
        BCDMON_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - BCDMON\\[4:0\\]
bits (RTC Month value (1 to 12))"]
    #[inline(always)]
    pub fn bcdmon(&mut self) -> BCDMON_W {
        BCDMON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BCD Month register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcdmon](index.html) module"]
pub struct BCDMON_SPEC;
impl crate::RegisterSpec for BCDMON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bcdmon::R](R) reader structure"]
impl crate::Readable for BCDMON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcdmon::W](W) writer structure"]
impl crate::Writable for BCDMON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BCDMON to value 0"]
impl crate::Resettable for BCDMON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
