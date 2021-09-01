#[doc = "Register `SSPCLK_PVSR` reader"]
pub struct R(crate::R<SSPCLK_PVSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSPCLK_PVSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSPCLK_PVSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSPCLK_PVSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSPCLK_PVSR` writer"]
pub struct W(crate::W<SSPCLK_PVSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSPCLK_PVSR_SPEC>;
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
impl From<crate::W<SSPCLK_PVSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSPCLK_PVSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SSPCLK prescale value select bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSPCP_A {
    #[doc = "0: 1/1"]
    BYPASS = 0,
    #[doc = "1: 1/2"]
    HALF = 1,
    #[doc = "2: 1/4"]
    BY4 = 2,
    #[doc = "3: 1/8"]
    BY8 = 3,
}
impl From<SSPCP_A> for u8 {
    #[inline(always)]
    fn from(variant: SSPCP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SSPCP` reader - SSPCLK prescale value select bits"]
pub struct SSPCP_R(crate::FieldReader<u8, SSPCP_A>);
impl SSPCP_R {
    pub(crate) fn new(bits: u8) -> Self {
        SSPCP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSPCP_A {
        match self.bits {
            0 => SSPCP_A::BYPASS,
            1 => SSPCP_A::HALF,
            2 => SSPCP_A::BY4,
            3 => SSPCP_A::BY8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        **self == SSPCP_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        **self == SSPCP_A::HALF
    }
    #[doc = "Checks if the value of the field is `BY4`"]
    #[inline(always)]
    pub fn is_by4(&self) -> bool {
        **self == SSPCP_A::BY4
    }
    #[doc = "Checks if the value of the field is `BY8`"]
    #[inline(always)]
    pub fn is_by8(&self) -> bool {
        **self == SSPCP_A::BY8
    }
}
impl core::ops::Deref for SSPCP_R {
    type Target = crate::FieldReader<u8, SSPCP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSPCP` writer - SSPCLK prescale value select bits"]
pub struct SSPCP_W<'a> {
    w: &'a mut W,
}
impl<'a> SSPCP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSPCP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1/1"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(SSPCP_A::BYPASS)
    }
    #[doc = "1/2"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(SSPCP_A::HALF)
    }
    #[doc = "1/4"]
    #[inline(always)]
    pub fn by4(self) -> &'a mut W {
        self.variant(SSPCP_A::BY4)
    }
    #[doc = "1/8"]
    #[inline(always)]
    pub fn by8(self) -> &'a mut W {
        self.variant(SSPCP_A::BY8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - SSPCLK prescale value select bits"]
    #[inline(always)]
    pub fn sspcp(&self) -> SSPCP_R {
        SSPCP_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SSPCLK prescale value select bits"]
    #[inline(always)]
    pub fn sspcp(&mut self) -> SSPCP_W {
        SSPCP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SSPCLK prescale value select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sspclk_pvsr](index.html) module"]
pub struct SSPCLK_PVSR_SPEC;
impl crate::RegisterSpec for SSPCLK_PVSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sspclk_pvsr::R](R) reader structure"]
impl crate::Readable for SSPCLK_PVSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sspclk_pvsr::W](W) writer structure"]
impl crate::Writable for SSPCLK_PVSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSPCLK_PVSR to value 0"]
impl crate::Resettable for SSPCLK_PVSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
