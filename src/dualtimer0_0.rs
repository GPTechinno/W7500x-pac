#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer Load register"]
    pub load: crate::Reg<load::LOAD_SPEC>,
    #[doc = "0x04 - Timer Counter Current Value register"]
    pub value: crate::Reg<value::VALUE_SPEC>,
    #[doc = "0x08 - Timer Control register"]
    pub control: crate::Reg<control::CONTROL_SPEC>,
    #[doc = "0x0c - Timer Interrupt Clear register"]
    pub intclr: crate::Reg<intclr::INTCLR_SPEC>,
    #[doc = "0x10 - Timer Raw Interrupt Status register"]
    pub ris: crate::Reg<ris::RIS_SPEC>,
    #[doc = "0x14 - Timer Masked Interrupt Status register"]
    pub mis: crate::Reg<mis::MIS_SPEC>,
    #[doc = "0x18 - Background Load register"]
    pub bgload: crate::Reg<bgload::BGLOAD_SPEC>,
    _reserved7: [u8; 0x64],
    #[doc = "0x80 - Clock enable register"]
    pub ce: crate::Reg<ce::CE_SPEC>,
}
#[doc = "LOAD register accessor: an alias for `Reg<LOAD_SPEC>`"]
pub type LOAD = crate::Reg<load::LOAD_SPEC>;
#[doc = "Timer Load register"]
pub mod load;
#[doc = "VALUE register accessor: an alias for `Reg<VALUE_SPEC>`"]
pub type VALUE = crate::Reg<value::VALUE_SPEC>;
#[doc = "Timer Counter Current Value register"]
pub mod value;
#[doc = "CONTROL register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Timer Control register"]
pub mod control;
#[doc = "INTCLR register accessor: an alias for `Reg<INTCLR_SPEC>`"]
pub type INTCLR = crate::Reg<intclr::INTCLR_SPEC>;
#[doc = "Timer Interrupt Clear register"]
pub mod intclr;
#[doc = "RIS register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "Timer Raw Interrupt Status register"]
pub mod ris;
#[doc = "MIS register accessor: an alias for `Reg<MIS_SPEC>`"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "Timer Masked Interrupt Status register"]
pub mod mis;
#[doc = "BGLOAD register accessor: an alias for `Reg<BGLOAD_SPEC>`"]
pub type BGLOAD = crate::Reg<bgload::BGLOAD_SPEC>;
#[doc = "Background Load register"]
pub mod bgload;
#[doc = "CE register accessor: an alias for `Reg<CE_SPEC>`"]
pub type CE = crate::Reg<ce::CE_SPEC>;
#[doc = "Clock enable register"]
pub mod ce;
