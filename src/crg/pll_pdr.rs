#[doc = "Register `PLL_PDR` reader"]
pub struct R(crate::R<PLL_PDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_PDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_PDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_PDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_PDR` writer"]
pub struct W(crate::W<PLL_PDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_PDR_SPEC>;
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
impl From<crate::W<PLL_PDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_PDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PLL power down flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLPD_A {
    #[doc = "0: `0`"]
    POWERDOWN = 0,
    #[doc = "1: `1`"]
    NORMAL = 1,
}
impl From<PLLPD_A> for bool {
    #[inline(always)]
    fn from(variant: PLLPD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLPD` reader - PLL power down flag"]
pub struct PLLPD_R(crate::FieldReader<bool, PLLPD_A>);
impl PLLPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLPD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLPD_A {
        match self.bits {
            false => PLLPD_A::POWERDOWN,
            true => PLLPD_A::NORMAL,
        }
    }
    #[doc = "Checks if the value of the field is `POWERDOWN`"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        **self == PLLPD_A::POWERDOWN
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == PLLPD_A::NORMAL
    }
}
impl core::ops::Deref for PLLPD_R {
    type Target = crate::FieldReader<bool, PLLPD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLPD` writer - PLL power down flag"]
pub struct PLLPD_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLPD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(PLLPD_A::POWERDOWN)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(PLLPD_A::NORMAL)
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
    #[doc = "Bit 0 - PLL power down flag"]
    #[inline(always)]
    pub fn pllpd(&self) -> PLLPD_R {
        PLLPD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL power down flag"]
    #[inline(always)]
    pub fn pllpd(&mut self) -> PLLPD_W {
        PLLPD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL power down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_pdr](index.html) module"]
pub struct PLL_PDR_SPEC;
impl crate::RegisterSpec for PLL_PDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_pdr::R](R) reader structure"]
impl crate::Readable for PLL_PDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_pdr::W](W) writer structure"]
impl crate::Writable for PLL_PDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL_PDR to value 0x01"]
impl crate::Resettable for PLL_PDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
