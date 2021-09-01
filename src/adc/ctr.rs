#[doc = "Register `CTR` reader"]
pub struct R(crate::R<CTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTR` writer"]
pub struct W(crate::W<CTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTR_SPEC>;
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
impl From<crate::W<CTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "power down flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWD_A {
    #[doc = "0: `0`"]
    ACTIVE = 0,
    #[doc = "1: `1`"]
    POWERDOWN = 1,
}
impl From<PWD_A> for bool {
    #[inline(always)]
    fn from(variant: PWD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWD` reader - power down flag"]
pub struct PWD_R(crate::FieldReader<bool, PWD_A>);
impl PWD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWD_A {
        match self.bits {
            false => PWD_A::ACTIVE,
            true => PWD_A::POWERDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == PWD_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `POWERDOWN`"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        **self == PWD_A::POWERDOWN
    }
}
impl core::ops::Deref for PWD_R {
    type Target = crate::FieldReader<bool, PWD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWD` writer - power down flag"]
pub struct PWD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(PWD_A::ACTIVE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(PWD_A::POWERDOWN)
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
#[doc = "Sampling mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPSEL_A {
    #[doc = "0: `0`"]
    ABNORMAL = 0,
    #[doc = "1: `1`"]
    NORMAL = 1,
}
impl From<SMPSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPSEL` reader - Sampling mode"]
pub struct SMPSEL_R(crate::FieldReader<bool, SMPSEL_A>);
impl SMPSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMPSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPSEL_A {
        match self.bits {
            false => SMPSEL_A::ABNORMAL,
            true => SMPSEL_A::NORMAL,
        }
    }
    #[doc = "Checks if the value of the field is `ABNORMAL`"]
    #[inline(always)]
    pub fn is_abnormal(&self) -> bool {
        **self == SMPSEL_A::ABNORMAL
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == SMPSEL_A::NORMAL
    }
}
impl core::ops::Deref for SMPSEL_R {
    type Target = crate::FieldReader<bool, SMPSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMPSEL` writer - Sampling mode"]
pub struct SMPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn abnormal(self) -> &'a mut W {
        self.variant(SMPSEL_A::ABNORMAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SMPSEL_A::NORMAL)
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
    #[doc = "Bit 1 - power down flag"]
    #[inline(always)]
    pub fn pwd(&self) -> PWD_R {
        PWD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Sampling mode"]
    #[inline(always)]
    pub fn smpsel(&self) -> SMPSEL_R {
        SMPSEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - power down flag"]
    #[inline(always)]
    pub fn pwd(&mut self) -> PWD_W {
        PWD_W { w: self }
    }
    #[doc = "Bit 0 - Sampling mode"]
    #[inline(always)]
    pub fn smpsel(&mut self) -> SMPSEL_W {
        SMPSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctr](index.html) module"]
pub struct CTR_SPEC;
impl crate::RegisterSpec for CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctr::R](R) reader structure"]
impl crate::Readable for CTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctr::W](W) writer structure"]
impl crate::Writable for CTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTR to value 0x03"]
impl crate::Resettable for CTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
