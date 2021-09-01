#[doc = "Register `RSTINFO` reader"]
pub struct R(crate::R<RSTINFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTINFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTINFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTINFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTINFO` writer"]
pub struct W(crate::W<RSTINFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTINFO_SPEC>;
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
impl From<crate::W<RSTINFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTINFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYS` reader - Reset Request Enable"]
pub struct SYS_R(crate::FieldReader<bool, bool>);
impl SYS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYS` writer - Reset Request Enable"]
pub struct SYS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `RST` reader - Interrupt Enable"]
pub struct RST_R(crate::FieldReader<bool, bool>);
impl RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RST` writer - Interrupt Enable"]
pub struct RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Reset Request Enable"]
    #[inline(always)]
    pub fn sys(&self) -> SYS_R {
        SYS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt Enable"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset Request Enable"]
    #[inline(always)]
    pub fn sys(&mut self) -> SYS_W {
        SYS_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt Enable"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W {
        RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Information Register (R/W)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstinfo](index.html) module"]
pub struct RSTINFO_SPEC;
impl crate::RegisterSpec for RSTINFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstinfo::R](R) reader structure"]
impl crate::Readable for RSTINFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstinfo::W](W) writer structure"]
impl crate::Writable for RSTINFO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSTINFO to value 0"]
impl crate::Resettable for RSTINFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
