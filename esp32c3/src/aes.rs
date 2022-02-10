#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Key material key_0 configure register"]
    pub key_0: crate::Reg<key_0::KEY_0_SPEC>,
    #[doc = "0x04 - Key material key_1 configure register"]
    pub key_1: crate::Reg<key_1::KEY_1_SPEC>,
    #[doc = "0x08 - Key material key_2 configure register"]
    pub key_2: crate::Reg<key_2::KEY_2_SPEC>,
    #[doc = "0x0c - Key material key_3 configure register"]
    pub key_3: crate::Reg<key_3::KEY_3_SPEC>,
    #[doc = "0x10 - Key material key_4 configure register"]
    pub key_4: crate::Reg<key_4::KEY_4_SPEC>,
    #[doc = "0x14 - Key material key_5 configure register"]
    pub key_5: crate::Reg<key_5::KEY_5_SPEC>,
    #[doc = "0x18 - Key material key_6 configure register"]
    pub key_6: crate::Reg<key_6::KEY_6_SPEC>,
    #[doc = "0x1c - Key material key_7 configure register"]
    pub key_7: crate::Reg<key_7::KEY_7_SPEC>,
    #[doc = "0x20 - source text material text_in_0 configure register"]
    pub text_in_0: crate::Reg<text_in_0::TEXT_IN_0_SPEC>,
    #[doc = "0x24 - source text material text_in_1 configure register"]
    pub text_in_1: crate::Reg<text_in_1::TEXT_IN_1_SPEC>,
    #[doc = "0x28 - source text material text_in_2 configure register"]
    pub text_in_2: crate::Reg<text_in_2::TEXT_IN_2_SPEC>,
    #[doc = "0x2c - source text material text_in_3 configure register"]
    pub text_in_3: crate::Reg<text_in_3::TEXT_IN_3_SPEC>,
    #[doc = "0x30 - result text material text_out_0 configure register"]
    pub text_out_0: crate::Reg<text_out_0::TEXT_OUT_0_SPEC>,
    #[doc = "0x34 - result text material text_out_1 configure register"]
    pub text_out_1: crate::Reg<text_out_1::TEXT_OUT_1_SPEC>,
    #[doc = "0x38 - result text material text_out_2 configure register"]
    pub text_out_2: crate::Reg<text_out_2::TEXT_OUT_2_SPEC>,
    #[doc = "0x3c - result text material text_out_3 configure register"]
    pub text_out_3: crate::Reg<text_out_3::TEXT_OUT_3_SPEC>,
    #[doc = "0x40 - AES Mode register"]
    pub mode: crate::Reg<mode::MODE_SPEC>,
    #[doc = "0x44 - AES Endian configure register"]
    pub endian: crate::Reg<endian::ENDIAN_SPEC>,
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
    #[doc = "0x90 - DMA-AES working mode register"]
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
    pub int_clear: crate::Reg<int_clear::INT_CLEAR_SPEC>,
    #[doc = "0xb0 - AES Interrupt enable register"]
    pub int_ena: crate::Reg<int_ena::INT_ENA_SPEC>,
    #[doc = "0xb4 - AES version control register"]
    pub date: crate::Reg<date::DATE_SPEC>,
    #[doc = "0xb8 - AES-DMA exit config"]
    pub dma_exit: crate::Reg<dma_exit::DMA_EXIT_SPEC>,
}
#[doc = "KEY_0 register accessor: an alias for `Reg<KEY_0_SPEC>`"]
pub type KEY_0 = crate::Reg<key_0::KEY_0_SPEC>;
#[doc = "Key material key_0 configure register"]
pub mod key_0;
#[doc = "KEY_1 register accessor: an alias for `Reg<KEY_1_SPEC>`"]
pub type KEY_1 = crate::Reg<key_1::KEY_1_SPEC>;
#[doc = "Key material key_1 configure register"]
pub mod key_1;
#[doc = "KEY_2 register accessor: an alias for `Reg<KEY_2_SPEC>`"]
pub type KEY_2 = crate::Reg<key_2::KEY_2_SPEC>;
#[doc = "Key material key_2 configure register"]
pub mod key_2;
#[doc = "KEY_3 register accessor: an alias for `Reg<KEY_3_SPEC>`"]
pub type KEY_3 = crate::Reg<key_3::KEY_3_SPEC>;
#[doc = "Key material key_3 configure register"]
pub mod key_3;
#[doc = "KEY_4 register accessor: an alias for `Reg<KEY_4_SPEC>`"]
pub type KEY_4 = crate::Reg<key_4::KEY_4_SPEC>;
#[doc = "Key material key_4 configure register"]
pub mod key_4;
#[doc = "KEY_5 register accessor: an alias for `Reg<KEY_5_SPEC>`"]
pub type KEY_5 = crate::Reg<key_5::KEY_5_SPEC>;
#[doc = "Key material key_5 configure register"]
pub mod key_5;
#[doc = "KEY_6 register accessor: an alias for `Reg<KEY_6_SPEC>`"]
pub type KEY_6 = crate::Reg<key_6::KEY_6_SPEC>;
#[doc = "Key material key_6 configure register"]
pub mod key_6;
#[doc = "KEY_7 register accessor: an alias for `Reg<KEY_7_SPEC>`"]
pub type KEY_7 = crate::Reg<key_7::KEY_7_SPEC>;
#[doc = "Key material key_7 configure register"]
pub mod key_7;
#[doc = "TEXT_IN_0 register accessor: an alias for `Reg<TEXT_IN_0_SPEC>`"]
pub type TEXT_IN_0 = crate::Reg<text_in_0::TEXT_IN_0_SPEC>;
#[doc = "source text material text_in_0 configure register"]
pub mod text_in_0;
#[doc = "TEXT_IN_1 register accessor: an alias for `Reg<TEXT_IN_1_SPEC>`"]
pub type TEXT_IN_1 = crate::Reg<text_in_1::TEXT_IN_1_SPEC>;
#[doc = "source text material text_in_1 configure register"]
pub mod text_in_1;
#[doc = "TEXT_IN_2 register accessor: an alias for `Reg<TEXT_IN_2_SPEC>`"]
pub type TEXT_IN_2 = crate::Reg<text_in_2::TEXT_IN_2_SPEC>;
#[doc = "source text material text_in_2 configure register"]
pub mod text_in_2;
#[doc = "TEXT_IN_3 register accessor: an alias for `Reg<TEXT_IN_3_SPEC>`"]
pub type TEXT_IN_3 = crate::Reg<text_in_3::TEXT_IN_3_SPEC>;
#[doc = "source text material text_in_3 configure register"]
pub mod text_in_3;
#[doc = "TEXT_OUT_0 register accessor: an alias for `Reg<TEXT_OUT_0_SPEC>`"]
pub type TEXT_OUT_0 = crate::Reg<text_out_0::TEXT_OUT_0_SPEC>;
#[doc = "result text material text_out_0 configure register"]
pub mod text_out_0;
#[doc = "TEXT_OUT_1 register accessor: an alias for `Reg<TEXT_OUT_1_SPEC>`"]
pub type TEXT_OUT_1 = crate::Reg<text_out_1::TEXT_OUT_1_SPEC>;
#[doc = "result text material text_out_1 configure register"]
pub mod text_out_1;
#[doc = "TEXT_OUT_2 register accessor: an alias for `Reg<TEXT_OUT_2_SPEC>`"]
pub type TEXT_OUT_2 = crate::Reg<text_out_2::TEXT_OUT_2_SPEC>;
#[doc = "result text material text_out_2 configure register"]
pub mod text_out_2;
#[doc = "TEXT_OUT_3 register accessor: an alias for `Reg<TEXT_OUT_3_SPEC>`"]
pub type TEXT_OUT_3 = crate::Reg<text_out_3::TEXT_OUT_3_SPEC>;
#[doc = "result text material text_out_3 configure register"]
pub mod text_out_3;
#[doc = "MODE register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "AES Mode register"]
pub mod mode;
#[doc = "ENDIAN register accessor: an alias for `Reg<ENDIAN_SPEC>`"]
pub type ENDIAN = crate::Reg<endian::ENDIAN_SPEC>;
#[doc = "AES Endian configure register"]
pub mod endian;
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
#[doc = "DMA-AES working mode register"]
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
#[doc = "INT_CLEAR register accessor: an alias for `Reg<INT_CLEAR_SPEC>`"]
pub type INT_CLEAR = crate::Reg<int_clear::INT_CLEAR_SPEC>;
#[doc = "AES Interrupt clear register"]
pub mod int_clear;
#[doc = "INT_ENA register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "AES Interrupt enable register"]
pub mod int_ena;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "AES version control register"]
pub mod date;
#[doc = "DMA_EXIT register accessor: an alias for `Reg<DMA_EXIT_SPEC>`"]
pub type DMA_EXIT = crate::Reg<dma_exit::DMA_EXIT_SPEC>;
#[doc = "AES-DMA exit config"]
pub mod dma_exit;
