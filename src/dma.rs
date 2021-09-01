#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA status register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x04 - DMA configuration register"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x08 - Channel Control Data Base Pointer register"]
    pub ctrl_base_ptr: crate::Reg<ctrl_base_ptr::CTRL_BASE_PTR_SPEC>,
    #[doc = "0x0c - Channel Alternate Control Data Base Pointer register"]
    pub alt_ctrl_base_ptr: crate::Reg<alt_ctrl_base_ptr::ALT_CTRL_BASE_PTR_SPEC>,
    #[doc = "0x10 - Channel Wait On Request Status register"]
    pub waitonreq_status: crate::Reg<waitonreq_status::WAITONREQ_STATUS_SPEC>,
    #[doc = "0x14 - Channel Software Request register"]
    pub chnl_sw_request: crate::Reg<chnl_sw_request::CHNL_SW_REQUEST_SPEC>,
    #[doc = "0x18 - Channel UseBurst Set register"]
    pub chnl_useburst_set: crate::Reg<chnl_useburst_set::CHNL_USEBURST_SET_SPEC>,
    #[doc = "0x1c - Channel UseBurst Clear register"]
    pub chnl_useburst_clr: crate::Reg<chnl_useburst_clr::CHNL_USEBURST_CLR_SPEC>,
    #[doc = "0x20 - Channel Request Mask Set register"]
    pub chnl_req_mask_set: crate::Reg<chnl_req_mask_set::CHNL_REQ_MASK_SET_SPEC>,
    #[doc = "0x24 - Channel Request Mask Clear register"]
    pub chnl_req_mask_clr: crate::Reg<chnl_req_mask_clr::CHNL_REQ_MASK_CLR_SPEC>,
    #[doc = "0x28 - Channel Enable Set register"]
    pub chnl_enable_set: crate::Reg<chnl_enable_set::CHNL_ENABLE_SET_SPEC>,
    #[doc = "0x2c - Channel Enable Clear register"]
    pub chnl_enable_clr: crate::Reg<chnl_enable_clr::CHNL_ENABLE_CLR_SPEC>,
    #[doc = "0x30 - Channel Primary-Alterante Set register"]
    pub chnl_pri_alt_set: crate::Reg<chnl_pri_alt_set::CHNL_PRI_ALT_SET_SPEC>,
    #[doc = "0x34 - Channel Primary-Alterante Clear register"]
    pub chnl_pri_alt_clr: crate::Reg<chnl_pri_alt_clr::CHNL_PRI_ALT_CLR_SPEC>,
    #[doc = "0x38 - Channel Priority Set register"]
    pub chnl_priority_set: crate::Reg<chnl_priority_set::CHNL_PRIORITY_SET_SPEC>,
    #[doc = "0x3c - Channel Priority Clear register"]
    pub chnl_priority_clr: crate::Reg<chnl_priority_clr::CHNL_PRIORITY_CLR_SPEC>,
    _reserved16: [u8; 0x0c],
    #[doc = "0x4c - Bus Error Clear register"]
    pub err_clr: crate::Reg<err_clr::ERR_CLR_SPEC>,
}
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "DMA status register"]
pub mod status;
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "DMA configuration register"]
pub mod cfg;
#[doc = "CTRL_BASE_PTR register accessor: an alias for `Reg<CTRL_BASE_PTR_SPEC>`"]
pub type CTRL_BASE_PTR = crate::Reg<ctrl_base_ptr::CTRL_BASE_PTR_SPEC>;
#[doc = "Channel Control Data Base Pointer register"]
pub mod ctrl_base_ptr;
#[doc = "ALT_CTRL_BASE_PTR register accessor: an alias for `Reg<ALT_CTRL_BASE_PTR_SPEC>`"]
pub type ALT_CTRL_BASE_PTR = crate::Reg<alt_ctrl_base_ptr::ALT_CTRL_BASE_PTR_SPEC>;
#[doc = "Channel Alternate Control Data Base Pointer register"]
pub mod alt_ctrl_base_ptr;
#[doc = "WAITONREQ_STATUS register accessor: an alias for `Reg<WAITONREQ_STATUS_SPEC>`"]
pub type WAITONREQ_STATUS = crate::Reg<waitonreq_status::WAITONREQ_STATUS_SPEC>;
#[doc = "Channel Wait On Request Status register"]
pub mod waitonreq_status;
#[doc = "CHNL_SW_REQUEST register accessor: an alias for `Reg<CHNL_SW_REQUEST_SPEC>`"]
pub type CHNL_SW_REQUEST = crate::Reg<chnl_sw_request::CHNL_SW_REQUEST_SPEC>;
#[doc = "Channel Software Request register"]
pub mod chnl_sw_request;
#[doc = "CHNL_USEBURST_SET register accessor: an alias for `Reg<CHNL_USEBURST_SET_SPEC>`"]
pub type CHNL_USEBURST_SET = crate::Reg<chnl_useburst_set::CHNL_USEBURST_SET_SPEC>;
#[doc = "Channel UseBurst Set register"]
pub mod chnl_useburst_set;
#[doc = "CHNL_USEBURST_CLR register accessor: an alias for `Reg<CHNL_USEBURST_CLR_SPEC>`"]
pub type CHNL_USEBURST_CLR = crate::Reg<chnl_useburst_clr::CHNL_USEBURST_CLR_SPEC>;
#[doc = "Channel UseBurst Clear register"]
pub mod chnl_useburst_clr;
#[doc = "CHNL_REQ_MASK_SET register accessor: an alias for `Reg<CHNL_REQ_MASK_SET_SPEC>`"]
pub type CHNL_REQ_MASK_SET = crate::Reg<chnl_req_mask_set::CHNL_REQ_MASK_SET_SPEC>;
#[doc = "Channel Request Mask Set register"]
pub mod chnl_req_mask_set;
#[doc = "CHNL_REQ_MASK_CLR register accessor: an alias for `Reg<CHNL_REQ_MASK_CLR_SPEC>`"]
pub type CHNL_REQ_MASK_CLR = crate::Reg<chnl_req_mask_clr::CHNL_REQ_MASK_CLR_SPEC>;
#[doc = "Channel Request Mask Clear register"]
pub mod chnl_req_mask_clr;
#[doc = "CHNL_ENABLE_SET register accessor: an alias for `Reg<CHNL_ENABLE_SET_SPEC>`"]
pub type CHNL_ENABLE_SET = crate::Reg<chnl_enable_set::CHNL_ENABLE_SET_SPEC>;
#[doc = "Channel Enable Set register"]
pub mod chnl_enable_set;
#[doc = "CHNL_ENABLE_CLR register accessor: an alias for `Reg<CHNL_ENABLE_CLR_SPEC>`"]
pub type CHNL_ENABLE_CLR = crate::Reg<chnl_enable_clr::CHNL_ENABLE_CLR_SPEC>;
#[doc = "Channel Enable Clear register"]
pub mod chnl_enable_clr;
#[doc = "CHNL_PRI_ALT_SET register accessor: an alias for `Reg<CHNL_PRI_ALT_SET_SPEC>`"]
pub type CHNL_PRI_ALT_SET = crate::Reg<chnl_pri_alt_set::CHNL_PRI_ALT_SET_SPEC>;
#[doc = "Channel Primary-Alterante Set register"]
pub mod chnl_pri_alt_set;
#[doc = "CHNL_PRI_ALT_CLR register accessor: an alias for `Reg<CHNL_PRI_ALT_CLR_SPEC>`"]
pub type CHNL_PRI_ALT_CLR = crate::Reg<chnl_pri_alt_clr::CHNL_PRI_ALT_CLR_SPEC>;
#[doc = "Channel Primary-Alterante Clear register"]
pub mod chnl_pri_alt_clr;
#[doc = "CHNL_PRIORITY_SET register accessor: an alias for `Reg<CHNL_PRIORITY_SET_SPEC>`"]
pub type CHNL_PRIORITY_SET = crate::Reg<chnl_priority_set::CHNL_PRIORITY_SET_SPEC>;
#[doc = "Channel Priority Set register"]
pub mod chnl_priority_set;
#[doc = "CHNL_PRIORITY_CLR register accessor: an alias for `Reg<CHNL_PRIORITY_CLR_SPEC>`"]
pub type CHNL_PRIORITY_CLR = crate::Reg<chnl_priority_clr::CHNL_PRIORITY_CLR_SPEC>;
#[doc = "Channel Priority Clear register"]
pub mod chnl_priority_clr;
#[doc = "ERR_CLR register accessor: an alias for `Reg<ERR_CLR_SPEC>`"]
pub type ERR_CLR = crate::Reg<err_clr::ERR_CLR_SPEC>;
#[doc = "Bus Error Clear register"]
pub mod err_clr;
