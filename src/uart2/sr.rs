#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXO` reader - RX buffer overrun, wirte 1 to clear"]
pub struct RXO_R(crate::FieldReader<bool>);
impl RXO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXO_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXO` writer - RX buffer overrun, wirte 1 to clear"]
pub struct RXO_W<'a> {
    w: &'a mut W,
}
impl<'a> RXO_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `TXO` reader - TX buffer overrun, wirte 1 to clear"]
pub struct TXO_R(crate::FieldReader<bool>);
impl TXO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXO_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXO` writer - TX buffer overrun, wirte 1 to clear"]
pub struct TXO_W<'a> {
    w: &'a mut W,
}
impl<'a> TXO_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `RXBF` reader - RX buffer full, read only"]
pub struct RXBF_R(crate::FieldReader<bool>);
impl RXBF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXBF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXBF_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXBF` writer - RX buffer full, read only"]
pub struct RXBF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBF_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `TXBF` reader - TX buffer full, read only"]
pub struct TXBF_R(crate::FieldReader<bool>);
impl TXBF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXBF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXBF_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXBF` writer - TX buffer full, read only"]
pub struct TXBF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBF_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - RX buffer overrun, wirte 1 to clear"]
    #[inline(always)]
    pub fn rxo(&self) -> RXO_R {
        RXO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - TX buffer overrun, wirte 1 to clear"]
    #[inline(always)]
    pub fn txo(&self) -> TXO_R {
        TXO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - RX buffer full, read only"]
    #[inline(always)]
    pub fn rxbf(&self) -> RXBF_R {
        RXBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - TX buffer full, read only"]
    #[inline(always)]
    pub fn txbf(&self) -> TXBF_R {
        TXBF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - RX buffer overrun, wirte 1 to clear"]
    #[inline(always)]
    pub fn rxo(&mut self) -> RXO_W {
        RXO_W { w: self }
    }
    #[doc = "Bit 2 - TX buffer overrun, wirte 1 to clear"]
    #[inline(always)]
    pub fn txo(&mut self) -> TXO_W {
        TXO_W { w: self }
    }
    #[doc = "Bit 1 - RX buffer full, read only"]
    #[inline(always)]
    pub fn rxbf(&mut self) -> RXBF_W {
        RXBF_W { w: self }
    }
    #[doc = "Bit 0 - TX buffer full, read only"]
    #[inline(always)]
    pub fn txbf(&mut self) -> TXBF_W {
        TXBF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
