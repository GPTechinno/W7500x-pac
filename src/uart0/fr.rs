#[doc = "Register `FR` reader"]
pub struct R(crate::R<FR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Ring indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RI_A {
    #[doc = "0: `0`"]
    HIGH = 0,
    #[doc = "1: `1`"]
    LOW = 1,
}
impl From<RI_A> for bool {
    #[inline(always)]
    fn from(variant: RI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RI` reader - Ring indicator"]
pub struct RI_R(crate::FieldReader<bool>);
impl RI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RI_A {
        match self.bits {
            false => RI_A::HIGH,
            true => RI_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == RI_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == RI_A::LOW
    }
}
impl core::ops::Deref for RI_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit FIFO empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFE_A {
    #[doc = "0: `0`"]
    TXHOLDEMPTY = 0,
    #[doc = "1: `1`"]
    TXFIFOEMPTY = 1,
}
impl From<TXFE_A> for bool {
    #[inline(always)]
    fn from(variant: TXFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFE` reader - Transmit FIFO empty"]
pub struct TXFE_R(crate::FieldReader<bool>);
impl TXFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXFE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFE_A {
        match self.bits {
            false => TXFE_A::TXHOLDEMPTY,
            true => TXFE_A::TXFIFOEMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `TXHOLDEMPTY`"]
    #[inline(always)]
    pub fn is_tx_hold_empty(&self) -> bool {
        **self == TXFE_A::TXHOLDEMPTY
    }
    #[doc = "Checks if the value of the field is `TXFIFOEMPTY`"]
    #[inline(always)]
    pub fn is_tx_fifoempty(&self) -> bool {
        **self == TXFE_A::TXFIFOEMPTY
    }
}
impl core::ops::Deref for TXFE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive FIFO full\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFF_A {
    #[doc = "0: `0`"]
    RXHOLDFULL = 0,
    #[doc = "1: `1`"]
    RXFIFOFULL = 1,
}
impl From<RXFF_A> for bool {
    #[inline(always)]
    fn from(variant: RXFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFF` reader - Receive FIFO full"]
pub struct RXFF_R(crate::FieldReader<bool>);
impl RXFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFF_A {
        match self.bits {
            false => RXFF_A::RXHOLDFULL,
            true => RXFF_A::RXFIFOFULL,
        }
    }
    #[doc = "Checks if the value of the field is `RXHOLDFULL`"]
    #[inline(always)]
    pub fn is_rx_hold_full(&self) -> bool {
        **self == RXFF_A::RXHOLDFULL
    }
    #[doc = "Checks if the value of the field is `RXFIFOFULL`"]
    #[inline(always)]
    pub fn is_rx_fifofull(&self) -> bool {
        **self == RXFF_A::RXFIFOFULL
    }
}
impl core::ops::Deref for RXFF_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit FIFO fULl\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFF_A {
    #[doc = "0: `0`"]
    TXHOLDFULL = 0,
    #[doc = "1: `1`"]
    TXFIFOFULL = 1,
}
impl From<TXFF_A> for bool {
    #[inline(always)]
    fn from(variant: TXFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFF` reader - Transmit FIFO fULl"]
pub struct TXFF_R(crate::FieldReader<bool>);
impl TXFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFF_A {
        match self.bits {
            false => TXFF_A::TXHOLDFULL,
            true => TXFF_A::TXFIFOFULL,
        }
    }
    #[doc = "Checks if the value of the field is `TXHOLDFULL`"]
    #[inline(always)]
    pub fn is_tx_hold_full(&self) -> bool {
        **self == TXFF_A::TXHOLDFULL
    }
    #[doc = "Checks if the value of the field is `TXFIFOFULL`"]
    #[inline(always)]
    pub fn is_tx_fifofull(&self) -> bool {
        **self == TXFF_A::TXFIFOFULL
    }
}
impl core::ops::Deref for TXFF_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive FIFO empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFE_A {
    #[doc = "0: `0`"]
    RXHOLDEMPTY = 0,
    #[doc = "1: `1`"]
    RXFIFOEMPTY = 1,
}
impl From<RXFE_A> for bool {
    #[inline(always)]
    fn from(variant: RXFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFE` reader - Receive FIFO empty"]
pub struct RXFE_R(crate::FieldReader<bool>);
impl RXFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFE_A {
        match self.bits {
            false => RXFE_A::RXHOLDEMPTY,
            true => RXFE_A::RXFIFOEMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `RXHOLDEMPTY`"]
    #[inline(always)]
    pub fn is_rx_hold_empty(&self) -> bool {
        **self == RXFE_A::RXHOLDEMPTY
    }
    #[doc = "Checks if the value of the field is `RXFIFOEMPTY`"]
    #[inline(always)]
    pub fn is_rx_fifoempty(&self) -> bool {
        **self == RXFE_A::RXFIFOEMPTY
    }
}
impl core::ops::Deref for RXFE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "UART busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "1: `1`"]
    BUSY = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - UART busy"]
pub struct BUSY_R(crate::FieldReader<bool>);
impl BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BUSY_A> {
        match self.bits {
            true => Some(BUSY_A::BUSY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == BUSY_A::BUSY
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Data carrier detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCD_A {
    #[doc = "1: `1`"]
    DATACARRIERDETECT = 1,
}
impl From<DCD_A> for bool {
    #[inline(always)]
    fn from(variant: DCD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCD` reader - Data carrier detect"]
pub struct DCD_R(crate::FieldReader<bool>);
impl DCD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DCD_A> {
        match self.bits {
            true => Some(DCD_A::DATACARRIERDETECT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DATACARRIERDETECT`"]
    #[inline(always)]
    pub fn is_data_carrier_detect(&self) -> bool {
        **self == DCD_A::DATACARRIERDETECT
    }
}
impl core::ops::Deref for DCD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Data set ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSR_A {
    #[doc = "1: `1`"]
    DATASETREADY = 1,
}
impl From<DSR_A> for bool {
    #[inline(always)]
    fn from(variant: DSR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSR` reader - Data set ready"]
pub struct DSR_R(crate::FieldReader<bool>);
impl DSR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DSR_A> {
        match self.bits {
            true => Some(DSR_A::DATASETREADY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DATASETREADY`"]
    #[inline(always)]
    pub fn is_data_set_ready(&self) -> bool {
        **self == DSR_A::DATASETREADY
    }
}
impl core::ops::Deref for DSR_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Clear to send\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTS_A {
    #[doc = "1: `1`"]
    CLEARTOSEND = 1,
}
impl From<CTS_A> for bool {
    #[inline(always)]
    fn from(variant: CTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTS` reader - Clear to send"]
pub struct CTS_R(crate::FieldReader<bool>);
impl CTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CTS_A> {
        match self.bits {
            true => Some(CTS_A::CLEARTOSEND),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEARTOSEND`"]
    #[inline(always)]
    pub fn is_clear_to_send(&self) -> bool {
        **self == CTS_A::CLEARTOSEND
    }
}
impl core::ops::Deref for CTS_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 8 - Ring indicator"]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO empty"]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO full"]
    #[inline(always)]
    pub fn rxff(&self) -> RXFF_R {
        RXFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO fULl"]
    #[inline(always)]
    pub fn txff(&self) -> TXFF_R {
        TXFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO empty"]
    #[inline(always)]
    pub fn rxfe(&self) -> RXFE_R {
        RXFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - UART busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Data carrier detect"]
    #[inline(always)]
    pub fn dcd(&self) -> DCD_R {
        DCD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Data set ready"]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Clear to send"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Flags\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fr](index.html) module"]
pub struct FR_SPEC;
impl crate::RegisterSpec for FR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fr::R](R) reader structure"]
impl crate::Readable for FR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FR to value 0xc0"]
impl crate::Resettable for FR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc0
    }
}
