#[doc = "Register `LCR_H` reader"]
pub struct R(crate::R<LCR_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCR_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCR_H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCR_H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCR_H` writer"]
pub struct W(crate::W<LCR_H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCR_H_SPEC>;
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
impl From<crate::W<LCR_H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCR_H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Stick parity select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPS_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<SPS_A> for bool {
    #[inline(always)]
    fn from(variant: SPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPS` reader - Stick parity select"]
pub struct SPS_R(crate::FieldReader<bool, SPS_A>);
impl SPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPS_A {
        match self.bits {
            false => SPS_A::DISABLE,
            true => SPS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SPS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SPS_A::ENABLE
    }
}
impl core::ops::Deref for SPS_R {
    type Target = crate::FieldReader<bool, SPS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPS` writer - Stick parity select"]
pub struct SPS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SPS_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SPS_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Word length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WLEN_A {
    #[doc = "0: `0`"]
    _5BITS = 0,
    #[doc = "1: `1`"]
    _6BITS = 1,
    #[doc = "2: `10`"]
    _7BITS = 2,
    #[doc = "3: `11`"]
    _8BITS = 3,
}
impl From<WLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: WLEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WLEN` reader - Word length"]
pub struct WLEN_R(crate::FieldReader<u8, WLEN_A>);
impl WLEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        WLEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WLEN_A {
        match self.bits {
            0 => WLEN_A::_5BITS,
            1 => WLEN_A::_6BITS,
            2 => WLEN_A::_7BITS,
            3 => WLEN_A::_8BITS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_5BITS`"]
    #[inline(always)]
    pub fn is_5bits(&self) -> bool {
        **self == WLEN_A::_5BITS
    }
    #[doc = "Checks if the value of the field is `_6BITS`"]
    #[inline(always)]
    pub fn is_6bits(&self) -> bool {
        **self == WLEN_A::_6BITS
    }
    #[doc = "Checks if the value of the field is `_7BITS`"]
    #[inline(always)]
    pub fn is_7bits(&self) -> bool {
        **self == WLEN_A::_7BITS
    }
    #[doc = "Checks if the value of the field is `_8BITS`"]
    #[inline(always)]
    pub fn is_8bits(&self) -> bool {
        **self == WLEN_A::_8BITS
    }
}
impl core::ops::Deref for WLEN_R {
    type Target = crate::FieldReader<u8, WLEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WLEN` writer - Word length"]
pub struct WLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WLEN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _5bits(self) -> &'a mut W {
        self.variant(WLEN_A::_5BITS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _6bits(self) -> &'a mut W {
        self.variant(WLEN_A::_6BITS)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _7bits(self) -> &'a mut W {
        self.variant(WLEN_A::_7BITS)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _8bits(self) -> &'a mut W {
        self.variant(WLEN_A::_8BITS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Enable FIFOs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<FEN_A> for bool {
    #[inline(always)]
    fn from(variant: FEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEN` reader - Enable FIFOs"]
pub struct FEN_R(crate::FieldReader<bool, FEN_A>);
impl FEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEN_A {
        match self.bits {
            false => FEN_A::DISABLE,
            true => FEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FEN_A::ENABLE
    }
}
impl core::ops::Deref for FEN_R {
    type Target = crate::FieldReader<bool, FEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEN` writer - Enable FIFOs"]
pub struct FEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FEN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Even parity select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPS_A {
    #[doc = "0: `0`"]
    ODD = 0,
    #[doc = "1: `1`"]
    EVEN = 1,
}
impl From<EPS_A> for bool {
    #[inline(always)]
    fn from(variant: EPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPS` reader - Even parity select"]
pub struct EPS_R(crate::FieldReader<bool, EPS_A>);
impl EPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPS_A {
        match self.bits {
            false => EPS_A::ODD,
            true => EPS_A::EVEN,
        }
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        **self == EPS_A::ODD
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        **self == EPS_A::EVEN
    }
}
impl core::ops::Deref for EPS_R {
    type Target = crate::FieldReader<bool, EPS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPS` writer - Even parity select"]
pub struct EPS_W<'a> {
    w: &'a mut W,
}
impl<'a> EPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(EPS_A::ODD)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(EPS_A::EVEN)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Parity enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<PEN_A> for bool {
    #[inline(always)]
    fn from(variant: PEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEN` reader - Parity enable"]
pub struct PEN_R(crate::FieldReader<bool, PEN_A>);
impl PEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEN_A {
        match self.bits {
            false => PEN_A::DISABLE,
            true => PEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == PEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == PEN_A::ENABLE
    }
}
impl core::ops::Deref for PEN_R {
    type Target = crate::FieldReader<bool, PEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEN` writer - Parity enable"]
pub struct PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PEN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Send break\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRK_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<BRK_A> for bool {
    #[inline(always)]
    fn from(variant: BRK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRK` reader - Send break"]
pub struct BRK_R(crate::FieldReader<bool, BRK_A>);
impl BRK_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRK_A {
        match self.bits {
            false => BRK_A::DISABLE,
            true => BRK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == BRK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == BRK_A::ENABLE
    }
}
impl core::ops::Deref for BRK_R {
    type Target = crate::FieldReader<bool, BRK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRK` writer - Send break"]
pub struct BRK_W<'a> {
    w: &'a mut W,
}
impl<'a> BRK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BRK_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BRK_A::ENABLE)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - Stick parity select"]
    #[inline(always)]
    pub fn sps(&self) -> SPS_R {
        SPS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Word length"]
    #[inline(always)]
    pub fn wlen(&self) -> WLEN_R {
        WLEN_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Enable FIFOs"]
    #[inline(always)]
    pub fn fen(&self) -> FEN_R {
        FEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Even parity select"]
    #[inline(always)]
    pub fn eps(&self) -> EPS_R {
        EPS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Parity enable"]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Send break"]
    #[inline(always)]
    pub fn brk(&self) -> BRK_R {
        BRK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Stick parity select"]
    #[inline(always)]
    pub fn sps(&mut self) -> SPS_W {
        SPS_W { w: self }
    }
    #[doc = "Bits 5:6 - Word length"]
    #[inline(always)]
    pub fn wlen(&mut self) -> WLEN_W {
        WLEN_W { w: self }
    }
    #[doc = "Bit 4 - Enable FIFOs"]
    #[inline(always)]
    pub fn fen(&mut self) -> FEN_W {
        FEN_W { w: self }
    }
    #[doc = "Bit 2 - Even parity select"]
    #[inline(always)]
    pub fn eps(&mut self) -> EPS_W {
        EPS_W { w: self }
    }
    #[doc = "Bit 1 - Parity enable"]
    #[inline(always)]
    pub fn pen(&mut self) -> PEN_W {
        PEN_W { w: self }
    }
    #[doc = "Bit 0 - Send break"]
    #[inline(always)]
    pub fn brk(&mut self) -> BRK_W {
        BRK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Line Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcr_h](index.html) module"]
pub struct LCR_H_SPEC;
impl crate::RegisterSpec for LCR_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcr_h::R](R) reader structure"]
impl crate::Readable for LCR_H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcr_h::W](W) writer structure"]
impl crate::Writable for LCR_H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCR_H to value 0"]
impl crate::Resettable for LCR_H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
