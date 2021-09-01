#[doc = "Register `DATAOUT` reader"]
pub struct R(crate::R<DATAOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATAOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATAOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATAOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATAOUT` writer"]
pub struct W(crate::W<DATAOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATAOUT_SPEC>;
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
impl From<crate::W<DATAOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATAOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAO15` reader - Port out data bit"]
pub struct DAO15_R(crate::FieldReader<bool, bool>);
impl DAO15_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAO15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAO15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAO15` writer - Port out data bit"]
pub struct DAO15_W<'a> {
    w: &'a mut W,
}
impl<'a> DAO15_W<'a> {
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
#[doc = "Field `DAO14` reader - Port out data bit"]
pub struct DAO14_R(crate::FieldReader<bool, bool>);
impl DAO14_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAO14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAO14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAO14` writer - Port out data bit"]
pub struct DAO14_W<'a> {
    w: &'a mut W,
}
impl<'a> DAO14_W<'a> {
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
#[doc = "Field `DAO13` reader - Port out data bit"]
pub struct DAO13_R(crate::FieldReader<bool, bool>);
impl DAO13_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAO13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAO13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAO13` writer - Port out data bit"]
pub struct DAO13_W<'a> {
    w: &'a mut W,
}
impl<'a> DAO13_W<'a> {
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
#[doc = "Field `DAO12` reader - Port out data bit"]
pub struct DAO12_R(crate::FieldReader<bool, bool>);
impl DAO12_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAO12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAO12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAO12` writer - Port out data bit"]
pub struct DAO12_W<'a> {
    w: &'a mut W,
}
impl<'a> DAO12_W<'a> {
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
#[doc = "Field `DAO11` reader - Port out data bit"]
pub struct DAO11_R(crate::FieldReader<bool, bool>);
impl DAO11_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAO11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAO11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAO11` writer - Port out data bit"]
pub struct DAO11_W<'a> {
    w: &'a mut W,
}
impl<'a> DAO11_W<'a> {
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
#[doc = "Field `DAO10` reader - Port out data bit"]
pub struct DAO10_R(crate::FieldReader<bool, bool>);
impl DAO10_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAO10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAO10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAO10` writer - Port out data bit"]
pub struct DAO10_W<'a> {
    w: &'a mut W,
}
impl<'a> DAO10_W<'a> {
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
#[doc = "Field `DAO9` reader - Port out data bit"]
pub struct DAO9_R(crate::FieldReader<bool, bool>);
impl DAO9_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAO9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAO9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAO9` writer - Port out data bit"]
pub struct DAO9_W<'a> {
    w: &'a mut W,
}
impl<'a> DAO9_W<'a> {
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
#[doc = "Field `DAO8` reader - Port out data bit"]
pub struct DAO8_R(crate::FieldReader<bool, bool>);
impl DAO8_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAO8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAO8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAO8` writer - Port out data bit"]
pub struct DAO8_W<'a> {
    w: &'a mut W,
}
impl<'a> DAO8_W<'a> {
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
#[doc = "Field `DAO7` reader - Port out data bit"]
pub struct DAO7_R(crate::FieldReader<bool, bool>);
impl DAO7_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAO7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAO7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAO7` writer - Port out data bit"]
pub struct DAO7_W<'a> {
    w: &'a mut W,
}
impl<'a> DAO7_W<'a> {
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
#[doc = "Field `DAO6` reader - Port out data bit"]
pub struct DAO6_R(crate::FieldReader<bool, bool>);
impl DAO6_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAO6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAO6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAO6` writer - Port out data bit"]
pub struct DAO6_W<'a> {
    w: &'a mut W,
}
impl<'a> DAO6_W<'a> {
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
#[doc = "Field `DAO5` reader - Port out data bit"]
pub struct DAO5_R(crate::FieldReader<bool, bool>);
impl DAO5_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAO5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAO5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAO5` writer - Port out data bit"]
pub struct DAO5_W<'a> {
    w: &'a mut W,
}
impl<'a> DAO5_W<'a> {
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
#[doc = "Field `DAO4` reader - Port out data bit"]
pub struct DAO4_R(crate::FieldReader<bool, bool>);
impl DAO4_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAO4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAO4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAO4` writer - Port out data bit"]
pub struct DAO4_W<'a> {
    w: &'a mut W,
}
impl<'a> DAO4_W<'a> {
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
#[doc = "Field `DAO3` reader - Port out data bit"]
pub struct DAO3_R(crate::FieldReader<bool, bool>);
impl DAO3_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAO3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAO3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAO3` writer - Port out data bit"]
pub struct DAO3_W<'a> {
    w: &'a mut W,
}
impl<'a> DAO3_W<'a> {
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
#[doc = "Field `DAO2` reader - Port out data bit"]
pub struct DAO2_R(crate::FieldReader<bool, bool>);
impl DAO2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAO2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAO2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAO2` writer - Port out data bit"]
pub struct DAO2_W<'a> {
    w: &'a mut W,
}
impl<'a> DAO2_W<'a> {
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
#[doc = "Field `DAO1` reader - Port out data bit"]
pub struct DAO1_R(crate::FieldReader<bool, bool>);
impl DAO1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAO1` writer - Port out data bit"]
pub struct DAO1_W<'a> {
    w: &'a mut W,
}
impl<'a> DAO1_W<'a> {
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
#[doc = "Field `DAO0` reader - Port out data bit"]
pub struct DAO0_R(crate::FieldReader<bool, bool>);
impl DAO0_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAO0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAO0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAO0` writer - Port out data bit"]
pub struct DAO0_W<'a> {
    w: &'a mut W,
}
impl<'a> DAO0_W<'a> {
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
    #[doc = "Bit 15 - Port out data bit"]
    #[inline(always)]
    pub fn dao15(&self) -> DAO15_R {
        DAO15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port out data bit"]
    #[inline(always)]
    pub fn dao14(&self) -> DAO14_R {
        DAO14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port out data bit"]
    #[inline(always)]
    pub fn dao13(&self) -> DAO13_R {
        DAO13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port out data bit"]
    #[inline(always)]
    pub fn dao12(&self) -> DAO12_R {
        DAO12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port out data bit"]
    #[inline(always)]
    pub fn dao11(&self) -> DAO11_R {
        DAO11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port out data bit"]
    #[inline(always)]
    pub fn dao10(&self) -> DAO10_R {
        DAO10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port out data bit"]
    #[inline(always)]
    pub fn dao9(&self) -> DAO9_R {
        DAO9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port out data bit"]
    #[inline(always)]
    pub fn dao8(&self) -> DAO8_R {
        DAO8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port out data bit"]
    #[inline(always)]
    pub fn dao7(&self) -> DAO7_R {
        DAO7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port out data bit"]
    #[inline(always)]
    pub fn dao6(&self) -> DAO6_R {
        DAO6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port out data bit"]
    #[inline(always)]
    pub fn dao5(&self) -> DAO5_R {
        DAO5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port out data bit"]
    #[inline(always)]
    pub fn dao4(&self) -> DAO4_R {
        DAO4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port out data bit"]
    #[inline(always)]
    pub fn dao3(&self) -> DAO3_R {
        DAO3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port out data bit"]
    #[inline(always)]
    pub fn dao2(&self) -> DAO2_R {
        DAO2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port out data bit"]
    #[inline(always)]
    pub fn dao1(&self) -> DAO1_R {
        DAO1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Port out data bit"]
    #[inline(always)]
    pub fn dao0(&self) -> DAO0_R {
        DAO0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Port out data bit"]
    #[inline(always)]
    pub fn dao15(&mut self) -> DAO15_W {
        DAO15_W { w: self }
    }
    #[doc = "Bit 14 - Port out data bit"]
    #[inline(always)]
    pub fn dao14(&mut self) -> DAO14_W {
        DAO14_W { w: self }
    }
    #[doc = "Bit 13 - Port out data bit"]
    #[inline(always)]
    pub fn dao13(&mut self) -> DAO13_W {
        DAO13_W { w: self }
    }
    #[doc = "Bit 12 - Port out data bit"]
    #[inline(always)]
    pub fn dao12(&mut self) -> DAO12_W {
        DAO12_W { w: self }
    }
    #[doc = "Bit 11 - Port out data bit"]
    #[inline(always)]
    pub fn dao11(&mut self) -> DAO11_W {
        DAO11_W { w: self }
    }
    #[doc = "Bit 10 - Port out data bit"]
    #[inline(always)]
    pub fn dao10(&mut self) -> DAO10_W {
        DAO10_W { w: self }
    }
    #[doc = "Bit 9 - Port out data bit"]
    #[inline(always)]
    pub fn dao9(&mut self) -> DAO9_W {
        DAO9_W { w: self }
    }
    #[doc = "Bit 8 - Port out data bit"]
    #[inline(always)]
    pub fn dao8(&mut self) -> DAO8_W {
        DAO8_W { w: self }
    }
    #[doc = "Bit 7 - Port out data bit"]
    #[inline(always)]
    pub fn dao7(&mut self) -> DAO7_W {
        DAO7_W { w: self }
    }
    #[doc = "Bit 6 - Port out data bit"]
    #[inline(always)]
    pub fn dao6(&mut self) -> DAO6_W {
        DAO6_W { w: self }
    }
    #[doc = "Bit 5 - Port out data bit"]
    #[inline(always)]
    pub fn dao5(&mut self) -> DAO5_W {
        DAO5_W { w: self }
    }
    #[doc = "Bit 4 - Port out data bit"]
    #[inline(always)]
    pub fn dao4(&mut self) -> DAO4_W {
        DAO4_W { w: self }
    }
    #[doc = "Bit 3 - Port out data bit"]
    #[inline(always)]
    pub fn dao3(&mut self) -> DAO3_W {
        DAO3_W { w: self }
    }
    #[doc = "Bit 2 - Port out data bit"]
    #[inline(always)]
    pub fn dao2(&mut self) -> DAO2_W {
        DAO2_W { w: self }
    }
    #[doc = "Bit 1 - Port out data bit"]
    #[inline(always)]
    pub fn dao1(&mut self) -> DAO1_W {
        DAO1_W { w: self }
    }
    #[doc = "Bit 0 - Port out data bit"]
    #[inline(always)]
    pub fn dao0(&mut self) -> DAO0_W {
        DAO0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Output Latch register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dataout](index.html) module"]
pub struct DATAOUT_SPEC;
impl crate::RegisterSpec for DATAOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dataout::R](R) reader structure"]
impl crate::Readable for DATAOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dataout::W](W) writer structure"]
impl crate::Writable for DATAOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATAOUT to value 0"]
impl crate::Resettable for DATAOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
