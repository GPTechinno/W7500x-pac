#[doc = "Register `IMSC` reader"]
pub struct R(crate::R<IMSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMSC` writer"]
pub struct W(crate::W<IMSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMSC_SPEC>;
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
impl From<crate::W<IMSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Overrun error interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OEIM_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<OEIM_A> for bool {
    #[inline(always)]
    fn from(variant: OEIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OEIM` reader - Overrun error interrupt mask"]
pub struct OEIM_R(crate::FieldReader<bool, OEIM_A>);
impl OEIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        OEIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OEIM_A {
        match self.bits {
            false => OEIM_A::DISABLE,
            true => OEIM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == OEIM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == OEIM_A::ENABLE
    }
}
impl core::ops::Deref for OEIM_R {
    type Target = crate::FieldReader<bool, OEIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OEIM` writer - Overrun error interrupt mask"]
pub struct OEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> OEIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OEIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OEIM_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(OEIM_A::ENABLE)
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
#[doc = "Break error interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEIM_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<BEIM_A> for bool {
    #[inline(always)]
    fn from(variant: BEIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BEIM` reader - Break error interrupt mask"]
pub struct BEIM_R(crate::FieldReader<bool, BEIM_A>);
impl BEIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        BEIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BEIM_A {
        match self.bits {
            false => BEIM_A::DISABLE,
            true => BEIM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == BEIM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == BEIM_A::ENABLE
    }
}
impl core::ops::Deref for BEIM_R {
    type Target = crate::FieldReader<bool, BEIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BEIM` writer - Break error interrupt mask"]
pub struct BEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> BEIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BEIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BEIM_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BEIM_A::ENABLE)
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
#[doc = "Parity error interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEIM_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<PEIM_A> for bool {
    #[inline(always)]
    fn from(variant: PEIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEIM` reader - Parity error interrupt mask"]
pub struct PEIM_R(crate::FieldReader<bool, PEIM_A>);
impl PEIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEIM_A {
        match self.bits {
            false => PEIM_A::DISABLE,
            true => PEIM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == PEIM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == PEIM_A::ENABLE
    }
}
impl core::ops::Deref for PEIM_R {
    type Target = crate::FieldReader<bool, PEIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEIM` writer - Parity error interrupt mask"]
pub struct PEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PEIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PEIM_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PEIM_A::ENABLE)
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
#[doc = "Framing error interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIM_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<FEIM_A> for bool {
    #[inline(always)]
    fn from(variant: FEIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEIM` reader - Framing error interrupt mask"]
pub struct FEIM_R(crate::FieldReader<bool, FEIM_A>);
impl FEIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        FEIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEIM_A {
        match self.bits {
            false => FEIM_A::DISABLE,
            true => FEIM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FEIM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FEIM_A::ENABLE
    }
}
impl core::ops::Deref for FEIM_R {
    type Target = crate::FieldReader<bool, FEIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEIM` writer - Framing error interrupt mask"]
pub struct FEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FEIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FEIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FEIM_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FEIM_A::ENABLE)
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
#[doc = "Receive interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTIM_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RTIM_A> for bool {
    #[inline(always)]
    fn from(variant: RTIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTIM` reader - Receive interrupt mask"]
pub struct RTIM_R(crate::FieldReader<bool, RTIM_A>);
impl RTIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTIM_A {
        match self.bits {
            false => RTIM_A::DISABLE,
            true => RTIM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RTIM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RTIM_A::ENABLE
    }
}
impl core::ops::Deref for RTIM_R {
    type Target = crate::FieldReader<bool, RTIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTIM` writer - Receive interrupt mask"]
pub struct RTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RTIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RTIM_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTIM_A::ENABLE)
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
#[doc = "Transmit interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXIM_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TXIM_A> for bool {
    #[inline(always)]
    fn from(variant: TXIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXIM` reader - Transmit interrupt mask"]
pub struct TXIM_R(crate::FieldReader<bool, TXIM_A>);
impl TXIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXIM_A {
        match self.bits {
            false => TXIM_A::DISABLE,
            true => TXIM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TXIM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TXIM_A::ENABLE
    }
}
impl core::ops::Deref for TXIM_R {
    type Target = crate::FieldReader<bool, TXIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXIM` writer - Transmit interrupt mask"]
pub struct TXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXIM_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXIM_A::ENABLE)
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
#[doc = "Receive interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIM_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RXIM_A> for bool {
    #[inline(always)]
    fn from(variant: RXIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXIM` reader - Receive interrupt mask"]
pub struct RXIM_R(crate::FieldReader<bool, RXIM_A>);
impl RXIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXIM_A {
        match self.bits {
            false => RXIM_A::DISABLE,
            true => RXIM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RXIM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RXIM_A::ENABLE
    }
}
impl core::ops::Deref for RXIM_R {
    type Target = crate::FieldReader<bool, RXIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIM` writer - Receive interrupt mask"]
pub struct RXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXIM_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXIM_A::ENABLE)
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
#[doc = "nUARTDSR modem interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSRMIM_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<DSRMIM_A> for bool {
    #[inline(always)]
    fn from(variant: DSRMIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSRMIM` reader - nUARTDSR modem interrupt mask"]
pub struct DSRMIM_R(crate::FieldReader<bool, DSRMIM_A>);
impl DSRMIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSRMIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSRMIM_A {
        match self.bits {
            false => DSRMIM_A::DISABLE,
            true => DSRMIM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == DSRMIM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == DSRMIM_A::ENABLE
    }
}
impl core::ops::Deref for DSRMIM_R {
    type Target = crate::FieldReader<bool, DSRMIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSRMIM` writer - nUARTDSR modem interrupt mask"]
pub struct DSRMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DSRMIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSRMIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DSRMIM_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DSRMIM_A::ENABLE)
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
#[doc = "nUARTDCD modem interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDMIM_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<DCDMIM_A> for bool {
    #[inline(always)]
    fn from(variant: DCDMIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCDMIM` reader - nUARTDCD modem interrupt mask"]
pub struct DCDMIM_R(crate::FieldReader<bool, DCDMIM_A>);
impl DCDMIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCDMIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDMIM_A {
        match self.bits {
            false => DCDMIM_A::DISABLE,
            true => DCDMIM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == DCDMIM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == DCDMIM_A::ENABLE
    }
}
impl core::ops::Deref for DCDMIM_R {
    type Target = crate::FieldReader<bool, DCDMIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDMIM` writer - nUARTDCD modem interrupt mask"]
pub struct DCDMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDMIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCDMIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DCDMIM_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DCDMIM_A::ENABLE)
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
#[doc = "nUARTCTS modem interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSMIM_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTSMIM_A> for bool {
    #[inline(always)]
    fn from(variant: CTSMIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSMIM` reader - nUARTCTS modem interrupt mask"]
pub struct CTSMIM_R(crate::FieldReader<bool, CTSMIM_A>);
impl CTSMIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTSMIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSMIM_A {
        match self.bits {
            false => CTSMIM_A::DISABLE,
            true => CTSMIM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CTSMIM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CTSMIM_A::ENABLE
    }
}
impl core::ops::Deref for CTSMIM_R {
    type Target = crate::FieldReader<bool, CTSMIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTSMIM` writer - nUARTCTS modem interrupt mask"]
pub struct CTSMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSMIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTSMIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTSMIM_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTSMIM_A::ENABLE)
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
#[doc = "nUARTRI modem interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIMIM_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RIMIM_A> for bool {
    #[inline(always)]
    fn from(variant: RIMIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIMIM` reader - nUARTRI modem interrupt mask"]
pub struct RIMIM_R(crate::FieldReader<bool, RIMIM_A>);
impl RIMIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RIMIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RIMIM_A {
        match self.bits {
            false => RIMIM_A::DISABLE,
            true => RIMIM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RIMIM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RIMIM_A::ENABLE
    }
}
impl core::ops::Deref for RIMIM_R {
    type Target = crate::FieldReader<bool, RIMIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RIMIM` writer - nUARTRI modem interrupt mask"]
pub struct RIMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RIMIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RIMIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RIMIM_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RIMIM_A::ENABLE)
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
    #[doc = "Bit 10 - Overrun error interrupt mask"]
    #[inline(always)]
    pub fn oeim(&self) -> OEIM_R {
        OEIM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Break error interrupt mask"]
    #[inline(always)]
    pub fn beim(&self) -> BEIM_R {
        BEIM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Parity error interrupt mask"]
    #[inline(always)]
    pub fn peim(&self) -> PEIM_R {
        PEIM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Framing error interrupt mask"]
    #[inline(always)]
    pub fn feim(&self) -> FEIM_R {
        FEIM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive interrupt mask"]
    #[inline(always)]
    pub fn rtim(&self) -> RTIM_R {
        RTIM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit interrupt mask"]
    #[inline(always)]
    pub fn txim(&self) -> TXIM_R {
        TXIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive interrupt mask"]
    #[inline(always)]
    pub fn rxim(&self) -> RXIM_R {
        RXIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - nUARTDSR modem interrupt mask"]
    #[inline(always)]
    pub fn dsrmim(&self) -> DSRMIM_R {
        DSRMIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - nUARTDCD modem interrupt mask"]
    #[inline(always)]
    pub fn dcdmim(&self) -> DCDMIM_R {
        DCDMIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - nUARTCTS modem interrupt mask"]
    #[inline(always)]
    pub fn ctsmim(&self) -> CTSMIM_R {
        CTSMIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - nUARTRI modem interrupt mask"]
    #[inline(always)]
    pub fn rimim(&self) -> RIMIM_R {
        RIMIM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - Overrun error interrupt mask"]
    #[inline(always)]
    pub fn oeim(&mut self) -> OEIM_W {
        OEIM_W { w: self }
    }
    #[doc = "Bit 9 - Break error interrupt mask"]
    #[inline(always)]
    pub fn beim(&mut self) -> BEIM_W {
        BEIM_W { w: self }
    }
    #[doc = "Bit 8 - Parity error interrupt mask"]
    #[inline(always)]
    pub fn peim(&mut self) -> PEIM_W {
        PEIM_W { w: self }
    }
    #[doc = "Bit 7 - Framing error interrupt mask"]
    #[inline(always)]
    pub fn feim(&mut self) -> FEIM_W {
        FEIM_W { w: self }
    }
    #[doc = "Bit 6 - Receive interrupt mask"]
    #[inline(always)]
    pub fn rtim(&mut self) -> RTIM_W {
        RTIM_W { w: self }
    }
    #[doc = "Bit 5 - Transmit interrupt mask"]
    #[inline(always)]
    pub fn txim(&mut self) -> TXIM_W {
        TXIM_W { w: self }
    }
    #[doc = "Bit 4 - Receive interrupt mask"]
    #[inline(always)]
    pub fn rxim(&mut self) -> RXIM_W {
        RXIM_W { w: self }
    }
    #[doc = "Bit 3 - nUARTDSR modem interrupt mask"]
    #[inline(always)]
    pub fn dsrmim(&mut self) -> DSRMIM_W {
        DSRMIM_W { w: self }
    }
    #[doc = "Bit 2 - nUARTDCD modem interrupt mask"]
    #[inline(always)]
    pub fn dcdmim(&mut self) -> DCDMIM_W {
        DCDMIM_W { w: self }
    }
    #[doc = "Bit 1 - nUARTCTS modem interrupt mask"]
    #[inline(always)]
    pub fn ctsmim(&mut self) -> CTSMIM_W {
        CTSMIM_W { w: self }
    }
    #[doc = "Bit 0 - nUARTRI modem interrupt mask"]
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
#[doc = "Interrupt Mask Set / Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imsc](index.html) module"]
pub struct IMSC_SPEC;
impl crate::RegisterSpec for IMSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imsc::R](R) reader structure"]
impl crate::Readable for IMSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imsc::W](W) writer structure"]
impl crate::Writable for IMSC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMSC to value 0"]
impl crate::Resettable for IMSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
