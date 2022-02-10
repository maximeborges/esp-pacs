#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Defines the algorithm of SHA accelerator"]
    pub mode: crate::Reg<mode::MODE_SPEC>,
    #[doc = "0x04 - String content register for calculating initial Hash Value (only effective for SHA-512/t)"]
    pub t_string: crate::Reg<t_string::T_STRING_SPEC>,
    #[doc = "0x08 - String length register for calculating initial Hash Value (only effective for SHA-512/t)"]
    pub t_length: crate::Reg<t_length::T_LENGTH_SPEC>,
    #[doc = "0x0c - Block number register (only effective for DMA-SHA)"]
    pub dma_block_num: crate::Reg<dma_block_num::DMA_BLOCK_NUM_SPEC>,
    #[doc = "0x10 - Starts the SHA accelerator for Typical SHA operation"]
    pub start: crate::Reg<start::START_SPEC>,
    #[doc = "0x14 - Continues SHA operation (only effective in Typical SHA mode)"]
    pub continue_op: crate::Reg<continue_op::CONTINUE_OP_SPEC>,
    #[doc = "0x18 - Indicates if SHA Accelerator is busy or not"]
    pub busy: crate::Reg<busy::BUSY_SPEC>,
    #[doc = "0x1c - Starts the SHA accelerator for DMA-SHA operation"]
    pub dma_start: crate::Reg<dma_start::DMA_START_SPEC>,
    #[doc = "0x20 - Continues SHA operation (only effective in DMA-SHA mode)"]
    pub dma_continue: crate::Reg<dma_continue::DMA_CONTINUE_SPEC>,
    #[doc = "0x24 - DMA-SHA interrupt clear register"]
    pub int_clear: crate::Reg<int_clear::INT_CLEAR_SPEC>,
    #[doc = "0x28 - DMA-SHA interrupt enable register"]
    pub int_ena: crate::Reg<int_ena::INT_ENA_SPEC>,
    #[doc = "0x2c - Version control register"]
    pub date: crate::Reg<date::DATE_SPEC>,
    _reserved12: [u8; 0x10],
    #[doc = "0x40..0x80 - Hash value"]
    pub h_: [crate::Reg<h_::H__SPEC>; 16],
    #[doc = "0x80..0x100 - Message"]
    pub m_: [crate::Reg<m_::M__SPEC>; 32],
}
#[doc = "MODE register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Defines the algorithm of SHA accelerator"]
pub mod mode;
#[doc = "T_STRING register accessor: an alias for `Reg<T_STRING_SPEC>`"]
pub type T_STRING = crate::Reg<t_string::T_STRING_SPEC>;
#[doc = "String content register for calculating initial Hash Value (only effective for SHA-512/t)"]
pub mod t_string;
#[doc = "T_LENGTH register accessor: an alias for `Reg<T_LENGTH_SPEC>`"]
pub type T_LENGTH = crate::Reg<t_length::T_LENGTH_SPEC>;
#[doc = "String length register for calculating initial Hash Value (only effective for SHA-512/t)"]
pub mod t_length;
#[doc = "DMA_BLOCK_NUM register accessor: an alias for `Reg<DMA_BLOCK_NUM_SPEC>`"]
pub type DMA_BLOCK_NUM = crate::Reg<dma_block_num::DMA_BLOCK_NUM_SPEC>;
#[doc = "Block number register (only effective for DMA-SHA)"]
pub mod dma_block_num;
#[doc = "START register accessor: an alias for `Reg<START_SPEC>`"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = "Starts the SHA accelerator for Typical SHA operation"]
pub mod start;
#[doc = "CONTINUE_OP register accessor: an alias for `Reg<CONTINUE_OP_SPEC>`"]
pub type CONTINUE_OP = crate::Reg<continue_op::CONTINUE_OP_SPEC>;
#[doc = "Continues SHA operation (only effective in Typical SHA mode)"]
pub mod continue_op;
#[doc = "BUSY register accessor: an alias for `Reg<BUSY_SPEC>`"]
pub type BUSY = crate::Reg<busy::BUSY_SPEC>;
#[doc = "Indicates if SHA Accelerator is busy or not"]
pub mod busy;
#[doc = "DMA_START register accessor: an alias for `Reg<DMA_START_SPEC>`"]
pub type DMA_START = crate::Reg<dma_start::DMA_START_SPEC>;
#[doc = "Starts the SHA accelerator for DMA-SHA operation"]
pub mod dma_start;
#[doc = "DMA_CONTINUE register accessor: an alias for `Reg<DMA_CONTINUE_SPEC>`"]
pub type DMA_CONTINUE = crate::Reg<dma_continue::DMA_CONTINUE_SPEC>;
#[doc = "Continues SHA operation (only effective in DMA-SHA mode)"]
pub mod dma_continue;
#[doc = "INT_CLEAR register accessor: an alias for `Reg<INT_CLEAR_SPEC>`"]
pub type INT_CLEAR = crate::Reg<int_clear::INT_CLEAR_SPEC>;
#[doc = "DMA-SHA interrupt clear register"]
pub mod int_clear;
#[doc = "INT_ENA register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "DMA-SHA interrupt enable register"]
pub mod int_ena;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
#[doc = "H_ register accessor: an alias for `Reg<H__SPEC>`"]
pub type H_ = crate::Reg<h_::H__SPEC>;
#[doc = "Hash value"]
pub mod h_;
#[doc = "M_ register accessor: an alias for `Reg<M__SPEC>`"]
pub type M_ = crate::Reg<m_::M__SPEC>;
#[doc = "Message"]
pub mod m_;
