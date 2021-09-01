#[doc = "Register `LOAD` reader"]
pub struct R(crate::R<LOAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOAD` writer"]
pub struct W(crate::W<LOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOAD_SPEC>;
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
impl From<crate::W<LOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WLR` reader - Watchdog timer Load"]
pub struct WLR_R(crate::FieldReader<u32, u32>);
impl WLR_R {
    pub(crate) fn new(bits: u32) -> Self {
        WLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WLR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WLR` writer - Watchdog timer Load"]
pub struct WLR_W<'a> {
    w: &'a mut W,
}
impl<'a> WLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Watchdog timer Load"]
    #[inline(always)]
    pub fn wlr(&self) -> WLR_R {
        WLR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Watchdog timer Load"]
    #[inline(always)]
    pub fn wlr(&mut self) -> WLR_W {
        WLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog timer Load register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [load](index.html) module"]
pub struct LOAD_SPEC;
impl crate::RegisterSpec for LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [load::R](R) reader structure"]
impl crate::Readable for LOAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [load::W](W) writer structure"]
impl crate::Writable for LOAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOAD to value 0"]
impl crate::Resettable for LOAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
