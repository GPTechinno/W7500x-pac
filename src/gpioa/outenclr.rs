#[doc = "Register `OUTENCLR` reader"]
pub struct R(crate::R<OUTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTENCLR` writer"]
pub struct W(crate::W<OUTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTENCLR_SPEC>;
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
impl From<crate::W<OUTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EC15` reader - clears the output enable bit, indicates the signal direction"]
pub struct EC15_R(crate::FieldReader<bool, bool>);
impl EC15_R {
    pub(crate) fn new(bits: bool) -> Self {
        EC15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EC15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EC15` writer - clears the output enable bit, indicates the signal direction"]
pub struct EC15_W<'a> {
    w: &'a mut W,
}
impl<'a> EC15_W<'a> {
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
#[doc = "Field `EC14` reader - clears the output enable bit, indicates the signal direction"]
pub struct EC14_R(crate::FieldReader<bool, bool>);
impl EC14_R {
    pub(crate) fn new(bits: bool) -> Self {
        EC14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EC14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EC14` writer - clears the output enable bit, indicates the signal direction"]
pub struct EC14_W<'a> {
    w: &'a mut W,
}
impl<'a> EC14_W<'a> {
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
#[doc = "Field `EC13` reader - clears the output enable bit, indicates the signal direction"]
pub struct EC13_R(crate::FieldReader<bool, bool>);
impl EC13_R {
    pub(crate) fn new(bits: bool) -> Self {
        EC13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EC13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EC13` writer - clears the output enable bit, indicates the signal direction"]
pub struct EC13_W<'a> {
    w: &'a mut W,
}
impl<'a> EC13_W<'a> {
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
#[doc = "Field `EC12` reader - clears the output enable bit, indicates the signal direction"]
pub struct EC12_R(crate::FieldReader<bool, bool>);
impl EC12_R {
    pub(crate) fn new(bits: bool) -> Self {
        EC12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EC12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EC12` writer - clears the output enable bit, indicates the signal direction"]
pub struct EC12_W<'a> {
    w: &'a mut W,
}
impl<'a> EC12_W<'a> {
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
#[doc = "Field `EC11` reader - clears the output enable bit, indicates the signal direction"]
pub struct EC11_R(crate::FieldReader<bool, bool>);
impl EC11_R {
    pub(crate) fn new(bits: bool) -> Self {
        EC11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EC11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EC11` writer - clears the output enable bit, indicates the signal direction"]
pub struct EC11_W<'a> {
    w: &'a mut W,
}
impl<'a> EC11_W<'a> {
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
#[doc = "Field `EC10` reader - clears the output enable bit, indicates the signal direction"]
pub struct EC10_R(crate::FieldReader<bool, bool>);
impl EC10_R {
    pub(crate) fn new(bits: bool) -> Self {
        EC10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EC10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EC10` writer - clears the output enable bit, indicates the signal direction"]
pub struct EC10_W<'a> {
    w: &'a mut W,
}
impl<'a> EC10_W<'a> {
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
#[doc = "Field `EC9` reader - clears the output enable bit, indicates the signal direction"]
pub struct EC9_R(crate::FieldReader<bool, bool>);
impl EC9_R {
    pub(crate) fn new(bits: bool) -> Self {
        EC9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EC9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EC9` writer - clears the output enable bit, indicates the signal direction"]
pub struct EC9_W<'a> {
    w: &'a mut W,
}
impl<'a> EC9_W<'a> {
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
#[doc = "Field `EC8` reader - clears the output enable bit, indicates the signal direction"]
pub struct EC8_R(crate::FieldReader<bool, bool>);
impl EC8_R {
    pub(crate) fn new(bits: bool) -> Self {
        EC8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EC8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EC8` writer - clears the output enable bit, indicates the signal direction"]
pub struct EC8_W<'a> {
    w: &'a mut W,
}
impl<'a> EC8_W<'a> {
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
#[doc = "Field `EC7` reader - clears the output enable bit, indicates the signal direction"]
pub struct EC7_R(crate::FieldReader<bool, bool>);
impl EC7_R {
    pub(crate) fn new(bits: bool) -> Self {
        EC7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EC7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EC7` writer - clears the output enable bit, indicates the signal direction"]
pub struct EC7_W<'a> {
    w: &'a mut W,
}
impl<'a> EC7_W<'a> {
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
#[doc = "Field `EC6` reader - clears the output enable bit, indicates the signal direction"]
pub struct EC6_R(crate::FieldReader<bool, bool>);
impl EC6_R {
    pub(crate) fn new(bits: bool) -> Self {
        EC6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EC6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EC6` writer - clears the output enable bit, indicates the signal direction"]
pub struct EC6_W<'a> {
    w: &'a mut W,
}
impl<'a> EC6_W<'a> {
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
#[doc = "Field `EC5` reader - clears the output enable bit, indicates the signal direction"]
pub struct EC5_R(crate::FieldReader<bool, bool>);
impl EC5_R {
    pub(crate) fn new(bits: bool) -> Self {
        EC5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EC5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EC5` writer - clears the output enable bit, indicates the signal direction"]
pub struct EC5_W<'a> {
    w: &'a mut W,
}
impl<'a> EC5_W<'a> {
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
#[doc = "Field `EC4` reader - clears the output enable bit, indicates the signal direction"]
pub struct EC4_R(crate::FieldReader<bool, bool>);
impl EC4_R {
    pub(crate) fn new(bits: bool) -> Self {
        EC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EC4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EC4` writer - clears the output enable bit, indicates the signal direction"]
pub struct EC4_W<'a> {
    w: &'a mut W,
}
impl<'a> EC4_W<'a> {
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
#[doc = "Field `EC3` reader - clears the output enable bit, indicates the signal direction"]
pub struct EC3_R(crate::FieldReader<bool, bool>);
impl EC3_R {
    pub(crate) fn new(bits: bool) -> Self {
        EC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EC3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EC3` writer - clears the output enable bit, indicates the signal direction"]
pub struct EC3_W<'a> {
    w: &'a mut W,
}
impl<'a> EC3_W<'a> {
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
#[doc = "Field `EC2` reader - clears the output enable bit, indicates the signal direction"]
pub struct EC2_R(crate::FieldReader<bool, bool>);
impl EC2_R {
    pub(crate) fn new(bits: bool) -> Self {
        EC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EC2` writer - clears the output enable bit, indicates the signal direction"]
pub struct EC2_W<'a> {
    w: &'a mut W,
}
impl<'a> EC2_W<'a> {
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
#[doc = "Field `EC1` reader - clears the output enable bit, indicates the signal direction"]
pub struct EC1_R(crate::FieldReader<bool, bool>);
impl EC1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EC1` writer - clears the output enable bit, indicates the signal direction"]
pub struct EC1_W<'a> {
    w: &'a mut W,
}
impl<'a> EC1_W<'a> {
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
#[doc = "Field `EC0` reader - clears the output enable bit, indicates the signal direction"]
pub struct EC0_R(crate::FieldReader<bool, bool>);
impl EC0_R {
    pub(crate) fn new(bits: bool) -> Self {
        EC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EC0` writer - clears the output enable bit, indicates the signal direction"]
pub struct EC0_W<'a> {
    w: &'a mut W,
}
impl<'a> EC0_W<'a> {
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
    #[doc = "Bit 15 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec15(&self) -> EC15_R {
        EC15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec14(&self) -> EC14_R {
        EC14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec13(&self) -> EC13_R {
        EC13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec12(&self) -> EC12_R {
        EC12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec11(&self) -> EC11_R {
        EC11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec10(&self) -> EC10_R {
        EC10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec9(&self) -> EC9_R {
        EC9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec8(&self) -> EC8_R {
        EC8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec7(&self) -> EC7_R {
        EC7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec6(&self) -> EC6_R {
        EC6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec5(&self) -> EC5_R {
        EC5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec4(&self) -> EC4_R {
        EC4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec3(&self) -> EC3_R {
        EC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec2(&self) -> EC2_R {
        EC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec1(&self) -> EC1_R {
        EC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec0(&self) -> EC0_R {
        EC0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec15(&mut self) -> EC15_W {
        EC15_W { w: self }
    }
    #[doc = "Bit 14 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec14(&mut self) -> EC14_W {
        EC14_W { w: self }
    }
    #[doc = "Bit 13 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec13(&mut self) -> EC13_W {
        EC13_W { w: self }
    }
    #[doc = "Bit 12 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec12(&mut self) -> EC12_W {
        EC12_W { w: self }
    }
    #[doc = "Bit 11 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec11(&mut self) -> EC11_W {
        EC11_W { w: self }
    }
    #[doc = "Bit 10 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec10(&mut self) -> EC10_W {
        EC10_W { w: self }
    }
    #[doc = "Bit 9 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec9(&mut self) -> EC9_W {
        EC9_W { w: self }
    }
    #[doc = "Bit 8 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec8(&mut self) -> EC8_W {
        EC8_W { w: self }
    }
    #[doc = "Bit 7 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec7(&mut self) -> EC7_W {
        EC7_W { w: self }
    }
    #[doc = "Bit 6 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec6(&mut self) -> EC6_W {
        EC6_W { w: self }
    }
    #[doc = "Bit 5 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec5(&mut self) -> EC5_W {
        EC5_W { w: self }
    }
    #[doc = "Bit 4 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec4(&mut self) -> EC4_W {
        EC4_W { w: self }
    }
    #[doc = "Bit 3 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec3(&mut self) -> EC3_W {
        EC3_W { w: self }
    }
    #[doc = "Bit 2 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec2(&mut self) -> EC2_W {
        EC2_W { w: self }
    }
    #[doc = "Bit 1 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec1(&mut self) -> EC1_W {
        EC1_W { w: self }
    }
    #[doc = "Bit 0 - clears the output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn ec0(&mut self) -> EC0_W {
        EC0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Enable Clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outenclr](index.html) module"]
pub struct OUTENCLR_SPEC;
impl crate::RegisterSpec for OUTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outenclr::R](R) reader structure"]
impl crate::Readable for OUTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outenclr::W](W) writer structure"]
impl crate::Writable for OUTENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUTENCLR to value 0"]
impl crate::Resettable for OUTENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
