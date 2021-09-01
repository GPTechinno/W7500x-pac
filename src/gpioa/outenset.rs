#[doc = "Register `OUTENSET` reader"]
pub struct R(crate::R<OUTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTENSET` writer"]
pub struct W(crate::W<OUTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTENSET_SPEC>;
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
impl From<crate::W<OUTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ES15` reader - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES15_R(crate::FieldReader<bool, bool>);
impl ES15_R {
    pub(crate) fn new(bits: bool) -> Self {
        ES15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ES15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ES15` writer - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES15_W<'a> {
    w: &'a mut W,
}
impl<'a> ES15_W<'a> {
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
#[doc = "Field `ES14` reader - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES14_R(crate::FieldReader<bool, bool>);
impl ES14_R {
    pub(crate) fn new(bits: bool) -> Self {
        ES14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ES14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ES14` writer - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES14_W<'a> {
    w: &'a mut W,
}
impl<'a> ES14_W<'a> {
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
#[doc = "Field `ES13` reader - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES13_R(crate::FieldReader<bool, bool>);
impl ES13_R {
    pub(crate) fn new(bits: bool) -> Self {
        ES13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ES13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ES13` writer - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES13_W<'a> {
    w: &'a mut W,
}
impl<'a> ES13_W<'a> {
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
#[doc = "Field `ES12` reader - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES12_R(crate::FieldReader<bool, bool>);
impl ES12_R {
    pub(crate) fn new(bits: bool) -> Self {
        ES12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ES12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ES12` writer - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES12_W<'a> {
    w: &'a mut W,
}
impl<'a> ES12_W<'a> {
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
#[doc = "Field `ES11` reader - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES11_R(crate::FieldReader<bool, bool>);
impl ES11_R {
    pub(crate) fn new(bits: bool) -> Self {
        ES11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ES11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ES11` writer - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES11_W<'a> {
    w: &'a mut W,
}
impl<'a> ES11_W<'a> {
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
#[doc = "Field `ES10` reader - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES10_R(crate::FieldReader<bool, bool>);
impl ES10_R {
    pub(crate) fn new(bits: bool) -> Self {
        ES10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ES10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ES10` writer - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES10_W<'a> {
    w: &'a mut W,
}
impl<'a> ES10_W<'a> {
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
#[doc = "Field `ES9` reader - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES9_R(crate::FieldReader<bool, bool>);
impl ES9_R {
    pub(crate) fn new(bits: bool) -> Self {
        ES9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ES9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ES9` writer - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES9_W<'a> {
    w: &'a mut W,
}
impl<'a> ES9_W<'a> {
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
#[doc = "Field `ES8` reader - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES8_R(crate::FieldReader<bool, bool>);
impl ES8_R {
    pub(crate) fn new(bits: bool) -> Self {
        ES8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ES8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ES8` writer - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES8_W<'a> {
    w: &'a mut W,
}
impl<'a> ES8_W<'a> {
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
#[doc = "Field `ES7` reader - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES7_R(crate::FieldReader<bool, bool>);
impl ES7_R {
    pub(crate) fn new(bits: bool) -> Self {
        ES7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ES7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ES7` writer - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES7_W<'a> {
    w: &'a mut W,
}
impl<'a> ES7_W<'a> {
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
#[doc = "Field `ES6` reader - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES6_R(crate::FieldReader<bool, bool>);
impl ES6_R {
    pub(crate) fn new(bits: bool) -> Self {
        ES6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ES6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ES6` writer - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES6_W<'a> {
    w: &'a mut W,
}
impl<'a> ES6_W<'a> {
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
#[doc = "Field `ES5` reader - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES5_R(crate::FieldReader<bool, bool>);
impl ES5_R {
    pub(crate) fn new(bits: bool) -> Self {
        ES5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ES5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ES5` writer - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES5_W<'a> {
    w: &'a mut W,
}
impl<'a> ES5_W<'a> {
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
#[doc = "Field `ES4` reader - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES4_R(crate::FieldReader<bool, bool>);
impl ES4_R {
    pub(crate) fn new(bits: bool) -> Self {
        ES4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ES4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ES4` writer - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES4_W<'a> {
    w: &'a mut W,
}
impl<'a> ES4_W<'a> {
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
#[doc = "Field `ES3` reader - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES3_R(crate::FieldReader<bool, bool>);
impl ES3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ES3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ES3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ES3` writer - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES3_W<'a> {
    w: &'a mut W,
}
impl<'a> ES3_W<'a> {
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
#[doc = "Field `ES2` reader - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES2_R(crate::FieldReader<bool, bool>);
impl ES2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ES2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ES2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ES2` writer - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES2_W<'a> {
    w: &'a mut W,
}
impl<'a> ES2_W<'a> {
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
#[doc = "Field `ES1` reader - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES1_R(crate::FieldReader<bool, bool>);
impl ES1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ES1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ES1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ES1` writer - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES1_W<'a> {
    w: &'a mut W,
}
impl<'a> ES1_W<'a> {
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
#[doc = "Field `ES0` reader - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES0_R(crate::FieldReader<bool, bool>);
impl ES0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ES0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ES0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ES0` writer - sets the corresponding output enable bit, indicates the signal direction"]
pub struct ES0_W<'a> {
    w: &'a mut W,
}
impl<'a> ES0_W<'a> {
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
    #[doc = "Bit 15 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es15(&self) -> ES15_R {
        ES15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es14(&self) -> ES14_R {
        ES14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es13(&self) -> ES13_R {
        ES13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es12(&self) -> ES12_R {
        ES12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es11(&self) -> ES11_R {
        ES11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es10(&self) -> ES10_R {
        ES10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es9(&self) -> ES9_R {
        ES9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es8(&self) -> ES8_R {
        ES8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es7(&self) -> ES7_R {
        ES7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es6(&self) -> ES6_R {
        ES6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es5(&self) -> ES5_R {
        ES5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es4(&self) -> ES4_R {
        ES4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es3(&self) -> ES3_R {
        ES3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es2(&self) -> ES2_R {
        ES2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es1(&self) -> ES1_R {
        ES1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es0(&self) -> ES0_R {
        ES0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es15(&mut self) -> ES15_W {
        ES15_W { w: self }
    }
    #[doc = "Bit 14 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es14(&mut self) -> ES14_W {
        ES14_W { w: self }
    }
    #[doc = "Bit 13 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es13(&mut self) -> ES13_W {
        ES13_W { w: self }
    }
    #[doc = "Bit 12 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es12(&mut self) -> ES12_W {
        ES12_W { w: self }
    }
    #[doc = "Bit 11 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es11(&mut self) -> ES11_W {
        ES11_W { w: self }
    }
    #[doc = "Bit 10 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es10(&mut self) -> ES10_W {
        ES10_W { w: self }
    }
    #[doc = "Bit 9 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es9(&mut self) -> ES9_W {
        ES9_W { w: self }
    }
    #[doc = "Bit 8 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es8(&mut self) -> ES8_W {
        ES8_W { w: self }
    }
    #[doc = "Bit 7 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es7(&mut self) -> ES7_W {
        ES7_W { w: self }
    }
    #[doc = "Bit 6 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es6(&mut self) -> ES6_W {
        ES6_W { w: self }
    }
    #[doc = "Bit 5 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es5(&mut self) -> ES5_W {
        ES5_W { w: self }
    }
    #[doc = "Bit 4 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es4(&mut self) -> ES4_W {
        ES4_W { w: self }
    }
    #[doc = "Bit 3 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es3(&mut self) -> ES3_W {
        ES3_W { w: self }
    }
    #[doc = "Bit 2 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es2(&mut self) -> ES2_W {
        ES2_W { w: self }
    }
    #[doc = "Bit 1 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es1(&mut self) -> ES1_W {
        ES1_W { w: self }
    }
    #[doc = "Bit 0 - sets the corresponding output enable bit, indicates the signal direction"]
    #[inline(always)]
    pub fn es0(&mut self) -> ES0_W {
        ES0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Enable Set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outenset](index.html) module"]
pub struct OUTENSET_SPEC;
impl crate::RegisterSpec for OUTENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outenset::R](R) reader structure"]
impl crate::Readable for OUTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outenset::W](W) writer structure"]
impl crate::Writable for OUTENSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUTENSET to value 0"]
impl crate::Resettable for OUTENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
