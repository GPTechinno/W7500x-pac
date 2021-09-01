#[doc = "Register `RIS` reader"]
pub struct R(crate::R<RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OEIM` reader - Overrun error interrupt status"]
pub struct OEIM_R(crate::FieldReader<bool, bool>);
impl OEIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        OEIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OEIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Break error interrupt status\n\nValue on reset: 0"]
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
#[doc = "Field `BEIM` reader - Break error interrupt status"]
pub struct BEIM_R(crate::FieldReader<bool, BEIM_A>);
impl BEIM_R {
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
    type Target = crate::FieldReader<bool, BEIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Parity error interrupt status\n\nValue on reset: 0"]
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
#[doc = "Field `PEIM` reader - Parity error interrupt status"]
pub struct PEIM_R(crate::FieldReader<bool, PEIM_A>);
impl PEIM_R {
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
    type Target = crate::FieldReader<bool, PEIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Framing error interrupt status\n\nValue on reset: 0"]
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
#[doc = "Field `FEIM` reader - Framing error interrupt status"]
pub struct FEIM_R(crate::FieldReader<bool, FEIM_A>);
impl FEIM_R {
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
    type Target = crate::FieldReader<bool, FEIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive interrupt status\n\nValue on reset: 0"]
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
#[doc = "Field `RTIM` reader - Receive interrupt status"]
pub struct RTIM_R(crate::FieldReader<bool, RTIM_A>);
impl RTIM_R {
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
    type Target = crate::FieldReader<bool, RTIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit interrupt status\n\nValue on reset: 0"]
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
#[doc = "Field `TXIM` reader - Transmit interrupt status"]
pub struct TXIM_R(crate::FieldReader<bool, TXIM_A>);
impl TXIM_R {
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
    type Target = crate::FieldReader<bool, TXIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive interrupt status\n\nValue on reset: 0"]
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
#[doc = "Field `RXIM` reader - Receive interrupt status"]
pub struct RXIM_R(crate::FieldReader<bool, RXIM_A>);
impl RXIM_R {
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
    type Target = crate::FieldReader<bool, RXIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "nUARTDSR modem interrupt status\n\nValue on reset: 0"]
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
#[doc = "Field `DSRMIM` reader - nUARTDSR modem interrupt status"]
pub struct DSRMIM_R(crate::FieldReader<bool, DSRMIM_A>);
impl DSRMIM_R {
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
    type Target = crate::FieldReader<bool, DSRMIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "nUARTDCD modem interrupt status\n\nValue on reset: 0"]
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
#[doc = "Field `DCDMIM` reader - nUARTDCD modem interrupt status"]
pub struct DCDMIM_R(crate::FieldReader<bool, DCDMIM_A>);
impl DCDMIM_R {
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
    type Target = crate::FieldReader<bool, DCDMIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "nUARTCTS modem interrupt status\n\nValue on reset: 0"]
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
#[doc = "Field `CTSMIM` reader - nUARTCTS modem interrupt status"]
pub struct CTSMIM_R(crate::FieldReader<bool, CTSMIM_A>);
impl CTSMIM_R {
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
    type Target = crate::FieldReader<bool, CTSMIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "nUARTRI modem interrupt status\n\nValue on reset: 0"]
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
#[doc = "Field `RIMIM` reader - nUARTRI modem interrupt status"]
pub struct RIMIM_R(crate::FieldReader<bool, RIMIM_A>);
impl RIMIM_R {
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
    type Target = crate::FieldReader<bool, RIMIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 10 - Overrun error interrupt status"]
    #[inline(always)]
    pub fn oeim(&self) -> OEIM_R {
        OEIM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Break error interrupt status"]
    #[inline(always)]
    pub fn beim(&self) -> BEIM_R {
        BEIM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Parity error interrupt status"]
    #[inline(always)]
    pub fn peim(&self) -> PEIM_R {
        PEIM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Framing error interrupt status"]
    #[inline(always)]
    pub fn feim(&self) -> FEIM_R {
        FEIM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive interrupt status"]
    #[inline(always)]
    pub fn rtim(&self) -> RTIM_R {
        RTIM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit interrupt status"]
    #[inline(always)]
    pub fn txim(&self) -> TXIM_R {
        TXIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive interrupt status"]
    #[inline(always)]
    pub fn rxim(&self) -> RXIM_R {
        RXIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - nUARTDSR modem interrupt status"]
    #[inline(always)]
    pub fn dsrmim(&self) -> DSRMIM_R {
        DSRMIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - nUARTDCD modem interrupt status"]
    #[inline(always)]
    pub fn dcdmim(&self) -> DCDMIM_R {
        DCDMIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - nUARTCTS modem interrupt status"]
    #[inline(always)]
    pub fn ctsmim(&self) -> CTSMIM_R {
        CTSMIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - nUARTRI modem interrupt status"]
    #[inline(always)]
    pub fn rimim(&self) -> RIMIM_R {
        RIMIM_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](index.html) module"]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ris::R](R) reader structure"]
impl crate::Readable for RIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
