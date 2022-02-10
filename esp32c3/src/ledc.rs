#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LEDC_LSCH0_CONF0."]
    pub lsch0_conf0: crate::Reg<lsch0_conf0::LSCH0_CONF0_SPEC>,
    #[doc = "0x04 - LEDC_LSCH0_HPOINT."]
    pub lsch0_hpoint: crate::Reg<lsch0_hpoint::LSCH0_HPOINT_SPEC>,
    #[doc = "0x08 - LEDC_LSCH0_DUTY."]
    pub lsch0_duty: crate::Reg<lsch0_duty::LSCH0_DUTY_SPEC>,
    #[doc = "0x0c - LEDC_LSCH0_CONF1."]
    pub lsch0_conf1: crate::Reg<lsch0_conf1::LSCH0_CONF1_SPEC>,
    #[doc = "0x10 - LEDC_LSCH0_DUTY_R."]
    pub lsch0_duty_r: crate::Reg<lsch0_duty_r::LSCH0_DUTY_R_SPEC>,
    #[doc = "0x14 - LEDC_LSCH1_CONF0."]
    pub lsch1_conf0: crate::Reg<lsch1_conf0::LSCH1_CONF0_SPEC>,
    #[doc = "0x18 - LEDC_LSCH1_HPOINT."]
    pub lsch1_hpoint: crate::Reg<lsch1_hpoint::LSCH1_HPOINT_SPEC>,
    #[doc = "0x1c - LEDC_LSCH1_DUTY."]
    pub lsch1_duty: crate::Reg<lsch1_duty::LSCH1_DUTY_SPEC>,
    #[doc = "0x20 - LEDC_LSCH1_CONF1."]
    pub lsch1_conf1: crate::Reg<lsch1_conf1::LSCH1_CONF1_SPEC>,
    #[doc = "0x24 - LEDC_LSCH1_DUTY_R."]
    pub lsch1_duty_r: crate::Reg<lsch1_duty_r::LSCH1_DUTY_R_SPEC>,
    #[doc = "0x28 - LEDC_LSCH2_CONF0."]
    pub lsch2_conf0: crate::Reg<lsch2_conf0::LSCH2_CONF0_SPEC>,
    #[doc = "0x2c - LEDC_LSCH2_HPOINT."]
    pub lsch2_hpoint: crate::Reg<lsch2_hpoint::LSCH2_HPOINT_SPEC>,
    #[doc = "0x30 - LEDC_LSCH2_DUTY."]
    pub lsch2_duty: crate::Reg<lsch2_duty::LSCH2_DUTY_SPEC>,
    #[doc = "0x34 - LEDC_LSCH2_CONF1."]
    pub lsch2_conf1: crate::Reg<lsch2_conf1::LSCH2_CONF1_SPEC>,
    #[doc = "0x38 - LEDC_LSCH2_DUTY_R."]
    pub lsch2_duty_r: crate::Reg<lsch2_duty_r::LSCH2_DUTY_R_SPEC>,
    #[doc = "0x3c - LEDC_LSCH3_CONF0."]
    pub lsch3_conf0: crate::Reg<lsch3_conf0::LSCH3_CONF0_SPEC>,
    #[doc = "0x40 - LEDC_LSCH3_HPOINT."]
    pub lsch3_hpoint: crate::Reg<lsch3_hpoint::LSCH3_HPOINT_SPEC>,
    #[doc = "0x44 - LEDC_LSCH3_DUTY."]
    pub lsch3_duty: crate::Reg<lsch3_duty::LSCH3_DUTY_SPEC>,
    #[doc = "0x48 - LEDC_LSCH3_CONF1."]
    pub lsch3_conf1: crate::Reg<lsch3_conf1::LSCH3_CONF1_SPEC>,
    #[doc = "0x4c - LEDC_LSCH3_DUTY_R."]
    pub lsch3_duty_r: crate::Reg<lsch3_duty_r::LSCH3_DUTY_R_SPEC>,
    #[doc = "0x50 - LEDC_LSCH4_CONF0."]
    pub lsch4_conf0: crate::Reg<lsch4_conf0::LSCH4_CONF0_SPEC>,
    #[doc = "0x54 - LEDC_LSCH4_HPOINT."]
    pub lsch4_hpoint: crate::Reg<lsch4_hpoint::LSCH4_HPOINT_SPEC>,
    #[doc = "0x58 - LEDC_LSCH4_DUTY."]
    pub lsch4_duty: crate::Reg<lsch4_duty::LSCH4_DUTY_SPEC>,
    #[doc = "0x5c - LEDC_LSCH4_CONF1."]
    pub lsch4_conf1: crate::Reg<lsch4_conf1::LSCH4_CONF1_SPEC>,
    #[doc = "0x60 - LEDC_LSCH4_DUTY_R."]
    pub lsch4_duty_r: crate::Reg<lsch4_duty_r::LSCH4_DUTY_R_SPEC>,
    #[doc = "0x64 - LEDC_LSCH5_CONF0."]
    pub lsch5_conf0: crate::Reg<lsch5_conf0::LSCH5_CONF0_SPEC>,
    #[doc = "0x68 - LEDC_LSCH5_HPOINT."]
    pub lsch5_hpoint: crate::Reg<lsch5_hpoint::LSCH5_HPOINT_SPEC>,
    #[doc = "0x6c - LEDC_LSCH5_DUTY."]
    pub lsch5_duty: crate::Reg<lsch5_duty::LSCH5_DUTY_SPEC>,
    #[doc = "0x70 - LEDC_LSCH5_CONF1."]
    pub lsch5_conf1: crate::Reg<lsch5_conf1::LSCH5_CONF1_SPEC>,
    #[doc = "0x74 - LEDC_LSCH5_DUTY_R."]
    pub lsch5_duty_r: crate::Reg<lsch5_duty_r::LSCH5_DUTY_R_SPEC>,
    _reserved30: [u8; 0x28],
    #[doc = "0xa0 - LEDC_LSTIMER0_CONF."]
    pub lstimer0_conf: crate::Reg<lstimer0_conf::LSTIMER0_CONF_SPEC>,
    #[doc = "0xa4 - LEDC_LSTIMER0_VALUE."]
    pub lstimer0_value: crate::Reg<lstimer0_value::LSTIMER0_VALUE_SPEC>,
    #[doc = "0xa8 - LEDC_LSTIMER1_CONF."]
    pub lstimer1_conf: crate::Reg<lstimer1_conf::LSTIMER1_CONF_SPEC>,
    #[doc = "0xac - LEDC_LSTIMER1_VALUE."]
    pub lstimer1_value: crate::Reg<lstimer1_value::LSTIMER1_VALUE_SPEC>,
    #[doc = "0xb0 - LEDC_LSTIMER2_CONF."]
    pub lstimer2_conf: crate::Reg<lstimer2_conf::LSTIMER2_CONF_SPEC>,
    #[doc = "0xb4 - LEDC_LSTIMER2_VALUE."]
    pub lstimer2_value: crate::Reg<lstimer2_value::LSTIMER2_VALUE_SPEC>,
    #[doc = "0xb8 - LEDC_LSTIMER3_CONF."]
    pub lstimer3_conf: crate::Reg<lstimer3_conf::LSTIMER3_CONF_SPEC>,
    #[doc = "0xbc - LEDC_LSTIMER3_VALUE."]
    pub lstimer3_value: crate::Reg<lstimer3_value::LSTIMER3_VALUE_SPEC>,
    #[doc = "0xc0 - LEDC_INT_RAW."]
    pub int_raw: crate::Reg<int_raw::INT_RAW_SPEC>,
    #[doc = "0xc4 - LEDC_INT_ST."]
    pub int_st: crate::Reg<int_st::INT_ST_SPEC>,
    #[doc = "0xc8 - LEDC_INT_ENA."]
    pub int_ena: crate::Reg<int_ena::INT_ENA_SPEC>,
    #[doc = "0xcc - LEDC_INT_CLR."]
    pub int_clr: crate::Reg<int_clr::INT_CLR_SPEC>,
    #[doc = "0xd0 - LEDC_CONF."]
    pub conf: crate::Reg<conf::CONF_SPEC>,
    _reserved43: [u8; 0x28],
    #[doc = "0xfc - LEDC_DATE."]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "LSCH0_CONF0 register accessor: an alias for `Reg<LSCH0_CONF0_SPEC>`"]
