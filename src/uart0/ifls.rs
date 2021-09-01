#[doc = "Register `IFLS` reader"]
pub struct R(crate::R<IFLS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFLS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFLS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFLS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFLS` writer"]
pub struct W(crate::W<IFLS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFLS_SPEC>;
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
impl From<crate::W<IFLS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFLS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receive interrupt FIFO level select\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXIFLSEL_A {
    #[doc = "0: `0`"]
    FULL1_8 = 0,
    #[doc = "1: `1`"]
    FULL1_4 = 1,
    #[doc = "2: `10`"]
    FULL1_2 = 2,
    #[doc = "3: `11`"]
    FULL3_4 = 3,
    #[doc = "4: `100`"]
    FULL7_8 = 4,
}
impl From<RXIFLSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RXIFLSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RXIFLSEL` reader - Receive interrupt FIFO level select"]
pub struct RXIFLSEL_R(crate::FieldReader<u8, RXIFLSEL_A>);
impl RXIFLSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXIFLSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXIFLSEL_A> {
        match self.bits {
            0 => Some(RXIFLSEL_A::FULL1_8),
            1 => Some(RXIFLSEL_A::FULL1_4),
            2 => Some(RXIFLSEL_A::FULL1_2),
            3 => Some(RXIFLSEL_A::FULL3_4),
            4 => Some(RXIFLSEL_A::FULL7_8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FULL1_8`"]
    #[inline(always)]
    pub fn is_full1_8(&self) -> bool {
        **self == RXIFLSEL_A::FULL1_8
    }
    #[doc = "Checks if the value of the field is `FULL1_4`"]
    #[inline(always)]
    pub fn is_full1_4(&self) -> bool {
        **self == RXIFLSEL_A::FULL1_4
    }
    #[doc = "Checks if the value of the field is `FULL1_2`"]
    #[inline(always)]
    pub fn is_full1_2(&self) -> bool {
        **self == RXIFLSEL_A::FULL1_2
    }
    #[doc = "Checks if the value of the field is `FULL3_4`"]
    #[inline(always)]
    pub fn is_full3_4(&self) -> bool {
        **self == RXIFLSEL_A::FULL3_4
    }
    #[doc = "Checks if the value of the field is `FULL7_8`"]
    #[inline(always)]
    pub fn is_full7_8(&self) -> bool {
        **self == RXIFLSEL_A::FULL7_8
    }
}
impl core::ops::Deref for RXIFLSEL_R {
    type Target = crate::FieldReader<u8, RXIFLSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIFLSEL` writer - Receive interrupt FIFO level select"]
pub struct RXIFLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIFLSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXIFLSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn full1_8(self) -> &'a mut W {
        self.variant(RXIFLSEL_A::FULL1_8)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn full1_4(self) -> &'a mut W {
        self.variant(RXIFLSEL_A::FULL1_4)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn full1_2(self) -> &'a mut W {
        self.variant(RXIFLSEL_A::FULL1_2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn full3_4(self) -> &'a mut W {
        self.variant(RXIFLSEL_A::FULL3_4)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn full7_8(self) -> &'a mut W {
        self.variant(RXIFLSEL_A::FULL7_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Transmit interrupt FIFO level select\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXIFLSEL_A {
    #[doc = "0: `0`"]
    FULL1_8 = 0,
    #[doc = "1: `1`"]
    FULL1_4 = 1,
    #[doc = "2: `10`"]
    FULL1_2 = 2,
    #[doc = "3: `11`"]
    FULL3_4 = 3,
    #[doc = "4: `100`"]
    FULL7_8 = 4,
}
impl From<TXIFLSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TXIFLSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TXIFLSEL` reader - Transmit interrupt FIFO level select"]
pub struct TXIFLSEL_R(crate::FieldReader<u8, TXIFLSEL_A>);
impl TXIFLSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXIFLSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXIFLSEL_A> {
        match self.bits {
            0 => Some(TXIFLSEL_A::FULL1_8),
            1 => Some(TXIFLSEL_A::FULL1_4),
            2 => Some(TXIFLSEL_A::FULL1_2),
            3 => Some(TXIFLSEL_A::FULL3_4),
            4 => Some(TXIFLSEL_A::FULL7_8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FULL1_8`"]
    #[inline(always)]
    pub fn is_full1_8(&self) -> bool {
        **self == TXIFLSEL_A::FULL1_8
    }
    #[doc = "Checks if the value of the field is `FULL1_4`"]
    #[inline(always)]
    pub fn is_full1_4(&self) -> bool {
        **self == TXIFLSEL_A::FULL1_4
    }
    #[doc = "Checks if the value of the field is `FULL1_2`"]
    #[inline(always)]
    pub fn is_full1_2(&self) -> bool {
        **self == TXIFLSEL_A::FULL1_2
    }
    #[doc = "Checks if the value of the field is `FULL3_4`"]
    #[inline(always)]
    pub fn is_full3_4(&self) -> bool {
        **self == TXIFLSEL_A::FULL3_4
    }
    #[doc = "Checks if the value of the field is `FULL7_8`"]
    #[inline(always)]
    pub fn is_full7_8(&self) -> bool {
        **self == TXIFLSEL_A::FULL7_8
    }
}
impl core::ops::Deref for TXIFLSEL_R {
    type Target = crate::FieldReader<u8, TXIFLSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXIFLSEL` writer - Transmit interrupt FIFO level select"]
pub struct TXIFLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIFLSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXIFLSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn full1_8(self) -> &'a mut W {
        self.variant(TXIFLSEL_A::FULL1_8)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn full1_4(self) -> &'a mut W {
        self.variant(TXIFLSEL_A::FULL1_4)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn full1_2(self) -> &'a mut W {
        self.variant(TXIFLSEL_A::FULL1_2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn full3_4(self) -> &'a mut W {
        self.variant(TXIFLSEL_A::FULL3_4)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn full7_8(self) -> &'a mut W {
        self.variant(TXIFLSEL_A::FULL7_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:5 - Receive interrupt FIFO level select"]
    #[inline(always)]
    pub fn rxiflsel(&self) -> RXIFLSEL_R {
        RXIFLSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - Transmit interrupt FIFO level select"]
    #[inline(always)]
    pub fn txiflsel(&self) -> TXIFLSEL_R {
        TXIFLSEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 3:5 - Receive interrupt FIFO level select"]
    #[inline(always)]
    pub fn rxiflsel(&mut self) -> RXIFLSEL_W {
        RXIFLSEL_W { w: self }
    }
    #[doc = "Bits 0:2 - Transmit interrupt FIFO level select"]
    #[inline(always)]
    pub fn txiflsel(&mut self) -> TXIFLSEL_W {
        TXIFLSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt FIFO Level Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifls](index.html) module"]
pub struct IFLS_SPEC;
impl crate::RegisterSpec for IFLS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ifls::R](R) reader structure"]
impl crate::Readable for IFLS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ifls::W](W) writer structure"]
impl crate::Writable for IFLS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFLS to value 0x12"]
impl crate::Resettable for IFLS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x12
    }
}
