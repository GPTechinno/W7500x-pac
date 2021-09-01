#[doc = "Register `CHNL_SW_REQUEST` writer"]
pub struct W(crate::W<CHNL_SW_REQUEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHNL_SW_REQUEST_SPEC>;
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
impl From<crate::W<CHNL_SW_REQUEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHNL_SW_REQUEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHNL_SW_REQUEST` writer - Set the appropriate bit to generate a software DMA request"]
pub struct CHNL_SW_REQUEST_W<'a> {
    w: &'a mut W,
}
impl<'a> CHNL_SW_REQUEST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:5 - Set the appropriate bit to generate a software DMA request"]
    #[inline(always)]
    pub fn chnl_sw_request(&mut self) -> CHNL_SW_REQUEST_W {
        CHNL_SW_REQUEST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Software Request register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chnl_sw_request](index.html) module"]
pub struct CHNL_SW_REQUEST_SPEC;
impl crate::RegisterSpec for CHNL_SW_REQUEST_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chnl_sw_request::W](W) writer structure"]
impl crate::Writable for CHNL_SW_REQUEST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHNL_SW_REQUEST to value 0"]
impl crate::Resettable for CHNL_SW_REQUEST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
