#[doc = "Register `BCDMIN` reader"]
pub struct R(crate::R<BCDMIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCDMIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCDMIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCDMIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCDMIN` writer"]
pub struct W(crate::W<BCDMIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCDMIN_SPEC>;
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
impl From<crate::W<BCDMIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCDMIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCDMIN` reader - BCDMIN\\[6:0\\]
bits (RTC Minute value (0 to 59))"]
pub struct BCDMIN_R(crate::FieldReader<u8, u8>);
impl BCDMIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        BCDMIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCDMIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCDMIN` writer - BCDMIN\\[6:0\\]
bits (RTC Minute value (0 to 59))"]
pub struct BCDMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> BCDMIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - BCDMIN\\[6:0\\]
bits (RTC Minute value (0 to 59))"]
    #[inline(always)]
    pub fn bcdmin(&self) -> BCDMIN_R {
        BCDMIN_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - BCDMIN\\[6:0\\]
bits (RTC Minute value (0 to 59))"]
    #[inline(always)]
    pub fn bcdmin(&mut self) -> BCDMIN_W {
        BCDMIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BCD Minute register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcdmin](index.html) module"]
pub struct BCDMIN_SPEC;
impl crate::RegisterSpec for BCDMIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bcdmin::R](R) reader structure"]
impl crate::Readable for BCDMIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcdmin::W](W) writer structure"]
impl crate::Writable for BCDMIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BCDMIN to value 0"]
impl crate::Resettable for BCDMIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
