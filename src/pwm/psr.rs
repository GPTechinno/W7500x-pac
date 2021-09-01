#[doc = "Register `PSR` reader"]
pub struct R(crate::R<PSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSR` writer"]
pub struct W(crate::W<PSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSR_SPEC>;
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
impl From<crate::W<PSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS7_A {
    #[doc = "0: `0`"]
    NOTPAUSED = 0,
    #[doc = "1: `1`"]
    PAUSED = 1,
}
impl From<PS7_A> for bool {
    #[inline(always)]
    fn from(variant: PS7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PS7` reader - Channel 7"]
pub struct PS7_R(crate::FieldReader<bool, PS7_A>);
impl PS7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PS7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS7_A {
        match self.bits {
            false => PS7_A::NOTPAUSED,
            true => PS7_A::PAUSED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPAUSED`"]
    #[inline(always)]
    pub fn is_not_paused(&self) -> bool {
        **self == PS7_A::NOTPAUSED
    }
    #[doc = "Checks if the value of the field is `PAUSED`"]
    #[inline(always)]
    pub fn is_paused(&self) -> bool {
        **self == PS7_A::PAUSED
    }
}
impl core::ops::Deref for PS7_R {
    type Target = crate::FieldReader<bool, PS7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PS7` writer - Channel 7"]
pub struct PS7_W<'a> {
    w: &'a mut W,
}
impl<'a> PS7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_paused(self) -> &'a mut W {
        self.variant(PS7_A::NOTPAUSED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn paused(self) -> &'a mut W {
        self.variant(PS7_A::PAUSED)
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
#[doc = "Channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS6_A {
    #[doc = "0: `0`"]
    NOTPAUSED = 0,
    #[doc = "1: `1`"]
    PAUSED = 1,
}
impl From<PS6_A> for bool {
    #[inline(always)]
    fn from(variant: PS6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PS6` reader - Channel 6"]
pub struct PS6_R(crate::FieldReader<bool, PS6_A>);
impl PS6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PS6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS6_A {
        match self.bits {
            false => PS6_A::NOTPAUSED,
            true => PS6_A::PAUSED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPAUSED`"]
    #[inline(always)]
    pub fn is_not_paused(&self) -> bool {
        **self == PS6_A::NOTPAUSED
    }
    #[doc = "Checks if the value of the field is `PAUSED`"]
    #[inline(always)]
    pub fn is_paused(&self) -> bool {
        **self == PS6_A::PAUSED
    }
}
impl core::ops::Deref for PS6_R {
    type Target = crate::FieldReader<bool, PS6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PS6` writer - Channel 6"]
pub struct PS6_W<'a> {
    w: &'a mut W,
}
impl<'a> PS6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_paused(self) -> &'a mut W {
        self.variant(PS6_A::NOTPAUSED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn paused(self) -> &'a mut W {
        self.variant(PS6_A::PAUSED)
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
#[doc = "Channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS5_A {
    #[doc = "0: `0`"]
    NOTPAUSED = 0,
    #[doc = "1: `1`"]
    PAUSED = 1,
}
impl From<PS5_A> for bool {
    #[inline(always)]
    fn from(variant: PS5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PS5` reader - Channel 5"]
pub struct PS5_R(crate::FieldReader<bool, PS5_A>);
impl PS5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PS5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS5_A {
        match self.bits {
            false => PS5_A::NOTPAUSED,
            true => PS5_A::PAUSED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPAUSED`"]
    #[inline(always)]
    pub fn is_not_paused(&self) -> bool {
        **self == PS5_A::NOTPAUSED
    }
    #[doc = "Checks if the value of the field is `PAUSED`"]
    #[inline(always)]
    pub fn is_paused(&self) -> bool {
        **self == PS5_A::PAUSED
    }
}
impl core::ops::Deref for PS5_R {
    type Target = crate::FieldReader<bool, PS5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PS5` writer - Channel 5"]
pub struct PS5_W<'a> {
    w: &'a mut W,
}
impl<'a> PS5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_paused(self) -> &'a mut W {
        self.variant(PS5_A::NOTPAUSED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn paused(self) -> &'a mut W {
        self.variant(PS5_A::PAUSED)
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
#[doc = "Channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS4_A {
    #[doc = "0: `0`"]
    NOTPAUSED = 0,
    #[doc = "1: `1`"]
    PAUSED = 1,
}
impl From<PS4_A> for bool {
    #[inline(always)]
    fn from(variant: PS4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PS4` reader - Channel 4"]
pub struct PS4_R(crate::FieldReader<bool, PS4_A>);
impl PS4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PS4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS4_A {
        match self.bits {
            false => PS4_A::NOTPAUSED,
            true => PS4_A::PAUSED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPAUSED`"]
    #[inline(always)]
    pub fn is_not_paused(&self) -> bool {
        **self == PS4_A::NOTPAUSED
    }
    #[doc = "Checks if the value of the field is `PAUSED`"]
    #[inline(always)]
    pub fn is_paused(&self) -> bool {
        **self == PS4_A::PAUSED
    }
}
impl core::ops::Deref for PS4_R {
    type Target = crate::FieldReader<bool, PS4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PS4` writer - Channel 4"]
pub struct PS4_W<'a> {
    w: &'a mut W,
}
impl<'a> PS4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_paused(self) -> &'a mut W {
        self.variant(PS4_A::NOTPAUSED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn paused(self) -> &'a mut W {
        self.variant(PS4_A::PAUSED)
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
#[doc = "Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS3_A {
    #[doc = "0: `0`"]
    NOTPAUSED = 0,
    #[doc = "1: `1`"]
    PAUSED = 1,
}
impl From<PS3_A> for bool {
    #[inline(always)]
    fn from(variant: PS3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PS3` reader - Channel 3"]
pub struct PS3_R(crate::FieldReader<bool, PS3_A>);
impl PS3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PS3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS3_A {
        match self.bits {
            false => PS3_A::NOTPAUSED,
            true => PS3_A::PAUSED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPAUSED`"]
    #[inline(always)]
    pub fn is_not_paused(&self) -> bool {
        **self == PS3_A::NOTPAUSED
    }
    #[doc = "Checks if the value of the field is `PAUSED`"]
    #[inline(always)]
    pub fn is_paused(&self) -> bool {
        **self == PS3_A::PAUSED
    }
}
impl core::ops::Deref for PS3_R {
    type Target = crate::FieldReader<bool, PS3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PS3` writer - Channel 3"]
pub struct PS3_W<'a> {
    w: &'a mut W,
}
impl<'a> PS3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_paused(self) -> &'a mut W {
        self.variant(PS3_A::NOTPAUSED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn paused(self) -> &'a mut W {
        self.variant(PS3_A::PAUSED)
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
#[doc = "Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS2_A {
    #[doc = "0: `0`"]
    NOTPAUSED = 0,
    #[doc = "1: `1`"]
    PAUSED = 1,
}
impl From<PS2_A> for bool {
    #[inline(always)]
    fn from(variant: PS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PS2` reader - Channel 2"]
pub struct PS2_R(crate::FieldReader<bool, PS2_A>);
impl PS2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PS2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS2_A {
        match self.bits {
            false => PS2_A::NOTPAUSED,
            true => PS2_A::PAUSED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPAUSED`"]
    #[inline(always)]
    pub fn is_not_paused(&self) -> bool {
        **self == PS2_A::NOTPAUSED
    }
    #[doc = "Checks if the value of the field is `PAUSED`"]
    #[inline(always)]
    pub fn is_paused(&self) -> bool {
        **self == PS2_A::PAUSED
    }
}
impl core::ops::Deref for PS2_R {
    type Target = crate::FieldReader<bool, PS2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PS2` writer - Channel 2"]
pub struct PS2_W<'a> {
    w: &'a mut W,
}
impl<'a> PS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_paused(self) -> &'a mut W {
        self.variant(PS2_A::NOTPAUSED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn paused(self) -> &'a mut W {
        self.variant(PS2_A::PAUSED)
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
#[doc = "Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS1_A {
    #[doc = "0: `0`"]
    NOTPAUSED = 0,
    #[doc = "1: `1`"]
    PAUSED = 1,
}
impl From<PS1_A> for bool {
    #[inline(always)]
    fn from(variant: PS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PS1` reader - Channel 1"]
pub struct PS1_R(crate::FieldReader<bool, PS1_A>);
impl PS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS1_A {
        match self.bits {
            false => PS1_A::NOTPAUSED,
            true => PS1_A::PAUSED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPAUSED`"]
    #[inline(always)]
    pub fn is_not_paused(&self) -> bool {
        **self == PS1_A::NOTPAUSED
    }
    #[doc = "Checks if the value of the field is `PAUSED`"]
    #[inline(always)]
    pub fn is_paused(&self) -> bool {
        **self == PS1_A::PAUSED
    }
}
impl core::ops::Deref for PS1_R {
    type Target = crate::FieldReader<bool, PS1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PS1` writer - Channel 1"]
pub struct PS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_paused(self) -> &'a mut W {
        self.variant(PS1_A::NOTPAUSED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn paused(self) -> &'a mut W {
        self.variant(PS1_A::PAUSED)
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
#[doc = "Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS0_A {
    #[doc = "0: `0`"]
    NOTPAUSED = 0,
    #[doc = "1: `1`"]
    PAUSED = 1,
}
impl From<PS0_A> for bool {
    #[inline(always)]
    fn from(variant: PS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PS0` reader - Channel 0"]
pub struct PS0_R(crate::FieldReader<bool, PS0_A>);
impl PS0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PS0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS0_A {
        match self.bits {
            false => PS0_A::NOTPAUSED,
            true => PS0_A::PAUSED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPAUSED`"]
    #[inline(always)]
    pub fn is_not_paused(&self) -> bool {
        **self == PS0_A::NOTPAUSED
    }
    #[doc = "Checks if the value of the field is `PAUSED`"]
    #[inline(always)]
    pub fn is_paused(&self) -> bool {
        **self == PS0_A::PAUSED
    }
}
impl core::ops::Deref for PS0_R {
    type Target = crate::FieldReader<bool, PS0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PS0` writer - Channel 0"]
pub struct PS0_W<'a> {
    w: &'a mut W,
}
impl<'a> PS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_paused(self) -> &'a mut W {
        self.variant(PS0_A::NOTPAUSED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn paused(self) -> &'a mut W {
        self.variant(PS0_A::PAUSED)
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
    #[doc = "Bit 7 - Channel 7"]
    #[inline(always)]
    pub fn ps7(&self) -> PS7_R {
        PS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 6"]
    #[inline(always)]
    pub fn ps6(&self) -> PS6_R {
        PS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5"]
    #[inline(always)]
    pub fn ps5(&self) -> PS5_R {
        PS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4"]
    #[inline(always)]
    pub fn ps4(&self) -> PS4_R {
        PS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3"]
    #[inline(always)]
    pub fn ps3(&self) -> PS3_R {
        PS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2"]
    #[inline(always)]
    pub fn ps2(&self) -> PS2_R {
        PS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1"]
    #[inline(always)]
    pub fn ps1(&self) -> PS1_R {
        PS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Channel 0"]
    #[inline(always)]
    pub fn ps0(&self) -> PS0_R {
        PS0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Channel 7"]
    #[inline(always)]
    pub fn ps7(&mut self) -> PS7_W {
        PS7_W { w: self }
    }
    #[doc = "Bit 6 - Channel 6"]
    #[inline(always)]
    pub fn ps6(&mut self) -> PS6_W {
        PS6_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5"]
    #[inline(always)]
    pub fn ps5(&mut self) -> PS5_W {
        PS5_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4"]
    #[inline(always)]
    pub fn ps4(&mut self) -> PS4_W {
        PS4_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3"]
    #[inline(always)]
    pub fn ps3(&mut self) -> PS3_W {
        PS3_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2"]
    #[inline(always)]
    pub fn ps2(&mut self) -> PS2_W {
        PS2_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1"]
    #[inline(always)]
    pub fn ps1(&mut self) -> PS1_W {
        PS1_W { w: self }
    }
    #[doc = "Bit 0 - Channel 0"]
    #[inline(always)]
    pub fn ps0(&mut self) -> PS0_W {
        PS0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pause register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psr](index.html) module"]
pub struct PSR_SPEC;
impl crate::RegisterSpec for PSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psr::R](R) reader structure"]
impl crate::Readable for PSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psr::W](W) writer structure"]
impl crate::Writable for PSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSR to value 0"]
impl crate::Resettable for PSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
