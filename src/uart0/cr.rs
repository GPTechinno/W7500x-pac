#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CTS hardware flow control enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSEN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CTSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSEn` reader - CTS hardware flow control enable"]
pub struct CTSEN_R(crate::FieldReader<bool, CTSEN_A>);
impl CTSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTSEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSEN_A {
        match self.bits {
            false => CTSEN_A::DISABLE,
            true => CTSEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CTSEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CTSEN_A::ENABLE
    }
}
impl core::ops::Deref for CTSEN_R {
    type Target = crate::FieldReader<bool, CTSEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTSEn` writer - CTS hardware flow control enable"]
pub struct CTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTSEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTSEN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTSEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "RTS hardware flow control enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSEN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSEn` reader - RTS hardware flow control enable"]
pub struct RTSEN_R(crate::FieldReader<bool, RTSEN_A>);
impl RTSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTSEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTSEN_A {
        match self.bits {
            false => RTSEN_A::DISABLE,
            true => RTSEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RTSEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RTSEN_A::ENABLE
    }
}
impl core::ops::Deref for RTSEN_R {
    type Target = crate::FieldReader<bool, RTSEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTSEn` writer - RTS hardware flow control enable"]
pub struct RTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTSEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RTSEN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTSEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Complement of Out2 modem status output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT2_A {
    #[doc = "0: `0`"]
    HIGH = 0,
    #[doc = "1: `1`"]
    LOW = 1,
}
impl From<OUT2_A> for bool {
    #[inline(always)]
    fn from(variant: OUT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Out2` reader - Complement of Out2 modem status output"]
pub struct OUT2_R(crate::FieldReader<bool, OUT2_A>);
impl OUT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUT2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT2_A {
        match self.bits {
            false => OUT2_A::HIGH,
            true => OUT2_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == OUT2_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == OUT2_A::LOW
    }
}
impl core::ops::Deref for OUT2_R {
    type Target = crate::FieldReader<bool, OUT2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Out2` writer - Complement of Out2 modem status output"]
pub struct OUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OUT2_A::HIGH)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OUT2_A::LOW)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Complement of Out1 modem status output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT1_A {
    #[doc = "0: `0`"]
    HIGH = 0,
    #[doc = "1: `1`"]
    LOW = 1,
}
impl From<OUT1_A> for bool {
    #[inline(always)]
    fn from(variant: OUT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Out1` reader - Complement of Out1 modem status output"]
pub struct OUT1_R(crate::FieldReader<bool, OUT1_A>);
impl OUT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUT1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT1_A {
        match self.bits {
            false => OUT1_A::HIGH,
            true => OUT1_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == OUT1_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == OUT1_A::LOW
    }
}
impl core::ops::Deref for OUT1_R {
    type Target = crate::FieldReader<bool, OUT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Out1` writer - Complement of Out1 modem status output"]
pub struct OUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OUT1_A::HIGH)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OUT1_A::LOW)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Request to send\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTS_A {
    #[doc = "0: `0`"]
    HIGH = 0,
    #[doc = "1: `1`"]
    LOW = 1,
}
impl From<RTS_A> for bool {
    #[inline(always)]
    fn from(variant: RTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTS` reader - Request to send"]
pub struct RTS_R(crate::FieldReader<bool, RTS_A>);
impl RTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTS_A {
        match self.bits {
            false => RTS_A::HIGH,
            true => RTS_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == RTS_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == RTS_A::LOW
    }
}
impl core::ops::Deref for RTS_R {
    type Target = crate::FieldReader<bool, RTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTS` writer - Request to send"]
pub struct RTS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(RTS_A::HIGH)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(RTS_A::LOW)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Data transmit ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTR_A {
    #[doc = "0: `0`"]
    HIGH = 0,
    #[doc = "1: `1`"]
    LOW = 1,
}
impl From<DTR_A> for bool {
    #[inline(always)]
    fn from(variant: DTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTR` reader - Data transmit ready"]
pub struct DTR_R(crate::FieldReader<bool, DTR_A>);
impl DTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTR_A {
        match self.bits {
            false => DTR_A::HIGH,
            true => DTR_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == DTR_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == DTR_A::LOW
    }
}
impl core::ops::Deref for DTR_R {
    type Target = crate::FieldReader<bool, DTR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTR` writer - Data transmit ready"]
pub struct DTR_W<'a> {
    w: &'a mut W,
}
impl<'a> DTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(DTR_A::HIGH)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(DTR_A::LOW)
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
#[doc = "Receive enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RXE_A> for bool {
    #[inline(always)]
    fn from(variant: RXE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXE` reader - Receive enable"]
pub struct RXE_R(crate::FieldReader<bool, RXE_A>);
impl RXE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXE_A {
        match self.bits {
            false => RXE_A::DISABLE,
            true => RXE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RXE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RXE_A::ENABLE
    }
}
impl core::ops::Deref for RXE_R {
    type Target = crate::FieldReader<bool, RXE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXE` writer - Receive enable"]
pub struct RXE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXE_A::ENABLE)
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
#[doc = "Transmit enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TXE_A> for bool {
    #[inline(always)]
    fn from(variant: TXE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXE` reader - Transmit enable"]
pub struct TXE_R(crate::FieldReader<bool, TXE_A>);
impl TXE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXE_A {
        match self.bits {
            false => TXE_A::DISABLE,
            true => TXE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TXE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TXE_A::ENABLE
    }
}
impl core::ops::Deref for TXE_R {
    type Target = crate::FieldReader<bool, TXE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXE` writer - Transmit enable"]
pub struct TXE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXE_A::ENABLE)
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
#[doc = "IrDA SIR low power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIRLP_A {
    #[doc = "0: low level bits are transmitted as an active high pulse with a width of 3/16th of the bit period."]
    DISABLE = 0,
    #[doc = "1: low level bits are transmitted with a pulse width that is 3 times the period of the IrLPBaud16 input signal, regardless of the selected bit rate."]
    ENABLE = 1,
}
impl From<SIRLP_A> for bool {
    #[inline(always)]
    fn from(variant: SIRLP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIRLP` reader - IrDA SIR low power mode"]
pub struct SIRLP_R(crate::FieldReader<bool, SIRLP_A>);
impl SIRLP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIRLP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIRLP_A {
        match self.bits {
            false => SIRLP_A::DISABLE,
            true => SIRLP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SIRLP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SIRLP_A::ENABLE
    }
}
impl core::ops::Deref for SIRLP_R {
    type Target = crate::FieldReader<bool, SIRLP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIRLP` writer - IrDA SIR low power mode"]
pub struct SIRLP_W<'a> {
    w: &'a mut W,
}
impl<'a> SIRLP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIRLP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "low level bits are transmitted as an active high pulse with a width of 3/16th of the bit period."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SIRLP_A::DISABLE)
    }
    #[doc = "low level bits are transmitted with a pulse width that is 3 times the period of the IrLPBaud16 input signal, regardless of the selected bit rate."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SIRLP_A::ENABLE)
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
#[doc = "SIR enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIREN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<SIREN_A> for bool {
    #[inline(always)]
    fn from(variant: SIREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIREN` reader - SIR enable"]
pub struct SIREN_R(crate::FieldReader<bool, SIREN_A>);
impl SIREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIREN_A {
        match self.bits {
            false => SIREN_A::DISABLE,
            true => SIREN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SIREN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SIREN_A::ENABLE
    }
}
impl core::ops::Deref for SIREN_R {
    type Target = crate::FieldReader<bool, SIREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIREN` writer - SIR enable"]
pub struct SIREN_W<'a> {
    w: &'a mut W,
}
impl<'a> SIREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SIREN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SIREN_A::ENABLE)
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
#[doc = "UART enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UARTEN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<UARTEN_A> for bool {
    #[inline(always)]
    fn from(variant: UARTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UARTEN` reader - UART enable"]
pub struct UARTEN_R(crate::FieldReader<bool, UARTEN_A>);
impl UARTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UARTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UARTEN_A {
        match self.bits {
            false => UARTEN_A::DISABLE,
            true => UARTEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == UARTEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == UARTEN_A::ENABLE
    }
}
impl core::ops::Deref for UARTEN_R {
    type Target = crate::FieldReader<bool, UARTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UARTEN` writer - UART enable"]
pub struct UARTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UARTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UARTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(UARTEN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(UARTEN_A::ENABLE)
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
    #[doc = "Bit 15 - CTS hardware flow control enable"]
    #[inline(always)]
    pub fn ctsen(&self) -> CTSEN_R {
        CTSEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RTS hardware flow control enable"]
    #[inline(always)]
    pub fn rtsen(&self) -> RTSEN_R {
        RTSEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Complement of Out2 modem status output"]
    #[inline(always)]
    pub fn out2(&self) -> OUT2_R {
        OUT2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Complement of Out1 modem status output"]
    #[inline(always)]
    pub fn out1(&self) -> OUT1_R {
        OUT1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Request to send"]
    #[inline(always)]
    pub fn rts(&self) -> RTS_R {
        RTS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Data transmit ready"]
    #[inline(always)]
    pub fn dtr(&self) -> DTR_R {
        DTR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Receive enable"]
    #[inline(always)]
    pub fn rxe(&self) -> RXE_R {
        RXE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmit enable"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IrDA SIR low power mode"]
    #[inline(always)]
    pub fn sirlp(&self) -> SIRLP_R {
        SIRLP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - SIR enable"]
    #[inline(always)]
    pub fn siren(&self) -> SIREN_R {
        SIREN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - UART enable"]
    #[inline(always)]
    pub fn uarten(&self) -> UARTEN_R {
        UARTEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - CTS hardware flow control enable"]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CTSEN_W {
        CTSEN_W { w: self }
    }
    #[doc = "Bit 14 - RTS hardware flow control enable"]
    #[inline(always)]
    pub fn rtsen(&mut self) -> RTSEN_W {
        RTSEN_W { w: self }
    }
    #[doc = "Bit 13 - Complement of Out2 modem status output"]
    #[inline(always)]
    pub fn out2(&mut self) -> OUT2_W {
        OUT2_W { w: self }
    }
    #[doc = "Bit 12 - Complement of Out1 modem status output"]
    #[inline(always)]
    pub fn out1(&mut self) -> OUT1_W {
        OUT1_W { w: self }
    }
    #[doc = "Bit 11 - Request to send"]
    #[inline(always)]
    pub fn rts(&mut self) -> RTS_W {
        RTS_W { w: self }
    }
    #[doc = "Bit 10 - Data transmit ready"]
    #[inline(always)]
    pub fn dtr(&mut self) -> DTR_W {
        DTR_W { w: self }
    }
    #[doc = "Bit 9 - Receive enable"]
    #[inline(always)]
    pub fn rxe(&mut self) -> RXE_W {
        RXE_W { w: self }
    }
    #[doc = "Bit 8 - Transmit enable"]
    #[inline(always)]
    pub fn txe(&mut self) -> TXE_W {
        TXE_W { w: self }
    }
    #[doc = "Bit 2 - IrDA SIR low power mode"]
    #[inline(always)]
    pub fn sirlp(&mut self) -> SIRLP_W {
        SIRLP_W { w: self }
    }
    #[doc = "Bit 1 - SIR enable"]
    #[inline(always)]
    pub fn siren(&mut self) -> SIREN_W {
        SIREN_W { w: self }
    }
    #[doc = "Bit 0 - UART enable"]
    #[inline(always)]
    pub fn uarten(&mut self) -> UARTEN_W {
        UARTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0x0300"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0300
    }
}
