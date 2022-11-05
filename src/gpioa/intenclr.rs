#[doc = "Register `INTENCLR` reader"]
pub struct R(crate::R<INTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IEC15` reader - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC15_R(crate::FieldReader<bool>);
impl IEC15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IEC15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEC15_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IEC15` writer - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC15_W<'a> {
    w: &'a mut W,
}
impl<'a> IEC15_W<'a> {
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
#[doc = "Field `IEC14` reader - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC14_R(crate::FieldReader<bool>);
impl IEC14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IEC14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEC14_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IEC14` writer - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC14_W<'a> {
    w: &'a mut W,
}
impl<'a> IEC14_W<'a> {
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
#[doc = "Field `IEC13` reader - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC13_R(crate::FieldReader<bool>);
impl IEC13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IEC13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEC13_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IEC13` writer - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC13_W<'a> {
    w: &'a mut W,
}
impl<'a> IEC13_W<'a> {
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
#[doc = "Field `IEC12` reader - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC12_R(crate::FieldReader<bool>);
impl IEC12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IEC12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEC12_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IEC12` writer - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC12_W<'a> {
    w: &'a mut W,
}
impl<'a> IEC12_W<'a> {
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
#[doc = "Field `IEC11` reader - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC11_R(crate::FieldReader<bool>);
impl IEC11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IEC11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEC11_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IEC11` writer - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC11_W<'a> {
    w: &'a mut W,
}
impl<'a> IEC11_W<'a> {
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
#[doc = "Field `IEC10` reader - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC10_R(crate::FieldReader<bool>);
impl IEC10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IEC10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEC10_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IEC10` writer - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC10_W<'a> {
    w: &'a mut W,
}
impl<'a> IEC10_W<'a> {
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
#[doc = "Field `IEC9` reader - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC9_R(crate::FieldReader<bool>);
impl IEC9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IEC9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEC9_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IEC9` writer - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC9_W<'a> {
    w: &'a mut W,
}
impl<'a> IEC9_W<'a> {
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
#[doc = "Field `IEC8` reader - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC8_R(crate::FieldReader<bool>);
impl IEC8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IEC8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEC8_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IEC8` writer - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC8_W<'a> {
    w: &'a mut W,
}
impl<'a> IEC8_W<'a> {
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
#[doc = "Field `IEC7` reader - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC7_R(crate::FieldReader<bool>);
impl IEC7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IEC7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEC7_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IEC7` writer - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC7_W<'a> {
    w: &'a mut W,
}
impl<'a> IEC7_W<'a> {
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
#[doc = "Field `IEC6` reader - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC6_R(crate::FieldReader<bool>);
impl IEC6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IEC6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEC6_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IEC6` writer - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC6_W<'a> {
    w: &'a mut W,
}
impl<'a> IEC6_W<'a> {
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
#[doc = "Field `IEC5` reader - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC5_R(crate::FieldReader<bool>);
impl IEC5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IEC5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEC5_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IEC5` writer - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC5_W<'a> {
    w: &'a mut W,
}
impl<'a> IEC5_W<'a> {
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
#[doc = "Field `IEC4` reader - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC4_R(crate::FieldReader<bool>);
impl IEC4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IEC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEC4_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IEC4` writer - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC4_W<'a> {
    w: &'a mut W,
}
impl<'a> IEC4_W<'a> {
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
#[doc = "Field `IEC3` reader - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC3_R(crate::FieldReader<bool>);
impl IEC3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IEC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEC3_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IEC3` writer - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC3_W<'a> {
    w: &'a mut W,
}
impl<'a> IEC3_W<'a> {
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
#[doc = "Field `IEC2` reader - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC2_R(crate::FieldReader<bool>);
impl IEC2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IEC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEC2_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IEC2` writer - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC2_W<'a> {
    w: &'a mut W,
}
impl<'a> IEC2_W<'a> {
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
#[doc = "Field `IEC1` reader - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC1_R(crate::FieldReader<bool>);
impl IEC1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IEC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEC1_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IEC1` writer - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC1_W<'a> {
    w: &'a mut W,
}
impl<'a> IEC1_W<'a> {
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
#[doc = "Field `IEC0` reader - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC0_R(crate::FieldReader<bool>);
impl IEC0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IEC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEC0_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IEC0` writer - clears the interrupt enable bit, indicates the interrupt"]
pub struct IEC0_W<'a> {
    w: &'a mut W,
}
impl<'a> IEC0_W<'a> {
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
    #[doc = "Bit 15 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec15(&self) -> IEC15_R {
        IEC15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec14(&self) -> IEC14_R {
        IEC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec13(&self) -> IEC13_R {
        IEC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec12(&self) -> IEC12_R {
        IEC12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec11(&self) -> IEC11_R {
        IEC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec10(&self) -> IEC10_R {
        IEC10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec9(&self) -> IEC9_R {
        IEC9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec8(&self) -> IEC8_R {
        IEC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec7(&self) -> IEC7_R {
        IEC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec6(&self) -> IEC6_R {
        IEC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec5(&self) -> IEC5_R {
        IEC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec4(&self) -> IEC4_R {
        IEC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec3(&self) -> IEC3_R {
        IEC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec2(&self) -> IEC2_R {
        IEC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec1(&self) -> IEC1_R {
        IEC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec0(&self) -> IEC0_R {
        IEC0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec15(&mut self) -> IEC15_W {
        IEC15_W { w: self }
    }
    #[doc = "Bit 14 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec14(&mut self) -> IEC14_W {
        IEC14_W { w: self }
    }
    #[doc = "Bit 13 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec13(&mut self) -> IEC13_W {
        IEC13_W { w: self }
    }
    #[doc = "Bit 12 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec12(&mut self) -> IEC12_W {
        IEC12_W { w: self }
    }
    #[doc = "Bit 11 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec11(&mut self) -> IEC11_W {
        IEC11_W { w: self }
    }
    #[doc = "Bit 10 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec10(&mut self) -> IEC10_W {
        IEC10_W { w: self }
    }
    #[doc = "Bit 9 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec9(&mut self) -> IEC9_W {
        IEC9_W { w: self }
    }
    #[doc = "Bit 8 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec8(&mut self) -> IEC8_W {
        IEC8_W { w: self }
    }
    #[doc = "Bit 7 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec7(&mut self) -> IEC7_W {
        IEC7_W { w: self }
    }
    #[doc = "Bit 6 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec6(&mut self) -> IEC6_W {
        IEC6_W { w: self }
    }
    #[doc = "Bit 5 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec5(&mut self) -> IEC5_W {
        IEC5_W { w: self }
    }
    #[doc = "Bit 4 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec4(&mut self) -> IEC4_W {
        IEC4_W { w: self }
    }
    #[doc = "Bit 3 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec3(&mut self) -> IEC3_W {
        IEC3_W { w: self }
    }
    #[doc = "Bit 2 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec2(&mut self) -> IEC2_W {
        IEC2_W { w: self }
    }
    #[doc = "Bit 1 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec1(&mut self) -> IEC1_W {
        IEC1_W { w: self }
    }
    #[doc = "Bit 0 - clears the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn iec0(&mut self) -> IEC0_W {
        IEC0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenclr::R](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
