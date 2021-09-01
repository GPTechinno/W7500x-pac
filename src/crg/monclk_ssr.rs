#[doc = "Register `MONCLK_SSR` reader"]
pub struct R(crate::R<MONCLK_SSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MONCLK_SSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MONCLK_SSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MONCLK_SSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MONCLK_SSR` writer"]
pub struct W(crate::W<MONCLK_SSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MONCLK_SSR_SPEC>;
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
impl From<crate::W<MONCLK_SSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MONCLK_SSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Monitoring Clock source select bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKMON_SEL_A {
    #[doc = "0: MCLK"]
    PLL = 0,
    #[doc = "1: `1`"]
    FCLK = 1,
    #[doc = "2: Internal 8MHz RC oscillator clock (RCLK)"]
    INTERNAL = 2,
    #[doc = "3: External oscillator clock (OCLK, 8MHz-24MHz)"]
    EXTERNAL = 3,
    #[doc = "4: `100`"]
    ADCCLK = 4,
    #[doc = "5: `101`"]
    SSPCLK = 5,
    #[doc = "6: `110`"]
    TIMCLK0 = 6,
    #[doc = "7: `111`"]
    TIMCLK1 = 7,
    #[doc = "8: `1000`"]
    PWMCLK0 = 8,
    #[doc = "9: `1001`"]
    PWMCLK1 = 9,
    #[doc = "10: `1010`"]
    PWMCLK2 = 10,
    #[doc = "11: `1011`"]
    PWMCLK3 = 11,
    #[doc = "12: `1100`"]
    PWMCLK4 = 12,
    #[doc = "13: `1101`"]
    PWMCLK5 = 13,
    #[doc = "14: `1110`"]
    PWMCLK6 = 14,
    #[doc = "15: `1111`"]
    PWMCLK7 = 15,
    #[doc = "16: `10000`"]
    UARTCLK = 16,
    #[doc = "17: `10001`"]
    MII_RCK = 17,
    #[doc = "18: `10010`"]
    MII_TCK = 18,
    #[doc = "19: `10011`"]
    RTCCLK = 19,
}
impl From<CLKMON_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKMON_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLKMON_SEL` reader - Monitoring Clock source select bits"]
pub struct CLKMON_SEL_R(crate::FieldReader<u8, CLKMON_SEL_A>);
impl CLKMON_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKMON_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKMON_SEL_A> {
        match self.bits {
            0 => Some(CLKMON_SEL_A::PLL),
            1 => Some(CLKMON_SEL_A::FCLK),
            2 => Some(CLKMON_SEL_A::INTERNAL),
            3 => Some(CLKMON_SEL_A::EXTERNAL),
            4 => Some(CLKMON_SEL_A::ADCCLK),
            5 => Some(CLKMON_SEL_A::SSPCLK),
            6 => Some(CLKMON_SEL_A::TIMCLK0),
            7 => Some(CLKMON_SEL_A::TIMCLK1),
            8 => Some(CLKMON_SEL_A::PWMCLK0),
            9 => Some(CLKMON_SEL_A::PWMCLK1),
            10 => Some(CLKMON_SEL_A::PWMCLK2),
            11 => Some(CLKMON_SEL_A::PWMCLK3),
            12 => Some(CLKMON_SEL_A::PWMCLK4),
            13 => Some(CLKMON_SEL_A::PWMCLK5),
            14 => Some(CLKMON_SEL_A::PWMCLK6),
            15 => Some(CLKMON_SEL_A::PWMCLK7),
            16 => Some(CLKMON_SEL_A::UARTCLK),
            17 => Some(CLKMON_SEL_A::MII_RCK),
            18 => Some(CLKMON_SEL_A::MII_TCK),
            19 => Some(CLKMON_SEL_A::RTCCLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        **self == CLKMON_SEL_A::PLL
    }
    #[doc = "Checks if the value of the field is `FCLK`"]
    #[inline(always)]
    pub fn is_fclk(&self) -> bool {
        **self == CLKMON_SEL_A::FCLK
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        **self == CLKMON_SEL_A::INTERNAL
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        **self == CLKMON_SEL_A::EXTERNAL
    }
    #[doc = "Checks if the value of the field is `ADCCLK`"]
    #[inline(always)]
    pub fn is_adcclk(&self) -> bool {
        **self == CLKMON_SEL_A::ADCCLK
    }
    #[doc = "Checks if the value of the field is `SSPCLK`"]
    #[inline(always)]
    pub fn is_sspclk(&self) -> bool {
        **self == CLKMON_SEL_A::SSPCLK
    }
    #[doc = "Checks if the value of the field is `TIMCLK0`"]
    #[inline(always)]
    pub fn is_timclk0(&self) -> bool {
        **self == CLKMON_SEL_A::TIMCLK0
    }
    #[doc = "Checks if the value of the field is `TIMCLK1`"]
    #[inline(always)]
    pub fn is_timclk1(&self) -> bool {
        **self == CLKMON_SEL_A::TIMCLK1
    }
    #[doc = "Checks if the value of the field is `PWMCLK0`"]
    #[inline(always)]
    pub fn is_pwmclk0(&self) -> bool {
        **self == CLKMON_SEL_A::PWMCLK0
    }
    #[doc = "Checks if the value of the field is `PWMCLK1`"]
    #[inline(always)]
    pub fn is_pwmclk1(&self) -> bool {
        **self == CLKMON_SEL_A::PWMCLK1
    }
    #[doc = "Checks if the value of the field is `PWMCLK2`"]
    #[inline(always)]
    pub fn is_pwmclk2(&self) -> bool {
        **self == CLKMON_SEL_A::PWMCLK2
    }
    #[doc = "Checks if the value of the field is `PWMCLK3`"]
    #[inline(always)]
    pub fn is_pwmclk3(&self) -> bool {
        **self == CLKMON_SEL_A::PWMCLK3
    }
    #[doc = "Checks if the value of the field is `PWMCLK4`"]
    #[inline(always)]
    pub fn is_pwmclk4(&self) -> bool {
        **self == CLKMON_SEL_A::PWMCLK4
    }
    #[doc = "Checks if the value of the field is `PWMCLK5`"]
    #[inline(always)]
    pub fn is_pwmclk5(&self) -> bool {
        **self == CLKMON_SEL_A::PWMCLK5
    }
    #[doc = "Checks if the value of the field is `PWMCLK6`"]
    #[inline(always)]
    pub fn is_pwmclk6(&self) -> bool {
        **self == CLKMON_SEL_A::PWMCLK6
    }
    #[doc = "Checks if the value of the field is `PWMCLK7`"]
    #[inline(always)]
    pub fn is_pwmclk7(&self) -> bool {
        **self == CLKMON_SEL_A::PWMCLK7
    }
    #[doc = "Checks if the value of the field is `UARTCLK`"]
    #[inline(always)]
    pub fn is_uartclk(&self) -> bool {
        **self == CLKMON_SEL_A::UARTCLK
    }
    #[doc = "Checks if the value of the field is `MII_RCK`"]
    #[inline(always)]
    pub fn is_mii_rck(&self) -> bool {
        **self == CLKMON_SEL_A::MII_RCK
    }
    #[doc = "Checks if the value of the field is `MII_TCK`"]
    #[inline(always)]
    pub fn is_mii_tck(&self) -> bool {
        **self == CLKMON_SEL_A::MII_TCK
    }
    #[doc = "Checks if the value of the field is `RTCCLK`"]
    #[inline(always)]
    pub fn is_rtcclk(&self) -> bool {
        **self == CLKMON_SEL_A::RTCCLK
    }
}
impl core::ops::Deref for CLKMON_SEL_R {
    type Target = crate::FieldReader<u8, CLKMON_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKMON_SEL` writer - Monitoring Clock source select bits"]
pub struct CLKMON_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKMON_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKMON_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "MCLK"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(CLKMON_SEL_A::PLL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn fclk(self) -> &'a mut W {
        self.variant(CLKMON_SEL_A::FCLK)
    }
    #[doc = "Internal 8MHz RC oscillator clock (RCLK)"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(CLKMON_SEL_A::INTERNAL)
    }
    #[doc = "External oscillator clock (OCLK, 8MHz-24MHz)"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(CLKMON_SEL_A::EXTERNAL)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn adcclk(self) -> &'a mut W {
        self.variant(CLKMON_SEL_A::ADCCLK)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn sspclk(self) -> &'a mut W {
        self.variant(CLKMON_SEL_A::SSPCLK)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn timclk0(self) -> &'a mut W {
        self.variant(CLKMON_SEL_A::TIMCLK0)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn timclk1(self) -> &'a mut W {
        self.variant(CLKMON_SEL_A::TIMCLK1)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwmclk0(self) -> &'a mut W {
        self.variant(CLKMON_SEL_A::PWMCLK0)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn pwmclk1(self) -> &'a mut W {
        self.variant(CLKMON_SEL_A::PWMCLK1)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn pwmclk2(self) -> &'a mut W {
        self.variant(CLKMON_SEL_A::PWMCLK2)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn pwmclk3(self) -> &'a mut W {
        self.variant(CLKMON_SEL_A::PWMCLK3)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn pwmclk4(self) -> &'a mut W {
        self.variant(CLKMON_SEL_A::PWMCLK4)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn pwmclk5(self) -> &'a mut W {
        self.variant(CLKMON_SEL_A::PWMCLK5)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pwmclk6(self) -> &'a mut W {
        self.variant(CLKMON_SEL_A::PWMCLK6)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn pwmclk7(self) -> &'a mut W {
        self.variant(CLKMON_SEL_A::PWMCLK7)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn uartclk(self) -> &'a mut W {
        self.variant(CLKMON_SEL_A::UARTCLK)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn mii_rck(self) -> &'a mut W {
        self.variant(CLKMON_SEL_A::MII_RCK)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn mii_tck(self) -> &'a mut W {
        self.variant(CLKMON_SEL_A::MII_TCK)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn rtcclk(self) -> &'a mut W {
        self.variant(CLKMON_SEL_A::RTCCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Monitoring Clock source select bits"]
    #[inline(always)]
    pub fn clkmon_sel(&self) -> CLKMON_SEL_R {
        CLKMON_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Monitoring Clock source select bits"]
    #[inline(always)]
    pub fn clkmon_sel(&mut self) -> CLKMON_SEL_W {
        CLKMON_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Select clock source for monitoring (monitoring pin : PA_02)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [monclk_ssr](index.html) module"]
pub struct MONCLK_SSR_SPEC;
impl crate::RegisterSpec for MONCLK_SSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [monclk_ssr::R](R) reader structure"]
impl crate::Readable for MONCLK_SSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [monclk_ssr::W](W) writer structure"]
impl crate::Writable for MONCLK_SSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MONCLK_SSR to value 0"]
impl crate::Resettable for MONCLK_SSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
