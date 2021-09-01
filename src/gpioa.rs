#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DATA register"]
    pub data: crate::Reg<data::DATA_SPEC>,
    #[doc = "0x04 - Data Output Latch register"]
    pub dataout: crate::Reg<dataout::DATAOUT_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Output Enable Set register"]
    pub outenset: crate::Reg<outenset::OUTENSET_SPEC>,
    #[doc = "0x14 - Output Enable Clear register"]
    pub outenclr: crate::Reg<outenclr::OUTENCLR_SPEC>,
    _reserved4: [u8; 0x08],
    #[doc = "0x20 - Interrupt Enable Set register"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x24 - Interrupt Enable Clear register"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    #[doc = "0x28 - Interrupt Type Set register"]
    pub inttypeset: crate::Reg<inttypeset::INTTYPESET_SPEC>,
    #[doc = "0x2c - Interrupt Type Clear register"]
    pub inttypeclr: crate::Reg<inttypeclr::INTTYPECLR_SPEC>,
    #[doc = "0x30 - Interrupt Polarity Set register"]
    pub intpolset: crate::Reg<intpolset::INTPOLSET_SPEC>,
    #[doc = "0x34 - Interrupt Polarity Clear register"]
    pub intpolclr: crate::Reg<intpolclr::INTPOLCLR_SPEC>,
    _reserved_10_intclear: [u8; 0x04],
    _reserved11: [u8; 0x03c4],
    #[doc = "0x400..0x800 - Lower byte Masked Access register"]
    pub lb_masked: [crate::Reg<lb_masked::LB_MASKED_SPEC>; 256],
    #[doc = "0x800..0xc00 - Upper byte Masked Access register"]
    pub ub_masked: [crate::Reg<ub_masked::UB_MASKED_SPEC>; 256],
}
impl RegisterBlock {
    #[doc = "0x38 - Interrupt Clear register"]
    #[inline(always)]
    pub fn intclear(&self) -> &crate::Reg<intclear::INTCLEAR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(56usize)
                as *const crate::Reg<intclear::INTCLEAR_SPEC>)
        }
    }
    #[doc = "0x38 - Interrupt Status register"]
    #[inline(always)]
    pub fn intstatus(&self) -> &crate::Reg<intstatus::INTSTATUS_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(56usize)
                as *const crate::Reg<intstatus::INTSTATUS_SPEC>)
        }
    }
}
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "DATA register"]
pub mod data;
#[doc = "DATAOUT register accessor: an alias for `Reg<DATAOUT_SPEC>`"]
pub type DATAOUT = crate::Reg<dataout::DATAOUT_SPEC>;
#[doc = "Data Output Latch register"]
pub mod dataout;
#[doc = "OUTENSET register accessor: an alias for `Reg<OUTENSET_SPEC>`"]
pub type OUTENSET = crate::Reg<outenset::OUTENSET_SPEC>;
#[doc = "Output Enable Set register"]
pub mod outenset;
#[doc = "OUTENCLR register accessor: an alias for `Reg<OUTENCLR_SPEC>`"]
pub type OUTENCLR = crate::Reg<outenclr::OUTENCLR_SPEC>;
#[doc = "Output Enable Clear register"]
pub mod outenclr;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable Set register"]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Enable Clear register"]
pub mod intenclr;
#[doc = "INTTYPESET register accessor: an alias for `Reg<INTTYPESET_SPEC>`"]
pub type INTTYPESET = crate::Reg<inttypeset::INTTYPESET_SPEC>;
#[doc = "Interrupt Type Set register"]
pub mod inttypeset;
#[doc = "INTTYPECLR register accessor: an alias for `Reg<INTTYPECLR_SPEC>`"]
pub type INTTYPECLR = crate::Reg<inttypeclr::INTTYPECLR_SPEC>;
#[doc = "Interrupt Type Clear register"]
pub mod inttypeclr;
#[doc = "INTPOLSET register accessor: an alias for `Reg<INTPOLSET_SPEC>`"]
pub type INTPOLSET = crate::Reg<intpolset::INTPOLSET_SPEC>;
#[doc = "Interrupt Polarity Set register"]
pub mod intpolset;
#[doc = "INTPOLCLR register accessor: an alias for `Reg<INTPOLCLR_SPEC>`"]
pub type INTPOLCLR = crate::Reg<intpolclr::INTPOLCLR_SPEC>;
#[doc = "Interrupt Polarity Clear register"]
pub mod intpolclr;
#[doc = "INTSTATUS register accessor: an alias for `Reg<INTSTATUS_SPEC>`"]
pub type INTSTATUS = crate::Reg<intstatus::INTSTATUS_SPEC>;
#[doc = "Interrupt Status register"]
pub mod intstatus;
#[doc = "INTCLEAR register accessor: an alias for `Reg<INTCLEAR_SPEC>`"]
pub type INTCLEAR = crate::Reg<intclear::INTCLEAR_SPEC>;
#[doc = "Interrupt Clear register"]
pub mod intclear;
#[doc = "LB_MASKED register accessor: an alias for `Reg<LB_MASKED_SPEC>`"]
pub type LB_MASKED = crate::Reg<lb_masked::LB_MASKED_SPEC>;
#[doc = "Lower byte Masked Access register"]
pub mod lb_masked;
#[doc = "UB_MASKED register accessor: an alias for `Reg<UB_MASKED_SPEC>`"]
pub type UB_MASKED = crate::Reg<ub_masked::UB_MASKED_SPEC>;
#[doc = "Upper byte Masked Access register"]
pub mod ub_masked;
