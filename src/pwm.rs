#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt enable register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x04 - Start Stop register"]
    pub ssr: crate::Reg<ssr::SSR_SPEC>,
    #[doc = "0x08 - Pause register"]
    pub psr: crate::Reg<psr::PSR_SPEC>,
}
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt enable register"]
pub mod ier;
#[doc = "SSR register accessor: an alias for `Reg<SSR_SPEC>`"]
pub type SSR = crate::Reg<ssr::SSR_SPEC>;
#[doc = "Start Stop register"]
pub mod ssr;
#[doc = "PSR register accessor: an alias for `Reg<PSR_SPEC>`"]
pub type PSR = crate::Reg<psr::PSR_SPEC>;
#[doc = "Pause register"]
pub mod psr;
