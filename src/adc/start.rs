#[doc = "Register `START` writer"]
pub struct W(crate::W<START_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<START_SPEC>;
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
impl From<crate::W<START_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<START_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Start ADC for conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_SRT_AW {
    #[doc = "0: `0`"]
    READY = 0,
    #[doc = "1: This bit clear automatically after conversion"]
    START = 1,
}
impl From<ADC_SRT_AW> for bool {
    #[inline(always)]
    fn from(variant: ADC_SRT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_SRT` writer - Start ADC for conversion"]
pub struct ADC_SRT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SRT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_SRT_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(ADC_SRT_AW::READY)
    }
    #[doc = "This bit clear automatically after conversion"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(ADC_SRT_AW::START)
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
impl W {
    #[doc = "Bit 0 - Start ADC for conversion"]
    #[inline(always)]
    pub fn adc_srt(&mut self) -> ADC_SRT_W {
        ADC_SRT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC start register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [start](index.html) module"]
pub struct START_SPEC;
impl crate::RegisterSpec for START_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [start::W](W) writer structure"]
impl crate::Writable for START_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets START to value 0"]
impl crate::Resettable for START_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
