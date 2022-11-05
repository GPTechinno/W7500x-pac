#[doc = "Register `PREYEAR` reader"]
pub struct R(crate::R<PREYEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PREYEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PREYEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PREYEAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PREYEAR` writer"]
pub struct W(crate::W<PREYEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PREYEAR_SPEC>;
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
impl From<crate::W<PREYEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PREYEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PREYEAR` reader - PREYEAR\\[15:0\\]
bits (RTC Predetermining Year value (0 to 4095))"]
pub struct PREYEAR_R(crate::FieldReader<u16>);
impl PREYEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PREYEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREYEAR_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREYEAR` writer - PREYEAR\\[15:0\\]
bits (RTC Predetermining Year value (0 to 4095))"]
pub struct PREYEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> PREYEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PREYEAR\\[15:0\\]
bits (RTC Predetermining Year value (0 to 4095))"]
    #[inline(always)]
    pub fn preyear(&self) -> PREYEAR_R {
        PREYEAR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PREYEAR\\[15:0\\]
bits (RTC Predetermining Year value (0 to 4095))"]
    #[inline(always)]
    pub fn preyear(&mut self) -> PREYEAR_W {
        PREYEAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Predetermining Year register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [preyear](index.html) module"]
pub struct PREYEAR_SPEC;
impl crate::RegisterSpec for PREYEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [preyear::R](R) reader structure"]
impl crate::Readable for PREYEAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [preyear::W](W) writer structure"]
impl crate::Writable for PREYEAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PREYEAR to value 0"]
impl crate::Resettable for PREYEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
