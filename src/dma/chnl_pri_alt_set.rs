#[doc = "Register `CHNL_PRI_ALT_SET` reader"]
pub struct R(crate::R<CHNL_PRI_ALT_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHNL_PRI_ALT_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHNL_PRI_ALT_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHNL_PRI_ALT_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHNL_PRI_ALT_SET` writer"]
pub struct W(crate::W<CHNL_PRI_ALT_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHNL_PRI_ALT_SET_SPEC>;
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
impl From<crate::W<CHNL_PRI_ALT_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHNL_PRI_ALT_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHNL_PRI_ALT_SET` reader - CHNL_PRI_ALT_SET\\[5:0\\]
bits (Returns the channel control data structure status, or selects the alternate data structure for the corresponding DMA channels)"]
pub struct CHNL_PRI_ALT_SET_R(crate::FieldReader<u8>);
impl CHNL_PRI_ALT_SET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHNL_PRI_ALT_SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHNL_PRI_ALT_SET_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHNL_PRI_ALT_SET` writer - CHNL_PRI_ALT_SET\\[5:0\\]
bits (Returns the channel control data structure status, or selects the alternate data structure for the corresponding DMA channels)"]
pub struct CHNL_PRI_ALT_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CHNL_PRI_ALT_SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - CHNL_PRI_ALT_SET\\[5:0\\]
bits (Returns the channel control data structure status, or selects the alternate data structure for the corresponding DMA channels)"]
    #[inline(always)]
    pub fn chnl_pri_alt_set(&self) -> CHNL_PRI_ALT_SET_R {
        CHNL_PRI_ALT_SET_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - CHNL_PRI_ALT_SET\\[5:0\\]
bits (Returns the channel control data structure status, or selects the alternate data structure for the corresponding DMA channels)"]
    #[inline(always)]
    pub fn chnl_pri_alt_set(&mut self) -> CHNL_PRI_ALT_SET_W {
        CHNL_PRI_ALT_SET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Primary-Alterante Set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chnl_pri_alt_set](index.html) module"]
pub struct CHNL_PRI_ALT_SET_SPEC;
impl crate::RegisterSpec for CHNL_PRI_ALT_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chnl_pri_alt_set::R](R) reader structure"]
impl crate::Readable for CHNL_PRI_ALT_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chnl_pri_alt_set::W](W) writer structure"]
impl crate::Writable for CHNL_PRI_ALT_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHNL_PRI_ALT_SET to value 0"]
impl crate::Resettable for CHNL_PRI_ALT_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
