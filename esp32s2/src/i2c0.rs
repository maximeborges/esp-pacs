#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configures the low level width of the SCL clock"]
    pub scl_low_period: crate::Reg<scl_low_period::SCL_LOW_PERIOD_SPEC>,
    #[doc = "0x04 - Transmission setting"]
    pub ctr: crate::Reg<ctr::CTR_SPEC>,
    #[doc = "0x08 - Describe I2C work status"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x0c - Setting time out control for receiving data"]
    pub to: crate::Reg<to::TO_SPEC>,
    #[doc = "0x10 - Local slave address setting"]
    pub slave_addr: crate::Reg<slave_addr::SLAVE_ADDR_SPEC>,
    #[doc = "0x14 - FIFO status register"]
    pub fifo_st: crate::Reg<fifo_st::FIFO_ST_SPEC>,
    #[doc = "0x18 - FIFO configuration register"]
    pub fifo_conf: crate::Reg<fifo_conf::FIFO_CONF_SPEC>,
    #[doc = "0x1c - RX FIFO read data"]
    pub data: crate::Reg<data::DATA_SPEC>,
    #[doc = "0x20 - Raw interrupt status"]
    pub int_raw: crate::Reg<int_raw::INT_RAW_SPEC>,
    #[doc = "0x24 - Interrupt clear bits"]
    pub int_clr: crate::Reg<int_clr::INT_CLR_SPEC>,
    #[doc = "0x28 - Interrupt enable bits"]
    pub int_ena: crate::Reg<int_ena::INT_ENA_SPEC>,
    #[doc = "0x2c - Status of captured I2C communication events"]
    pub int_status: crate::Reg<int_status::INT_STATUS_SPEC>,
    #[doc = "0x30 - Configures the hold time after a negative SCL edge"]
    pub sda_hold: crate::Reg<sda_hold::SDA_HOLD_SPEC>,
    #[doc = "0x34 - Configures the sample time after a positive SCL edge"]
    pub sda_sample: crate::Reg<sda_sample::SDA_SAMPLE_SPEC>,
    #[doc = "0x38 - Configures the high level width of the SCL clock"]
    pub scl_high_period: crate::Reg<scl_high_period::SCL_HIGH_PERIOD_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x40 - Configures the interval between pulling SDA low and pulling SCL low when the master generates a START condition"]
    pub scl_start_hold: crate::Reg<scl_start_hold::SCL_START_HOLD_SPEC>,
    #[doc = "0x44 - Configures the interval between the positive edge of SCL and the negative edge of SDA"]
    pub scl_rstart_setup: crate::Reg<scl_rstart_setup::SCL_RSTART_SETUP_SPEC>,
    #[doc = "0x48 - Configures the delay after the SCL clock edge for a stop condition"]
    pub scl_stop_hold: crate::Reg<scl_stop_hold::SCL_STOP_HOLD_SPEC>,
    #[doc = "0x4c - Configures the delay between the SDA and SCL positive edge for a stop condition"]
    pub scl_stop_setup: crate::Reg<scl_stop_setup::SCL_STOP_SETUP_SPEC>,
    #[doc = "0x50 - SCL filter configuration register"]
    pub scl_filter_cfg: crate::Reg<scl_filter_cfg::SCL_FILTER_CFG_SPEC>,
    #[doc = "0x54 - SDA filter configuration register"]
    pub sda_filter_cfg: crate::Reg<sda_filter_cfg::SDA_FILTER_CFG_SPEC>,
    #[doc = "0x58..0x78 - I2C command register %s"]
    pub comd: [crate::Reg<comd::COMD_SPEC>; 8],
    #[doc = "0x78 - I2C command register 8"]
    pub comd8: crate::Reg<comd8::COMD8_SPEC>,
    #[doc = "0x7c - I2C command register 9"]
    pub comd9: crate::Reg<comd9::COMD9_SPEC>,
    #[doc = "0x80 - I2C command register 10"]
    pub comd10: crate::Reg<comd10::COMD10_SPEC>,
    #[doc = "0x84 - I2C command register 11"]
    pub comd11: crate::Reg<comd11::COMD11_SPEC>,
    #[doc = "0x88 - I2C command register 12"]
    pub comd12: crate::Reg<comd12::COMD12_SPEC>,
    #[doc = "0x8c - I2C command register 13"]
    pub comd13: crate::Reg<comd13::COMD13_SPEC>,
    #[doc = "0x90 - I2C command register 14"]
    pub comd14: crate::Reg<comd14::COMD14_SPEC>,
    #[doc = "0x94 - I2C command register 15"]
    pub comd15: crate::Reg<comd15::COMD15_SPEC>,
    #[doc = "0x98 - SCL status time out register"]
    pub scl_st_time_out: crate::Reg<scl_st_time_out::SCL_ST_TIME_OUT_SPEC>,
    #[doc = "0x9c - SCL main status time out register"]
    pub scl_main_st_time_out: crate::Reg<scl_main_st_time_out::SCL_MAIN_ST_TIME_OUT_SPEC>,
    #[doc = "0xa0 - Power configuration register"]
    pub scl_sp_conf: crate::Reg<scl_sp_conf::SCL_SP_CONF_SPEC>,
    #[doc = "0xa4 - Set SCL stretch of I2C slave"]
    pub scl_stretch_conf: crate::Reg<scl_stretch_conf::SCL_STRETCH_CONF_SPEC>,
    _reserved34: [u8; 0x50],
    #[doc = "0xf8 - Version control register"]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "SCL_LOW_PERIOD register accessor: an alias for `Reg<SCL_LOW_PERIOD_SPEC>`"]
