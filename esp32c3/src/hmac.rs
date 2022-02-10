#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x40],
    #[doc = "0x40 - Process control register 0."]
    pub set_start: crate::Reg<set_start::SET_START_SPEC>,
    #[doc = "0x44 - Configure purpose."]
    pub set_para_purpose: crate::Reg<set_para_purpose::SET_PARA_PURPOSE_SPEC>,
    #[doc = "0x48 - Configure key."]
    pub set_para_key: crate::Reg<set_para_key::SET_PARA_KEY_SPEC>,
    #[doc = "0x4c - Finish initial configuration."]
    pub set_para_finish: crate::Reg<set_para_finish::SET_PARA_FINISH_SPEC>,
    #[doc = "0x50 - Process control register 1."]
    pub set_message_one: crate::Reg<set_message_one::SET_MESSAGE_ONE_SPEC>,
    #[doc = "0x54 - Process control register 2."]
    pub set_message_ing: crate::Reg<set_message_ing::SET_MESSAGE_ING_SPEC>,
    #[doc = "0x58 - Process control register 3."]
    pub set_message_end: crate::Reg<set_message_end::SET_MESSAGE_END_SPEC>,
    #[doc = "0x5c - Process control register 4."]
    pub set_result_finish: crate::Reg<set_result_finish::SET_RESULT_FINISH_SPEC>,
    #[doc = "0x60 - Invalidate register 0."]
    pub set_invalidate_jtag: crate::Reg<set_invalidate_jtag::SET_INVALIDATE_JTAG_SPEC>,
    #[doc = "0x64 - Invalidate register 1."]
    pub set_invalidate_ds: crate::Reg<set_invalidate_ds::SET_INVALIDATE_DS_SPEC>,
    #[doc = "0x68 - Error register."]
    pub query_error: crate::Reg<query_error::QUERY_ERROR_SPEC>,
    #[doc = "0x6c - Busy register."]
    pub query_busy: crate::Reg<query_busy::QUERY_BUSY_SPEC>,
    _reserved12: [u8; 0x10],
    #[doc = "0x80..0xc0 - Message block memory."]
    pub wr_message_mem: [crate::Reg<wr_message_mem::WR_MESSAGE_MEM_SPEC>; 64],
    #[doc = "0xc0..0xe0 - Result from upstream."]
    pub rd_result_mem: [crate::Reg<rd_result_mem::RD_RESULT_MEM_SPEC>; 32],
    _reserved14: [u8; 0x10],
    #[doc = "0xf0 - Process control register 5."]
    pub set_message_pad: crate::Reg<set_message_pad::SET_MESSAGE_PAD_SPEC>,
    #[doc = "0xf4 - Process control register 6."]
    pub one_block: crate::Reg<one_block::ONE_BLOCK_SPEC>,
    #[doc = "0xf8 - Jtag register 0."]
    pub soft_jtag_ctrl: crate::Reg<soft_jtag_ctrl::SOFT_JTAG_CTRL_SPEC>,
    #[doc = "0xfc - Jtag register 1."]
    pub wr_jtag: crate::Reg<wr_jtag::WR_JTAG_SPEC>,
}
#[doc = "SET_START register accessor: an alias for `Reg<SET_START_SPEC>`"]
pub type SET_START = crate::Reg<set_start::SET_START_SPEC>;
#[doc = "Process control register 0."]
pub mod set_start;
#[doc = "SET_PARA_PURPOSE register accessor: an alias for `Reg<SET_PARA_PURPOSE_SPEC>`"]
pub type SET_PARA_PURPOSE = crate::Reg<set_para_purpose::SET_PARA_PURPOSE_SPEC>;
#[doc = "Configure purpose."]
pub mod set_para_purpose;
#[doc = "SET_PARA_KEY register accessor: an alias for `Reg<SET_PARA_KEY_SPEC>`"]
pub type SET_PARA_KEY = crate::Reg<set_para_key::SET_PARA_KEY_SPEC>;
#[doc = "Configure key."]
pub mod set_para_key;
#[doc = "SET_PARA_FINISH register accessor: an alias for `Reg<SET_PARA_FINISH_SPEC>`"]
pub type SET_PARA_FINISH = crate::Reg<set_para_finish::SET_PARA_FINISH_SPEC>;
#[doc = "Finish initial configuration."]
pub mod set_para_finish;
#[doc = "SET_MESSAGE_ONE register accessor: an alias for `Reg<SET_MESSAGE_ONE_SPEC>`"]
pub type SET_MESSAGE_ONE = crate::Reg<set_message_one::SET_MESSAGE_ONE_SPEC>;
#[doc = "Process control register 1."]
pub mod set_message_one;
#[doc = "SET_MESSAGE_ING register accessor: an alias for `Reg<SET_MESSAGE_ING_SPEC>`"]
pub type SET_MESSAGE_ING = crate::Reg<set_message_ing::SET_MESSAGE_ING_SPEC>;
#[doc = "Process control register 2."]
pub mod set_message_ing;
#[doc = "SET_MESSAGE_END register accessor: an alias for `Reg<SET_MESSAGE_END_SPEC>`"]
pub type SET_MESSAGE_END = crate::Reg<set_message_end::SET_MESSAGE_END_SPEC>;
#[doc = "Process control register 3."]
pub mod set_message_end;
#[doc = "SET_RESULT_FINISH register accessor: an alias for `Reg<SET_RESULT_FINISH_SPEC>`"]
pub type SET_RESULT_FINISH = crate::Reg<set_result_finish::SET_RESULT_FINISH_SPEC>;
#[doc = "Process control register 4."]
pub mod set_result_finish;
#[doc = "SET_INVALIDATE_JTAG register accessor: an alias for `Reg<SET_INVALIDATE_JTAG_SPEC>`"]
pub type SET_INVALIDATE_JTAG = crate::Reg<set_invalidate_jtag::SET_INVALIDATE_JTAG_SPEC>;
#[doc = "Invalidate register 0."]
pub mod set_invalidate_jtag;
#[doc = "SET_INVALIDATE_DS register accessor: an alias for `Reg<SET_INVALIDATE_DS_SPEC>`"]
pub type SET_INVALIDATE_DS = crate::Reg<set_invalidate_ds::SET_INVALIDATE_DS_SPEC>;
#[doc = "Invalidate register 1."]
pub mod set_invalidate_ds;
#[doc = "QUERY_ERROR register accessor: an alias for `Reg<QUERY_ERROR_SPEC>`"]
pub type QUERY_ERROR = crate::Reg<query_error::QUERY_ERROR_SPEC>;
#[doc = "Error register."]
pub mod query_error;
#[doc = "QUERY_BUSY register accessor: an alias for `Reg<QUERY_BUSY_SPEC>`"]
pub type QUERY_BUSY = crate::Reg<query_busy::QUERY_BUSY_SPEC>;
#[doc = "Busy register."]
pub mod query_busy;
#[doc = "WR_MESSAGE_MEM register accessor: an alias for `Reg<WR_MESSAGE_MEM_SPEC>`"]
pub type WR_MESSAGE_MEM = crate::Reg<wr_message_mem::WR_MESSAGE_MEM_SPEC>;
#[doc = "Message block memory."]
pub mod wr_message_mem;
#[doc = "RD_RESULT_MEM register accessor: an alias for `Reg<RD_RESULT_MEM_SPEC>`"]
pub type RD_RESULT_MEM = crate::Reg<rd_result_mem::RD_RESULT_MEM_SPEC>;
#[doc = "Result from upstream."]
pub mod rd_result_mem;
#[doc = "SET_MESSAGE_PAD register accessor: an alias for `Reg<SET_MESSAGE_PAD_SPEC>`"]
pub type SET_MESSAGE_PAD = crate::Reg<set_message_pad::SET_MESSAGE_PAD_SPEC>;
#[doc = "Process control register 5."]
pub mod set_message_pad;
#[doc = "ONE_BLOCK register accessor: an alias for `Reg<ONE_BLOCK_SPEC>`"]
pub type ONE_BLOCK = crate::Reg<one_block::ONE_BLOCK_SPEC>;
#[doc = "Process control register 6."]
pub mod one_block;
#[doc = "SOFT_JTAG_CTRL register accessor: an alias for `Reg<SOFT_JTAG_CTRL_SPEC>`"]
pub type SOFT_JTAG_CTRL = crate::Reg<soft_jtag_ctrl::SOFT_JTAG_CTRL_SPEC>;
#[doc = "Jtag register 0."]
pub mod soft_jtag_ctrl;
#[doc = "WR_JTAG register accessor: an alias for `Reg<WR_JTAG_SPEC>`"]
pub type WR_JTAG = crate::Reg<wr_jtag::WR_JTAG_SPEC>;
#[doc = "Jtag register 1."]
pub mod wr_jtag;
