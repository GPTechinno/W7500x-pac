#[doc = "Register `WDOGCLK_HS_PVSR` reader"]
pub struct R(crate::R<WDOGCLK_HS_PVSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOGCLK_HS_PVSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOGCLK_HS_PVSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOGCLK_HS_PVSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDOGCLK_HS_PVSR` writer"]
pub struct W(crate::W<WDOGCLK_HS_PVSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOGCLK_HS_PVSR_SPEC>;
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
impl From<crate::W<WDOGCLK_HS_PVSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDOGCLK_HS_PVSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "WDOGCLK High Speed prescale value select bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WDPRE_A {
    #[doc = "0: 1/1"]
    BYPASS = 0,
    #[doc = "1: 1/2"]
    HALF = 1,
    #[doc = "2: 1/4"]
    BY4 = 2,
    #[doc = "3: 1/8"]
    BY8 = 3,
    #[doc = "4: 1/16"]
    BY16 = 4,
    #[doc = "5: 1/32"]
    BY32 = 5,
    #[doc = "6: 1/64"]
    BY64 = 6,
    #[doc = "7: 1/128"]
    BY128 = 7,
}
impl From<WDPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: WDPRE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WDPRE` reader - WDOGCLK High Speed prescale value select bits"]
pub struct WDPRE_R(crate::FieldReader<u8, WDPRE_A>);
impl WDPRE_R {
    pub(crate) fn new(bits: u8) -> Self {
        WDPRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDPRE_A {
        match self.bits {
            0 => WDPRE_A::BYPASS,
            1 => WDPRE_A::HALF,
            2 => WDPRE_A::BY4,
            3 => WDPRE_A::BY8,
            4 => WDPRE_A::BY16,
            5 => WDPRE_A::BY32,
            6 => WDPRE_A::BY64,
            7 => WDPRE_A::BY128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        **self == WDPRE_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        **self == WDPRE_A::HALF
    }
    #[doc = "Checks if the value of the field is `BY4`"]
    #[inline(always)]
    pub fn is_by4(&self) -> bool {
        **self == WDPRE_A::BY4
    }
    #[doc = "Checks if the value of the field is `BY8`"]
    #[inline(always)]
    pub fn is_by8(&self) -> bool {
        **self == WDPRE_A::BY8
    }
    #[doc = "Checks if the value of the field is `BY16`"]
    #[inline(always)]
    pub fn is_by16(&self) -> bool {
        **self == WDPRE_A::BY16
    }
    #[doc = "Checks if the value of the field is `BY32`"]
    #[inline(always)]
    pub fn is_by32(&self) -> bool {
        **self == WDPRE_A::BY32
    }
    #[doc = "Checks if the value of the field is `BY64`"]
    #[inline(always)]
    pub fn is_by64(&self) -> bool {
        **self == WDPRE_A::BY64
    }
    #[doc = "Checks if the value of the field is `BY128`"]
    #[inline(always)]
    pub fn is_by128(&self) -> bool {
        **self == WDPRE_A::BY128
    }
}
impl core::ops::Deref for WDPRE_R {
    type Target = crate::FieldReader<u8, WDPRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDPRE` writer - WDOGCLK High Speed prescale value select bits"]
pub struct WDPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> WDPRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDPRE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1/1"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(WDPRE_A::BYPASS)
    }
    #[doc = "1/2"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(WDPRE_A::HALF)
    }
    #[doc = "1/4"]
    #[inline(always)]
    pub fn by4(self) -> &'a mut W {
        self.variant(WDPRE_A::BY4)
    }
    #[doc = "1/8"]
    #[inline(always)]
    pub fn by8(self) -> &'a mut W {
        self.variant(WDPRE_A::BY8)
    }
    #[doc = "1/16"]
    #[inline(always)]
    pub fn by16(self) -> &'a mut W {
        self.variant(WDPRE_A::BY16)
    }
    #[doc = "1/32"]
    #[inline(always)]
    pub fn by32(self) -> &'a mut W {
        self.variant(WDPRE_A::BY32)
    }
    #[doc = "1/64"]
    #[inline(always)]
    pub fn by64(self) -> &'a mut W {
        self.variant(WDPRE_A::BY64)
    }
    #[doc = "1/128"]
    #[inline(always)]
    pub fn by128(self) -> &'a mut W {
        self.variant(WDPRE_A::BY128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - WDOGCLK High Speed prescale value select bits"]
    #[inline(always)]
    pub fn wdpre(&self) -> WDPRE_R {
        WDPRE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - WDOGCLK High Speed prescale value select bits"]
    #[inline(always)]
    pub fn wdpre(&mut self) -> WDPRE_W {
        WDPRE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDOGCLK High Speed prescale value select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdogclk_hs_pvsr](index.html) module"]
pub struct WDOGCLK_HS_PVSR_SPEC;
impl crate::RegisterSpec for WDOGCLK_HS_PVSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdogclk_hs_pvsr::R](R) reader structure"]
impl crate::Readable for WDOGCLK_HS_PVSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdogclk_hs_pvsr::W](W) writer structure"]
impl crate::Writable for WDOGCLK_HS_PVSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDOGCLK_HS_PVSR to value 0"]
impl crate::Resettable for WDOGCLK_HS_PVSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
