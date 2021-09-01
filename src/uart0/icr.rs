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
#[doc = "Overrun error interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OEIM_AW {
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<OEIM_AW> for bool {
    #[inline(always)]
    fn from(variant: OEIM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OEIM` writer - Overrun error interrupt clear"]
pub struct OEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> OEIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OEIM_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OEIM_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Break error interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEIM_AW {
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<BEIM_AW> for bool {
    #[inline(always)]
    fn from(variant: BEIM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BEIM` writer - Break error interrupt clear"]
pub struct BEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> BEIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BEIM_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BEIM_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Parity error interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEIM_AW {
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<PEIM_AW> for bool {
    #[inline(always)]
    fn from(variant: PEIM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEIM` writer - Parity error interrupt clear"]
pub struct PEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PEIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEIM_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PEIM_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Framing error interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIM_AW {
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<FEIM_AW> for bool {
    #[inline(always)]
    fn from(variant: FEIM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEIM` writer - Framing error interrupt clear"]
pub struct FEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FEIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FEIM_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FEIM_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Receive interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTIM_AW {
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<RTIM_AW> for bool {
    #[inline(always)]
    fn from(variant: RTIM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTIM` writer - Receive interrupt clear"]
pub struct RTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RTIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTIM_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RTIM_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Transmit interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXIM_AW {
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<TXIM_AW> for bool {
    #[inline(always)]
    fn from(variant: TXIM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXIM` writer - Transmit interrupt clear"]
pub struct TXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXIM_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TXIM_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Receive interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIM_AW {
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<RXIM_AW> for bool {
    #[inline(always)]
    fn from(variant: RXIM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXIM` writer - Receive interrupt clear"]
pub struct RXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXIM_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXIM_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "nUARTDSR modem interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSRMIM_AW {
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<DSRMIM_AW> for bool {
    #[inline(always)]
    fn from(variant: DSRMIM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSRMIM` writer - nUARTDSR modem interrupt clear"]
pub struct DSRMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DSRMIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSRMIM_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DSRMIM_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "nUARTDCD modem interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDMIM_AW {
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<DCDMIM_AW> for bool {
    #[inline(always)]
    fn from(variant: DCDMIM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCDMIM` writer - nUARTDCD modem interrupt clear"]
pub struct DCDMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDMIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCDMIM_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DCDMIM_AW::CLEAR)
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
#[doc = "nUARTCTS modem interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSMIM_AW {
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<CTSMIM_AW> for bool {
    #[inline(always)]
    fn from(variant: CTSMIM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSMIM` writer - nUARTCTS modem interrupt clear"]
pub struct CTSMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSMIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTSMIM_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTSMIM_AW::CLEAR)
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
#[doc = "nUARTRI modem interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIMIM_AW {
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<RIMIM_AW> for bool {
    #[inline(always)]
    fn from(variant: RIMIM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIMIM` writer - nUARTRI modem interrupt clear"]
pub struct RIMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RIMIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RIMIM_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RIMIM_AW::CLEAR)
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
    #[doc = "Bit 10 - Overrun error interrupt clear"]
    #[inline(always)]
    pub fn oeim(&mut self) -> OEIM_W {
        OEIM_W { w: self }
    }
    #[doc = "Bit 9 - Break error interrupt clear"]
    #[inline(always)]
    pub fn beim(&mut self) -> BEIM_W {
        BEIM_W { w: self }
    }
    #[doc = "Bit 8 - Parity error interrupt clear"]
    #[inline(always)]
    pub fn peim(&mut self) -> PEIM_W {
        PEIM_W { w: self }
    }
    #[doc = "Bit 7 - Framing error interrupt clear"]
    #[inline(always)]
    pub fn feim(&mut self) -> FEIM_W {
        FEIM_W { w: self }
    }
    #[doc = "Bit 6 - Receive interrupt clear"]
    #[inline(always)]
    pub fn rtim(&mut self) -> RTIM_W {
        RTIM_W { w: self }
    }
    #[doc = "Bit 5 - Transmit interrupt clear"]
    #[inline(always)]
    pub fn txim(&mut self) -> TXIM_W {
        TXIM_W { w: self }
    }
    #[doc = "Bit 4 - Receive interrupt clear"]
    #[inline(always)]
    pub fn rxim(&mut self) -> RXIM_W {
        RXIM_W { w: self }
    }
    #[doc = "Bit 3 - nUARTDSR modem interrupt clear"]
    #[inline(always)]
    pub fn dsrmim(&mut self) -> DSRMIM_W {
        DSRMIM_W { w: self }
    }
    #[doc = "Bit 2 - nUARTDCD modem interrupt clear"]
    #[inline(always)]
    pub fn dcdmim(&mut self) -> DCDMIM_W {
        DCDMIM_W { w: self }
    }
    #[doc = "Bit 1 - nUARTCTS modem interrupt clear"]
    #[inline(always)]
    pub fn ctsmim(&mut self) -> CTSMIM_W {
        CTSMIM_W { w: self }
    }
    #[doc = "Bit 0 - nUARTRI modem interrupt clear"]
    #[inline(always)]
    pub fn rimim(&mut self) -> RIMIM_W {
        RIMIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
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
