#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - configure low scl period"]
    pub scl_low: crate::Reg<scl_low::SCL_LOW_SPEC>,
    #[doc = "0x04 - configure i2c ctrl"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x08 - get i2c status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x0c - configure time out"]
    pub to: crate::Reg<to::TO_SPEC>,
    #[doc = "0x10 - configure slave id"]
    pub slave_addr: crate::Reg<slave_addr::SLAVE_ADDR_SPEC>,
    #[doc = "0x14 - configure high scl period"]
    pub scl_high: crate::Reg<scl_high::SCL_HIGH_SPEC>,
    #[doc = "0x18 - configure sda duty"]
    pub sda_duty: crate::Reg<sda_duty::SDA_DUTY_SPEC>,
    #[doc = "0x1c - configure scl start period"]
    pub scl_start_period: crate::Reg<scl_start_period::SCL_START_PERIOD_SPEC>,
    #[doc = "0x20 - configure scl stop period"]
    pub scl_stop_period: crate::Reg<scl_stop_period::SCL_STOP_PERIOD_SPEC>,
    #[doc = "0x24 - interrupt clear register"]
    pub int_clr: crate::Reg<int_clr::INT_CLR_SPEC>,
    #[doc = "0x28 - interrupt raw register"]
    pub int_raw: crate::Reg<int_raw::INT_RAW_SPEC>,
    #[doc = "0x2c - interrupt state register"]
    pub int_st: crate::Reg<int_st::INT_ST_SPEC>,
    #[doc = "0x30 - interrupt enable register"]
    pub int_ena: crate::Reg<int_ena::INT_ENA_SPEC>,
    #[doc = "0x34 - get i2c data status"]
    pub data: crate::Reg<data::DATA_SPEC>,
    #[doc = "0x38 - i2c commond0 register"]
    pub cmd0: crate::Reg<cmd0::CMD0_SPEC>,
    #[doc = "0x3c - i2c commond1 register"]
    pub cmd1: crate::Reg<cmd1::CMD1_SPEC>,
    #[doc = "0x40 - i2c commond2 register"]
    pub cmd2: crate::Reg<cmd2::CMD2_SPEC>,
    #[doc = "0x44 - i2c commond3 register"]
    pub cmd3: crate::Reg<cmd3::CMD3_SPEC>,
    #[doc = "0x48 - i2c commond4 register"]
    pub cmd4: crate::Reg<cmd4::CMD4_SPEC>,
    #[doc = "0x4c - i2c commond5_register"]
    pub cmd5: crate::Reg<cmd5::CMD5_SPEC>,
    #[doc = "0x50 - i2c commond6 register"]
    pub cmd6: crate::Reg<cmd6::CMD6_SPEC>,
    #[doc = "0x54 - i2c commond7 register"]
    pub cmd7: crate::Reg<cmd7::CMD7_SPEC>,
    #[doc = "0x58 - i2c commond8 register"]
    pub cmd8: crate::Reg<cmd8::CMD8_SPEC>,
    #[doc = "0x5c - i2c commond9 register"]
    pub cmd9: crate::Reg<cmd9::CMD9_SPEC>,
    #[doc = "0x60 - i2c commond10 register"]
    pub cmd10: crate::Reg<cmd10::CMD10_SPEC>,
    #[doc = "0x64 - i2c commond11 register"]
    pub cmd11: crate::Reg<cmd11::CMD11_SPEC>,
    #[doc = "0x68 - i2c commond12 register"]
    pub cmd12: crate::Reg<cmd12::CMD12_SPEC>,
    #[doc = "0x6c - i2c commond13 register"]
    pub cmd13: crate::Reg<cmd13::CMD13_SPEC>,
    #[doc = "0x70 - i2c commond14 register"]
    pub cmd14: crate::Reg<cmd14::CMD14_SPEC>,
    #[doc = "0x74 - i2c commond15 register"]
    pub cmd15: crate::Reg<cmd15::CMD15_SPEC>,
    _reserved30: [u8; 0x84],
    #[doc = "0xfc - version register"]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "SCL_LOW register accessor: an alias for `Reg<SCL_LOW_SPEC>`"]
