#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data register"]
    pub dr: crate::Reg<dr::DR_SPEC>,
    #[doc = "0x04 - Status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x08 - Control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    _reserved_3_icr: [u8; 0x04],
    #[doc = "0x10 - Baudrate Divider register"]
    pub bdr: crate::Reg<bdr::BDR_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x0c - Interrupt Clear register"]
    #[inline(always)]
    pub fn icr(&self) -> &crate::Reg<icr::ICR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<icr::ICR_SPEC>)
        }
    }
    #[doc = "0x0c - Interrupt Status register"]
    #[inline(always)]
    pub fn isr(&self) -> &crate::Reg<isr::ISR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<isr::ISR_SPEC>)
        }
    }
}
#[doc = "DR register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "Data register"]
pub mod dr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control register"]
pub mod cr;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status register"]
pub mod isr;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt Clear register"]
pub mod icr;
#[doc = "BDR register accessor: an alias for `Reg<BDR_SPEC>`"]
pub type BDR = crate::Reg<bdr::BDR_SPEC>;
#[doc = "Baudrate Divider register"]
pub mod bdr;
