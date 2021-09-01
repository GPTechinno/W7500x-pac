#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Remap Control Register (R/W)"]
    pub remap: crate::Reg<remap::REMAP_SPEC>,
    #[doc = "0x04 - PMU Control Register (R/W)"]
    pub pmuctrl: crate::Reg<pmuctrl::PMUCTRL_SPEC>,
    #[doc = "0x08 - Reset Option Register (R/W)"]
    pub resetop: crate::Reg<resetop::RESETOP_SPEC>,
    #[doc = "0x0c - EMI Control Register (R/W)"]
    pub emictrl: crate::Reg<emictrl::EMICTRL_SPEC>,
    #[doc = "0x10 - Reset Information Register (R/W)"]
    pub rstinfo: crate::Reg<rstinfo::RSTINFO_SPEC>,
}
#[doc = "REMAP register accessor: an alias for `Reg<REMAP_SPEC>`"]
pub type REMAP = crate::Reg<remap::REMAP_SPEC>;
#[doc = "Remap Control Register (R/W)"]
pub mod remap;
#[doc = "PMUCTRL register accessor: an alias for `Reg<PMUCTRL_SPEC>`"]
pub type PMUCTRL = crate::Reg<pmuctrl::PMUCTRL_SPEC>;
#[doc = "PMU Control Register (R/W)"]
pub mod pmuctrl;
#[doc = "RESETOP register accessor: an alias for `Reg<RESETOP_SPEC>`"]
pub type RESETOP = crate::Reg<resetop::RESETOP_SPEC>;
#[doc = "Reset Option Register (R/W)"]
pub mod resetop;
#[doc = "EMICTRL register accessor: an alias for `Reg<EMICTRL_SPEC>`"]
pub type EMICTRL = crate::Reg<emictrl::EMICTRL_SPEC>;
#[doc = "EMI Control Register (R/W)"]
pub mod emictrl;
#[doc = "RSTINFO register accessor: an alias for `Reg<RSTINFO_SPEC>`"]
pub type RSTINFO = crate::Reg<rstinfo::RSTINFO_SPEC>;
#[doc = "Reset Information Register (R/W)"]
pub mod rstinfo;
