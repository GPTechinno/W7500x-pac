#[doc = "Register `SSR` reader"]
pub struct R(crate::R<SSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSR` writer"]
pub struct W(crate::W<SSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSR_SPEC>;
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
impl From<crate::W<SSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS7_A {
    #[doc = "0: `0`"]
    STOP = 0,
    #[doc = "1: `1`"]
    START = 1,
}
impl From<SS7_A> for bool {
    #[inline(always)]
    fn from(variant: SS7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SS7` reader - Channel 7"]
pub struct SS7_R(crate::FieldReader<bool>);
impl SS7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SS7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS7_A {
        match self.bits {
            false => SS7_A::STOP,
            true => SS7_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == SS7_A::STOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == SS7_A::START
    }
}
impl core::ops::Deref for SS7_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SS7` writer - Channel 7"]
pub struct SS7_W<'a> {
    w: &'a mut W,
}
impl<'a> SS7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SS7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(SS7_A::STOP)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SS7_A::START)
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS6_A {
    #[doc = "0: `0`"]
    STOP = 0,
    #[doc = "1: `1`"]
    START = 1,
}
impl From<SS6_A> for bool {
    #[inline(always)]
    fn from(variant: SS6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SS6` reader - Channel 6"]
pub struct SS6_R(crate::FieldReader<bool>);
impl SS6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SS6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS6_A {
        match self.bits {
            false => SS6_A::STOP,
            true => SS6_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == SS6_A::STOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == SS6_A::START
    }
}
impl core::ops::Deref for SS6_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SS6` writer - Channel 6"]
pub struct SS6_W<'a> {
    w: &'a mut W,
}
impl<'a> SS6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SS6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(SS6_A::STOP)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SS6_A::START)
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS5_A {
    #[doc = "0: `0`"]
    STOP = 0,
    #[doc = "1: `1`"]
    START = 1,
}
impl From<SS5_A> for bool {
    #[inline(always)]
    fn from(variant: SS5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SS5` reader - Channel 5"]
pub struct SS5_R(crate::FieldReader<bool>);
impl SS5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SS5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS5_A {
        match self.bits {
            false => SS5_A::STOP,
            true => SS5_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == SS5_A::STOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == SS5_A::START
    }
}
impl core::ops::Deref for SS5_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SS5` writer - Channel 5"]
pub struct SS5_W<'a> {
    w: &'a mut W,
}
impl<'a> SS5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SS5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(SS5_A::STOP)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SS5_A::START)
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS4_A {
    #[doc = "0: `0`"]
    STOP = 0,
    #[doc = "1: `1`"]
    START = 1,
}
impl From<SS4_A> for bool {
    #[inline(always)]
    fn from(variant: SS4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SS4` reader - Channel 4"]
pub struct SS4_R(crate::FieldReader<bool>);
impl SS4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SS4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS4_A {
        match self.bits {
            false => SS4_A::STOP,
            true => SS4_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == SS4_A::STOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == SS4_A::START
    }
}
impl core::ops::Deref for SS4_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SS4` writer - Channel 4"]
pub struct SS4_W<'a> {
    w: &'a mut W,
}
impl<'a> SS4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SS4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(SS4_A::STOP)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SS4_A::START)
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS3_A {
    #[doc = "0: `0`"]
    STOP = 0,
    #[doc = "1: `1`"]
    START = 1,
}
impl From<SS3_A> for bool {
    #[inline(always)]
    fn from(variant: SS3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SS3` reader - Channel 3"]
pub struct SS3_R(crate::FieldReader<bool>);
impl SS3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SS3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS3_A {
        match self.bits {
            false => SS3_A::STOP,
            true => SS3_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == SS3_A::STOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == SS3_A::START
    }
}
impl core::ops::Deref for SS3_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SS3` writer - Channel 3"]
pub struct SS3_W<'a> {
    w: &'a mut W,
}
impl<'a> SS3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SS3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(SS3_A::STOP)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SS3_A::START)
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS2_A {
    #[doc = "0: `0`"]
    STOP = 0,
    #[doc = "1: `1`"]
    START = 1,
}
impl From<SS2_A> for bool {
    #[inline(always)]
    fn from(variant: SS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SS2` reader - Channel 2"]
pub struct SS2_R(crate::FieldReader<bool>);
impl SS2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SS2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS2_A {
        match self.bits {
            false => SS2_A::STOP,
            true => SS2_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == SS2_A::STOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == SS2_A::START
    }
}
impl core::ops::Deref for SS2_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SS2` writer - Channel 2"]
pub struct SS2_W<'a> {
    w: &'a mut W,
}
impl<'a> SS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SS2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(SS2_A::STOP)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SS2_A::START)
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS1_A {
    #[doc = "0: `0`"]
    STOP = 0,
    #[doc = "1: `1`"]
    START = 1,
}
impl From<SS1_A> for bool {
    #[inline(always)]
    fn from(variant: SS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SS1` reader - Channel 1"]
pub struct SS1_R(crate::FieldReader<bool>);
impl SS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS1_A {
        match self.bits {
            false => SS1_A::STOP,
            true => SS1_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == SS1_A::STOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == SS1_A::START
    }
}
impl core::ops::Deref for SS1_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SS1` writer - Channel 1"]
pub struct SS1_W<'a> {
    w: &'a mut W,
}
impl<'a> SS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SS1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(SS1_A::STOP)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SS1_A::START)
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS0_A {
    #[doc = "0: `0`"]
    STOP = 0,
    #[doc = "1: `1`"]
    START = 1,
}
impl From<SS0_A> for bool {
    #[inline(always)]
    fn from(variant: SS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SS0` reader - Channel 0"]
pub struct SS0_R(crate::FieldReader<bool>);
impl SS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SS0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS0_A {
        match self.bits {
            false => SS0_A::STOP,
            true => SS0_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == SS0_A::STOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == SS0_A::START
    }
}
impl core::ops::Deref for SS0_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SS0` writer - Channel 0"]
pub struct SS0_W<'a> {
    w: &'a mut W,
}
impl<'a> SS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SS0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(SS0_A::STOP)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SS0_A::START)
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - Channel 7"]
    #[inline(always)]
    pub fn ss7(&self) -> SS7_R {
        SS7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6"]
    #[inline(always)]
    pub fn ss6(&self) -> SS6_R {
        SS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5"]
    #[inline(always)]
    pub fn ss5(&self) -> SS5_R {
        SS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4"]
    #[inline(always)]
    pub fn ss4(&self) -> SS4_R {
        SS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3"]
    #[inline(always)]
    pub fn ss3(&self) -> SS3_R {
        SS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2"]
    #[inline(always)]
    pub fn ss2(&self) -> SS2_R {
        SS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1"]
    #[inline(always)]
    pub fn ss1(&self) -> SS1_R {
        SS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Channel 0"]
    #[inline(always)]
    pub fn ss0(&self) -> SS0_R {
        SS0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Channel 7"]
    #[inline(always)]
    pub fn ss7(&mut self) -> SS7_W {
        SS7_W { w: self }
    }
    #[doc = "Bit 6 - Channel 6"]
    #[inline(always)]
    pub fn ss6(&mut self) -> SS6_W {
        SS6_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5"]
    #[inline(always)]
    pub fn ss5(&mut self) -> SS5_W {
        SS5_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4"]
    #[inline(always)]
    pub fn ss4(&mut self) -> SS4_W {
        SS4_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3"]
    #[inline(always)]
    pub fn ss3(&mut self) -> SS3_W {
        SS3_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2"]
    #[inline(always)]
    pub fn ss2(&mut self) -> SS2_W {
        SS2_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1"]
    #[inline(always)]
    pub fn ss1(&mut self) -> SS1_W {
        SS1_W { w: self }
    }
    #[doc = "Bit 0 - Channel 0"]
    #[inline(always)]
    pub fn ss0(&mut self) -> SS0_W {
        SS0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start Stop register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssr](index.html) module"]
pub struct SSR_SPEC;
impl crate::RegisterSpec for SSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssr::R](R) reader structure"]
impl crate::Readable for SSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssr::W](W) writer structure"]
impl crate::Writable for SSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSR to value 0"]
impl crate::Resettable for SSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
