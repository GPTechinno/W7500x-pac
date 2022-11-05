#[doc = "Register `CHSEL` reader"]
pub struct R(crate::R<CHSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHSEL` writer"]
pub struct W(crate::W<CHSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHSEL_SPEC>;
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
impl From<crate::W<CHSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ADC Channel Select bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHSEL_A {
    #[doc = "0: `0`"]
    CHANNEL0 = 0,
    #[doc = "1: `1`"]
    CHANNEL1 = 1,
    #[doc = "2: `10`"]
    CHANNEL2 = 2,
    #[doc = "3: `11`"]
    CHANNEL3 = 3,
    #[doc = "4: `100`"]
    CHANNEL4 = 4,
    #[doc = "5: `101`"]
    CHANNEL5 = 5,
    #[doc = "6: `110`"]
    CHANNEL6 = 6,
    #[doc = "7: `111`"]
    CHANNEL7 = 7,
    #[doc = "8: `1000`"]
    NOCHANNEL = 8,
    #[doc = "15: `1111`"]
    LDOOUTPUT1V5 = 15,
}
impl From<CHSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CHSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHSEL` reader - ADC Channel Select bits"]
pub struct CHSEL_R(crate::FieldReader<u8>);
impl CHSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CHSEL_A> {
        match self.bits {
            0 => Some(CHSEL_A::CHANNEL0),
            1 => Some(CHSEL_A::CHANNEL1),
            2 => Some(CHSEL_A::CHANNEL2),
            3 => Some(CHSEL_A::CHANNEL3),
            4 => Some(CHSEL_A::CHANNEL4),
            5 => Some(CHSEL_A::CHANNEL5),
            6 => Some(CHSEL_A::CHANNEL6),
            7 => Some(CHSEL_A::CHANNEL7),
            8 => Some(CHSEL_A::NOCHANNEL),
            15 => Some(CHSEL_A::LDOOUTPUT1V5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CHANNEL0`"]
    #[inline(always)]
    pub fn is_channel0(&self) -> bool {
        **self == CHSEL_A::CHANNEL0
    }
    #[doc = "Checks if the value of the field is `CHANNEL1`"]
    #[inline(always)]
    pub fn is_channel1(&self) -> bool {
        **self == CHSEL_A::CHANNEL1
    }
    #[doc = "Checks if the value of the field is `CHANNEL2`"]
    #[inline(always)]
    pub fn is_channel2(&self) -> bool {
        **self == CHSEL_A::CHANNEL2
    }
    #[doc = "Checks if the value of the field is `CHANNEL3`"]
    #[inline(always)]
    pub fn is_channel3(&self) -> bool {
        **self == CHSEL_A::CHANNEL3
    }
    #[doc = "Checks if the value of the field is `CHANNEL4`"]
    #[inline(always)]
    pub fn is_channel4(&self) -> bool {
        **self == CHSEL_A::CHANNEL4
    }
    #[doc = "Checks if the value of the field is `CHANNEL5`"]
    #[inline(always)]
    pub fn is_channel5(&self) -> bool {
        **self == CHSEL_A::CHANNEL5
    }
    #[doc = "Checks if the value of the field is `CHANNEL6`"]
    #[inline(always)]
    pub fn is_channel6(&self) -> bool {
        **self == CHSEL_A::CHANNEL6
    }
    #[doc = "Checks if the value of the field is `CHANNEL7`"]
    #[inline(always)]
    pub fn is_channel7(&self) -> bool {
        **self == CHSEL_A::CHANNEL7
    }
    #[doc = "Checks if the value of the field is `NOCHANNEL`"]
    #[inline(always)]
    pub fn is_no_channel(&self) -> bool {
        **self == CHSEL_A::NOCHANNEL
    }
    #[doc = "Checks if the value of the field is `LDOOUTPUT1V5`"]
    #[inline(always)]
    pub fn is_ldooutput1v5(&self) -> bool {
        **self == CHSEL_A::LDOOUTPUT1V5
    }
}
impl core::ops::Deref for CHSEL_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL` writer - ADC Channel Select bits"]
pub struct CHSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn channel0(self) -> &'a mut W {
        self.variant(CHSEL_A::CHANNEL0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn channel1(self) -> &'a mut W {
        self.variant(CHSEL_A::CHANNEL1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn channel2(self) -> &'a mut W {
        self.variant(CHSEL_A::CHANNEL2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn channel3(self) -> &'a mut W {
        self.variant(CHSEL_A::CHANNEL3)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn channel4(self) -> &'a mut W {
        self.variant(CHSEL_A::CHANNEL4)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn channel5(self) -> &'a mut W {
        self.variant(CHSEL_A::CHANNEL5)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn channel6(self) -> &'a mut W {
        self.variant(CHSEL_A::CHANNEL6)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn channel7(self) -> &'a mut W {
        self.variant(CHSEL_A::CHANNEL7)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn no_channel(self) -> &'a mut W {
        self.variant(CHSEL_A::NOCHANNEL)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn ldooutput1v5(self) -> &'a mut W {
        self.variant(CHSEL_A::LDOOUTPUT1V5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - ADC Channel Select bits"]
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC Channel Select bits"]
    #[inline(always)]
    pub fn chsel(&mut self) -> CHSEL_W {
        CHSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC channel select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chsel](index.html) module"]
pub struct CHSEL_SPEC;
impl crate::RegisterSpec for CHSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chsel::R](R) reader structure"]
impl crate::Readable for CHSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chsel::W](W) writer structure"]
impl crate::Writable for CHSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHSEL to value 0"]
impl crate::Resettable for CHSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
