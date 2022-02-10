#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x20 - AES key register %s"]
    pub key_: [crate::Reg<key_::KEY__SPEC>; 8],
    #[doc = "0x20..0x30 - Source data register %s"]
    pub text_in_: [crate::Reg<text_in_::TEXT_IN__SPEC>; 4],
    #[doc = "0x30..0x40 - Result data register %s"]
    pub text_out_: [crate::Reg<text_out_::TEXT_OUT__SPEC>; 4],
    #[doc = "0x40 - AES Mode register"]
    pub mode: crate::Reg<mode::MODE_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x48 - AES trigger register"]
    pub trigger: crate::Reg<trigger::TRIGGER_SPEC>,
    #[doc = "0x4c - AES state register"]
    pub state: crate::Reg<state::STATE_SPEC>,
    #[doc = "0x50..0x60 - The memory that stores initialization vector"]
    pub iv_mem: [crate::Reg<iv_mem::IV_MEM_SPEC>; 16],
    #[doc = "0x60..0x70 - The memory that stores GCM hash subkey"]
    pub h_mem: [crate::Reg<h_mem::H_MEM_SPEC>; 16],
    #[doc = "0x70..0x80 - The memory that stores J0"]
    pub j0_mem: [crate::Reg<j0_mem::J0_MEM_SPEC>; 16],
    #[doc = "0x80..0x90 - The memory that stores T0"]
    pub t0_mem: [crate::Reg<t0_mem::T0_MEM_SPEC>; 16],
    #[doc = "0x90 - AES accelerator working mode register"]
    pub dma_enable: crate::Reg<dma_enable::DMA_ENABLE_SPEC>,
    #[doc = "0x94 - AES cipher block mode register"]
    pub block_mode: crate::Reg<block_mode::BLOCK_MODE_SPEC>,
    #[doc = "0x98 - AES block number register"]
    pub block_num: crate::Reg<block_num::BLOCK_NUM_SPEC>,
    #[doc = "0x9c - Standard incrementing function configure register"]
    pub inc_sel: crate::Reg<inc_sel::INC_SEL_SPEC>,
    #[doc = "0xa0 - Additional Authential Data block number register"]
    pub aad_block_num: crate::Reg<aad_block_num::AAD_BLOCK_NUM_SPEC>,
    #[doc = "0xa4 - AES remainder bit number register"]
    pub remainder_bit_num: crate::Reg<remainder_bit_num::REMAINDER_BIT_NUM_SPEC>,
    #[doc = "0xa8 - AES continue register"]
    pub continue_: crate::Reg<continue_::CONTINUE_SPEC>,
    #[doc = "0xac - AES Interrupt clear register"]
    pub int_clr: crate::Reg<int_clr::INT_CLR_SPEC>,
    #[doc = "0xb0 - DMA-AES Interrupt enable register"]
    pub int_ena: crate::Reg<int_ena::INT_ENA_SPEC>,
    #[doc = "0xb4 - AES version control register"]
    pub date: crate::Reg<date::DATE_SPEC>,
    #[doc = "0xb8 - AES-DMA exit config"]
    pub dma_exit: crate::Reg<dma_exit::DMA_EXIT_SPEC>,
}
#[doc = "KEY_ register accessor: an alias for `Reg<KEY__SPEC>`"]
pub type KEY_ = crate::Reg<key_::KEY__SPEC>;
#[doc = "AES key register %s"]
pub mod key_;
#[doc = "TEXT_IN_ register accessor: an alias for `Reg<TEXT_IN__SPEC>`"]
pub type TEXT_IN_ = crate::Reg<text_in_::TEXT_IN__SPEC>;
#[doc = "Source data register %s"]
pub mod text_in_;
#[doc = "TEXT_OUT_ register accessor: an alias for `Reg<TEXT_OUT__SPEC>`"]
pub type TEXT_OUT_ = crate::Reg<text_out_::TEXT_OUT__SPEC>;
#[doc = "Result data register %s"]
pub mod text_out_;
#[doc = "MODE register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "AES Mode register"]
pub mod mode;
#[doc = "TRIGGER register accessor: an alias for `Reg<TRIGGER_SPEC>`"]
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
#[doc = "AES trigger register"]
pub mod trigger;
#[doc = "STATE register accessor: an alias for `Reg<STATE_SPEC>`"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "AES state register"]
pub mod state;
#[doc = "IV_MEM register accessor: an alias for `Reg<IV_MEM_SPEC>`"]
pub type IV_MEM = crate::Reg<iv_mem::IV_MEM_SPEC>;
#[doc = "The memory that stores initialization vector"]
pub mod iv_mem;
#[doc = "H_MEM register accessor: an alias for `Reg<H_MEM_SPEC>`"]
pub type H_MEM = crate::Reg<h_mem::H_MEM_SPEC>;
#[doc = "The memory that stores GCM hash subkey"]
pub mod h_mem;
#[doc = "J0_MEM register accessor: an alias for `Reg<J0_MEM_SPEC>`"]
pub type J0_MEM = crate::Reg<j0_mem::J0_MEM_SPEC>;
#[doc = "The memory that stores J0"]
pub mod j0_mem;
#[doc = "T0_MEM register accessor: an alias for `Reg<T0_MEM_SPEC>`"]
pub type T0_MEM = crate::Reg<t0_mem::T0_MEM_SPEC>;
#[doc = "The memory that stores T0"]
pub mod t0_mem;
#[doc = "DMA_ENABLE register accessor: an alias for `Reg<DMA_ENABLE_SPEC>`"]
pub type DMA_ENABLE = crate::Reg<dma_enable::DMA_ENABLE_SPEC>;
#[doc = "AES accelerator working mode register"]
pub mod dma_enable;
#[doc = "BLOCK_MODE register accessor: an alias for `Reg<BLOCK_MODE_SPEC>`"]
pub type BLOCK_MODE = crate::Reg<block_mode::BLOCK_MODE_SPEC>;
#[doc = "AES cipher block mode register"]
pub mod block_mode;
#[doc = "BLOCK_NUM register accessor: an alias for `Reg<BLOCK_NUM_SPEC>`"]
pub type BLOCK_NUM = crate::Reg<block_num::BLOCK_NUM_SPEC>;
#[doc = "AES block number register"]
pub mod block_num;
#[doc = "INC_SEL register accessor: an alias for `Reg<INC_SEL_SPEC>`"]
pub type INC_SEL = crate::Reg<inc_sel::INC_SEL_SPEC>;
#[doc = "Standard incrementing function configure register"]
pub mod inc_sel;
#[doc = "AAD_BLOCK_NUM register accessor: an alias for `Reg<AAD_BLOCK_NUM_SPEC>`"]
pub type AAD_BLOCK_NUM = crate::Reg<aad_block_num::AAD_BLOCK_NUM_SPEC>;
#[doc = "Additional Authential Data block number register"]
pub mod aad_block_num;
#[doc = "REMAINDER_BIT_NUM register accessor: an alias for `Reg<REMAINDER_BIT_NUM_SPEC>`"]
pub type REMAINDER_BIT_NUM = crate::Reg<remainder_bit_num::REMAINDER_BIT_NUM_SPEC>;
#[doc = "AES remainder bit number register"]
pub mod remainder_bit_num;
#[doc = "CONTINUE register accessor: an alias for `Reg<CONTINUE_SPEC>`"]
pub type CONTINUE = crate::Reg<continue_::CONTINUE_SPEC>;
#[doc = "AES continue register"]
pub mod continue_;
#[doc = "INT_CLR register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "AES Interrupt clear register"]
pub mod int_clr;
#[doc = "INT_ENA register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "DMA-AES Interrupt enable register"]
pub mod int_ena;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "AES version control register"]
pub mod date;
#[doc = "DMA_EXIT register accessor: an alias for `Reg<DMA_EXIT_SPEC>`"]
pub type DMA_EXIT = crate::Reg<dma_exit::DMA_EXIT_SPEC>;
#[doc = "AES-DMA exit config"]
pub mod dma_exit;
