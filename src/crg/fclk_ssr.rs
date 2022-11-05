#[doc = "Register `FCLK_SSR` reader"]
pub struct R(crate::R<FCLK_SSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCLK_SSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCLK_SSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCLK_SSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCLK_SSR` writer"]
pub struct W(crate::W<FCLK_SSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCLK_SSR_SPEC>;
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
impl From<crate::W<FCLK_SSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCLK_SSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FCLK source select bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FCKSRC_A {
    #[doc = "2: Internal 8MHz RC oscillator clock (RCLK)"]
    INTERNAL = 2,
    #[doc = "3: External oscillator clock (OCLK, 8MHz-24MHz)"]
    EXTERNAL = 3,
}
impl From<FCKSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: FCKSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FCKSRC` reader - FCLK source select bits"]
pub struct FCKSRC_R(crate::FieldReader<u8>);
impl FCKSRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FCKSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FCKSRC_A> {
        match self.bits {
            2 => Some(FCKSRC_A::INTERNAL),
            3 => Some(FCKSRC_A::EXTERNAL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        **self == FCKSRC_A::INTERNAL
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        **self == FCKSRC_A::EXTERNAL
    }
}
impl core::ops::Deref for FCKSRC_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCKSRC` writer - FCLK source select bits"]
pub struct FCKSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FCKSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCKSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Internal 8MHz RC oscillator clock (RCLK)"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(FCKSRC_A::INTERNAL)
    }
    #[doc = "External oscillator clock (OCLK, 8MHz-24MHz)"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(FCKSRC_A::EXTERNAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - FCLK source select bits"]
    #[inline(always)]
    pub fn fcksrc(&self) -> FCKSRC_R {
        FCKSRC_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - FCLK source select bits"]
    #[inline(always)]
    pub fn fcksrc(&mut self) -> FCKSRC_W {
        FCKSRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FCLK source select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fclk_ssr](index.html) module"]
pub struct FCLK_SSR_SPEC;
impl crate::RegisterSpec for FCLK_SSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fclk_ssr::R](R) reader structure"]
impl crate::Readable for FCLK_SSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fclk_ssr::W](W) writer structure"]
impl crate::Writable for FCLK_SSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCLK_SSR to value 0x01"]
impl crate::Resettable for FCLK_SSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
