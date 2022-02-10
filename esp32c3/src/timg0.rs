#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIMG_T0CONFIG_REG."]
    pub t0config: crate::Reg<t0config::T0CONFIG_SPEC>,
    #[doc = "0x04 - TIMG_T0LO_REG."]
    pub t0lo: crate::Reg<t0lo::T0LO_SPEC>,
    #[doc = "0x08 - TIMG_T0HI_REG."]
    pub t0hi: crate::Reg<t0hi::T0HI_SPEC>,
    #[doc = "0x0c - TIMG_T0UPDATE_REG."]
    pub t0update: crate::Reg<t0update::T0UPDATE_SPEC>,
    #[doc = "0x10 - TIMG_T0ALARMLO_REG."]
    pub t0alarmlo: crate::Reg<t0alarmlo::T0ALARMLO_SPEC>,
    #[doc = "0x14 - TIMG_T0ALARMHI_REG."]
    pub t0alarmhi: crate::Reg<t0alarmhi::T0ALARMHI_SPEC>,
    #[doc = "0x18 - TIMG_T0LOADLO_REG."]
    pub t0loadlo: crate::Reg<t0loadlo::T0LOADLO_SPEC>,
    #[doc = "0x1c - TIMG_T0LOADHI_REG."]
    pub t0loadhi: crate::Reg<t0loadhi::T0LOADHI_SPEC>,
    #[doc = "0x20 - TIMG_T0LOAD_REG."]
    pub t0load: crate::Reg<t0load::T0LOAD_SPEC>,
    _reserved9: [u8; 0x24],
    #[doc = "0x48 - TIMG_WDTCONFIG0_REG."]
    pub wdtconfig0: crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>,
    #[doc = "0x4c - TIMG_WDTCONFIG1_REG."]
    pub wdtconfig1: crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>,
    #[doc = "0x50 - TIMG_WDTCONFIG2_REG."]
    pub wdtconfig2: crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>,
    #[doc = "0x54 - TIMG_WDTCONFIG3_REG."]
    pub wdtconfig3: crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>,
    #[doc = "0x58 - TIMG_WDTCONFIG4_REG."]
    pub wdtconfig4: crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>,
    #[doc = "0x5c - TIMG_WDTCONFIG5_REG."]
    pub wdtconfig5: crate::Reg<wdtconfig5::WDTCONFIG5_SPEC>,
    #[doc = "0x60 - TIMG_WDTFEED_REG."]
    pub wdtfeed: crate::Reg<wdtfeed::WDTFEED_SPEC>,
    #[doc = "0x64 - TIMG_WDTWPROTECT_REG."]
    pub wdtwprotect: crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>,
    #[doc = "0x68 - TIMG_RTCCALICFG_REG."]
    pub rtccalicfg: crate::Reg<rtccalicfg::RTCCALICFG_SPEC>,
    #[doc = "0x6c - TIMG_RTCCALICFG1_REG."]
    pub rtccalicfg1: crate::Reg<rtccalicfg1::RTCCALICFG1_SPEC>,
    #[doc = "0x70 - INT_ENA_TIMG_REG"]
    pub int_ena_timg: crate::Reg<int_ena_timg::INT_ENA_TIMG_SPEC>,
    #[doc = "0x74 - INT_RAW_TIMG_REG"]
    pub int_raw_timers: crate::Reg<int_raw_timers::INT_RAW_TIMERS_SPEC>,
    #[doc = "0x78 - INT_ST_TIMG_REG"]
    pub int_st_timg: crate::Reg<int_st_timg::INT_ST_TIMG_SPEC>,
    #[doc = "0x7c - INT_CLR_TIMG_REG"]
    pub int_clr_timers: crate::Reg<int_clr_timers::INT_CLR_TIMERS_SPEC>,
    #[doc = "0x80 - TIMG_RTCCALICFG2_REG."]
    pub rtccalicfg2: crate::Reg<rtccalicfg2::RTCCALICFG2_SPEC>,
    _reserved24: [u8; 0x74],
    #[doc = "0xf8 - TIMG_NTIMG_DATE_REG."]
    pub ntimg_date: crate::Reg<ntimg_date::NTIMG_DATE_SPEC>,
    #[doc = "0xfc - TIMG_REGCLK_REG."]
    pub regclk: crate::Reg<regclk::REGCLK_SPEC>,
}
#[doc = "T0CONFIG register accessor: an alias for `Reg<T0CONFIG_SPEC>`"]
pub type T0CONFIG = crate::Reg<t0config::T0CONFIG_SPEC>;
#[doc = "TIMG_T0CONFIG_REG."]
pub mod t0config;
#[doc = "T0LO register accessor: an alias for `Reg<T0LO_SPEC>`"]
pub type T0LO = crate::Reg<t0lo::T0LO_SPEC>;
#[doc = "TIMG_T0LO_REG."]
pub mod t0lo;
#[doc = "T0HI register accessor: an alias for `Reg<T0HI_SPEC>`"]
pub type T0HI = crate::Reg<t0hi::T0HI_SPEC>;
#[doc = "TIMG_T0HI_REG."]
pub mod t0hi;
#[doc = "T0UPDATE register accessor: an alias for `Reg<T0UPDATE_SPEC>`"]
pub type T0UPDATE = crate::Reg<t0update::T0UPDATE_SPEC>;
#[doc = "TIMG_T0UPDATE_REG."]
pub mod t0update;
#[doc = "T0ALARMLO register accessor: an alias for `Reg<T0ALARMLO_SPEC>`"]
pub type T0ALARMLO = crate::Reg<t0alarmlo::T0ALARMLO_SPEC>;
#[doc = "TIMG_T0ALARMLO_REG."]
pub mod t0alarmlo;
#[doc = "T0ALARMHI register accessor: an alias for `Reg<T0ALARMHI_SPEC>`"]
pub type T0ALARMHI = crate::Reg<t0alarmhi::T0ALARMHI_SPEC>;
#[doc = "TIMG_T0ALARMHI_REG."]
pub mod t0alarmhi;
#[doc = "T0LOADLO register accessor: an alias for `Reg<T0LOADLO_SPEC>`"]
pub type T0LOADLO = crate::Reg<t0loadlo::T0LOADLO_SPEC>;
#[doc = "TIMG_T0LOADLO_REG."]
pub mod t0loadlo;
#[doc = "T0LOADHI register accessor: an alias for `Reg<T0LOADHI_SPEC>`"]
pub type T0LOADHI = crate::Reg<t0loadhi::T0LOADHI_SPEC>;
#[doc = "TIMG_T0LOADHI_REG."]
pub mod t0loadhi;
#[doc = "T0LOAD register accessor: an alias for `Reg<T0LOAD_SPEC>`"]
pub type T0LOAD = crate::Reg<t0load::T0LOAD_SPEC>;
#[doc = "TIMG_T0LOAD_REG."]
pub mod t0load;
#[doc = "WDTCONFIG0 register accessor: an alias for `Reg<WDTCONFIG0_SPEC>`"]
pub type WDTCONFIG0 = crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>;
#[doc = "TIMG_WDTCONFIG0_REG."]
pub mod wdtconfig0;
#[doc = "WDTCONFIG1 register accessor: an alias for `Reg<WDTCONFIG1_SPEC>`"]
pub type WDTCONFIG1 = crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>;
#[doc = "TIMG_WDTCONFIG1_REG."]
pub mod wdtconfig1;
#[doc = "WDTCONFIG2 register accessor: an alias for `Reg<WDTCONFIG2_SPEC>`"]
pub type WDTCONFIG2 = crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>;
#[doc = "TIMG_WDTCONFIG2_REG."]
pub mod wdtconfig2;
#[doc = "WDTCONFIG3 register accessor: an alias for `Reg<WDTCONFIG3_SPEC>`"]
pub type WDTCONFIG3 = crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>;
#[doc = "TIMG_WDTCONFIG3_REG."]
pub mod wdtconfig3;
#[doc = "WDTCONFIG4 register accessor: an alias for `Reg<WDTCONFIG4_SPEC>`"]
pub type WDTCONFIG4 = crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>;
#[doc = "TIMG_WDTCONFIG4_REG."]
pub mod wdtconfig4;
#[doc = "WDTCONFIG5 register accessor: an alias for `Reg<WDTCONFIG5_SPEC>`"]
pub type WDTCONFIG5 = crate::Reg<wdtconfig5::WDTCONFIG5_SPEC>;
#[doc = "TIMG_WDTCONFIG5_REG."]
pub mod wdtconfig5;
#[doc = "WDTFEED register accessor: an alias for `Reg<WDTFEED_SPEC>`"]
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
#[doc = "TIMG_WDTFEED_REG."]
pub mod wdtfeed;
#[doc = "WDTWPROTECT register accessor: an alias for `Reg<WDTWPROTECT_SPEC>`"]
pub type WDTWPROTECT = crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>;
#[doc = "TIMG_WDTWPROTECT_REG."]
pub mod wdtwprotect;
#[doc = "RTCCALICFG register accessor: an alias for `Reg<RTCCALICFG_SPEC>`"]
pub type RTCCALICFG = crate::Reg<rtccalicfg::RTCCALICFG_SPEC>;
#[doc = "TIMG_RTCCALICFG_REG."]
pub mod rtccalicfg;
#[doc = "RTCCALICFG1 register accessor: an alias for `Reg<RTCCALICFG1_SPEC>`"]
pub type RTCCALICFG1 = crate::Reg<rtccalicfg1::RTCCALICFG1_SPEC>;
#[doc = "TIMG_RTCCALICFG1_REG."]
pub mod rtccalicfg1;
#[doc = "INT_ENA_TIMG register accessor: an alias for `Reg<INT_ENA_TIMG_SPEC>`"]
pub type INT_ENA_TIMG = crate::Reg<int_ena_timg::INT_ENA_TIMG_SPEC>;
#[doc = "INT_ENA_TIMG_REG"]
pub mod int_ena_timg;
#[doc = "INT_RAW_TIMERS register accessor: an alias for `Reg<INT_RAW_TIMERS_SPEC>`"]
pub type INT_RAW_TIMERS = crate::Reg<int_raw_timers::INT_RAW_TIMERS_SPEC>;
#[doc = "INT_RAW_TIMG_REG"]
pub mod int_raw_timers;
#[doc = "INT_ST_TIMG register accessor: an alias for `Reg<INT_ST_TIMG_SPEC>`"]
pub type INT_ST_TIMG = crate::Reg<int_st_timg::INT_ST_TIMG_SPEC>;
#[doc = "INT_ST_TIMG_REG"]
pub mod int_st_timg;
#[doc = "INT_CLR_TIMERS register accessor: an alias for `Reg<INT_CLR_TIMERS_SPEC>`"]
pub type INT_CLR_TIMERS = crate::Reg<int_clr_timers::INT_CLR_TIMERS_SPEC>;
#[doc = "INT_CLR_TIMG_REG"]
pub mod int_clr_timers;
#[doc = "RTCCALICFG2 register accessor: an alias for `Reg<RTCCALICFG2_SPEC>`"]
pub type RTCCALICFG2 = crate::Reg<rtccalicfg2::RTCCALICFG2_SPEC>;
#[doc = "TIMG_RTCCALICFG2_REG."]
pub mod rtccalicfg2;
#[doc = "NTIMG_DATE register accessor: an alias for `Reg<NTIMG_DATE_SPEC>`"]
pub type NTIMG_DATE = crate::Reg<ntimg_date::NTIMG_DATE_SPEC>;
#[doc = "TIMG_NTIMG_DATE_REG."]
pub mod ntimg_date;
#[doc = "REGCLK register accessor: an alias for `Reg<REGCLK_SPEC>`"]
pub type REGCLK = crate::Reg<regclk::REGCLK_SPEC>;
#[doc = "TIMG_REGCLK_REG."]
pub mod regclk;
