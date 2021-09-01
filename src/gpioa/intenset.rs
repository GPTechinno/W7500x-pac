#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IES15` reader - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES15_R(crate::FieldReader<bool, bool>);
impl IES15_R {
    pub(crate) fn new(bits: bool) -> Self {
        IES15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IES15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IES15` writer - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES15_W<'a> {
    w: &'a mut W,
}
impl<'a> IES15_W<'a> {
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
#[doc = "Field `IES14` reader - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES14_R(crate::FieldReader<bool, bool>);
impl IES14_R {
    pub(crate) fn new(bits: bool) -> Self {
        IES14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IES14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IES14` writer - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES14_W<'a> {
    w: &'a mut W,
}
impl<'a> IES14_W<'a> {
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
#[doc = "Field `IES13` reader - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES13_R(crate::FieldReader<bool, bool>);
impl IES13_R {
    pub(crate) fn new(bits: bool) -> Self {
        IES13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IES13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IES13` writer - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES13_W<'a> {
    w: &'a mut W,
}
impl<'a> IES13_W<'a> {
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
#[doc = "Field `IES12` reader - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES12_R(crate::FieldReader<bool, bool>);
impl IES12_R {
    pub(crate) fn new(bits: bool) -> Self {
        IES12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IES12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IES12` writer - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES12_W<'a> {
    w: &'a mut W,
}
impl<'a> IES12_W<'a> {
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
#[doc = "Field `IES11` reader - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES11_R(crate::FieldReader<bool, bool>);
impl IES11_R {
    pub(crate) fn new(bits: bool) -> Self {
        IES11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IES11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IES11` writer - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES11_W<'a> {
    w: &'a mut W,
}
impl<'a> IES11_W<'a> {
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
#[doc = "Field `IES10` reader - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES10_R(crate::FieldReader<bool, bool>);
impl IES10_R {
    pub(crate) fn new(bits: bool) -> Self {
        IES10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IES10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IES10` writer - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES10_W<'a> {
    w: &'a mut W,
}
impl<'a> IES10_W<'a> {
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
#[doc = "Field `IES9` reader - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES9_R(crate::FieldReader<bool, bool>);
impl IES9_R {
    pub(crate) fn new(bits: bool) -> Self {
        IES9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IES9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IES9` writer - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES9_W<'a> {
    w: &'a mut W,
}
impl<'a> IES9_W<'a> {
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
#[doc = "Field `IES8` reader - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES8_R(crate::FieldReader<bool, bool>);
impl IES8_R {
    pub(crate) fn new(bits: bool) -> Self {
        IES8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IES8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IES8` writer - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES8_W<'a> {
    w: &'a mut W,
}
impl<'a> IES8_W<'a> {
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
#[doc = "Field `IES7` reader - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES7_R(crate::FieldReader<bool, bool>);
impl IES7_R {
    pub(crate) fn new(bits: bool) -> Self {
        IES7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IES7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IES7` writer - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES7_W<'a> {
    w: &'a mut W,
}
impl<'a> IES7_W<'a> {
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
#[doc = "Field `IES6` reader - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES6_R(crate::FieldReader<bool, bool>);
impl IES6_R {
    pub(crate) fn new(bits: bool) -> Self {
        IES6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IES6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IES6` writer - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES6_W<'a> {
    w: &'a mut W,
}
impl<'a> IES6_W<'a> {
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
#[doc = "Field `IES5` reader - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES5_R(crate::FieldReader<bool, bool>);
impl IES5_R {
    pub(crate) fn new(bits: bool) -> Self {
        IES5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IES5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IES5` writer - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES5_W<'a> {
    w: &'a mut W,
}
impl<'a> IES5_W<'a> {
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
#[doc = "Field `IES4` reader - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES4_R(crate::FieldReader<bool, bool>);
impl IES4_R {
    pub(crate) fn new(bits: bool) -> Self {
        IES4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IES4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IES4` writer - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES4_W<'a> {
    w: &'a mut W,
}
impl<'a> IES4_W<'a> {
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
#[doc = "Field `IES3` reader - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES3_R(crate::FieldReader<bool, bool>);
impl IES3_R {
    pub(crate) fn new(bits: bool) -> Self {
        IES3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IES3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IES3` writer - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES3_W<'a> {
    w: &'a mut W,
}
impl<'a> IES3_W<'a> {
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
#[doc = "Field `IES2` reader - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES2_R(crate::FieldReader<bool, bool>);
impl IES2_R {
    pub(crate) fn new(bits: bool) -> Self {
        IES2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IES2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IES2` writer - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES2_W<'a> {
    w: &'a mut W,
}
impl<'a> IES2_W<'a> {
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
#[doc = "Field `IES1` reader - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES1_R(crate::FieldReader<bool, bool>);
impl IES1_R {
    pub(crate) fn new(bits: bool) -> Self {
        IES1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IES1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IES1` writer - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES1_W<'a> {
    w: &'a mut W,
}
impl<'a> IES1_W<'a> {
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
#[doc = "Field `IES0` reader - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES0_R(crate::FieldReader<bool, bool>);
impl IES0_R {
    pub(crate) fn new(bits: bool) -> Self {
        IES0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IES0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IES0` writer - sets the interrupt enable bit, indicates the interrupt"]
pub struct IES0_W<'a> {
    w: &'a mut W,
}
impl<'a> IES0_W<'a> {
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
    #[doc = "Bit 15 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies15(&self) -> IES15_R {
        IES15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies14(&self) -> IES14_R {
        IES14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies13(&self) -> IES13_R {
        IES13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies12(&self) -> IES12_R {
        IES12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies11(&self) -> IES11_R {
        IES11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies10(&self) -> IES10_R {
        IES10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies9(&self) -> IES9_R {
        IES9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies8(&self) -> IES8_R {
        IES8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies7(&self) -> IES7_R {
        IES7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies6(&self) -> IES6_R {
        IES6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies5(&self) -> IES5_R {
        IES5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies4(&self) -> IES4_R {
        IES4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies3(&self) -> IES3_R {
        IES3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies2(&self) -> IES2_R {
        IES2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies1(&self) -> IES1_R {
        IES1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies0(&self) -> IES0_R {
        IES0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies15(&mut self) -> IES15_W {
        IES15_W { w: self }
    }
    #[doc = "Bit 14 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies14(&mut self) -> IES14_W {
        IES14_W { w: self }
    }
    #[doc = "Bit 13 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies13(&mut self) -> IES13_W {
        IES13_W { w: self }
    }
    #[doc = "Bit 12 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies12(&mut self) -> IES12_W {
        IES12_W { w: self }
    }
    #[doc = "Bit 11 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies11(&mut self) -> IES11_W {
        IES11_W { w: self }
    }
    #[doc = "Bit 10 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies10(&mut self) -> IES10_W {
        IES10_W { w: self }
    }
    #[doc = "Bit 9 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies9(&mut self) -> IES9_W {
        IES9_W { w: self }
    }
    #[doc = "Bit 8 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies8(&mut self) -> IES8_W {
        IES8_W { w: self }
    }
    #[doc = "Bit 7 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies7(&mut self) -> IES7_W {
        IES7_W { w: self }
    }
    #[doc = "Bit 6 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies6(&mut self) -> IES6_W {
        IES6_W { w: self }
    }
    #[doc = "Bit 5 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies5(&mut self) -> IES5_W {
        IES5_W { w: self }
    }
    #[doc = "Bit 4 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies4(&mut self) -> IES4_W {
        IES4_W { w: self }
    }
    #[doc = "Bit 3 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies3(&mut self) -> IES3_W {
        IES3_W { w: self }
    }
    #[doc = "Bit 2 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies2(&mut self) -> IES2_W {
        IES2_W { w: self }
    }
    #[doc = "Bit 1 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies1(&mut self) -> IES1_W {
        IES1_W { w: self }
    }
    #[doc = "Bit 0 - sets the interrupt enable bit, indicates the interrupt"]
    #[inline(always)]
    pub fn ies0(&mut self) -> IES0_W {
        IES0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
