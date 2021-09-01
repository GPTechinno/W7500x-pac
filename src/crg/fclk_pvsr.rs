#[doc = "Register `FCLK_PVSR` reader"]
pub struct R(crate::R<FCLK_PVSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCLK_PVSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCLK_PVSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCLK_PVSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCLK_PVSR` writer"]
pub struct W(crate::W<FCLK_PVSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCLK_PVSR_SPEC>;
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
impl From<crate::W<FCLK_PVSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCLK_PVSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FCLK prescale value select bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FCKPRE_A {
    #[doc = "0: 1/1"]
    BYPASS = 0,
    #[doc = "1: 1/2"]
    HALF = 1,
    #[doc = "2: 1/4"]
    BY4 = 2,
    #[doc = "3: 1/8"]
    BY8 = 3,
}
impl From<FCKPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: FCKPRE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FCKPRE` reader - FCLK prescale value select bits"]
pub struct FCKPRE_R(crate::FieldReader<u8, FCKPRE_A>);
impl FCKPRE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FCKPRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCKPRE_A {
        match self.bits {
            0 => FCKPRE_A::BYPASS,
            1 => FCKPRE_A::HALF,
            2 => FCKPRE_A::BY4,
            3 => FCKPRE_A::BY8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        **self == FCKPRE_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        **self == FCKPRE_A::HALF
    }
    #[doc = "Checks if the value of the field is `BY4`"]
    #[inline(always)]
    pub fn is_by4(&self) -> bool {
        **self == FCKPRE_A::BY4
    }
    #[doc = "Checks if the value of the field is `BY8`"]
    #[inline(always)]
    pub fn is_by8(&self) -> bool {
        **self == FCKPRE_A::BY8
    }
}
impl core::ops::Deref for FCKPRE_R {
    type Target = crate::FieldReader<u8, FCKPRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCKPRE` writer - FCLK prescale value select bits"]
pub struct FCKPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> FCKPRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCKPRE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1/1"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(FCKPRE_A::BYPASS)
    }
    #[doc = "1/2"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(FCKPRE_A::HALF)
    }
    #[doc = "1/4"]
    #[inline(always)]
    pub fn by4(self) -> &'a mut W {
        self.variant(FCKPRE_A::BY4)
    }
    #[doc = "1/8"]
    #[inline(always)]
    pub fn by8(self) -> &'a mut W {
        self.variant(FCKPRE_A::BY8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - FCLK prescale value select bits"]
    #[inline(always)]
    pub fn fckpre(&self) -> FCKPRE_R {
        FCKPRE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - FCLK prescale value select bits"]
    #[inline(always)]
    pub fn fckpre(&mut self) -> FCKPRE_W {
        FCKPRE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FCLK prescale value select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fclk_pvsr](index.html) module"]
pub struct FCLK_PVSR_SPEC;
impl crate::RegisterSpec for FCLK_PVSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fclk_pvsr::R](R) reader structure"]
impl crate::Readable for FCLK_PVSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fclk_pvsr::W](W) writer structure"]
impl crate::Writable for FCLK_PVSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCLK_PVSR to value 0"]
impl crate::Resettable for FCLK_PVSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
