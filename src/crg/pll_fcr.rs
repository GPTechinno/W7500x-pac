#[doc = "Register `PLL_FCR` reader"]
pub struct R(crate::R<PLL_FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_FCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_FCR` writer"]
pub struct W(crate::W<PLL_FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_FCR_SPEC>;
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
impl From<crate::W<PLL_FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M` reader - Loop divider control bits"]
pub struct M_R(crate::FieldReader<u8>);
impl M_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M` writer - Loop divider control bits"]
pub struct M_W<'a> {
    w: &'a mut W,
}
impl<'a> M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `N` reader - Pre divider control bits"]
pub struct N_R(crate::FieldReader<u8>);
impl N_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for N_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `N` writer - Pre divider control bits"]
pub struct N_W<'a> {
    w: &'a mut W,
}
impl<'a> N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `OD` reader - Output divider control bits"]
pub struct OD_R(crate::FieldReader<u8>);
impl OD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OD_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OD` writer - Output divider control bits"]
pub struct OD_W<'a> {
    w: &'a mut W,
}
impl<'a> OD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:21 - Loop divider control bits"]
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Pre divider control bits"]
    #[inline(always)]
    pub fn n(&self) -> N_R {
        N_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:1 - Output divider control bits"]
    #[inline(always)]
    pub fn od(&self) -> OD_R {
        OD_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 16:21 - Loop divider control bits"]
    #[inline(always)]
    pub fn m(&mut self) -> M_W {
        M_W { w: self }
    }
    #[doc = "Bits 8:13 - Pre divider control bits"]
    #[inline(always)]
    pub fn n(&mut self) -> N_W {
        N_W { w: self }
    }
    #[doc = "Bits 0:1 - Output divider control bits"]
    #[inline(always)]
    pub fn od(&mut self) -> OD_W {
        OD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL frequency calculating register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_fcr](index.html) module"]
pub struct PLL_FCR_SPEC;
impl crate::RegisterSpec for PLL_FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_fcr::R](R) reader structure"]
impl crate::Readable for PLL_FCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_fcr::W](W) writer structure"]
impl crate::Writable for PLL_FCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL_FCR to value 0x0005_0200"]
impl crate::Resettable for PLL_FCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0005_0200
    }
}
