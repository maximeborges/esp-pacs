#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - x"]
    pub config: crate::Reg<config::CONFIG_SPEC>,
    #[doc = "0x04 - x"]
    pub apb_addr: crate::Reg<apb_addr::APB_ADDR_SPEC>,
    #[doc = "0x08 - x"]
    pub mem_addr: crate::Reg<mem_addr::MEM_ADDR_SPEC>,
    #[doc = "0x0c - x"]
    pub reg_map0: crate::Reg<reg_map0::REG_MAP0_SPEC>,
    #[doc = "0x10 - x"]
    pub reg_map1: crate::Reg<reg_map1::REG_MAP1_SPEC>,
    #[doc = "0x14 - x"]
    pub reg_map2: crate::Reg<reg_map2::REG_MAP2_SPEC>,
    #[doc = "0x18 - x"]
    pub reg_map3: crate::Reg<reg_map3::REG_MAP3_SPEC>,
    #[doc = "0x1c - x"]
    pub int_raw: crate::Reg<int_raw::INT_RAW_SPEC>,
    #[doc = "0x20 - x"]
    pub int_st: crate::Reg<int_st::INT_ST_SPEC>,
    #[doc = "0x24 - x"]
    pub int_ena: crate::Reg<int_ena::INT_ENA_SPEC>,
    #[doc = "0x28 - x"]
    pub int_clr: crate::Reg<int_clr::INT_CLR_SPEC>,
    _reserved11: [u8; 0xd0],
    #[doc = "0xfc - x"]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "x"]
pub mod config;
#[doc = "APB_ADDR register accessor: an alias for `Reg<APB_ADDR_SPEC>`"]
pub type APB_ADDR = crate::Reg<apb_addr::APB_ADDR_SPEC>;
#[doc = "x"]
pub mod apb_addr;
#[doc = "MEM_ADDR register accessor: an alias for `Reg<MEM_ADDR_SPEC>`"]
pub type MEM_ADDR = crate::Reg<mem_addr::MEM_ADDR_SPEC>;
#[doc = "x"]
pub mod mem_addr;
#[doc = "REG_MAP0 register accessor: an alias for `Reg<REG_MAP0_SPEC>`"]
pub type REG_MAP0 = crate::Reg<reg_map0::REG_MAP0_SPEC>;
#[doc = "x"]
pub mod reg_map0;
#[doc = "REG_MAP1 register accessor: an alias for `Reg<REG_MAP1_SPEC>`"]
pub type REG_MAP1 = crate::Reg<reg_map1::REG_MAP1_SPEC>;
#[doc = "x"]
pub mod reg_map1;
#[doc = "REG_MAP2 register accessor: an alias for `Reg<REG_MAP2_SPEC>`"]
pub type REG_MAP2 = crate::Reg<reg_map2::REG_MAP2_SPEC>;
#[doc = "x"]
pub mod reg_map2;
#[doc = "REG_MAP3 register accessor: an alias for `Reg<REG_MAP3_SPEC>`"]
pub type REG_MAP3 = crate::Reg<reg_map3::REG_MAP3_SPEC>;
#[doc = "x"]
pub mod reg_map3;
#[doc = "INT_RAW register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "x"]
pub mod int_raw;
#[doc = "INT_ST register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "x"]
pub mod int_st;
#[doc = "INT_ENA register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "x"]
pub mod int_ena;
#[doc = "INT_CLR register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "x"]
pub mod int_clr;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "x"]
pub mod date;
