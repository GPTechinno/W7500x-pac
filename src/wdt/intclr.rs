#[doc = "Register `INTCLR` writer"]
pub struct W(crate::W<INTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTCLR_SPEC>;
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
impl From<crate::W<INTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Watchdog timer Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIC_AW {
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<WIC_AW> for bool {
    #[inline(always)]
    fn from(variant: WIC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIC` writer - Watchdog timer Interrupt Clear"]
pub struct WIC_W<'a> {
    w: &'a mut W,
}
impl<'a> WIC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WIC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(WIC_AW::CLEAR)
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
impl W {
    #[doc = "Bit 0 - Watchdog timer Interrupt Clear"]
    #[inline(always)]
    pub fn wic(&mut self) -> WIC_W {
        WIC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog timer Clear Interrupt register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intclr](index.html) module"]
pub struct INTCLR_SPEC;
impl crate::RegisterSpec for INTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [intclr::W](W) writer structure"]
impl crate::Writable for INTCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTCLR to value 0"]
impl crate::Resettable for INTCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
