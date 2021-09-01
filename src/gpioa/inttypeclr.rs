#[doc = "Register `INTTYPECLR` reader"]
pub struct R(crate::R<INTTYPECLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTTYPECLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTTYPECLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTTYPECLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTTYPECLR` writer"]
pub struct W(crate::W<INTTYPECLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTTYPECLR_SPEC>;
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
impl From<crate::W<INTTYPECLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTTYPECLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITC15` reader - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC15_R(crate::FieldReader<bool, bool>);
impl ITC15_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITC15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITC15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITC15` writer - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC15_W<'a> {
    w: &'a mut W,
}
impl<'a> ITC15_W<'a> {
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
#[doc = "Field `ITC14` reader - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC14_R(crate::FieldReader<bool, bool>);
impl ITC14_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITC14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITC14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITC14` writer - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC14_W<'a> {
    w: &'a mut W,
}
impl<'a> ITC14_W<'a> {
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
#[doc = "Field `ITC13` reader - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC13_R(crate::FieldReader<bool, bool>);
impl ITC13_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITC13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITC13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITC13` writer - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC13_W<'a> {
    w: &'a mut W,
}
impl<'a> ITC13_W<'a> {
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
#[doc = "Field `ITC12` reader - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC12_R(crate::FieldReader<bool, bool>);
impl ITC12_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITC12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITC12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITC12` writer - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC12_W<'a> {
    w: &'a mut W,
}
impl<'a> ITC12_W<'a> {
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
#[doc = "Field `ITC11` reader - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC11_R(crate::FieldReader<bool, bool>);
impl ITC11_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITC11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITC11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITC11` writer - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC11_W<'a> {
    w: &'a mut W,
}
impl<'a> ITC11_W<'a> {
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
#[doc = "Field `ITC10` reader - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC10_R(crate::FieldReader<bool, bool>);
impl ITC10_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITC10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITC10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITC10` writer - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC10_W<'a> {
    w: &'a mut W,
}
impl<'a> ITC10_W<'a> {
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
#[doc = "Field `ITC9` reader - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC9_R(crate::FieldReader<bool, bool>);
impl ITC9_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITC9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITC9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITC9` writer - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC9_W<'a> {
    w: &'a mut W,
}
impl<'a> ITC9_W<'a> {
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
#[doc = "Field `ITC8` reader - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC8_R(crate::FieldReader<bool, bool>);
impl ITC8_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITC8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITC8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITC8` writer - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC8_W<'a> {
    w: &'a mut W,
}
impl<'a> ITC8_W<'a> {
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
#[doc = "Field `ITC7` reader - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC7_R(crate::FieldReader<bool, bool>);
impl ITC7_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITC7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITC7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITC7` writer - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC7_W<'a> {
    w: &'a mut W,
}
impl<'a> ITC7_W<'a> {
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
#[doc = "Field `ITC6` reader - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC6_R(crate::FieldReader<bool, bool>);
impl ITC6_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITC6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITC6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITC6` writer - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC6_W<'a> {
    w: &'a mut W,
}
impl<'a> ITC6_W<'a> {
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
#[doc = "Field `ITC5` reader - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC5_R(crate::FieldReader<bool, bool>);
impl ITC5_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITC5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITC5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITC5` writer - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC5_W<'a> {
    w: &'a mut W,
}
impl<'a> ITC5_W<'a> {
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
#[doc = "Field `ITC4` reader - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC4_R(crate::FieldReader<bool, bool>);
impl ITC4_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITC4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITC4` writer - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC4_W<'a> {
    w: &'a mut W,
}
impl<'a> ITC4_W<'a> {
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
#[doc = "Field `ITC3` reader - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC3_R(crate::FieldReader<bool, bool>);
impl ITC3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITC3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITC3` writer - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC3_W<'a> {
    w: &'a mut W,
}
impl<'a> ITC3_W<'a> {
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
#[doc = "Field `ITC2` reader - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC2_R(crate::FieldReader<bool, bool>);
impl ITC2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITC2` writer - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC2_W<'a> {
    w: &'a mut W,
}
impl<'a> ITC2_W<'a> {
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
#[doc = "Field `ITC1` reader - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC1_R(crate::FieldReader<bool, bool>);
impl ITC1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITC1` writer - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC1_W<'a> {
    w: &'a mut W,
}
impl<'a> ITC1_W<'a> {
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
#[doc = "Field `ITC0` reader - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC0_R(crate::FieldReader<bool, bool>);
impl ITC0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITC0` writer - clears the interrupt type bit, indicates for edge or level"]
pub struct ITC0_W<'a> {
    w: &'a mut W,
}
impl<'a> ITC0_W<'a> {
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
    #[doc = "Bit 15 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc15(&self) -> ITC15_R {
        ITC15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc14(&self) -> ITC14_R {
        ITC14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc13(&self) -> ITC13_R {
        ITC13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc12(&self) -> ITC12_R {
        ITC12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc11(&self) -> ITC11_R {
        ITC11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc10(&self) -> ITC10_R {
        ITC10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc9(&self) -> ITC9_R {
        ITC9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc8(&self) -> ITC8_R {
        ITC8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc7(&self) -> ITC7_R {
        ITC7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc6(&self) -> ITC6_R {
        ITC6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc5(&self) -> ITC5_R {
        ITC5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc4(&self) -> ITC4_R {
        ITC4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc3(&self) -> ITC3_R {
        ITC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc2(&self) -> ITC2_R {
        ITC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc1(&self) -> ITC1_R {
        ITC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc0(&self) -> ITC0_R {
        ITC0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc15(&mut self) -> ITC15_W {
        ITC15_W { w: self }
    }
    #[doc = "Bit 14 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc14(&mut self) -> ITC14_W {
        ITC14_W { w: self }
    }
    #[doc = "Bit 13 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc13(&mut self) -> ITC13_W {
        ITC13_W { w: self }
    }
    #[doc = "Bit 12 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc12(&mut self) -> ITC12_W {
        ITC12_W { w: self }
    }
    #[doc = "Bit 11 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc11(&mut self) -> ITC11_W {
        ITC11_W { w: self }
    }
    #[doc = "Bit 10 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc10(&mut self) -> ITC10_W {
        ITC10_W { w: self }
    }
    #[doc = "Bit 9 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc9(&mut self) -> ITC9_W {
        ITC9_W { w: self }
    }
    #[doc = "Bit 8 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc8(&mut self) -> ITC8_W {
        ITC8_W { w: self }
    }
    #[doc = "Bit 7 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc7(&mut self) -> ITC7_W {
        ITC7_W { w: self }
    }
    #[doc = "Bit 6 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc6(&mut self) -> ITC6_W {
        ITC6_W { w: self }
    }
    #[doc = "Bit 5 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc5(&mut self) -> ITC5_W {
        ITC5_W { w: self }
    }
    #[doc = "Bit 4 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc4(&mut self) -> ITC4_W {
        ITC4_W { w: self }
    }
    #[doc = "Bit 3 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc3(&mut self) -> ITC3_W {
        ITC3_W { w: self }
    }
    #[doc = "Bit 2 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc2(&mut self) -> ITC2_W {
        ITC2_W { w: self }
    }
    #[doc = "Bit 1 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc1(&mut self) -> ITC1_W {
        ITC1_W { w: self }
    }
    #[doc = "Bit 0 - clears the interrupt type bit, indicates for edge or level"]
    #[inline(always)]
    pub fn itc0(&mut self) -> ITC0_W {
        ITC0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Type Clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inttypeclr](index.html) module"]
pub struct INTTYPECLR_SPEC;
impl crate::RegisterSpec for INTTYPECLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inttypeclr::R](R) reader structure"]
impl crate::Readable for INTTYPECLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inttypeclr::W](W) writer structure"]
impl crate::Writable for INTTYPECLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTTYPECLR to value 0"]
impl crate::Resettable for INTTYPECLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
