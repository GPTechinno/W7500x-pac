#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register 0"]
    pub cr0: crate::Reg<cr0::CR0_SPEC>,
    #[doc = "0x04 - Control register 1"]
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    #[doc = "0x08 - Data register"]
    pub dr: crate::Reg<dr::DR_SPEC>,
    #[doc = "0x0c - Status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x10 - Clock prescale register"]
    pub cpsr: crate::Reg<cpsr::CPSR_SPEC>,
    #[doc = "0x14 - Interrupt mask set or clear register"]
    pub imsc: crate::Reg<imsc::IMSC_SPEC>,
    #[doc = "0x18 - Raw interrupt status register"]
    pub ris: crate::Reg<ris::RIS_SPEC>,
    #[doc = "0x1c - Masked interrupt status register"]
    pub mis: crate::Reg<mis::MIS_SPEC>,
    #[doc = "0x20 - Interrupt clear register"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x24 - DMA control register"]
    pub dmacr: crate::Reg<dmacr::DMACR_SPEC>,
}
#[doc = "CR0 register accessor: an alias for `Reg<CR0_SPEC>`"]
pub type CR0 = crate::Reg<cr0::CR0_SPEC>;
#[doc = "Control register 0"]
pub mod cr0;
#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "Control register 1"]
pub mod cr1;
#[doc = "DR register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "Data register"]
pub mod dr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
#[doc = "CPSR register accessor: an alias for `Reg<CPSR_SPEC>`"]
pub type CPSR = crate::Reg<cpsr::CPSR_SPEC>;
#[doc = "Clock prescale register"]
pub mod cpsr;
#[doc = "IMSC register accessor: an alias for `Reg<IMSC_SPEC>`"]
pub type IMSC = crate::Reg<imsc::IMSC_SPEC>;
#[doc = "Interrupt mask set or clear register"]
pub mod imsc;
#[doc = "RIS register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "Raw interrupt status register"]
pub mod ris;
#[doc = "MIS register accessor: an alias for `Reg<MIS_SPEC>`"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "Masked interrupt status register"]
pub mod mis;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt clear register"]
pub mod icr;
#[doc = "DMACR register accessor: an alias for `Reg<DMACR_SPEC>`"]
pub type DMACR = crate::Reg<dmacr::DMACR_SPEC>;
#[doc = "DMA control register"]
pub mod dmacr;
