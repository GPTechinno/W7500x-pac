#[doc = "Register `MIICLK_ECR` reader"]
pub struct R(crate::R<MIICLK_ECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIICLK_ECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIICLK_ECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIICLK_ECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIICLK_ECR` writer"]
pub struct W(crate::W<MIICLK_ECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIICLK_ECR_SPEC>;
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
impl From<crate::W<MIICLK_ECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIICLK_ECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN_MIITEN` reader - MII TX Clock source enable flag"]
pub struct EN_MIITEN_R(crate::FieldReader<bool>);
impl EN_MIITEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_MIITEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_MIITEN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN_MIITEN` writer - MII TX Clock source enable flag"]
pub struct EN_MIITEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_MIITEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `EN_MIIREN` reader - MII RX Clock source enable register flag"]
pub struct EN_MIIREN_R(crate::FieldReader<bool>);
impl EN_MIIREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_MIIREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_MIIREN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN_MIIREN` writer - MII RX Clock source enable register flag"]
pub struct EN_MIIREN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_MIIREN_W<'a> {
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
    #[doc = "Bit 1 - MII TX Clock source enable flag"]
    #[inline(always)]
    pub fn en_miiten(&self) -> EN_MIITEN_R {
        EN_MIITEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - MII RX Clock source enable register flag"]
    #[inline(always)]
    pub fn en_miiren(&self) -> EN_MIIREN_R {
        EN_MIIREN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - MII TX Clock source enable flag"]
    #[inline(always)]
    pub fn en_miiten(&mut self) -> EN_MIITEN_W {
        EN_MIITEN_W { w: self }
    }
    #[doc = "Bit 0 - MII RX Clock source enable register flag"]
    #[inline(always)]
    pub fn en_miiren(&mut self) -> EN_MIIREN_W {
        EN_MIIREN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MII clock enable control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miiclk_ecr](index.html) module"]
pub struct MIICLK_ECR_SPEC;
impl crate::RegisterSpec for MIICLK_ECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [miiclk_ecr::R](R) reader structure"]
impl crate::Readable for MIICLK_ECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [miiclk_ecr::W](W) writer structure"]
impl crate::Writable for MIICLK_ECR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIICLK_ECR to value 0x03"]
impl crate::Resettable for MIICLK_ECR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
