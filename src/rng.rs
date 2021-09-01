#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RNG run register"]
    pub run: crate::Reg<run::RUN_SPEC>,
    #[doc = "0x04 - RNG seed value register"]
    pub seed: crate::Reg<seed::SEED_SPEC>,
    #[doc = "0x08 - RNG Clock source select register"]
    pub clksel: crate::Reg<clksel::CLKSEL_SPEC>,
    #[doc = "0x0c - RNG MODE select register"]
    pub mode: crate::Reg<mode::MODE_SPEC>,
    #[doc = "0x10 - RNG random number value register"]
    pub rn: crate::Reg<rn::RN_SPEC>,
    #[doc = "0x14 - RNG polynomial register"]
    pub poly: crate::Reg<poly::POLY_SPEC>,
}
#[doc = "RUN register accessor: an alias for `Reg<RUN_SPEC>`"]
pub type RUN = crate::Reg<run::RUN_SPEC>;
#[doc = "RNG run register"]
pub mod run;
#[doc = "SEED register accessor: an alias for `Reg<SEED_SPEC>`"]
pub type SEED = crate::Reg<seed::SEED_SPEC>;
#[doc = "RNG seed value register"]
pub mod seed;
#[doc = "CLKSEL register accessor: an alias for `Reg<CLKSEL_SPEC>`"]
pub type CLKSEL = crate::Reg<clksel::CLKSEL_SPEC>;
#[doc = "RNG Clock source select register"]
pub mod clksel;
#[doc = "MODE register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "RNG MODE select register"]
pub mod mode;
#[doc = "RN register accessor: an alias for `Reg<RN_SPEC>`"]
pub type RN = crate::Reg<rn::RN_SPEC>;
#[doc = "RNG random number value register"]
pub mod rn;
#[doc = "POLY register accessor: an alias for `Reg<POLY_SPEC>`"]
pub type POLY = crate::Reg<poly::POLY_SPEC>;
#[doc = "RNG polynomial register"]
pub mod poly;
