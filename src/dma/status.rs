#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Current state of the control state machine\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STATE_A {
    #[doc = "0: `0`"]
    IDLE = 0,
    #[doc = "1: `1`"]
    READINGCHANCTRLDATA = 1,
    #[doc = "2: `10`"]
    READINGSRCDATAENDPTR = 2,
    #[doc = "3: `11`"]
    READINGDSTDATAENDPTR = 3,
    #[doc = "4: `100`"]
    READINGSRCDATA = 4,
    #[doc = "5: `101`"]
    WRITINGDSTDATA = 5,
    #[doc = "6: `110`"]
    WRITINGCHANCTRLDATA = 6,
    #[doc = "8: `1000`"]
    STALLED = 8,
    #[doc = "9: `1001`"]
    DONE = 9,
    #[doc = "10: `1010`"]
    PERIPHSCATGATHTRANS = 10,
}
impl From<STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STATE` reader - Current state of the control state machine"]
pub struct STATE_R(crate::FieldReader<u8>);
impl STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STATE_A> {
        match self.bits {
            0 => Some(STATE_A::IDLE),
            1 => Some(STATE_A::READINGCHANCTRLDATA),
            2 => Some(STATE_A::READINGSRCDATAENDPTR),
            3 => Some(STATE_A::READINGDSTDATAENDPTR),
            4 => Some(STATE_A::READINGSRCDATA),
            5 => Some(STATE_A::WRITINGDSTDATA),
            6 => Some(STATE_A::WRITINGCHANCTRLDATA),
            8 => Some(STATE_A::STALLED),
            9 => Some(STATE_A::DONE),
            10 => Some(STATE_A::PERIPHSCATGATHTRANS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == STATE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `READINGCHANCTRLDATA`"]
    #[inline(always)]
    pub fn is_reading_chan_ctrl_data(&self) -> bool {
        **self == STATE_A::READINGCHANCTRLDATA
    }
    #[doc = "Checks if the value of the field is `READINGSRCDATAENDPTR`"]
    #[inline(always)]
    pub fn is_reading_src_data_end_ptr(&self) -> bool {
        **self == STATE_A::READINGSRCDATAENDPTR
    }
    #[doc = "Checks if the value of the field is `READINGDSTDATAENDPTR`"]
    #[inline(always)]
    pub fn is_reading_dst_data_end_ptr(&self) -> bool {
        **self == STATE_A::READINGDSTDATAENDPTR
    }
    #[doc = "Checks if the value of the field is `READINGSRCDATA`"]
    #[inline(always)]
    pub fn is_reading_src_data(&self) -> bool {
        **self == STATE_A::READINGSRCDATA
    }
    #[doc = "Checks if the value of the field is `WRITINGDSTDATA`"]
    #[inline(always)]
    pub fn is_writing_dst_data(&self) -> bool {
        **self == STATE_A::WRITINGDSTDATA
    }
    #[doc = "Checks if the value of the field is `WRITINGCHANCTRLDATA`"]
    #[inline(always)]
    pub fn is_writing_chan_ctrl_data(&self) -> bool {
        **self == STATE_A::WRITINGCHANCTRLDATA
    }
    #[doc = "Checks if the value of the field is `STALLED`"]
    #[inline(always)]
    pub fn is_stalled(&self) -> bool {
        **self == STATE_A::STALLED
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        **self == STATE_A::DONE
    }
    #[doc = "Checks if the value of the field is `PERIPHSCATGATHTRANS`"]
    #[inline(always)]
    pub fn is_periph_scat_gath_trans(&self) -> bool {
        **self == STATE_A::PERIPHSCATGATHTRANS
    }
}
impl core::ops::Deref for STATE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable status of the controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Enable status of the controller"]
pub struct ENABLE_R(crate::FieldReader<bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DISABLE,
            true => ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ENABLE_A::ENABLE
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 4:7 - Current state of the control state machine"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 0 - Enable status of the controller"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
}
#[doc = "DMA status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0x0005_0000"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0005_0000
    }
}
