#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer %s configuration register"]
    pub t0config: crate::Reg<tconfig::TCONFIG_SPEC>,
    #[doc = "0x04 - Timer %s current value, low 32 bits"]
    pub t0lo: crate::Reg<tlo::TLO_SPEC>,
    #[doc = "0x08 - Timer %s current value, high 22 bits"]
    pub t0hi: crate::Reg<thi::THI_SPEC>,
    #[doc = "0x0c - Write to copy current timer value to TIMGn_T%s_(LO/HI)_REG"]
    pub t0update: crate::Reg<tupdate::TUPDATE_SPEC>,
    #[doc = "0x10 - Timer %s alarm value, low 32 bits"]
    pub t0alarmlo: crate::Reg<talarmlo::TALARMLO_SPEC>,
    #[doc = "0x14 - Timer %s alarm value, high bits"]
    pub t0alarmhi: crate::Reg<talarmhi::TALARMHI_SPEC>,
    #[doc = "0x18 - Timer %s reload value, low 32 bits"]
    pub t0loadlo: crate::Reg<tloadlo::TLOADLO_SPEC>,
    #[doc = "0x1c - Timer %s reload value, high 22 bits"]
    pub t0loadhi: crate::Reg<tloadhi::TLOADHI_SPEC>,
    #[doc = "0x20 - Write to reload timer from TIMG_T%s_(LOADLOLOADHI)_REG"]
    pub t0load: crate::Reg<tload::TLOAD_SPEC>,
    #[doc = "0x24 - Timer %s configuration register"]
    pub t1config: crate::Reg<tconfig::TCONFIG_SPEC>,
    #[doc = "0x28 - Timer %s current value, low 32 bits"]
    pub t1lo: crate::Reg<tlo::TLO_SPEC>,
    #[doc = "0x2c - Timer %s current value, high 22 bits"]
    pub t1hi: crate::Reg<thi::THI_SPEC>,
    #[doc = "0x30 - Write to copy current timer value to TIMGn_T%s_(LO/HI)_REG"]
    pub t1update: crate::Reg<tupdate::TUPDATE_SPEC>,
    #[doc = "0x34 - Timer %s alarm value, low 32 bits"]
    pub t1alarmlo: crate::Reg<talarmlo::TALARMLO_SPEC>,
    #[doc = "0x38 - Timer %s alarm value, high bits"]
    pub t1alarmhi: crate::Reg<talarmhi::TALARMHI_SPEC>,
    #[doc = "0x3c - Timer %s reload value, low 32 bits"]
    pub t1loadlo: crate::Reg<tloadlo::TLOADLO_SPEC>,
    #[doc = "0x40 - Timer %s reload value, high 22 bits"]
    pub t1loadhi: crate::Reg<tloadhi::TLOADHI_SPEC>,
    #[doc = "0x44 - Write to reload timer from TIMG_T%s_(LOADLOLOADHI)_REG"]
    pub t1load: crate::Reg<tload::TLOAD_SPEC>,
    #[doc = "0x48 - Watchdog timer configuration register"]
    pub wdtconfig0: crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>,
    #[doc = "0x4c - Watchdog timer prescaler register"]
    pub wdtconfig1: crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>,
    #[doc = "0x50 - Watchdog timer stage 0 timeout value"]
    pub wdtconfig2: crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>,
    #[doc = "0x54 - Watchdog timer stage 1 timeout value"]
    pub wdtconfig3: crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>,
    #[doc = "0x58 - Watchdog timer stage 2 timeout value"]
    pub wdtconfig4: crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>,
    #[doc = "0x5c - Watchdog timer stage 3 timeout value"]
    pub wdtconfig5: crate::Reg<wdtconfig5::WDTCONFIG5_SPEC>,
    #[doc = "0x60 - Write to feed the watchdog timer"]
    pub wdtfeed: crate::Reg<wdtfeed::WDTFEED_SPEC>,
    #[doc = "0x64 - Watchdog write protect register"]
    pub wdtwprotect: crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>,
    #[doc = "0x68 - RTC calibration configure register"]
    pub rtccalicfg: crate::Reg<rtccalicfg::RTCCALICFG_SPEC>,
    #[doc = "0x6c - RTC calibration configure1 register"]
    pub rtccalicfg1: crate::Reg<rtccalicfg1::RTCCALICFG1_SPEC>,
    #[doc = "0x70 - Interrupt enable bits"]
    pub int_ena_timers: crate::Reg<int_ena_timers::INT_ENA_TIMERS_SPEC>,
    #[doc = "0x74 - Raw interrupt status"]
    pub int_raw_timers: crate::Reg<int_raw_timers::INT_RAW_TIMERS_SPEC>,
    #[doc = "0x78 - Masked interrupt status"]
    pub int_st_timers: crate::Reg<int_st_timers::INT_ST_TIMERS_SPEC>,
    #[doc = "0x7c - Interrupt clear bits"]
    pub int_clr_timers: crate::Reg<int_clr_timers::INT_CLR_TIMERS_SPEC>,
    #[doc = "0x80 - Timer group calibration register"]
    pub rtccalicfg2: crate::Reg<rtccalicfg2::RTCCALICFG2_SPEC>,
    _reserved33: [u8; 0x74],
    #[doc = "0xf8 - Timer version control register"]
    pub ntimers_date: crate::Reg<ntimers_date::NTIMERS_DATE_SPEC>,
    #[doc = "0xfc - Timer group clock gate register"]
    pub regclk: crate::Reg<regclk::REGCLK_SPEC>,
}
#[doc = "TCONFIG register accessor: an alias for `Reg<TCONFIG_SPEC>`"]
pub type TCONFIG = crate::Reg<tconfig::TCONFIG_SPEC>;
#[doc = "Timer %s configuration register"]
pub mod tconfig;
#[doc = "TLO register accessor: an alias for `Reg<TLO_SPEC>`"]
pub type TLO = crate::Reg<tlo::TLO_SPEC>;
#[doc = "Timer %s current value, low 32 bits"]
pub mod tlo;
#[doc = "THI register accessor: an alias for `Reg<THI_SPEC>`"]
pub type THI = crate::Reg<thi::THI_SPEC>;
#[doc = "Timer %s current value, high 22 bits"]
pub mod thi;
#[doc = "TUPDATE register accessor: an alias for `Reg<TUPDATE_SPEC>`"]
pub type TUPDATE = crate::Reg<tupdate::TUPDATE_SPEC>;
#[doc = "Write to copy current timer value to TIMGn_T%s_(LO/HI)_REG"]
pub mod tupdate;
#[doc = "TALARMLO register accessor: an alias for `Reg<TALARMLO_SPEC>`"]
pub type TALARMLO = crate::Reg<talarmlo::TALARMLO_SPEC>;
#[doc = "Timer %s alarm value, low 32 bits"]
pub mod talarmlo;
#[doc = "TALARMHI register accessor: an alias for `Reg<TALARMHI_SPEC>`"]
pub type TALARMHI = crate::Reg<talarmhi::TALARMHI_SPEC>;
#[doc = "Timer %s alarm value, high bits"]
pub mod talarmhi;
#[doc = "TLOADLO register accessor: an alias for `Reg<TLOADLO_SPEC>`"]
pub type TLOADLO = crate::Reg<tloadlo::TLOADLO_SPEC>;
#[doc = "Timer %s reload value, low 32 bits"]
pub mod tloadlo;
#[doc = "TLOADHI register accessor: an alias for `Reg<TLOADHI_SPEC>`"]
pub type TLOADHI = crate::Reg<tloadhi::TLOADHI_SPEC>;
#[doc = "Timer %s reload value, high 22 bits"]
pub mod tloadhi;
#[doc = "TLOAD register accessor: an alias for `Reg<TLOAD_SPEC>`"]
pub type TLOAD = crate::Reg<tload::TLOAD_SPEC>;
#[doc = "Write to reload timer from TIMG_T%s_(LOADLOLOADHI)_REG"]
pub mod tload;
#[doc = "WDTCONFIG0 register accessor: an alias for `Reg<WDTCONFIG0_SPEC>`"]
pub type WDTCONFIG0 = crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>;
#[doc = "Watchdog timer configuration register"]
pub mod wdtconfig0;
#[doc = "WDTCONFIG1 register accessor: an alias for `Reg<WDTCONFIG1_SPEC>`"]
pub type WDTCONFIG1 = crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>;
#[doc = "Watchdog timer prescaler register"]
pub mod wdtconfig1;
#[doc = "WDTCONFIG2 register accessor: an alias for `Reg<WDTCONFIG2_SPEC>`"]
pub type WDTCONFIG2 = crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>;
#[doc = "Watchdog timer stage 0 timeout value"]
pub mod wdtconfig2;
#[doc = "WDTCONFIG3 register accessor: an alias for `Reg<WDTCONFIG3_SPEC>`"]
pub type WDTCONFIG3 = crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>;
#[doc = "Watchdog timer stage 1 timeout value"]
pub mod wdtconfig3;
#[doc = "WDTCONFIG4 register accessor: an alias for `Reg<WDTCONFIG4_SPEC>`"]
pub type WDTCONFIG4 = crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>;
#[doc = "Watchdog timer stage 2 timeout value"]
pub mod wdtconfig4;
#[doc = "WDTCONFIG5 register accessor: an alias for `Reg<WDTCONFIG5_SPEC>`"]
pub type WDTCONFIG5 = crate::Reg<wdtconfig5::WDTCONFIG5_SPEC>;
#[doc = "Watchdog timer stage 3 timeout value"]
pub mod wdtconfig5;
#[doc = "WDTFEED register accessor: an alias for `Reg<WDTFEED_SPEC>`"]
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
#[doc = "Write to feed the watchdog timer"]
pub mod wdtfeed;
#[doc = "WDTWPROTECT register accessor: an alias for `Reg<WDTWPROTECT_SPEC>`"]
pub type WDTWPROTECT = crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>;
#[doc = "Watchdog write protect register"]
pub mod wdtwprotect;
#[doc = "RTCCALICFG register accessor: an alias for `Reg<RTCCALICFG_SPEC>`"]
pub type RTCCALICFG = crate::Reg<rtccalicfg::RTCCALICFG_SPEC>;
#[doc = "RTC calibration configure register"]
pub mod rtccalicfg;
#[doc = "RTCCALICFG1 register accessor: an alias for `Reg<RTCCALICFG1_SPEC>`"]
pub type RTCCALICFG1 = crate::Reg<rtccalicfg1::RTCCALICFG1_SPEC>;
#[doc = "RTC calibration configure1 register"]
pub mod rtccalicfg1;
#[doc = "INT_ENA_TIMERS register accessor: an alias for `Reg<INT_ENA_TIMERS_SPEC>`"]
pub type INT_ENA_TIMERS = crate::Reg<int_ena_timers::INT_ENA_TIMERS_SPEC>;
#[doc = "Interrupt enable bits"]
pub mod int_ena_timers;
#[doc = "INT_RAW_TIMERS register accessor: an alias for `Reg<INT_RAW_TIMERS_SPEC>`"]
pub type INT_RAW_TIMERS = crate::Reg<int_raw_timers::INT_RAW_TIMERS_SPEC>;
#[doc = "Raw interrupt status"]
pub mod int_raw_timers;
#[doc = "INT_ST_TIMERS register accessor: an alias for `Reg<INT_ST_TIMERS_SPEC>`"]
pub type INT_ST_TIMERS = crate::Reg<int_st_timers::INT_ST_TIMERS_SPEC>;
#[doc = "Masked interrupt status"]
pub mod int_st_timers;
#[doc = "INT_CLR_TIMERS register accessor: an alias for `Reg<INT_CLR_TIMERS_SPEC>`"]
pub type INT_CLR_TIMERS = crate::Reg<int_clr_timers::INT_CLR_TIMERS_SPEC>;
#[doc = "Interrupt clear bits"]
pub mod int_clr_timers;
#[doc = "RTCCALICFG2 register accessor: an alias for `Reg<RTCCALICFG2_SPEC>`"]
pub type RTCCALICFG2 = crate::Reg<rtccalicfg2::RTCCALICFG2_SPEC>;
#[doc = "Timer group calibration register"]
pub mod rtccalicfg2;
#[doc = "NTIMERS_DATE register accessor: an alias for `Reg<NTIMERS_DATE_SPEC>`"]
pub type NTIMERS_DATE = crate::Reg<ntimers_date::NTIMERS_DATE_SPEC>;
#[doc = "Timer version control register"]
pub mod ntimers_date;
#[doc = "REGCLK register accessor: an alias for `Reg<REGCLK_SPEC>`"]
pub type REGCLK = crate::Reg<regclk::REGCLK_SPEC>;
#[doc = "Timer group clock gate register"]
pub mod regclk;
