#[doc = "Register `INTPOLCLR` reader"]
pub struct R(crate::R<INTPOLCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTPOLCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTPOLCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTPOLCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTPOLCLR` writer"]
pub struct W(crate::W<INTPOLCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTPOLCLR_SPEC>;
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
impl From<crate::W<INTPOLCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTPOLCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IPC15` reader - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC15_R(crate::FieldReader<bool, bool>);
impl IPC15_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPC15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPC15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPC15` writer - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC15_W<'a> {
    w: &'a mut W,
}
impl<'a> IPC15_W<'a> {
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
#[doc = "Field `IPC14` reader - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC14_R(crate::FieldReader<bool, bool>);
impl IPC14_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPC14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPC14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPC14` writer - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC14_W<'a> {
    w: &'a mut W,
}
impl<'a> IPC14_W<'a> {
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
#[doc = "Field `IPC13` reader - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC13_R(crate::FieldReader<bool, bool>);
impl IPC13_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPC13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPC13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPC13` writer - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC13_W<'a> {
    w: &'a mut W,
}
impl<'a> IPC13_W<'a> {
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
#[doc = "Field `IPC12` reader - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC12_R(crate::FieldReader<bool, bool>);
impl IPC12_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPC12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPC12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPC12` writer - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC12_W<'a> {
    w: &'a mut W,
}
impl<'a> IPC12_W<'a> {
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
#[doc = "Field `IPC11` reader - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC11_R(crate::FieldReader<bool, bool>);
impl IPC11_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPC11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPC11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPC11` writer - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC11_W<'a> {
    w: &'a mut W,
}
impl<'a> IPC11_W<'a> {
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
#[doc = "Field `IPC10` reader - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC10_R(crate::FieldReader<bool, bool>);
impl IPC10_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPC10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPC10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPC10` writer - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC10_W<'a> {
    w: &'a mut W,
}
impl<'a> IPC10_W<'a> {
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
#[doc = "Field `IPC9` reader - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC9_R(crate::FieldReader<bool, bool>);
impl IPC9_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPC9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPC9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPC9` writer - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC9_W<'a> {
    w: &'a mut W,
}
impl<'a> IPC9_W<'a> {
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
#[doc = "Field `IPC8` reader - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC8_R(crate::FieldReader<bool, bool>);
impl IPC8_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPC8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPC8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPC8` writer - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC8_W<'a> {
    w: &'a mut W,
}
impl<'a> IPC8_W<'a> {
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
#[doc = "Field `IPC7` reader - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC7_R(crate::FieldReader<bool, bool>);
impl IPC7_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPC7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPC7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPC7` writer - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC7_W<'a> {
    w: &'a mut W,
}
impl<'a> IPC7_W<'a> {
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
#[doc = "Field `IPC6` reader - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC6_R(crate::FieldReader<bool, bool>);
impl IPC6_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPC6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPC6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPC6` writer - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC6_W<'a> {
    w: &'a mut W,
}
impl<'a> IPC6_W<'a> {
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
#[doc = "Field `IPC5` reader - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC5_R(crate::FieldReader<bool, bool>);
impl IPC5_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPC5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPC5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPC5` writer - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC5_W<'a> {
    w: &'a mut W,
}
impl<'a> IPC5_W<'a> {
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
#[doc = "Field `IPC4` reader - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC4_R(crate::FieldReader<bool, bool>);
impl IPC4_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPC4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPC4` writer - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC4_W<'a> {
    w: &'a mut W,
}
impl<'a> IPC4_W<'a> {
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
#[doc = "Field `IPC3` reader - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC3_R(crate::FieldReader<bool, bool>);
impl IPC3_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPC3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPC3` writer - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC3_W<'a> {
    w: &'a mut W,
}
impl<'a> IPC3_W<'a> {
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
#[doc = "Field `IPC2` reader - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC2_R(crate::FieldReader<bool, bool>);
impl IPC2_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPC2` writer - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC2_W<'a> {
    w: &'a mut W,
}
impl<'a> IPC2_W<'a> {
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
#[doc = "Field `IPC1` reader - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC1_R(crate::FieldReader<bool, bool>);
impl IPC1_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPC1` writer - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC1_W<'a> {
    w: &'a mut W,
}
impl<'a> IPC1_W<'a> {
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
#[doc = "Field `IPC0` reader - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC0_R(crate::FieldReader<bool, bool>);
impl IPC0_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPC0` writer - clears the interrupt polarity bit, indicates for edge or level"]
pub struct IPC0_W<'a> {
    w: &'a mut W,
}
impl<'a> IPC0_W<'a> {
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
    #[doc = "Bit 15 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc15(&self) -> IPC15_R {
        IPC15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc14(&self) -> IPC14_R {
        IPC14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc13(&self) -> IPC13_R {
        IPC13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc12(&self) -> IPC12_R {
        IPC12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc11(&self) -> IPC11_R {
        IPC11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc10(&self) -> IPC10_R {
        IPC10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc9(&self) -> IPC9_R {
        IPC9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc8(&self) -> IPC8_R {
        IPC8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc7(&self) -> IPC7_R {
        IPC7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc6(&self) -> IPC6_R {
        IPC6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc5(&self) -> IPC5_R {
        IPC5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc4(&self) -> IPC4_R {
        IPC4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc3(&self) -> IPC3_R {
        IPC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc2(&self) -> IPC2_R {
        IPC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc1(&self) -> IPC1_R {
        IPC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc0(&self) -> IPC0_R {
        IPC0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc15(&mut self) -> IPC15_W {
        IPC15_W { w: self }
    }
    #[doc = "Bit 14 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc14(&mut self) -> IPC14_W {
        IPC14_W { w: self }
    }
    #[doc = "Bit 13 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc13(&mut self) -> IPC13_W {
        IPC13_W { w: self }
    }
    #[doc = "Bit 12 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc12(&mut self) -> IPC12_W {
        IPC12_W { w: self }
    }
    #[doc = "Bit 11 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc11(&mut self) -> IPC11_W {
        IPC11_W { w: self }
    }
    #[doc = "Bit 10 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc10(&mut self) -> IPC10_W {
        IPC10_W { w: self }
    }
    #[doc = "Bit 9 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc9(&mut self) -> IPC9_W {
        IPC9_W { w: self }
    }
    #[doc = "Bit 8 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc8(&mut self) -> IPC8_W {
        IPC8_W { w: self }
    }
    #[doc = "Bit 7 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc7(&mut self) -> IPC7_W {
        IPC7_W { w: self }
    }
    #[doc = "Bit 6 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc6(&mut self) -> IPC6_W {
        IPC6_W { w: self }
    }
    #[doc = "Bit 5 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc5(&mut self) -> IPC5_W {
        IPC5_W { w: self }
    }
    #[doc = "Bit 4 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc4(&mut self) -> IPC4_W {
        IPC4_W { w: self }
    }
    #[doc = "Bit 3 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc3(&mut self) -> IPC3_W {
        IPC3_W { w: self }
    }
    #[doc = "Bit 2 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc2(&mut self) -> IPC2_W {
        IPC2_W { w: self }
    }
    #[doc = "Bit 1 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc1(&mut self) -> IPC1_W {
        IPC1_W { w: self }
    }
    #[doc = "Bit 0 - clears the interrupt polarity bit, indicates for edge or level"]
    #[inline(always)]
    pub fn ipc0(&mut self) -> IPC0_W {
        IPC0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Polarity Clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intpolclr](index.html) module"]
pub struct INTPOLCLR_SPEC;
impl crate::RegisterSpec for INTPOLCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intpolclr::R](R) reader structure"]
impl crate::Readable for INTPOLCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intpolclr::W](W) writer structure"]
impl crate::Writable for INTPOLCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTPOLCLR to value 0"]
impl crate::Resettable for INTPOLCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
