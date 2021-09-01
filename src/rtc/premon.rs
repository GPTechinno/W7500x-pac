#[doc = "Register `PREMON` reader"]
pub struct R(crate::R<PREMON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PREMON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PREMON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PREMON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PREMON` writer"]
pub struct W(crate::W<PREMON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PREMON_SPEC>;
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
impl From<crate::W<PREMON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PREMON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PREMON` reader - PREMON\\[3:0\\]
bits (RTC Predetermining Month value (1 to 12))"]
pub struct PREMON_R(crate::FieldReader<u8, u8>);
impl PREMON_R {
    pub(crate) fn new(bits: u8) -> Self {
        PREMON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREMON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREMON` writer - PREMON\\[3:0\\]
bits (RTC Predetermining Month value (1 to 12))"]
pub struct PREMON_W<'a> {
    w: &'a mut W,
}
impl<'a> PREMON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - PREMON\\[3:0\\]
bits (RTC Predetermining Month value (1 to 12))"]
    #[inline(always)]
    pub fn premon(&self) -> PREMON_R {
        PREMON_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - PREMON\\[3:0\\]
bits (RTC Predetermining Month value (1 to 12))"]
    #[inline(always)]
    pub fn premon(&mut self) -> PREMON_W {
        PREMON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Predetermining Month register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [premon](index.html) module"]
pub struct PREMON_SPEC;
impl crate::RegisterSpec for PREMON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [premon::R](R) reader structure"]
impl crate::Readable for PREMON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [premon::W](W) writer structure"]
impl crate::Writable for PREMON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PREMON to value 0"]
impl crate::Resettable for PREMON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
