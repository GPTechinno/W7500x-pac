#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC control register"]
    pub ctr: crate::Reg<ctr::CTR_SPEC>,
    #[doc = "0x04 - ADC channel select register"]
    pub chsel: crate::Reg<chsel::CHSEL_SPEC>,
    #[doc = "0x08 - ADC start register"]
    pub start: crate::Reg<start::START_SPEC>,
    #[doc = "0x0c - ADC conversion data register"]
    pub data: crate::Reg<data::DATA_SPEC>,
    #[doc = "0x10 - ADC interrupt register"]
    pub int: crate::Reg<int::INT_SPEC>,
    _reserved5: [u8; 0x08],
    #[doc = "0x1c - ADC interrupt clear register"]
    pub intclr: crate::Reg<intclr::INTCLR_SPEC>,
}
#[doc = "CTR register accessor: an alias for `Reg<CTR_SPEC>`"]
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
#[doc = "ADC control register"]
pub mod ctr;
#[doc = "CHSEL register accessor: an alias for `Reg<CHSEL_SPEC>`"]
pub type CHSEL = crate::Reg<chsel::CHSEL_SPEC>;
#[doc = "ADC channel select register"]
pub mod chsel;
#[doc = "START register accessor: an alias for `Reg<START_SPEC>`"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = "ADC start register"]
pub mod start;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "ADC conversion data register"]
pub mod data;
#[doc = "INT register accessor: an alias for `Reg<INT_SPEC>`"]
pub type INT = crate::Reg<int::INT_SPEC>;
#[doc = "ADC interrupt register"]
pub mod int;
#[doc = "INTCLR register accessor: an alias for `Reg<INTCLR_SPEC>`"]
pub type INTCLR = crate::Reg<intclr::INTCLR_SPEC>;
#[doc = "ADC interrupt clear register"]
pub mod intclr;
