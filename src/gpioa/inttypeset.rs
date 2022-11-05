#[doc = "Register `INTTYPESET` reader"]
pub struct R(crate::R<INTTYPESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTTYPESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTTYPESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTTYPESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTTYPESET` writer"]
pub struct W(crate::W<INTTYPESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTTYPESET_SPEC>;
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
impl From<crate::W<INTTYPESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTTYPESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITS15` reader - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS15_R(crate::FieldReader<bool>);
impl ITS15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ITS15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITS15_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITS15` writer - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS15_W<'a> {
    w: &'a mut W,
}
impl<'a> ITS15_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "Field `ITS14` reader - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS14_R(crate::FieldReader<bool>);
impl ITS14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ITS14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITS14_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITS14` writer - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS14_W<'a> {
    w: &'a mut W,
}
impl<'a> ITS14_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u32 & 1) << 14);
        self.w
    }
}
#[doc = "Field `ITS13` reader - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS13_R(crate::FieldReader<bool>);
impl ITS13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ITS13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITS13_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITS13` writer - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS13_W<'a> {
    w: &'a mut W,
}
impl<'a> ITS13_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `ITS12` reader - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS12_R(crate::FieldReader<bool>);
impl ITS12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ITS12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITS12_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITS12` writer - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS12_W<'a> {
    w: &'a mut W,
}
impl<'a> ITS12_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
#[doc = "Field `ITS11` reader - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS11_R(crate::FieldReader<bool>);
impl ITS11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ITS11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITS11_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITS11` writer - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS11_W<'a> {
    w: &'a mut W,
}
impl<'a> ITS11_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Field `ITS10` reader - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS10_R(crate::FieldReader<bool>);
impl ITS10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ITS10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITS10_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITS10` writer - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS10_W<'a> {
    w: &'a mut W,
}
impl<'a> ITS10_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "Field `ITS9` reader - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS9_R(crate::FieldReader<bool>);
impl ITS9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ITS9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITS9_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITS9` writer - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS9_W<'a> {
    w: &'a mut W,
}
impl<'a> ITS9_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "Field `ITS8` reader - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS8_R(crate::FieldReader<bool>);
impl ITS8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ITS8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITS8_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITS8` writer - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS8_W<'a> {
    w: &'a mut W,
}
impl<'a> ITS8_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `ITS7` reader - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS7_R(crate::FieldReader<bool>);
impl ITS7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ITS7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITS7_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITS7` writer - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS7_W<'a> {
    w: &'a mut W,
}
impl<'a> ITS7_W<'a> {
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
#[doc = "Field `ITS6` reader - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS6_R(crate::FieldReader<bool>);
impl ITS6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ITS6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITS6_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITS6` writer - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS6_W<'a> {
    w: &'a mut W,
}
impl<'a> ITS6_W<'a> {
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
#[doc = "Field `ITS5` reader - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS5_R(crate::FieldReader<bool>);
impl ITS5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ITS5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITS5_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITS5` writer - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS5_W<'a> {
    w: &'a mut W,
}
impl<'a> ITS5_W<'a> {
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
#[doc = "Field `ITS4` reader - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS4_R(crate::FieldReader<bool>);
impl ITS4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ITS4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITS4_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITS4` writer - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS4_W<'a> {
    w: &'a mut W,
}
impl<'a> ITS4_W<'a> {
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
#[doc = "Field `ITS3` reader - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS3_R(crate::FieldReader<bool>);
impl ITS3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ITS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITS3_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITS3` writer - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS3_W<'a> {
    w: &'a mut W,
}
impl<'a> ITS3_W<'a> {
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
#[doc = "Field `ITS2` reader - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS2_R(crate::FieldReader<bool>);
impl ITS2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ITS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITS2_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITS2` writer - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS2_W<'a> {
    w: &'a mut W,
}
impl<'a> ITS2_W<'a> {
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
#[doc = "Field `ITS1` reader - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS1_R(crate::FieldReader<bool>);
impl ITS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ITS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITS1_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITS1` writer - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS1_W<'a> {
    w: &'a mut W,
}
impl<'a> ITS1_W<'a> {
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
#[doc = "Field `ITS0` reader - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS0_R(crate::FieldReader<bool>);
impl ITS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ITS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITS0_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITS0` writer - sets the interrupt type bit, indicates for edge or level"]
pub struct ITS0_W<'a> {
    w: &'a mut W,
}
impl<'a> ITS0_W<'a> {
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
    #[doc = "Bit 15 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its15(&self) -> ITS15_R {
        ITS15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its14(&self) -> ITS14_R {
        ITS14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its13(&self) -> ITS13_R {
        ITS13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its12(&self) -> ITS12_R {
        ITS12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its11(&self) -> ITS11_R {
        ITS11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its10(&self) -> ITS10_R {
        ITS10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its9(&self) -> ITS9_R {
        ITS9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its8(&self) -> ITS8_R {
        ITS8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its7(&self) -> ITS7_R {
        ITS7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its6(&self) -> ITS6_R {
        ITS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its5(&self) -> ITS5_R {
        ITS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its4(&self) -> ITS4_R {
        ITS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its3(&self) -> ITS3_R {
        ITS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its2(&self) -> ITS2_R {
        ITS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its1(&self) -> ITS1_R {
        ITS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its0(&self) -> ITS0_R {
        ITS0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its15(&mut self) -> ITS15_W {
        ITS15_W { w: self }
    }
    #[doc = "Bit 14 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its14(&mut self) -> ITS14_W {
        ITS14_W { w: self }
    }
    #[doc = "Bit 13 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its13(&mut self) -> ITS13_W {
        ITS13_W { w: self }
    }
    #[doc = "Bit 12 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its12(&mut self) -> ITS12_W {
        ITS12_W { w: self }
    }
    #[doc = "Bit 11 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its11(&mut self) -> ITS11_W {
        ITS11_W { w: self }
    }
    #[doc = "Bit 10 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its10(&mut self) -> ITS10_W {
        ITS10_W { w: self }
    }
    #[doc = "Bit 9 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its9(&mut self) -> ITS9_W {
        ITS9_W { w: self }
    }
    #[doc = "Bit 8 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its8(&mut self) -> ITS8_W {
        ITS8_W { w: self }
    }
    #[doc = "Bit 7 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its7(&mut self) -> ITS7_W {
        ITS7_W { w: self }
    }
    #[doc = "Bit 6 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its6(&mut self) -> ITS6_W {
        ITS6_W { w: self }
    }
    #[doc = "Bit 5 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its5(&mut self) -> ITS5_W {
        ITS5_W { w: self }
    }
    #[doc = "Bit 4 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its4(&mut self) -> ITS4_W {
        ITS4_W { w: self }
    }
    #[doc = "Bit 3 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its3(&mut self) -> ITS3_W {
        ITS3_W { w: self }
    }
    #[doc = "Bit 2 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its2(&mut self) -> ITS2_W {
        ITS2_W { w: self }
    }
    #[doc = "Bit 1 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its1(&mut self) -> ITS1_W {
        ITS1_W { w: self }
    }
    #[doc = "Bit 0 - sets the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn its0(&mut self) -> ITS0_W {
        ITS0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Type Set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inttypeset](index.html) module"]
pub struct INTTYPESET_SPEC;
impl crate::RegisterSpec for INTTYPESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inttypeset::R](R) reader structure"]
impl crate::Readable for INTTYPESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inttypeset::W](W) writer structure"]
impl crate::Writable for INTTYPESET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTTYPESET to value 0"]
impl crate::Resettable for INTTYPESET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