pub type LSCH0_CONF0 = crate::Reg<lsch0_conf0::LSCH0_CONF0_SPEC>;
#[doc = "LEDC_LSCH0_CONF0."]
pub mod lsch0_conf0;
#[doc = "LSCH0_HPOINT register accessor: an alias for `Reg<LSCH0_HPOINT_SPEC>`"]
pub type LSCH0_HPOINT = crate::Reg<lsch0_hpoint::LSCH0_HPOINT_SPEC>;
#[doc = "LEDC_LSCH0_HPOINT."]
pub mod lsch0_hpoint;
#[doc = "LSCH0_DUTY register accessor: an alias for `Reg<LSCH0_DUTY_SPEC>`"]
pub type LSCH0_DUTY = crate::Reg<lsch0_duty::LSCH0_DUTY_SPEC>;
#[doc = "LEDC_LSCH0_DUTY."]
pub mod lsch0_duty;
#[doc = "LSCH0_CONF1 register accessor: an alias for `Reg<LSCH0_CONF1_SPEC>`"]
pub type LSCH0_CONF1 = crate::Reg<lsch0_conf1::LSCH0_CONF1_SPEC>;
#[doc = "LEDC_LSCH0_CONF1."]
pub mod lsch0_conf1;
#[doc = "LSCH0_DUTY_R register accessor: an alias for `Reg<LSCH0_DUTY_R_SPEC>`"]
pub type LSCH0_DUTY_R = crate::Reg<lsch0_duty_r::LSCH0_DUTY_R_SPEC>;
#[doc = "LEDC_LSCH0_DUTY_R."]
pub mod lsch0_duty_r;
#[doc = "LSCH1_CONF0 register accessor: an alias for `Reg<LSCH1_CONF0_SPEC>`"]
pub type LSCH1_CONF0 = crate::Reg<lsch1_conf0::LSCH1_CONF0_SPEC>;
#[doc = "LEDC_LSCH1_CONF0."]
pub mod lsch1_conf0;
#[doc = "LSCH1_HPOINT register accessor: an alias for `Reg<LSCH1_HPOINT_SPEC>`"]
pub type LSCH1_HPOINT = crate::Reg<lsch1_hpoint::LSCH1_HPOINT_SPEC>;
#[doc = "LEDC_LSCH1_HPOINT."]
pub mod lsch1_hpoint;
#[doc = "LSCH1_DUTY register accessor: an alias for `Reg<LSCH1_DUTY_SPEC>`"]
pub type LSCH1_DUTY = crate::Reg<lsch1_duty::LSCH1_DUTY_SPEC>;
#[doc = "LEDC_LSCH1_DUTY."]
pub mod lsch1_duty;
#[doc = "LSCH1_CONF1 register accessor: an alias for `Reg<LSCH1_CONF1_SPEC>`"]
pub type LSCH1_CONF1 = crate::Reg<lsch1_conf1::LSCH1_CONF1_SPEC>;
#[doc = "LEDC_LSCH1_CONF1."]
pub mod lsch1_conf1;
#[doc = "LSCH1_DUTY_R register accessor: an alias for `Reg<LSCH1_DUTY_R_SPEC>`"]
pub type LSCH1_DUTY_R = crate::Reg<lsch1_duty_r::LSCH1_DUTY_R_SPEC>;
#[doc = "LEDC_LSCH1_DUTY_R."]
pub mod lsch1_duty_r;
#[doc = "LSCH2_CONF0 register accessor: an alias for `Reg<LSCH2_CONF0_SPEC>`"]
pub type LSCH2_CONF0 = crate::Reg<lsch2_conf0::LSCH2_CONF0_SPEC>;
#[doc = "LEDC_LSCH2_CONF0."]
pub mod lsch2_conf0;
#[doc = "LSCH2_HPOINT register accessor: an alias for `Reg<LSCH2_HPOINT_SPEC>`"]
pub type LSCH2_HPOINT = crate::Reg<lsch2_hpoint::LSCH2_HPOINT_SPEC>;
#[doc = "LEDC_LSCH2_HPOINT."]
pub mod lsch2_hpoint;
#[doc = "LSCH2_DUTY register accessor: an alias for `Reg<LSCH2_DUTY_SPEC>`"]
pub type LSCH2_DUTY = crate::Reg<lsch2_duty::LSCH2_DUTY_SPEC>;
#[doc = "LEDC_LSCH2_DUTY."]
pub mod lsch2_duty;
#[doc = "LSCH2_CONF1 register accessor: an alias for `Reg<LSCH2_CONF1_SPEC>`"]
pub type LSCH2_CONF1 = crate::Reg<lsch2_conf1::LSCH2_CONF1_SPEC>;
#[doc = "LEDC_LSCH2_CONF1."]
pub mod lsch2_conf1;
#[doc = "LSCH2_DUTY_R register accessor: an alias for `Reg<LSCH2_DUTY_R_SPEC>`"]
pub type LSCH2_DUTY_R = crate::Reg<lsch2_duty_r::LSCH2_DUTY_R_SPEC>;
#[doc = "LEDC_LSCH2_DUTY_R."]
pub mod lsch2_duty_r;
#[doc = "LSCH3_CONF0 register accessor: an alias for `Reg<LSCH3_CONF0_SPEC>`"]
pub type LSCH3_CONF0 = crate::Reg<lsch3_conf0::LSCH3_CONF0_SPEC>;
#[doc = "LEDC_LSCH3_CONF0."]
pub mod lsch3_conf0;
#[doc = "LSCH3_HPOINT register accessor: an alias for `Reg<LSCH3_HPOINT_SPEC>`"]
pub type LSCH3_HPOINT = crate::Reg<lsch3_hpoint::LSCH3_HPOINT_SPEC>;
#[doc = "LEDC_LSCH3_HPOINT."]
pub mod lsch3_hpoint;
#[doc = "LSCH3_DUTY register accessor: an alias for `Reg<LSCH3_DUTY_SPEC>`"]
pub type LSCH3_DUTY = crate::Reg<lsch3_duty::LSCH3_DUTY_SPEC>;
#[doc = "LEDC_LSCH3_DUTY."]
pub mod lsch3_duty;
#[doc = "LSCH3_CONF1 register accessor: an alias for `Reg<LSCH3_CONF1_SPEC>`"]
pub type LSCH3_CONF1 = crate::Reg<lsch3_conf1::LSCH3_CONF1_SPEC>;
#[doc = "LEDC_LSCH3_CONF1."]
pub mod lsch3_conf1;
#[doc = "LSCH3_DUTY_R register accessor: an alias for `Reg<LSCH3_DUTY_R_SPEC>`"]
pub type LSCH3_DUTY_R = crate::Reg<lsch3_duty_r::LSCH3_DUTY_R_SPEC>;
#[doc = "LEDC_LSCH3_DUTY_R."]
pub mod lsch3_duty_r;
#[doc = "LSCH4_CONF0 register accessor: an alias for `Reg<LSCH4_CONF0_SPEC>`"]
pub type LSCH4_CONF0 = crate::Reg<lsch4_conf0::LSCH4_CONF0_SPEC>;
#[doc = "LEDC_LSCH4_CONF0."]
pub mod lsch4_conf0;
#[doc = "LSCH4_HPOINT register accessor: an alias for `Reg<LSCH4_HPOINT_SPEC>`"]
pub type LSCH4_HPOINT = crate::Reg<lsch4_hpoint::LSCH4_HPOINT_SPEC>;
#[doc = "LEDC_LSCH4_HPOINT."]
pub mod lsch4_hpoint;
#[doc = "LSCH4_DUTY register accessor: an alias for `Reg<LSCH4_DUTY_SPEC>`"]
pub type LSCH4_DUTY = crate::Reg<lsch4_duty::LSCH4_DUTY_SPEC>;
#[doc = "LEDC_LSCH4_DUTY."]
pub mod lsch4_duty;
#[doc = "LSCH4_CONF1 register accessor: an alias for `Reg<LSCH4_CONF1_SPEC>`"]
pub type LSCH4_CONF1 = crate::Reg<lsch4_conf1::LSCH4_CONF1_SPEC>;
#[doc = "LEDC_LSCH4_CONF1."]
pub mod lsch4_conf1;
#[doc = "LSCH4_DUTY_R register accessor: an alias for `Reg<LSCH4_DUTY_R_SPEC>`"]
pub type LSCH4_DUTY_R = crate::Reg<lsch4_duty_r::LSCH4_DUTY_R_SPEC>;
#[doc = "LEDC_LSCH4_DUTY_R."]
pub mod lsch4_duty_r;
#[doc = "LSCH5_CONF0 register accessor: an alias for `Reg<LSCH5_CONF0_SPEC>`"]
pub type LSCH5_CONF0 = crate::Reg<lsch5_conf0::LSCH5_CONF0_SPEC>;
#[doc = "LEDC_LSCH5_CONF0."]
pub mod lsch5_conf0;
#[doc = "LSCH5_HPOINT register accessor: an alias for `Reg<LSCH5_HPOINT_SPEC>`"]
pub type LSCH5_HPOINT = crate::Reg<lsch5_hpoint::LSCH5_HPOINT_SPEC>;
#[doc = "LEDC_LSCH5_HPOINT."]
pub mod lsch5_hpoint;
#[doc = "LSCH5_DUTY register accessor: an alias for `Reg<LSCH5_DUTY_SPEC>`"]
pub type LSCH5_DUTY = crate::Reg<lsch5_duty::LSCH5_DUTY_SPEC>;
#[doc = "LEDC_LSCH5_DUTY."]
pub mod lsch5_duty;
#[doc = "LSCH5_CONF1 register accessor: an alias for `Reg<LSCH5_CONF1_SPEC>`"]
pub type LSCH5_CONF1 = crate::Reg<lsch5_conf1::LSCH5_CONF1_SPEC>;
#[doc = "LEDC_LSCH5_CONF1."]
pub mod lsch5_conf1;
#[doc = "LSCH5_DUTY_R register accessor: an alias for `Reg<LSCH5_DUTY_R_SPEC>`"]
pub type LSCH5_DUTY_R = crate::Reg<lsch5_duty_r::LSCH5_DUTY_R_SPEC>;
#[doc = "LEDC_LSCH5_DUTY_R."]
pub mod lsch5_duty_r;
#[doc = "LSTIMER0_CONF register accessor: an alias for `Reg<LSTIMER0_CONF_SPEC>`"]
pub type LSTIMER0_CONF = crate::Reg<lstimer0_conf::LSTIMER0_CONF_SPEC>;
#[doc = "LEDC_LSTIMER0_CONF."]
pub mod lstimer0_conf;
#[doc = "LSTIMER0_VALUE register accessor: an alias for `Reg<LSTIMER0_VALUE_SPEC>`"]
pub type LSTIMER0_VALUE = crate::Reg<lstimer0_value::LSTIMER0_VALUE_SPEC>;
#[doc = "LEDC_LSTIMER0_VALUE."]
pub mod lstimer0_value;
#[doc = "LSTIMER1_CONF register accessor: an alias for `Reg<LSTIMER1_CONF_SPEC>`"]
pub type LSTIMER1_CONF = crate::Reg<lstimer1_conf::LSTIMER1_CONF_SPEC>;
#[doc = "LEDC_LSTIMER1_CONF."]
pub mod lstimer1_conf;
#[doc = "LSTIMER1_VALUE register accessor: an alias for `Reg<LSTIMER1_VALUE_SPEC>`"]
pub type LSTIMER1_VALUE = crate::Reg<lstimer1_value::LSTIMER1_VALUE_SPEC>;
#[doc = "LEDC_LSTIMER1_VALUE."]
pub mod lstimer1_value;
#[doc = "LSTIMER2_CONF register accessor: an alias for `Reg<LSTIMER2_CONF_SPEC>`"]
pub type LSTIMER2_CONF = crate::Reg<lstimer2_conf::LSTIMER2_CONF_SPEC>;
#[doc = "LEDC_LSTIMER2_CONF."]
pub mod lstimer2_conf;
#[doc = "LSTIMER2_VALUE register accessor: an alias for `Reg<LSTIMER2_VALUE_SPEC>`"]
pub type LSTIMER2_VALUE = crate::Reg<lstimer2_value::LSTIMER2_VALUE_SPEC>;
#[doc = "LEDC_LSTIMER2_VALUE."]
pub mod lstimer2_value;
#[doc = "LSTIMER3_CONF register accessor: an alias for `Reg<LSTIMER3_CONF_SPEC>`"]
pub type LSTIMER3_CONF = crate::Reg<lstimer3_conf::LSTIMER3_CONF_SPEC>;
#[doc = "LEDC_LSTIMER3_CONF."]
pub mod lstimer3_conf;
#[doc = "LSTIMER3_VALUE register accessor: an alias for `Reg<LSTIMER3_VALUE_SPEC>`"]
pub type LSTIMER3_VALUE = crate::Reg<lstimer3_value::LSTIMER3_VALUE_SPEC>;
#[doc = "LEDC_LSTIMER3_VALUE."]
pub mod lstimer3_value;
#[doc = "INT_RAW register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "LEDC_INT_RAW."]
pub mod int_raw;
#[doc = "INT_ST register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "LEDC_INT_ST."]
pub mod int_st;
#[doc = "INT_ENA register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "LEDC_INT_ENA."]
pub mod int_ena;
#[doc = "INT_CLR register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "LEDC_INT_CLR."]
pub mod int_clr;
#[doc = "CONF register accessor: an alias for `Reg<CONF_SPEC>`"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "LEDC_CONF."]
pub mod conf;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "LEDC_DATE."]
pub mod date;
