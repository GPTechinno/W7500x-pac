#[doc = "Register `LOCK` reader"]
pub struct R(crate::R<LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOCK` writer"]
pub struct W(crate::W<LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOCK_SPEC>;
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
impl From<crate::W<LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ERW\\[31:1\\]
bits (Enable Register Writes)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum ERW_A {
    #[doc = "0: `0`"]
    LOCK = 0,
    #[doc = "449635665: `11010110011001110010101010001`"]
    UNLOCK = 449635665,
}
impl From<ERW_A> for u32 {
    #[inline(always)]
    fn from(variant: ERW_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ERW` reader - ERW\\[31:1\\]
bits (Enable Register Writes)"]
pub struct ERW_R(crate::FieldReader<u32>);
impl ERW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ERW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ERW_A> {
        match self.bits {
            0 => Some(ERW_A::LOCK),
            449635665 => Some(ERW_A::UNLOCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK`"]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        **self == ERW_A::LOCK
    }
    #[doc = "Checks if the value of the field is `UNLOCK`"]
    #[inline(always)]
    pub fn is_un_lock(&self) -> bool {
        **self == ERW_A::UNLOCK
    }
}
impl core::ops::Deref for ERW_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERW` writer - ERW\\[31:1\\]
bits (Enable Register Writes)"]
pub struct ERW_W<'a> {
    w: &'a mut W,
}
impl<'a> ERW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERW_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(ERW_A::LOCK)
    }
    #[doc = "`11010110011001110010101010001`"]
    #[inline(always)]
    pub fn un_lock(self) -> &'a mut W {
        self.variant(ERW_A::UNLOCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff_ffff << 1)) | ((value as u32 & 0x7fff_ffff) << 1);
        self.w
    }
}
#[doc = "Register Write Enable status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WES_A {
    #[doc = "0: `0`"]
    NOTLOCKED = 0,
    #[doc = "1: `1`"]
    LOCKED = 1,
}
impl From<WES_A> for bool {
    #[inline(always)]
    fn from(variant: WES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WES` reader - Register Write Enable status"]
pub struct WES_R(crate::FieldReader<bool>);
impl WES_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WES_A {
        match self.bits {
            false => WES_A::NOTLOCKED,
            true => WES_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLOCKED`"]
    #[inline(always)]
    pub fn is_not_locked(&self) -> bool {
        **self == WES_A::NOTLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == WES_A::LOCKED
    }
}
impl core::ops::Deref for WES_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WES` writer - Register Write Enable status"]
pub struct WES_W<'a> {
    w: &'a mut W,
}
impl<'a> WES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WES_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_locked(self) -> &'a mut W {
        self.variant(WES_A::NOTLOCKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(WES_A::LOCKED)
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
    #[doc = "Bits 1:31 - ERW\\[31:1\\]
bits (Enable Register Writes)"]
    #[inline(always)]
    pub fn erw(&self) -> ERW_R {
        ERW_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 0 - Register Write Enable status"]
    #[inline(always)]
    pub fn wes(&self) -> WES_R {
        WES_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:31 - ERW\\[31:1\\]
bits (Enable Register Writes)"]
    #[inline(always)]
    pub fn erw(&mut self) -> ERW_W {
        ERW_W { w: self }
    }
    #[doc = "Bit 0 - Register Write Enable status"]
    #[inline(always)]
    pub fn wes(&mut self) -> WES_W {
        WES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog timer Lock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lock](index.html) module"]
pub struct LOCK_SPEC;
impl crate::RegisterSpec for LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lock::R](R) reader structure"]
impl crate::Readable for LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lock::W](W) writer structure"]
impl crate::Writable for LOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOCK to value 0"]
impl crate::Resettable for LOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
