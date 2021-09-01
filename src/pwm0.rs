#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt register"]
    pub ir: crate::Reg<ir::IR_SPEC>,
    #[doc = "0x04 - Interrupt enable register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x08 - Interrupt clear register"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x0c - Timer/Counter register"]
    pub tcr: crate::Reg<tcr::TCR_SPEC>,
    #[doc = "0x10 - Prescale counter register"]
    pub pcr: crate::Reg<pcr::PCR_SPEC>,
    #[doc = "0x14 - Prescale register"]
    pub pr: crate::Reg<pr::PR_SPEC>,
    #[doc = "0x18 - Match register"]
    pub mr: crate::Reg<mr::MR_SPEC>,
    #[doc = "0x1c - Limit register"]
    pub lr: crate::Reg<lr::LR_SPEC>,
    #[doc = "0x20 - Up-Down mode register"]
    pub udmr: crate::Reg<udmr::UDMR_SPEC>,
    #[doc = "0x24 - Timer/Counter mode register"]
    pub tcmr: crate::Reg<tcmr::TCMR_SPEC>,
    #[doc = "0x28 - PWM output enable and external input enable register"]
    pub peeer: crate::Reg<peeer::PEEER_SPEC>,
    #[doc = "0x2c - Capture mode register"]
    pub cmr: crate::Reg<cmr::CMR_SPEC>,
    #[doc = "0x30 - Capture register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x34 - Periodic mode register"]
    pub pdmr: crate::Reg<pdmr::PDMR_SPEC>,
    #[doc = "0x38 - Dead-zone enable register"]
    pub dzer: crate::Reg<dzer::DZER_SPEC>,
    #[doc = "0x3c - Dead-zone counter register"]
    pub dzcr: crate::Reg<dzcr::DZCR_SPEC>,
}
#[doc = "IR register accessor: an alias for `Reg<IR_SPEC>`"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "Interrupt register"]
pub mod ir;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt enable register"]
pub mod ier;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt clear register"]
pub mod icr;
#[doc = "TCR register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "Timer/Counter register"]
pub mod tcr;
#[doc = "PCR register accessor: an alias for `Reg<PCR_SPEC>`"]
pub type PCR = crate::Reg<pcr::PCR_SPEC>;
#[doc = "Prescale counter register"]
pub mod pcr;
#[doc = "PR register accessor: an alias for `Reg<PR_SPEC>`"]
pub type PR = crate::Reg<pr::PR_SPEC>;
#[doc = "Prescale register"]
pub mod pr;
#[doc = "MR register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Match register"]
pub mod mr;
#[doc = "LR register accessor: an alias for `Reg<LR_SPEC>`"]
pub type LR = crate::Reg<lr::LR_SPEC>;
#[doc = "Limit register"]
pub mod lr;
#[doc = "UDMR register accessor: an alias for `Reg<UDMR_SPEC>`"]
pub type UDMR = crate::Reg<udmr::UDMR_SPEC>;
#[doc = "Up-Down mode register"]
pub mod udmr;
#[doc = "TCMR register accessor: an alias for `Reg<TCMR_SPEC>`"]
pub type TCMR = crate::Reg<tcmr::TCMR_SPEC>;
#[doc = "Timer/Counter mode register"]
pub mod tcmr;
#[doc = "PEEER register accessor: an alias for `Reg<PEEER_SPEC>`"]
pub type PEEER = crate::Reg<peeer::PEEER_SPEC>;
#[doc = "PWM output enable and external input enable register"]
pub mod peeer;
#[doc = "CMR register accessor: an alias for `Reg<CMR_SPEC>`"]
pub type CMR = crate::Reg<cmr::CMR_SPEC>;
#[doc = "Capture mode register"]
pub mod cmr;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Capture register"]
pub mod cr;
#[doc = "PDMR register accessor: an alias for `Reg<PDMR_SPEC>`"]
pub type PDMR = crate::Reg<pdmr::PDMR_SPEC>;
#[doc = "Periodic mode register"]
pub mod pdmr;
#[doc = "DZER register accessor: an alias for `Reg<DZER_SPEC>`"]
pub type DZER = crate::Reg<dzer::DZER_SPEC>;
#[doc = "Dead-zone enable register"]
pub mod dzer;
#[doc = "DZCR register accessor: an alias for `Reg<DZCR_SPEC>`"]
pub type DZCR = crate::Reg<dzcr::DZCR_SPEC>;
#[doc = "Dead-zone counter register"]
pub mod dzcr;
