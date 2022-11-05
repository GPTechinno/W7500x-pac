#[doc = "Register `OSC_PDR` reader"]
pub struct R(crate::R<OSC_PDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC_PDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC_PDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC_PDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC_PDR` writer"]
pub struct W(crate::W<OSC_PDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSC_PDR_SPEC>;
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
impl From<crate::W<OSC_PDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSC_PDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Internal 8MHz RC oscillator power down flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCPD_A {
    #[doc = "0: `0`"]
    NORMAL = 0,
    #[doc = "1: `1`"]
    POWERDOWN = 1,
}
impl From<OSCPD_A> for bool {
    #[inline(always)]
    fn from(variant: OSCPD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCPD` reader - Internal 8MHz RC oscillator power down flag"]
pub struct OSCPD_R(crate::FieldReader<bool>);
impl OSCPD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSCPD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCPD_A {
        match self.bits {
            false => OSCPD_A::NORMAL,
            true => OSCPD_A::POWERDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == OSCPD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `POWERDOWN`"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        **self == OSCPD_A::POWERDOWN
    }
}
impl core::ops::Deref for OSCPD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSCPD` writer - Internal 8MHz RC oscillator power down flag"]
pub struct OSCPD_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCPD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(OSCPD_A::NORMAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(OSCPD_A::POWERDOWN)
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
    #[doc = "Bit 0 - Internal 8MHz RC oscillator power down flag"]
    #[inline(always)]
    pub fn oscpd(&self) -> OSCPD_R {
        OSCPD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal 8MHz RC oscillator power down flag"]
    #[inline(always)]
    pub fn oscpd(&mut self) -> OSCPD_W {
        OSCPD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscillator power down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_pdr](index.html) module"]
pub struct OSC_PDR_SPEC;
impl crate::RegisterSpec for OSC_PDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osc_pdr::R](R) reader structure"]
impl crate::Readable for OSC_PDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc_pdr::W](W) writer structure"]
impl crate::Writable for OSC_PDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSC_PDR to value 0"]
impl crate::Resettable for OSC_PDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
