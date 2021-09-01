#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Capture Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIE_A {
    #[doc = "0: `0`"]
    NOTENABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<CIE_A> for bool {
    #[inline(always)]
    fn from(variant: CIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIE` reader - Capture Interrupt Enable"]
pub struct CIE_R(crate::FieldReader<bool, CIE_A>);
impl CIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIE_A {
        match self.bits {
            false => CIE_A::NOTENABLED,
            true => CIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTENABLED`"]
    #[inline(always)]
    pub fn is_not_enabled(&self) -> bool {
        **self == CIE_A::NOTENABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CIE_A::ENABLED
    }
}
impl core::ops::Deref for CIE_R {
    type Target = crate::FieldReader<bool, CIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CIE` writer - Capture Interrupt Enable"]
pub struct CIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_enabled(self) -> &'a mut W {
        self.variant(CIE_A::NOTENABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CIE_A::ENABLED)
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
#[doc = "Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OIE_A {
    #[doc = "0: `0`"]
    NOTENABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<OIE_A> for bool {
    #[inline(always)]
    fn from(variant: OIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OIE` reader - Overflow Interrupt Enable"]
pub struct OIE_R(crate::FieldReader<bool, OIE_A>);
impl OIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OIE_A {
        match self.bits {
            false => OIE_A::NOTENABLED,
            true => OIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTENABLED`"]
    #[inline(always)]
    pub fn is_not_enabled(&self) -> bool {
        **self == OIE_A::NOTENABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == OIE_A::ENABLED
    }
}
impl core::ops::Deref for OIE_R {
    type Target = crate::FieldReader<bool, OIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OIE` writer - Overflow Interrupt Enable"]
pub struct OIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_enabled(self) -> &'a mut W {
        self.variant(OIE_A::NOTENABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OIE_A::ENABLED)
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
#[doc = "Match Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIE_A {
    #[doc = "0: `0`"]
    NOTENABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<MIE_A> for bool {
    #[inline(always)]
    fn from(variant: MIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIE` reader - Match Interrupt Enable"]
pub struct MIE_R(crate::FieldReader<bool, MIE_A>);
impl MIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIE_A {
        match self.bits {
            false => MIE_A::NOTENABLED,
            true => MIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTENABLED`"]
    #[inline(always)]
    pub fn is_not_enabled(&self) -> bool {
        **self == MIE_A::NOTENABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MIE_A::ENABLED
    }
}
impl core::ops::Deref for MIE_R {
    type Target = crate::FieldReader<bool, MIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIE` writer - Match Interrupt Enable"]
pub struct MIE_W<'a> {
    w: &'a mut W,
}
impl<'a> MIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_enabled(self) -> &'a mut W {
        self.variant(MIE_A::NOTENABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MIE_A::ENABLED)
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
impl R {
    #[doc = "Bit 2 - Capture Interrupt Enable"]
    #[inline(always)]
    pub fn cie(&self) -> CIE_R {
        CIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Match Interrupt Enable"]
    #[inline(always)]
    pub fn mie(&self) -> MIE_R {
        MIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Capture Interrupt Enable"]
    #[inline(always)]
    pub fn cie(&mut self) -> CIE_W {
        CIE_W { w: self }
    }
    #[doc = "Bit 1 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn oie(&mut self) -> OIE_W {
        OIE_W { w: self }
    }
    #[doc = "Bit 0 - Match Interrupt Enable"]
    #[inline(always)]
    pub fn mie(&mut self) -> MIE_W {
        MIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
