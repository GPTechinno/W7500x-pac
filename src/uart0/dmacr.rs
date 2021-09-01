#[doc = "Register `DMACR` reader"]
pub struct R(crate::R<DMACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACR` writer"]
pub struct W(crate::W<DMACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACR_SPEC>;
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
impl From<crate::W<DMACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DMA on error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAONERR_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<DMAONERR_A> for bool {
    #[inline(always)]
    fn from(variant: DMAONERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAONERR` reader - DMA on error"]
pub struct DMAONERR_R(crate::FieldReader<bool, DMAONERR_A>);
impl DMAONERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAONERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAONERR_A {
        match self.bits {
            false => DMAONERR_A::DISABLE,
            true => DMAONERR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == DMAONERR_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == DMAONERR_A::ENABLE
    }
}
impl core::ops::Deref for DMAONERR_R {
    type Target = crate::FieldReader<bool, DMAONERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAONERR` writer - DMA on error"]
pub struct DMAONERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAONERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAONERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMAONERR_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMAONERR_A::ENABLE)
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
#[doc = "Transmit DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDMAE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TXDMAE_A> for bool {
    #[inline(always)]
    fn from(variant: TXDMAE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDMAE` reader - Transmit DMA enable"]
pub struct TXDMAE_R(crate::FieldReader<bool, TXDMAE_A>);
impl TXDMAE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXDMAE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDMAE_A {
        match self.bits {
            false => TXDMAE_A::DISABLE,
            true => TXDMAE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TXDMAE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TXDMAE_A::ENABLE
    }
}
impl core::ops::Deref for TXDMAE_R {
    type Target = crate::FieldReader<bool, TXDMAE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDMAE` writer - Transmit DMA enable"]
pub struct TXDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMAE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXDMAE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXDMAE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXDMAE_A::ENABLE)
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
#[doc = "Receive DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDMAE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RXDMAE_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMAE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDMAE` reader - Receive DMA enable"]
pub struct RXDMAE_R(crate::FieldReader<bool, RXDMAE_A>);
impl RXDMAE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXDMAE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDMAE_A {
        match self.bits {
            false => RXDMAE_A::DISABLE,
            true => RXDMAE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RXDMAE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RXDMAE_A::ENABLE
    }
}
impl core::ops::Deref for RXDMAE_R {
    type Target = crate::FieldReader<bool, RXDMAE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDMAE` writer - Receive DMA enable"]
pub struct RXDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMAE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXDMAE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXDMAE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXDMAE_A::ENABLE)
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
    #[doc = "Bit 2 - DMA on error"]
    #[inline(always)]
    pub fn dmaonerr(&self) -> DMAONERR_R {
        DMAONERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit DMA enable"]
    #[inline(always)]
    pub fn txdmae(&self) -> TXDMAE_R {
        TXDMAE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Receive DMA enable"]
    #[inline(always)]
    pub fn rxdmae(&self) -> RXDMAE_R {
        RXDMAE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - DMA on error"]
    #[inline(always)]
    pub fn dmaonerr(&mut self) -> DMAONERR_W {
        DMAONERR_W { w: self }
    }
    #[doc = "Bit 1 - Transmit DMA enable"]
    #[inline(always)]
    pub fn txdmae(&mut self) -> TXDMAE_W {
        TXDMAE_W { w: self }
    }
    #[doc = "Bit 0 - Receive DMA enable"]
    #[inline(always)]
    pub fn rxdmae(&mut self) -> RXDMAE_W {
        RXDMAE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacr](index.html) module"]
pub struct DMACR_SPEC;
impl crate::RegisterSpec for DMACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmacr::R](R) reader structure"]
impl crate::Readable for DMACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmacr::W](W) writer structure"]
impl crate::Writable for DMACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACR to value 0"]
impl crate::Resettable for DMACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
