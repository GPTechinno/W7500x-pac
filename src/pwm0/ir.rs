#[doc = "Register `IR` reader"]
pub struct R(crate::R<IR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Capture Interrupt occurs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CI_A {
    #[doc = "0: `0`"]
    NOTOCCURED = 0,
    #[doc = "1: `1`"]
    OCCURED = 1,
}
impl From<CI_A> for bool {
    #[inline(always)]
    fn from(variant: CI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CI` reader - Capture Interrupt occurs"]
pub struct CI_R(crate::FieldReader<bool>);
impl CI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CI_A {
        match self.bits {
            false => CI_A::NOTOCCURED,
            true => CI_A::OCCURED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTOCCURED`"]
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        **self == CI_A::NOTOCCURED
    }
    #[doc = "Checks if the value of the field is `OCCURED`"]
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        **self == CI_A::OCCURED
    }
}
impl core::ops::Deref for CI_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Overflow Interrupt occurs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OI_A {
    #[doc = "0: `0`"]
    NOTOCCURED = 0,
    #[doc = "1: `1`"]
    OCCURED = 1,
}
impl From<OI_A> for bool {
    #[inline(always)]
    fn from(variant: OI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OI` reader - Overflow Interrupt occurs"]
pub struct OI_R(crate::FieldReader<bool>);
impl OI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OI_A {
        match self.bits {
            false => OI_A::NOTOCCURED,
            true => OI_A::OCCURED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTOCCURED`"]
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        **self == OI_A::NOTOCCURED
    }
    #[doc = "Checks if the value of the field is `OCCURED`"]
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        **self == OI_A::OCCURED
    }
}
impl core::ops::Deref for OI_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Match Interrupt occurs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI_A {
    #[doc = "0: `0`"]
    NOTOCCURED = 0,
    #[doc = "1: `1`"]
    OCCURED = 1,
}
impl From<MI_A> for bool {
    #[inline(always)]
    fn from(variant: MI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MI` reader - Match Interrupt occurs"]
pub struct MI_R(crate::FieldReader<bool>);
impl MI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI_A {
        match self.bits {
            false => MI_A::NOTOCCURED,
            true => MI_A::OCCURED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTOCCURED`"]
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        **self == MI_A::NOTOCCURED
    }
    #[doc = "Checks if the value of the field is `OCCURED`"]
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        **self == MI_A::OCCURED
    }
}
impl core::ops::Deref for MI_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 2 - Capture Interrupt occurs"]
    #[inline(always)]
    pub fn ci(&self) -> CI_R {
        CI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Overflow Interrupt occurs"]
    #[inline(always)]
    pub fn oi(&self) -> OI_R {
        OI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Match Interrupt occurs"]
    #[inline(always)]
    pub fn mi(&self) -> MI_R {
        MI_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ir](index.html) module"]
pub struct IR_SPEC;
impl crate::RegisterSpec for IR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ir::R](R) reader structure"]
impl crate::Readable for IR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IR to value 0"]
impl crate::Resettable for IR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
