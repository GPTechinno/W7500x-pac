#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog timer Load register"]
    pub load: crate::Reg<load::LOAD_SPEC>,
    #[doc = "0x04 - Watchdog timer Value register"]
    pub value: crate::Reg<value::VALUE_SPEC>,
    #[doc = "0x08 - Watchdog timer Control register"]
    pub control: crate::Reg<control::CONTROL_SPEC>,
    #[doc = "0x0c - Watchdog timer Clear Interrupt register"]
    pub intclr: crate::Reg<intclr::INTCLR_SPEC>,
    #[doc = "0x10 - Watchdog timer Raw Interrupt Status register"]
    pub ris: crate::Reg<ris::RIS_SPEC>,
    #[doc = "0x14 - Watchdog timer Masked Interrupt Status register"]
    pub mis: crate::Reg<mis::MIS_SPEC>,
    _reserved6: [u8; 0x0be8],
    #[doc = "0xc00 - Watchdog timer Lock register"]
    pub lock: crate::Reg<lock::LOCK_SPEC>,
}
#[doc = "LOAD register accessor: an alias for `Reg<LOAD_SPEC>`"]
pub type LOAD = crate::Reg<load::LOAD_SPEC>;
#[doc = "Watchdog timer Load register"]
pub mod load;
#[doc = "VALUE register accessor: an alias for `Reg<VALUE_SPEC>`"]
pub type VALUE = crate::Reg<value::VALUE_SPEC>;
#[doc = "Watchdog timer Value register"]
pub mod value;
#[doc = "CONTROL register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Watchdog timer Control register"]
pub mod control;
#[doc = "INTCLR register accessor: an alias for `Reg<INTCLR_SPEC>`"]
pub type INTCLR = crate::Reg<intclr::INTCLR_SPEC>;
#[doc = "Watchdog timer Clear Interrupt register"]
pub mod intclr;
#[doc = "RIS register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "Watchdog timer Raw Interrupt Status register"]
pub mod ris;
#[doc = "MIS register accessor: an alias for `Reg<MIS_SPEC>`"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "Watchdog timer Masked Interrupt Status register"]
pub mod mis;
#[doc = "LOCK register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Watchdog timer Lock register"]
pub mod lock;
