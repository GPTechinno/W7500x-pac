#[doc = "Register `RESETOP` reader"]
pub struct R(crate::R<RESETOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESETOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESETOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESETOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESETOP` writer"]
pub struct W(crate::W<RESETOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESETOP_SPEC>;
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
impl From<crate::W<RESETOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESETOP_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Option Register (R/W)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resetop](index.html) module"]
pub struct RESETOP_SPEC;
impl crate::RegisterSpec for RESETOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resetop::R](R) reader structure"]
impl crate::Readable for RESETOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [resetop::W](W) writer structure"]
impl crate::Writable for RESETOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESETOP to value 0"]
impl crate::Resettable for RESETOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
