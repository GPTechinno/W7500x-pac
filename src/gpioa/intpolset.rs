#[doc = "Register `INTPOLSET` reader"]
pub struct R(crate::R<INTPOLSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTPOLSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTPOLSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTPOLSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTPOLSET` writer"]
pub struct W(crate::W<INTPOLSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTPOLSET_SPEC>;
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
impl From<crate::W<INTPOLSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTPOLSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IPS15` reader - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS15_R(crate::FieldReader<bool>);
impl IPS15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IPS15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPS15_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPS15` writer - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS15_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS15_W<'a> {
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
#[doc = "Field `IPS14` reader - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS14_R(crate::FieldReader<bool>);
impl IPS14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IPS14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPS14_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPS14` writer - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS14_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS14_W<'a> {
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
#[doc = "Field `IPS13` reader - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS13_R(crate::FieldReader<bool>);
impl IPS13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IPS13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPS13_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPS13` writer - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS13_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS13_W<'a> {
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
#[doc = "Field `IPS12` reader - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS12_R(crate::FieldReader<bool>);
impl IPS12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IPS12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPS12_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPS12` writer - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS12_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS12_W<'a> {
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
#[doc = "Field `IPS11` reader - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS11_R(crate::FieldReader<bool>);
impl IPS11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IPS11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPS11_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPS11` writer - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS11_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS11_W<'a> {
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
#[doc = "Field `IPS10` reader - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS10_R(crate::FieldReader<bool>);
impl IPS10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IPS10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPS10_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPS10` writer - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS10_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS10_W<'a> {
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
#[doc = "Field `IPS9` reader - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS9_R(crate::FieldReader<bool>);
impl IPS9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IPS9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPS9_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPS9` writer - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS9_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS9_W<'a> {
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
#[doc = "Field `IPS8` reader - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS8_R(crate::FieldReader<bool>);
impl IPS8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IPS8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPS8_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPS8` writer - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS8_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS8_W<'a> {
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
#[doc = "Field `IPS7` reader - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS7_R(crate::FieldReader<bool>);
impl IPS7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IPS7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPS7_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPS7` writer - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS7_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS7_W<'a> {
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
#[doc = "Field `IPS6` reader - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS6_R(crate::FieldReader<bool>);
impl IPS6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IPS6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPS6_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPS6` writer - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS6_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS6_W<'a> {
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
#[doc = "Field `IPS5` reader - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS5_R(crate::FieldReader<bool>);
impl IPS5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IPS5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPS5_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPS5` writer - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS5_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS5_W<'a> {
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
#[doc = "Field `IPS4` reader - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS4_R(crate::FieldReader<bool>);
impl IPS4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IPS4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPS4_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPS4` writer - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS4_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS4_W<'a> {
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
#[doc = "Field `IPS3` reader - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS3_R(crate::FieldReader<bool>);
impl IPS3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IPS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPS3_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPS3` writer - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS3_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS3_W<'a> {
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
#[doc = "Field `IPS2` reader - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS2_R(crate::FieldReader<bool>);
impl IPS2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IPS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPS2_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPS2` writer - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS2_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS2_W<'a> {
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
#[doc = "Field `IPS1` reader - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS1_R(crate::FieldReader<bool>);
impl IPS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IPS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPS1_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPS1` writer - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS1_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS1_W<'a> {
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
#[doc = "Field `IPS0` reader - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS0_R(crate::FieldReader<bool>);
impl IPS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IPS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPS0_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPS0` writer - sets the interrupt polarity bit, indicates for edge or level"]
pub struct IPS0_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS0_W<'a> {
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
    #[doc = "Bit 15 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips15(&self) -> IPS15_R {
        IPS15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips14(&self) -> IPS14_R {
        IPS14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips13(&self) -> IPS13_R {
        IPS13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips12(&self) -> IPS12_R {
        IPS12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips11(&self) -> IPS11_R {
        IPS11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips10(&self) -> IPS10_R {
        IPS10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips9(&self) -> IPS9_R {
        IPS9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips8(&self) -> IPS8_R {
        IPS8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips7(&self) -> IPS7_R {
        IPS7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips6(&self) -> IPS6_R {
        IPS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips5(&self) -> IPS5_R {
        IPS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips4(&self) -> IPS4_R {
        IPS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips3(&self) -> IPS3_R {
        IPS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips2(&self) -> IPS2_R {
        IPS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips1(&self) -> IPS1_R {
        IPS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips0(&self) -> IPS0_R {
        IPS0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips15(&mut self) -> IPS15_W {
        IPS15_W { w: self }
    }
    #[doc = "Bit 14 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips14(&mut self) -> IPS14_W {
        IPS14_W { w: self }
    }
    #[doc = "Bit 13 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips13(&mut self) -> IPS13_W {
        IPS13_W { w: self }
    }
    #[doc = "Bit 12 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips12(&mut self) -> IPS12_W {
        IPS12_W { w: self }
    }
    #[doc = "Bit 11 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips11(&mut self) -> IPS11_W {
        IPS11_W { w: self }
    }
    #[doc = "Bit 10 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips10(&mut self) -> IPS10_W {
        IPS10_W { w: self }
    }
    #[doc = "Bit 9 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips9(&mut self) -> IPS9_W {
        IPS9_W { w: self }
    }
    #[doc = "Bit 8 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips8(&mut self) -> IPS8_W {
        IPS8_W { w: self }
    }
    #[doc = "Bit 7 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips7(&mut self) -> IPS7_W {
        IPS7_W { w: self }
    }
    #[doc = "Bit 6 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips6(&mut self) -> IPS6_W {
        IPS6_W { w: self }
    }
    #[doc = "Bit 5 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips5(&mut self) -> IPS5_W {
        IPS5_W { w: self }
    }
    #[doc = "Bit 4 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips4(&mut self) -> IPS4_W {
        IPS4_W { w: self }
    }
    #[doc = "Bit 3 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips3(&mut self) -> IPS3_W {
        IPS3_W { w: self }
    }
    #[doc = "Bit 2 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips2(&mut self) -> IPS2_W {
        IPS2_W { w: self }
    }
    #[doc = "Bit 1 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips1(&mut self) -> IPS1_W {
        IPS1_W { w: self }
    }
    #[doc = "Bit 0 - sets the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ips0(&mut self) -> IPS0_W {
        IPS0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Polarity Set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intpolset](index.html) module"]
pub struct INTPOLSET_SPEC;
impl crate::RegisterSpec for INTPOLSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intpolset::R](R) reader structure"]
impl crate::Readable for INTPOLSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intpolset::W](W) writer structure"]
impl crate::Writable for INTPOLSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTPOLSET to value 0"]
impl crate::Resettable for INTPOLSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
