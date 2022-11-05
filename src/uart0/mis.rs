#[doc = "Register `MIS` reader"]
pub struct R(crate::R<MIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Overrun error masked interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OEIM_A {
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<OEIM_A> for bool {
    #[inline(always)]
    fn from(variant: OEIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OEIM` reader - Overrun error masked interrupt status"]
pub struct OEIM_R(crate::FieldReader<bool>);
impl OEIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OEIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OEIM_A> {
        match self.bits {
            true => Some(OEIM_A::SET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == OEIM_A::SET
    }
}
impl core::ops::Deref for OEIM_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Break error masked interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEIM_A {
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<BEIM_A> for bool {
    #[inline(always)]
    fn from(variant: BEIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BEIM` reader - Break error masked interrupt status"]
pub struct BEIM_R(crate::FieldReader<bool>);
impl BEIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BEIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BEIM_A> {
        match self.bits {
            true => Some(BEIM_A::SET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == BEIM_A::SET
    }
}
impl core::ops::Deref for BEIM_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Parity error masked interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEIM_A {
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<PEIM_A> for bool {
    #[inline(always)]
    fn from(variant: PEIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEIM` reader - Parity error masked interrupt status"]
pub struct PEIM_R(crate::FieldReader<bool>);
impl PEIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PEIM_A> {
        match self.bits {
            true => Some(PEIM_A::SET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == PEIM_A::SET
    }
}
impl core::ops::Deref for PEIM_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Framing error masked interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIM_A {
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<FEIM_A> for bool {
    #[inline(always)]
    fn from(variant: FEIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEIM` reader - Framing error masked interrupt status"]
pub struct FEIM_R(crate::FieldReader<bool>);
impl FEIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FEIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FEIM_A> {
        match self.bits {
            true => Some(FEIM_A::SET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == FEIM_A::SET
    }
}
impl core::ops::Deref for FEIM_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive masked interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTIM_A {
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<RTIM_A> for bool {
    #[inline(always)]
    fn from(variant: RTIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTIM` reader - Receive masked interrupt status"]
pub struct RTIM_R(crate::FieldReader<bool>);
impl RTIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RTIM_A> {
        match self.bits {
            true => Some(RTIM_A::SET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == RTIM_A::SET
    }
}
impl core::ops::Deref for RTIM_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit masked interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXIM_A {
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<TXIM_A> for bool {
    #[inline(always)]
    fn from(variant: TXIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXIM` reader - Transmit masked interrupt status"]
pub struct TXIM_R(crate::FieldReader<bool>);
impl TXIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXIM_A> {
        match self.bits {
            true => Some(TXIM_A::SET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == TXIM_A::SET
    }
}
impl core::ops::Deref for TXIM_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive masked interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIM_A {
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<RXIM_A> for bool {
    #[inline(always)]
    fn from(variant: RXIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXIM` reader - Receive masked interrupt status"]
pub struct RXIM_R(crate::FieldReader<bool>);
impl RXIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXIM_A> {
        match self.bits {
            true => Some(RXIM_A::SET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == RXIM_A::SET
    }
}
impl core::ops::Deref for RXIM_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "nUARTDSR modem masked interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSRMIM_A {
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<DSRMIM_A> for bool {
    #[inline(always)]
    fn from(variant: DSRMIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSRMIM` reader - nUARTDSR modem masked interrupt status"]
pub struct DSRMIM_R(crate::FieldReader<bool>);
impl DSRMIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSRMIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DSRMIM_A> {
        match self.bits {
            true => Some(DSRMIM_A::SET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == DSRMIM_A::SET
    }
}
impl core::ops::Deref for DSRMIM_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "nUARTDCD modem masked interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDMIM_A {
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<DCDMIM_A> for bool {
    #[inline(always)]
    fn from(variant: DCDMIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCDMIM` reader - nUARTDCD modem masked interrupt status"]
pub struct DCDMIM_R(crate::FieldReader<bool>);
impl DCDMIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCDMIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DCDMIM_A> {
        match self.bits {
            true => Some(DCDMIM_A::SET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == DCDMIM_A::SET
    }
}
impl core::ops::Deref for DCDMIM_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "nUARTCTS modem masked interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSMIM_A {
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<CTSMIM_A> for bool {
    #[inline(always)]
    fn from(variant: CTSMIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSMIM` reader - nUARTCTS modem masked interrupt status"]
pub struct CTSMIM_R(crate::FieldReader<bool>);
impl CTSMIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTSMIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CTSMIM_A> {
        match self.bits {
            true => Some(CTSMIM_A::SET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == CTSMIM_A::SET
    }
}
impl core::ops::Deref for CTSMIM_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "nUARTRI modem masked interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIMIM_A {
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<RIMIM_A> for bool {
    #[inline(always)]
    fn from(variant: RIMIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIMIM` reader - nUARTRI modem masked interrupt status"]
pub struct RIMIM_R(crate::FieldReader<bool>);
impl RIMIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RIMIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RIMIM_A> {
        match self.bits {
            true => Some(RIMIM_A::SET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == RIMIM_A::SET
    }
}
impl core::ops::Deref for RIMIM_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 10 - Overrun error masked interrupt status"]
    #[inline(always)]
    pub fn oeim(&self) -> OEIM_R {
        OEIM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Break error masked interrupt status"]
    #[inline(always)]
    pub fn beim(&self) -> BEIM_R {
        BEIM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity error masked interrupt status"]
    #[inline(always)]
    pub fn peim(&self) -> PEIM_R {
        PEIM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Framing error masked interrupt status"]
    #[inline(always)]
    pub fn feim(&self) -> FEIM_R {
        FEIM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive masked interrupt status"]
    #[inline(always)]
    pub fn rtim(&self) -> RTIM_R {
        RTIM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit masked interrupt status"]
    #[inline(always)]
    pub fn txim(&self) -> TXIM_R {
        TXIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive masked interrupt status"]
    #[inline(always)]
    pub fn rxim(&self) -> RXIM_R {
        RXIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - nUARTDSR modem masked interrupt status"]
    #[inline(always)]
    pub fn dsrmim(&self) -> DSRMIM_R {
        DSRMIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - nUARTDCD modem masked interrupt status"]
    #[inline(always)]
    pub fn dcdmim(&self) -> DCDMIM_R {
        DCDMIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - nUARTCTS modem masked interrupt status"]
    #[inline(always)]
    pub fn ctsmim(&self) -> CTSMIM_R {
        CTSMIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - nUARTRI modem masked interrupt status"]
    #[inline(always)]
    pub fn rimim(&self) -> RIMIM_R {
        RIMIM_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](index.html) module"]
pub struct MIS_SPEC;
impl crate::RegisterSpec for MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mis::R](R) reader structure"]
impl crate::Readable for MIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