pub type SCL_LOW = crate::Reg<scl_low::SCL_LOW_SPEC>;
#[doc = "configure low scl period"]
pub mod scl_low;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "configure i2c ctrl"]
pub mod ctrl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "get i2c status"]
pub mod status;
#[doc = "TO register accessor: an alias for `Reg<TO_SPEC>`"]
pub type TO = crate::Reg<to::TO_SPEC>;
#[doc = "configure time out"]
pub mod to;
#[doc = "SLAVE_ADDR register accessor: an alias for `Reg<SLAVE_ADDR_SPEC>`"]
pub type SLAVE_ADDR = crate::Reg<slave_addr::SLAVE_ADDR_SPEC>;
#[doc = "configure slave id"]
pub mod slave_addr;
#[doc = "SCL_HIGH register accessor: an alias for `Reg<SCL_HIGH_SPEC>`"]
pub type SCL_HIGH = crate::Reg<scl_high::SCL_HIGH_SPEC>;
#[doc = "configure high scl period"]
pub mod scl_high;
#[doc = "SDA_DUTY register accessor: an alias for `Reg<SDA_DUTY_SPEC>`"]
pub type SDA_DUTY = crate::Reg<sda_duty::SDA_DUTY_SPEC>;
#[doc = "configure sda duty"]
pub mod sda_duty;
#[doc = "SCL_START_PERIOD register accessor: an alias for `Reg<SCL_START_PERIOD_SPEC>`"]
pub type SCL_START_PERIOD = crate::Reg<scl_start_period::SCL_START_PERIOD_SPEC>;
#[doc = "configure scl start period"]
pub mod scl_start_period;
#[doc = "SCL_STOP_PERIOD register accessor: an alias for `Reg<SCL_STOP_PERIOD_SPEC>`"]
pub type SCL_STOP_PERIOD = crate::Reg<scl_stop_period::SCL_STOP_PERIOD_SPEC>;
#[doc = "configure scl stop period"]
pub mod scl_stop_period;
#[doc = "INT_CLR register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "interrupt clear register"]
pub mod int_clr;
#[doc = "INT_RAW register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "interrupt raw register"]
pub mod int_raw;
#[doc = "INT_ST register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "interrupt state register"]
pub mod int_st;
#[doc = "INT_ENA register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "interrupt enable register"]
pub mod int_ena;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "get i2c data status"]
pub mod data;
#[doc = "CMD0 register accessor: an alias for `Reg<CMD0_SPEC>`"]
pub type CMD0 = crate::Reg<cmd0::CMD0_SPEC>;
#[doc = "i2c commond0 register"]
pub mod cmd0;
#[doc = "CMD1 register accessor: an alias for `Reg<CMD1_SPEC>`"]
pub type CMD1 = crate::Reg<cmd1::CMD1_SPEC>;
#[doc = "i2c commond1 register"]
pub mod cmd1;
#[doc = "CMD2 register accessor: an alias for `Reg<CMD2_SPEC>`"]
pub type CMD2 = crate::Reg<cmd2::CMD2_SPEC>;
#[doc = "i2c commond2 register"]
pub mod cmd2;
#[doc = "CMD3 register accessor: an alias for `Reg<CMD3_SPEC>`"]
pub type CMD3 = crate::Reg<cmd3::CMD3_SPEC>;
#[doc = "i2c commond3 register"]
pub mod cmd3;
#[doc = "CMD4 register accessor: an alias for `Reg<CMD4_SPEC>`"]
pub type CMD4 = crate::Reg<cmd4::CMD4_SPEC>;
#[doc = "i2c commond4 register"]
pub mod cmd4;
#[doc = "CMD5 register accessor: an alias for `Reg<CMD5_SPEC>`"]
pub type CMD5 = crate::Reg<cmd5::CMD5_SPEC>;
#[doc = "i2c commond5_register"]
pub mod cmd5;
#[doc = "CMD6 register accessor: an alias for `Reg<CMD6_SPEC>`"]
pub type CMD6 = crate::Reg<cmd6::CMD6_SPEC>;
#[doc = "i2c commond6 register"]
pub mod cmd6;
#[doc = "CMD7 register accessor: an alias for `Reg<CMD7_SPEC>`"]
pub type CMD7 = crate::Reg<cmd7::CMD7_SPEC>;
#[doc = "i2c commond7 register"]
pub mod cmd7;
#[doc = "CMD8 register accessor: an alias for `Reg<CMD8_SPEC>`"]
pub type CMD8 = crate::Reg<cmd8::CMD8_SPEC>;
#[doc = "i2c commond8 register"]
pub mod cmd8;
#[doc = "CMD9 register accessor: an alias for `Reg<CMD9_SPEC>`"]
pub type CMD9 = crate::Reg<cmd9::CMD9_SPEC>;
#[doc = "i2c commond9 register"]
pub mod cmd9;
#[doc = "CMD10 register accessor: an alias for `Reg<CMD10_SPEC>`"]
pub type CMD10 = crate::Reg<cmd10::CMD10_SPEC>;
#[doc = "i2c commond10 register"]
pub mod cmd10;
#[doc = "CMD11 register accessor: an alias for `Reg<CMD11_SPEC>`"]
pub type CMD11 = crate::Reg<cmd11::CMD11_SPEC>;
#[doc = "i2c commond11 register"]
pub mod cmd11;
#[doc = "CMD12 register accessor: an alias for `Reg<CMD12_SPEC>`"]
pub type CMD12 = crate::Reg<cmd12::CMD12_SPEC>;
#[doc = "i2c commond12 register"]
pub mod cmd12;
#[doc = "CMD13 register accessor: an alias for `Reg<CMD13_SPEC>`"]
pub type CMD13 = crate::Reg<cmd13::CMD13_SPEC>;
#[doc = "i2c commond13 register"]
pub mod cmd13;
#[doc = "CMD14 register accessor: an alias for `Reg<CMD14_SPEC>`"]
pub type CMD14 = crate::Reg<cmd14::CMD14_SPEC>;
#[doc = "i2c commond14 register"]
pub mod cmd14;
#[doc = "CMD15 register accessor: an alias for `Reg<CMD15_SPEC>`"]
pub type CMD15 = crate::Reg<cmd15::CMD15_SPEC>;
#[doc = "i2c commond15 register"]
pub mod cmd15;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "version register"]
pub mod date;
