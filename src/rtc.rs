#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub rtccon: crate::Reg<rtccon::RTCCON_SPEC>,
    #[doc = "0x04 - Interrupt Mask register"]
    pub rtcinte: crate::Reg<rtcinte::RTCINTE_SPEC>,
    #[doc = "0x08 - Interrupt Pending register"]
    pub rtcintp: crate::Reg<rtcintp::RTCINTP_SPEC>,
    #[doc = "0x0c - Alarm Mask register"]
    pub rtcamr: crate::Reg<rtcamr::RTCAMR_SPEC>,
    #[doc = "0x10 - BCD Second register"]
    pub bcdsec: crate::Reg<bcdsec::BCDSEC_SPEC>,
    #[doc = "0x14 - BCD Minute register"]
    pub bcdmin: crate::Reg<bcdmin::BCDMIN_SPEC>,
    #[doc = "0x18 - BCD Hour register"]
    pub bcdhour: crate::Reg<bcdhour::BCDHOUR_SPEC>,
    #[doc = "0x1c - BCD Day register"]
    pub bcdday: crate::Reg<bcdday::BCDDAY_SPEC>,
    #[doc = "0x20 - BCD Date register"]
    pub bcddate: crate::Reg<bcddate::BCDDATE_SPEC>,
    #[doc = "0x24 - BCD Month register"]
    pub bcdmon: crate::Reg<bcdmon::BCDMON_SPEC>,
    #[doc = "0x28 - BCD Year register"]
    pub bcdyear: crate::Reg<bcdyear::BCDYEAR_SPEC>,
    #[doc = "0x2c - Predetermining Second register"]
    pub presec: crate::Reg<presec::PRESEC_SPEC>,
    #[doc = "0x30 - Predetermining Minute register"]
    pub premin: crate::Reg<premin::PREMIN_SPEC>,
    #[doc = "0x34 - Predetermining Hour register"]
    pub prehour: crate::Reg<prehour::PREHOUR_SPEC>,
    #[doc = "0x38 - Predetermining Day register"]
    pub preday: crate::Reg<preday::PREDAY_SPEC>,
    #[doc = "0x3c - Predetermining Date register"]
    pub predate: crate::Reg<predate::PREDATE_SPEC>,
    #[doc = "0x40 - Predetermining Month register"]
    pub premon: crate::Reg<premon::PREMON_SPEC>,
    #[doc = "0x44 - Predetermining Year register"]
    pub preyear: crate::Reg<preyear::PREYEAR_SPEC>,
    #[doc = "0x48 - Consolidated Time0 register"]
    pub rtctime0: crate::Reg<rtctime0::RTCTIME0_SPEC>,
    #[doc = "0x4c - Consolidated Time1 register"]
    pub rtctime1: crate::Reg<rtctime1::RTCTIME1_SPEC>,
}
#[doc = "RTCCON register accessor: an alias for `Reg<RTCCON_SPEC>`"]
pub type RTCCON = crate::Reg<rtccon::RTCCON_SPEC>;
#[doc = "control register"]
pub mod rtccon;
#[doc = "RTCINTE register accessor: an alias for `Reg<RTCINTE_SPEC>`"]
pub type RTCINTE = crate::Reg<rtcinte::RTCINTE_SPEC>;
#[doc = "Interrupt Mask register"]
pub mod rtcinte;
#[doc = "RTCINTP register accessor: an alias for `Reg<RTCINTP_SPEC>`"]
pub type RTCINTP = crate::Reg<rtcintp::RTCINTP_SPEC>;
#[doc = "Interrupt Pending register"]
pub mod rtcintp;
#[doc = "RTCAMR register accessor: an alias for `Reg<RTCAMR_SPEC>`"]
pub type RTCAMR = crate::Reg<rtcamr::RTCAMR_SPEC>;
#[doc = "Alarm Mask register"]
pub mod rtcamr;
#[doc = "BCDSEC register accessor: an alias for `Reg<BCDSEC_SPEC>`"]
pub type BCDSEC = crate::Reg<bcdsec::BCDSEC_SPEC>;
#[doc = "BCD Second register"]
pub mod bcdsec;
#[doc = "BCDMIN register accessor: an alias for `Reg<BCDMIN_SPEC>`"]
pub type BCDMIN = crate::Reg<bcdmin::BCDMIN_SPEC>;
#[doc = "BCD Minute register"]
pub mod bcdmin;
#[doc = "BCDHOUR register accessor: an alias for `Reg<BCDHOUR_SPEC>`"]
pub type BCDHOUR = crate::Reg<bcdhour::BCDHOUR_SPEC>;
#[doc = "BCD Hour register"]
pub mod bcdhour;
#[doc = "BCDDAY register accessor: an alias for `Reg<BCDDAY_SPEC>`"]
pub type BCDDAY = crate::Reg<bcdday::BCDDAY_SPEC>;
#[doc = "BCD Day register"]
pub mod bcdday;
#[doc = "BCDDATE register accessor: an alias for `Reg<BCDDATE_SPEC>`"]
pub type BCDDATE = crate::Reg<bcddate::BCDDATE_SPEC>;
#[doc = "BCD Date register"]
pub mod bcddate;
#[doc = "BCDMON register accessor: an alias for `Reg<BCDMON_SPEC>`"]
pub type BCDMON = crate::Reg<bcdmon::BCDMON_SPEC>;
#[doc = "BCD Month register"]
pub mod bcdmon;
#[doc = "BCDYEAR register accessor: an alias for `Reg<BCDYEAR_SPEC>`"]
pub type BCDYEAR = crate::Reg<bcdyear::BCDYEAR_SPEC>;
#[doc = "BCD Year register"]
pub mod bcdyear;
#[doc = "PRESEC register accessor: an alias for `Reg<PRESEC_SPEC>`"]
pub type PRESEC = crate::Reg<presec::PRESEC_SPEC>;
#[doc = "Predetermining Second register"]
pub mod presec;
#[doc = "PREMIN register accessor: an alias for `Reg<PREMIN_SPEC>`"]
pub type PREMIN = crate::Reg<premin::PREMIN_SPEC>;
#[doc = "Predetermining Minute register"]
pub mod premin;
#[doc = "PREHOUR register accessor: an alias for `Reg<PREHOUR_SPEC>`"]
pub type PREHOUR = crate::Reg<prehour::PREHOUR_SPEC>;
#[doc = "Predetermining Hour register"]
pub mod prehour;
#[doc = "PREDAY register accessor: an alias for `Reg<PREDAY_SPEC>`"]
pub type PREDAY = crate::Reg<preday::PREDAY_SPEC>;
#[doc = "Predetermining Day register"]
pub mod preday;
#[doc = "PREDATE register accessor: an alias for `Reg<PREDATE_SPEC>`"]
pub type PREDATE = crate::Reg<predate::PREDATE_SPEC>;
#[doc = "Predetermining Date register"]
pub mod predate;
#[doc = "PREMON register accessor: an alias for `Reg<PREMON_SPEC>`"]
pub type PREMON = crate::Reg<premon::PREMON_SPEC>;
#[doc = "Predetermining Month register"]
pub mod premon;
#[doc = "PREYEAR register accessor: an alias for `Reg<PREYEAR_SPEC>`"]
pub type PREYEAR = crate::Reg<preyear::PREYEAR_SPEC>;
#[doc = "Predetermining Year register"]
pub mod preyear;
#[doc = "RTCTIME0 register accessor: an alias for `Reg<RTCTIME0_SPEC>`"]
pub type RTCTIME0 = crate::Reg<rtctime0::RTCTIME0_SPEC>;
#[doc = "Consolidated Time0 register"]
pub mod rtctime0;
#[doc = "RTCTIME1 register accessor: an alias for `Reg<RTCTIME1_SPEC>`"]
pub type RTCTIME1 = crate::Reg<rtctime1::RTCTIME1_SPEC>;
#[doc = "Consolidated Time1 register"]
pub mod rtctime1;