pub type SCL_LOW_PERIOD = crate::Reg<scl_low_period::SCL_LOW_PERIOD_SPEC>;
#[doc = "Configures the low level width of the SCL clock"]
pub mod scl_low_period;
#[doc = "CTR register accessor: an alias for `Reg<CTR_SPEC>`"]
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
#[doc = "Transmission setting"]
pub mod ctr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Describe I2C work status"]
pub mod sr;
#[doc = "TO register accessor: an alias for `Reg<TO_SPEC>`"]
pub type TO = crate::Reg<to::TO_SPEC>;
#[doc = "Setting time out control for receiving data"]
pub mod to;
#[doc = "SLAVE_ADDR register accessor: an alias for `Reg<SLAVE_ADDR_SPEC>`"]
pub type SLAVE_ADDR = crate::Reg<slave_addr::SLAVE_ADDR_SPEC>;
#[doc = "Local slave address setting"]
pub mod slave_addr;
#[doc = "FIFO_ST register accessor: an alias for `Reg<FIFO_ST_SPEC>`"]
pub type FIFO_ST = crate::Reg<fifo_st::FIFO_ST_SPEC>;
#[doc = "FIFO status register"]
pub mod fifo_st;
#[doc = "FIFO_CONF register accessor: an alias for `Reg<FIFO_CONF_SPEC>`"]
pub type FIFO_CONF = crate::Reg<fifo_conf::FIFO_CONF_SPEC>;
#[doc = "FIFO configuration register"]
pub mod fifo_conf;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "RX FIFO read data"]
pub mod data;
#[doc = "INT_RAW register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Raw interrupt status"]
pub mod int_raw;
#[doc = "INT_CLR register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt clear bits"]
pub mod int_clr;
#[doc = "INT_ENA register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt enable bits"]
pub mod int_ena;
#[doc = "INT_STATUS register accessor: an alias for `Reg<INT_STATUS_SPEC>`"]
pub type INT_STATUS = crate::Reg<int_status::INT_STATUS_SPEC>;
#[doc = "Status of captured I2C communication events"]
pub mod int_status;
#[doc = "SDA_HOLD register accessor: an alias for `Reg<SDA_HOLD_SPEC>`"]
pub type SDA_HOLD = crate::Reg<sda_hold::SDA_HOLD_SPEC>;
#[doc = "Configures the hold time after a negative SCL edge"]
pub mod sda_hold;
#[doc = "SDA_SAMPLE register accessor: an alias for `Reg<SDA_SAMPLE_SPEC>`"]
pub type SDA_SAMPLE = crate::Reg<sda_sample::SDA_SAMPLE_SPEC>;
#[doc = "Configures the sample time after a positive SCL edge"]
pub mod sda_sample;
#[doc = "SCL_HIGH_PERIOD register accessor: an alias for `Reg<SCL_HIGH_PERIOD_SPEC>`"]
pub type SCL_HIGH_PERIOD = crate::Reg<scl_high_period::SCL_HIGH_PERIOD_SPEC>;
#[doc = "Configures the high level width of the SCL clock"]
pub mod scl_high_period;
#[doc = "SCL_START_HOLD register accessor: an alias for `Reg<SCL_START_HOLD_SPEC>`"]
pub type SCL_START_HOLD = crate::Reg<scl_start_hold::SCL_START_HOLD_SPEC>;
#[doc = "Configures the interval between pulling SDA low and pulling SCL low when the master generates a START condition"]
pub mod scl_start_hold;
#[doc = "SCL_RSTART_SETUP register accessor: an alias for `Reg<SCL_RSTART_SETUP_SPEC>`"]
pub type SCL_RSTART_SETUP = crate::Reg<scl_rstart_setup::SCL_RSTART_SETUP_SPEC>;
#[doc = "Configures the interval between the positive edge of SCL and the negative edge of SDA"]
pub mod scl_rstart_setup;
#[doc = "SCL_STOP_HOLD register accessor: an alias for `Reg<SCL_STOP_HOLD_SPEC>`"]
pub type SCL_STOP_HOLD = crate::Reg<scl_stop_hold::SCL_STOP_HOLD_SPEC>;
#[doc = "Configures the delay after the SCL clock edge for a stop condition"]
pub mod scl_stop_hold;
#[doc = "SCL_STOP_SETUP register accessor: an alias for `Reg<SCL_STOP_SETUP_SPEC>`"]
pub type SCL_STOP_SETUP = crate::Reg<scl_stop_setup::SCL_STOP_SETUP_SPEC>;
#[doc = "Configures the delay between the SDA and SCL positive edge for a stop condition"]
pub mod scl_stop_setup;
#[doc = "SCL_FILTER_CFG register accessor: an alias for `Reg<SCL_FILTER_CFG_SPEC>`"]
pub type SCL_FILTER_CFG = crate::Reg<scl_filter_cfg::SCL_FILTER_CFG_SPEC>;
#[doc = "SCL filter configuration register"]
pub mod scl_filter_cfg;
#[doc = "SDA_FILTER_CFG register accessor: an alias for `Reg<SDA_FILTER_CFG_SPEC>`"]
pub type SDA_FILTER_CFG = crate::Reg<sda_filter_cfg::SDA_FILTER_CFG_SPEC>;
#[doc = "SDA filter configuration register"]
pub mod sda_filter_cfg;
#[doc = "COMD register accessor: an alias for `Reg<COMD_SPEC>`"]
pub type COMD = crate::Reg<comd::COMD_SPEC>;
#[doc = "I2C command register %s"]
pub mod comd;
#[doc = "COMD8 register accessor: an alias for `Reg<COMD8_SPEC>`"]
pub type COMD8 = crate::Reg<comd8::COMD8_SPEC>;
#[doc = "I2C command register 8"]
pub mod comd8;
#[doc = "COMD9 register accessor: an alias for `Reg<COMD9_SPEC>`"]
pub type COMD9 = crate::Reg<comd9::COMD9_SPEC>;
#[doc = "I2C command register 9"]
pub mod comd9;
#[doc = "COMD10 register accessor: an alias for `Reg<COMD10_SPEC>`"]
pub type COMD10 = crate::Reg<comd10::COMD10_SPEC>;
#[doc = "I2C command register 10"]
pub mod comd10;
#[doc = "COMD11 register accessor: an alias for `Reg<COMD11_SPEC>`"]
pub type COMD11 = crate::Reg<comd11::COMD11_SPEC>;
#[doc = "I2C command register 11"]
pub mod comd11;
#[doc = "COMD12 register accessor: an alias for `Reg<COMD12_SPEC>`"]
pub type COMD12 = crate::Reg<comd12::COMD12_SPEC>;
#[doc = "I2C command register 12"]
pub mod comd12;
#[doc = "COMD13 register accessor: an alias for `Reg<COMD13_SPEC>`"]
pub type COMD13 = crate::Reg<comd13::COMD13_SPEC>;
#[doc = "I2C command register 13"]
pub mod comd13;
#[doc = "COMD14 register accessor: an alias for `Reg<COMD14_SPEC>`"]
pub type COMD14 = crate::Reg<comd14::COMD14_SPEC>;
#[doc = "I2C command register 14"]
pub mod comd14;
#[doc = "COMD15 register accessor: an alias for `Reg<COMD15_SPEC>`"]
pub type COMD15 = crate::Reg<comd15::COMD15_SPEC>;
#[doc = "I2C command register 15"]
pub mod comd15;
#[doc = "SCL_ST_TIME_OUT register accessor: an alias for `Reg<SCL_ST_TIME_OUT_SPEC>`"]
pub type SCL_ST_TIME_OUT = crate::Reg<scl_st_time_out::SCL_ST_TIME_OUT_SPEC>;
#[doc = "SCL status time out register"]
pub mod scl_st_time_out;
#[doc = "SCL_MAIN_ST_TIME_OUT register accessor: an alias for `Reg<SCL_MAIN_ST_TIME_OUT_SPEC>`"]
pub type SCL_MAIN_ST_TIME_OUT = crate::Reg<scl_main_st_time_out::SCL_MAIN_ST_TIME_OUT_SPEC>;
#[doc = "SCL main status time out register"]
pub mod scl_main_st_time_out;
#[doc = "SCL_SP_CONF register accessor: an alias for `Reg<SCL_SP_CONF_SPEC>`"]
pub type SCL_SP_CONF = crate::Reg<scl_sp_conf::SCL_SP_CONF_SPEC>;
#[doc = "Power configuration register"]
pub mod scl_sp_conf;
#[doc = "SCL_STRETCH_CONF register accessor: an alias for `Reg<SCL_STRETCH_CONF_SPEC>`"]
pub type SCL_STRETCH_CONF = crate::Reg<scl_stretch_conf::SCL_STRETCH_CONF_SPEC>;
#[doc = "Set SCL stretch of I2C slave"]
pub mod scl_stretch_conf;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
