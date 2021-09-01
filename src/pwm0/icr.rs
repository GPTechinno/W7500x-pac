#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Capture Interrupt is cleared\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIC_AW {
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<CIC_AW> for bool {
    #[inline(always)]
    fn from(variant: CIC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIC` writer - Capture Interrupt is cleared"]
pub struct CIC_W<'a> {
    w: &'a mut W,
}
impl<'a> CIC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CIC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CIC_AW::CLEAR)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Overflow Interrupt is cleared\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OIC_AW {
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<OIC_AW> for bool {
    #[inline(always)]
    fn from(variant: OIC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OIC` writer - Overflow Interrupt is cleared"]
pub struct OIC_W<'a> {
    w: &'a mut W,
}
impl<'a> OIC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OIC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OIC_AW::CLEAR)
    }
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
#[doc = "Match Interrupt is cleared\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIC_AW {
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<MIC_AW> for bool {
    #[inline(always)]
    fn from(variant: MIC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIC` writer - Match Interrupt is cleared"]
pub struct MIC_W<'a> {
    w: &'a mut W,
}
impl<'a> MIC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MIC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MIC_AW::CLEAR)
    }
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
impl W {
    #[doc = "Bit 2 - Capture Interrupt is cleared"]
    #[inline(always)]
    pub fn cic(&mut self) -> CIC_W {
        CIC_W { w: self }
    }
    #[doc = "Bit 1 - Overflow Interrupt is cleared"]
    #[inline(always)]
    pub fn oic(&mut self) -> OIC_W {
        OIC_W { w: self }
    }
    #[doc = "Bit 0 - Match Interrupt is cleared"]
    #[inline(always)]
    pub fn mic(&mut self) -> MIC_W {
        MIC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
