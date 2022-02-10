#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - core0 monitor enable configuration register"]
    pub core_0_interrupt_ena: crate::Reg<core_0_interrupt_ena::CORE_0_INTERRUPT_ENA_SPEC>,
    #[doc = "0x04 - core0 monitor interrupt status register"]
    pub core_0_interrupt_raw: crate::Reg<core_0_interrupt_raw::CORE_0_INTERRUPT_RAW_SPEC>,
    #[doc = "0x08 - core0 monitor interrupt enable register"]
    pub core_0_interrupt_rls: crate::Reg<core_0_interrupt_rls::CORE_0_INTERRUPT_RLS_SPEC>,
    #[doc = "0x0c - core0 monitor interrupt clr register"]
    pub core_0_interrupt_clr: crate::Reg<core_0_interrupt_clr::CORE_0_INTERRUPT_CLR_SPEC>,
    #[doc = "0x10 - core0 dram0 region0 addr configuration register"]
    pub core_0_area_dram0_0_min: crate::Reg<core_0_area_dram0_0_min::CORE_0_AREA_DRAM0_0_MIN_SPEC>,
    #[doc = "0x14 - core0 dram0 region0 addr configuration register"]
    pub core_0_area_dram0_0_max: crate::Reg<core_0_area_dram0_0_max::CORE_0_AREA_DRAM0_0_MAX_SPEC>,
    #[doc = "0x18 - core0 dram0 region1 addr configuration register"]
    pub core_0_area_dram0_1_min: crate::Reg<core_0_area_dram0_1_min::CORE_0_AREA_DRAM0_1_MIN_SPEC>,
    #[doc = "0x1c - core0 dram0 region1 addr configuration register"]
    pub core_0_area_dram0_1_max: crate::Reg<core_0_area_dram0_1_max::CORE_0_AREA_DRAM0_1_MAX_SPEC>,
    #[doc = "0x20 - core0 PIF region0 addr configuration register"]
    pub core_0_area_pif_0_min: crate::Reg<core_0_area_pif_0_min::CORE_0_AREA_PIF_0_MIN_SPEC>,
    #[doc = "0x24 - core0 PIF region0 addr configuration register"]
    pub core_0_area_pif_0_max: crate::Reg<core_0_area_pif_0_max::CORE_0_AREA_PIF_0_MAX_SPEC>,
    #[doc = "0x28 - core0 PIF region1 addr configuration register"]
    pub core_0_area_pif_1_min: crate::Reg<core_0_area_pif_1_min::CORE_0_AREA_PIF_1_MIN_SPEC>,
    #[doc = "0x2c - core0 PIF region1 addr configuration register"]
    pub core_0_area_pif_1_max: crate::Reg<core_0_area_pif_1_max::CORE_0_AREA_PIF_1_MAX_SPEC>,
    #[doc = "0x30 - core0 area sp status register"]
    pub core_0_area_sp: crate::Reg<core_0_area_sp::CORE_0_AREA_SP_SPEC>,
    #[doc = "0x34 - core0 area pc status register"]
    pub core_0_area_pc: crate::Reg<core_0_area_pc::CORE_0_AREA_PC_SPEC>,
    #[doc = "0x38 - core0 sp unstable configuration register"]
    pub core_0_sp_unstable: crate::Reg<core_0_sp_unstable::CORE_0_SP_UNSTABLE_SPEC>,
    #[doc = "0x3c - core0 sp region configuration regsiter"]
    pub core_0_sp_min: crate::Reg<core_0_sp_min::CORE_0_SP_MIN_SPEC>,
    #[doc = "0x40 - core0 sp region configuration regsiter"]
    pub core_0_sp_max: crate::Reg<core_0_sp_max::CORE_0_SP_MAX_SPEC>,
    #[doc = "0x44 - core0 sp pc status register"]
    pub core_0_sp_pc: crate::Reg<core_0_sp_pc::CORE_0_SP_PC_SPEC>,
    #[doc = "0x48 - core0 pdebug configuration register"]
    pub core_0_rcd_pdebugenable: crate::Reg<core_0_rcd_pdebugenable::CORE_0_RCD_PDEBUGENABLE_SPEC>,
    #[doc = "0x4c - core0 pdebug status register"]
    pub core_0_rcd_recording: crate::Reg<core_0_rcd_recording::CORE_0_RCD_RECORDING_SPEC>,
    #[doc = "0x50 - core0 pdebug status register"]
    pub core_0_rcd_pdebuginst: crate::Reg<core_0_rcd_pdebuginst::CORE_0_RCD_PDEBUGINST_SPEC>,
    #[doc = "0x54 - core0 pdebug status register"]
    pub core_0_rcd_pdebugstatus: crate::Reg<core_0_rcd_pdebugstatus::CORE_0_RCD_PDEBUGSTATUS_SPEC>,
    #[doc = "0x58 - core0 pdebug status register"]
    pub core_0_rcd_pdebugdata: crate::Reg<core_0_rcd_pdebugdata::CORE_0_RCD_PDEBUGDATA_SPEC>,
    #[doc = "0x5c - core0 pdebug status register"]
    pub core_0_rcd_pdebugpc: crate::Reg<core_0_rcd_pdebugpc::CORE_0_RCD_PDEBUGPC_SPEC>,
    #[doc = "0x60 - core0 pdebug status register"]
    pub core_0_rcd_pdebugls0stat:
        crate::Reg<core_0_rcd_pdebugls0stat::CORE_0_RCD_PDEBUGLS0STAT_SPEC>,
    #[doc = "0x64 - core0 pdebug status register"]
    pub core_0_rcd_pdebugls0addr:
        crate::Reg<core_0_rcd_pdebugls0addr::CORE_0_RCD_PDEBUGLS0ADDR_SPEC>,
    #[doc = "0x68 - core0 pdebug status register"]
    pub core_0_rcd_pdebugls0data:
        crate::Reg<core_0_rcd_pdebugls0data::CORE_0_RCD_PDEBUGLS0DATA_SPEC>,
    #[doc = "0x6c - core0 pdebug status register"]
    pub core_0_rcd_sp: crate::Reg<core_0_rcd_sp::CORE_0_RCD_SP_SPEC>,
    #[doc = "0x70 - core0 bus busy status regsiter"]
    pub core_0_iram0_exception_monitor_0:
        crate::Reg<core_0_iram0_exception_monitor_0::CORE_0_IRAM0_EXCEPTION_MONITOR_0_SPEC>,
    #[doc = "0x74 - core0 bus busy status regsiter"]
    pub core_0_iram0_exception_monitor_1:
        crate::Reg<core_0_iram0_exception_monitor_1::CORE_0_IRAM0_EXCEPTION_MONITOR_1_SPEC>,
    #[doc = "0x78 - core0 bus busy status regsiter"]
    pub core_0_dram0_exception_monitor_0:
        crate::Reg<core_0_dram0_exception_monitor_0::CORE_0_DRAM0_EXCEPTION_MONITOR_0_SPEC>,
    #[doc = "0x7c - core0 bus busy status regsiter"]
    pub core_0_dram0_exception_monitor_1:
        crate::Reg<core_0_dram0_exception_monitor_1::CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC>,
    #[doc = "0x80 - core0 bus busy status regsiter"]
    pub core_0_dram0_exception_monitor_2:
        crate::Reg<core_0_dram0_exception_monitor_2::CORE_0_DRAM0_EXCEPTION_MONITOR_2_SPEC>,
    #[doc = "0x84 - core0 bus busy status regsiter"]
    pub core_0_dram0_exception_monitor_3:
        crate::Reg<core_0_dram0_exception_monitor_3::CORE_0_DRAM0_EXCEPTION_MONITOR_3_SPEC>,
    #[doc = "0x88 - core0 bus busy configuration regsiter"]
    pub core_0_dram0_exception_monitor_4:
        crate::Reg<core_0_dram0_exception_monitor_4::CORE_0_DRAM0_EXCEPTION_MONITOR_4_SPEC>,
    #[doc = "0x8c - core0 bus busy configuration regsiter"]
    pub core_0_dram0_exception_monitor_5:
        crate::Reg<core_0_dram0_exception_monitor_5::CORE_0_DRAM0_EXCEPTION_MONITOR_5_SPEC>,
    #[doc = "0x90 - Core1 monitor enable configuration register"]
    pub core_1_interrupt_ena: crate::Reg<core_1_interrupt_ena::CORE_1_INTERRUPT_ENA_SPEC>,
    #[doc = "0x94 - Core1 monitor interrupt status register"]
    pub core_1_interrupt_raw: crate::Reg<core_1_interrupt_raw::CORE_1_INTERRUPT_RAW_SPEC>,
    #[doc = "0x98 - Core1 monitor interrupt enable register"]
    pub core_1_interrupt_rls: crate::Reg<core_1_interrupt_rls::CORE_1_INTERRUPT_RLS_SPEC>,
    #[doc = "0x9c - Core1 monitor interrupt clr register"]
    pub core_1_interrupt_clr: crate::Reg<core_1_interrupt_clr::CORE_1_INTERRUPT_CLR_SPEC>,
    #[doc = "0xa0 - Core1 dram0 region0 addr configuration register"]
    pub core_1_area_dram0_0_min: crate::Reg<core_1_area_dram0_0_min::CORE_1_AREA_DRAM0_0_MIN_SPEC>,
    #[doc = "0xa4 - Core1 dram0 region0 addr configuration register"]
    pub core_1_area_dram0_0_max: crate::Reg<core_1_area_dram0_0_max::CORE_1_AREA_DRAM0_0_MAX_SPEC>,
    #[doc = "0xa8 - Core1 dram0 region1 addr configuration register"]
    pub core_1_area_dram0_1_min: crate::Reg<core_1_area_dram0_1_min::CORE_1_AREA_DRAM0_1_MIN_SPEC>,
    #[doc = "0xac - Core1 dram0 region1 addr configuration register"]
    pub core_1_area_dram0_1_max: crate::Reg<core_1_area_dram0_1_max::CORE_1_AREA_DRAM0_1_MAX_SPEC>,
    #[doc = "0xb0 - Core1 PIF region0 addr configuration register"]
    pub core_1_area_pif_0_min: crate::Reg<core_1_area_pif_0_min::CORE_1_AREA_PIF_0_MIN_SPEC>,
    #[doc = "0xb4 - Core1 PIF region0 addr configuration register"]
    pub core_1_area_pif_0_max: crate::Reg<core_1_area_pif_0_max::CORE_1_AREA_PIF_0_MAX_SPEC>,
    #[doc = "0xb8 - Core1 PIF region1 addr configuration register"]
    pub core_1_area_pif_1_min: crate::Reg<core_1_area_pif_1_min::CORE_1_AREA_PIF_1_MIN_SPEC>,
    #[doc = "0xbc - Core1 PIF region1 addr configuration register"]
    pub core_1_area_pif_1_max: crate::Reg<core_1_area_pif_1_max::CORE_1_AREA_PIF_1_MAX_SPEC>,
    #[doc = "0xc0 - Core1 area sp status register"]
    pub core_1_area_pc: crate::Reg<core_1_area_pc::CORE_1_AREA_PC_SPEC>,
    #[doc = "0xc4 - Core1 area pc status register"]
    pub core_1_area_sp: crate::Reg<core_1_area_sp::CORE_1_AREA_SP_SPEC>,
    #[doc = "0xc8 - Core1 sp unstable configuration register"]
    pub core_1_sp_unstable: crate::Reg<core_1_sp_unstable::CORE_1_SP_UNSTABLE_SPEC>,
    #[doc = "0xcc - Core1 sp region configuration regsiter"]
    pub core_1_sp_min: crate::Reg<core_1_sp_min::CORE_1_SP_MIN_SPEC>,
    #[doc = "0xd0 - Core1 sp region configuration regsiter"]
    pub core_1_sp_max: crate::Reg<core_1_sp_max::CORE_1_SP_MAX_SPEC>,
    #[doc = "0xd4 - Core1 sp pc status register"]
    pub core_1_sp_pc: crate::Reg<core_1_sp_pc::CORE_1_SP_PC_SPEC>,
    #[doc = "0xd8 - Core1 pdebug configuration register"]
    pub core_1_rcd_pdebugenable: crate::Reg<core_1_rcd_pdebugenable::CORE_1_RCD_PDEBUGENABLE_SPEC>,
    #[doc = "0xdc - Core1 pdebug status register"]
    pub core_1_rcd_recording: crate::Reg<core_1_rcd_recording::CORE_1_RCD_RECORDING_SPEC>,
    #[doc = "0xe0 - Core1 pdebug status register"]
    pub core_1_rcd_pdebuginst: crate::Reg<core_1_rcd_pdebuginst::CORE_1_RCD_PDEBUGINST_SPEC>,
    #[doc = "0xe4 - Core1 pdebug status register"]
    pub core_1_rcd_pdebugstatus: crate::Reg<core_1_rcd_pdebugstatus::CORE_1_RCD_PDEBUGSTATUS_SPEC>,
    #[doc = "0xe8 - Core1 pdebug status register"]
    pub core_1_rcd_pdebugdata: crate::Reg<core_1_rcd_pdebugdata::CORE_1_RCD_PDEBUGDATA_SPEC>,
    #[doc = "0xec - Core1 pdebug status register"]
    pub core_1_rcd_pdebugpc: crate::Reg<core_1_rcd_pdebugpc::CORE_1_RCD_PDEBUGPC_SPEC>,
    #[doc = "0xf0 - Core1 pdebug status register"]
    pub core_1_rcd_pdebugls0stat:
        crate::Reg<core_1_rcd_pdebugls0stat::CORE_1_RCD_PDEBUGLS0STAT_SPEC>,
    #[doc = "0xf4 - Core1 pdebug status register"]
    pub core_1_rcd_pdebugls0addr:
        crate::Reg<core_1_rcd_pdebugls0addr::CORE_1_RCD_PDEBUGLS0ADDR_SPEC>,
    #[doc = "0xf8 - Core1 pdebug status register"]
    pub core_1_rcd_pdebugls0data:
        crate::Reg<core_1_rcd_pdebugls0data::CORE_1_RCD_PDEBUGLS0DATA_SPEC>,
    #[doc = "0xfc - Core1 pdebug status register"]
    pub core_1_rcd_sp: crate::Reg<core_1_rcd_sp::CORE_1_RCD_SP_SPEC>,
    #[doc = "0x100 - Core1 bus busy status regsiter"]
    pub core_1_iram0_exception_monitor_0:
        crate::Reg<core_1_iram0_exception_monitor_0::CORE_1_IRAM0_EXCEPTION_MONITOR_0_SPEC>,
    #[doc = "0x104 - Core1 bus busy status regsiter"]
    pub core_1_iram0_exception_monitor_1:
        crate::Reg<core_1_iram0_exception_monitor_1::CORE_1_IRAM0_EXCEPTION_MONITOR_1_SPEC>,
    #[doc = "0x108 - Core1 bus busy status regsiter"]
    pub core_1_dram0_exception_monitor_0:
        crate::Reg<core_1_dram0_exception_monitor_0::CORE_1_DRAM0_EXCEPTION_MONITOR_0_SPEC>,
    #[doc = "0x10c - Core1 bus busy status regsiter"]
    pub core_1_dram0_exception_monitor_1:
        crate::Reg<core_1_dram0_exception_monitor_1::CORE_1_DRAM0_EXCEPTION_MONITOR_1_SPEC>,
    #[doc = "0x110 - Core1 bus busy status regsiter"]
    pub core_1_dram0_exception_monitor_2:
        crate::Reg<core_1_dram0_exception_monitor_2::CORE_1_DRAM0_EXCEPTION_MONITOR_2_SPEC>,
    #[doc = "0x114 - Core1 bus busy status regsiter"]
    pub core_1_dram0_exception_monitor_3:
        crate::Reg<core_1_dram0_exception_monitor_3::CORE_1_DRAM0_EXCEPTION_MONITOR_3_SPEC>,
    #[doc = "0x118 - Core1 bus busy status regsiter"]
    pub core_1_dram0_exception_monitor_4:
        crate::Reg<core_1_dram0_exception_monitor_4::CORE_1_DRAM0_EXCEPTION_MONITOR_4_SPEC>,
    #[doc = "0x11c - Core1 bus busy status regsiter"]
    pub core_1_dram0_exception_monitor_5:
        crate::Reg<core_1_dram0_exception_monitor_5::CORE_1_DRAM0_EXCEPTION_MONITOR_5_SPEC>,
    #[doc = "0x120 - bus busy configuration register"]
    pub core_x_iram0_dram0_exception_monitor_0: crate::Reg<
        core_x_iram0_dram0_exception_monitor_0::CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC,
    >,
    #[doc = "0x124 - bus busy configuration register"]
    pub core_x_iram0_dram0_exception_monitor_1: crate::Reg<
        core_x_iram0_dram0_exception_monitor_1::CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC,
    >,
    #[doc = "0x128 - log set register"]
    pub log_setting: crate::Reg<log_setting::LOG_SETTING_SPEC>,
    #[doc = "0x12c - log check data register"]
    pub log_data_0: crate::Reg<log_data_0::LOG_DATA_0_SPEC>,
    #[doc = "0x130 - log check data register"]
    pub log_data_1: crate::Reg<log_data_1::LOG_DATA_1_SPEC>,
    #[doc = "0x134 - log check data register"]
    pub log_data_2: crate::Reg<log_data_2::LOG_DATA_2_SPEC>,
    #[doc = "0x138 - log check data register"]
    pub log_data_3: crate::Reg<log_data_3::LOG_DATA_3_SPEC>,
    #[doc = "0x13c - log check data mask register"]
    pub log_data_mask: crate::Reg<log_data_mask::LOG_DATA_MASK_SPEC>,
    #[doc = "0x140 - log check region configuration register"]
    pub log_min: crate::Reg<log_min::LOG_MIN_SPEC>,
    #[doc = "0x144 - log check region configuration register"]
    pub log_max: crate::Reg<log_max::LOG_MAX_SPEC>,
    #[doc = "0x148 - log mem region configuration register"]
    pub log_mem_start: crate::Reg<log_mem_start::LOG_MEM_START_SPEC>,
    #[doc = "0x14c - log mem region configuration register"]
    pub log_mem_end: crate::Reg<log_mem_end::LOG_MEM_END_SPEC>,
    #[doc = "0x150 - log mem addr status register"]
    pub log_mem_writing_addr: crate::Reg<log_mem_writing_addr::LOG_MEM_WRITING_ADDR_SPEC>,
    #[doc = "0x154 - log mem status register"]
    pub log_mem_full_flag: crate::Reg<log_mem_full_flag::LOG_MEM_FULL_FLAG_SPEC>,
    _reserved86: [u8; 0xa4],
    #[doc = "0x1fc - version register"]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "CORE_0_INTERRUPT_ENA register accessor: an alias for `Reg<CORE_0_INTERRUPT_ENA_SPEC>`"]
pub type CORE_0_INTERRUPT_ENA = crate::Reg<core_0_interrupt_ena::CORE_0_INTERRUPT_ENA_SPEC>;
#[doc = "core0 monitor enable configuration register"]
pub mod core_0_interrupt_ena;
#[doc = "CORE_0_INTERRUPT_RAW register accessor: an alias for `Reg<CORE_0_INTERRUPT_RAW_SPEC>`"]
pub type CORE_0_INTERRUPT_RAW = crate::Reg<core_0_interrupt_raw::CORE_0_INTERRUPT_RAW_SPEC>;
#[doc = "core0 monitor interrupt status register"]
pub mod core_0_interrupt_raw;
#[doc = "CORE_0_INTERRUPT_RLS register accessor: an alias for `Reg<CORE_0_INTERRUPT_RLS_SPEC>`"]
pub type CORE_0_INTERRUPT_RLS = crate::Reg<core_0_interrupt_rls::CORE_0_INTERRUPT_RLS_SPEC>;
#[doc = "core0 monitor interrupt enable register"]
pub mod core_0_interrupt_rls;
#[doc = "CORE_0_INTERRUPT_CLR register accessor: an alias for `Reg<CORE_0_INTERRUPT_CLR_SPEC>`"]
pub type CORE_0_INTERRUPT_CLR = crate::Reg<core_0_interrupt_clr::CORE_0_INTERRUPT_CLR_SPEC>;
#[doc = "core0 monitor interrupt clr register"]
pub mod core_0_interrupt_clr;
#[doc = "CORE_0_AREA_DRAM0_0_MIN register accessor: an alias for `Reg<CORE_0_AREA_DRAM0_0_MIN_SPEC>`"]
pub type CORE_0_AREA_DRAM0_0_MIN =
    crate::Reg<core_0_area_dram0_0_min::CORE_0_AREA_DRAM0_0_MIN_SPEC>;
#[doc = "core0 dram0 region0 addr configuration register"]
pub mod core_0_area_dram0_0_min;
#[doc = "CORE_0_AREA_DRAM0_0_MAX register accessor: an alias for `Reg<CORE_0_AREA_DRAM0_0_MAX_SPEC>`"]
pub type CORE_0_AREA_DRAM0_0_MAX =
    crate::Reg<core_0_area_dram0_0_max::CORE_0_AREA_DRAM0_0_MAX_SPEC>;
#[doc = "core0 dram0 region0 addr configuration register"]
pub mod core_0_area_dram0_0_max;
#[doc = "CORE_0_AREA_DRAM0_1_MIN register accessor: an alias for `Reg<CORE_0_AREA_DRAM0_1_MIN_SPEC>`"]
pub type CORE_0_AREA_DRAM0_1_MIN =
    crate::Reg<core_0_area_dram0_1_min::CORE_0_AREA_DRAM0_1_MIN_SPEC>;
#[doc = "core0 dram0 region1 addr configuration register"]
pub mod core_0_area_dram0_1_min;
#[doc = "CORE_0_AREA_DRAM0_1_MAX register accessor: an alias for `Reg<CORE_0_AREA_DRAM0_1_MAX_SPEC>`"]
pub type CORE_0_AREA_DRAM0_1_MAX =
    crate::Reg<core_0_area_dram0_1_max::CORE_0_AREA_DRAM0_1_MAX_SPEC>;
#[doc = "core0 dram0 region1 addr configuration register"]
pub mod core_0_area_dram0_1_max;
#[doc = "CORE_0_AREA_PIF_0_MIN register accessor: an alias for `Reg<CORE_0_AREA_PIF_0_MIN_SPEC>`"]
pub type CORE_0_AREA_PIF_0_MIN = crate::Reg<core_0_area_pif_0_min::CORE_0_AREA_PIF_0_MIN_SPEC>;
#[doc = "core0 PIF region0 addr configuration register"]
pub mod core_0_area_pif_0_min;
#[doc = "CORE_0_AREA_PIF_0_MAX register accessor: an alias for `Reg<CORE_0_AREA_PIF_0_MAX_SPEC>`"]
pub type CORE_0_AREA_PIF_0_MAX = crate::Reg<core_0_area_pif_0_max::CORE_0_AREA_PIF_0_MAX_SPEC>;
#[doc = "core0 PIF region0 addr configuration register"]
pub mod core_0_area_pif_0_max;
#[doc = "CORE_0_AREA_PIF_1_MIN register accessor: an alias for `Reg<CORE_0_AREA_PIF_1_MIN_SPEC>`"]
pub type CORE_0_AREA_PIF_1_MIN = crate::Reg<core_0_area_pif_1_min::CORE_0_AREA_PIF_1_MIN_SPEC>;
#[doc = "core0 PIF region1 addr configuration register"]
pub mod core_0_area_pif_1_min;
#[doc = "CORE_0_AREA_PIF_1_MAX register accessor: an alias for `Reg<CORE_0_AREA_PIF_1_MAX_SPEC>`"]
pub type CORE_0_AREA_PIF_1_MAX = crate::Reg<core_0_area_pif_1_max::CORE_0_AREA_PIF_1_MAX_SPEC>;
#[doc = "core0 PIF region1 addr configuration register"]
pub mod core_0_area_pif_1_max;
#[doc = "CORE_0_AREA_SP register accessor: an alias for `Reg<CORE_0_AREA_SP_SPEC>`"]
pub type CORE_0_AREA_SP = crate::Reg<core_0_area_sp::CORE_0_AREA_SP_SPEC>;
#[doc = "core0 area sp status register"]
pub mod core_0_area_sp;
#[doc = "CORE_0_AREA_PC register accessor: an alias for `Reg<CORE_0_AREA_PC_SPEC>`"]
pub type CORE_0_AREA_PC = crate::Reg<core_0_area_pc::CORE_0_AREA_PC_SPEC>;
#[doc = "core0 area pc status register"]
pub mod core_0_area_pc;
#[doc = "CORE_0_SP_UNSTABLE register accessor: an alias for `Reg<CORE_0_SP_UNSTABLE_SPEC>`"]
pub type CORE_0_SP_UNSTABLE = crate::Reg<core_0_sp_unstable::CORE_0_SP_UNSTABLE_SPEC>;
#[doc = "core0 sp unstable configuration register"]
pub mod core_0_sp_unstable;
#[doc = "CORE_0_SP_MIN register accessor: an alias for `Reg<CORE_0_SP_MIN_SPEC>`"]
pub type CORE_0_SP_MIN = crate::Reg<core_0_sp_min::CORE_0_SP_MIN_SPEC>;
#[doc = "core0 sp region configuration regsiter"]
pub mod core_0_sp_min;
#[doc = "CORE_0_SP_MAX register accessor: an alias for `Reg<CORE_0_SP_MAX_SPEC>`"]
pub type CORE_0_SP_MAX = crate::Reg<core_0_sp_max::CORE_0_SP_MAX_SPEC>;
#[doc = "core0 sp region configuration regsiter"]
pub mod core_0_sp_max;
#[doc = "CORE_0_SP_PC register accessor: an alias for `Reg<CORE_0_SP_PC_SPEC>`"]
pub type CORE_0_SP_PC = crate::Reg<core_0_sp_pc::CORE_0_SP_PC_SPEC>;
#[doc = "core0 sp pc status register"]
pub mod core_0_sp_pc;
#[doc = "CORE_0_RCD_PDEBUGENABLE register accessor: an alias for `Reg<CORE_0_RCD_PDEBUGENABLE_SPEC>`"]
pub type CORE_0_RCD_PDEBUGENABLE =
    crate::Reg<core_0_rcd_pdebugenable::CORE_0_RCD_PDEBUGENABLE_SPEC>;
#[doc = "core0 pdebug configuration register"]
pub mod core_0_rcd_pdebugenable;
#[doc = "CORE_0_RCD_RECORDING register accessor: an alias for `Reg<CORE_0_RCD_RECORDING_SPEC>`"]
pub type CORE_0_RCD_RECORDING = crate::Reg<core_0_rcd_recording::CORE_0_RCD_RECORDING_SPEC>;
#[doc = "core0 pdebug status register"]
pub mod core_0_rcd_recording;
#[doc = "CORE_0_RCD_PDEBUGINST register accessor: an alias for `Reg<CORE_0_RCD_PDEBUGINST_SPEC>`"]
pub type CORE_0_RCD_PDEBUGINST = crate::Reg<core_0_rcd_pdebuginst::CORE_0_RCD_PDEBUGINST_SPEC>;
#[doc = "core0 pdebug status register"]
pub mod core_0_rcd_pdebuginst;
#[doc = "CORE_0_RCD_PDEBUGSTATUS register accessor: an alias for `Reg<CORE_0_RCD_PDEBUGSTATUS_SPEC>`"]
pub type CORE_0_RCD_PDEBUGSTATUS =
    crate::Reg<core_0_rcd_pdebugstatus::CORE_0_RCD_PDEBUGSTATUS_SPEC>;
#[doc = "core0 pdebug status register"]
pub mod core_0_rcd_pdebugstatus;
#[doc = "CORE_0_RCD_PDEBUGDATA register accessor: an alias for `Reg<CORE_0_RCD_PDEBUGDATA_SPEC>`"]
pub type CORE_0_RCD_PDEBUGDATA = crate::Reg<core_0_rcd_pdebugdata::CORE_0_RCD_PDEBUGDATA_SPEC>;
#[doc = "core0 pdebug status register"]
pub mod core_0_rcd_pdebugdata;
#[doc = "CORE_0_RCD_PDEBUGPC register accessor: an alias for `Reg<CORE_0_RCD_PDEBUGPC_SPEC>`"]
pub type CORE_0_RCD_PDEBUGPC = crate::Reg<core_0_rcd_pdebugpc::CORE_0_RCD_PDEBUGPC_SPEC>;
#[doc = "core0 pdebug status register"]
pub mod core_0_rcd_pdebugpc;
#[doc = "CORE_0_RCD_PDEBUGLS0STAT register accessor: an alias for `Reg<CORE_0_RCD_PDEBUGLS0STAT_SPEC>`"]
pub type CORE_0_RCD_PDEBUGLS0STAT =
    crate::Reg<core_0_rcd_pdebugls0stat::CORE_0_RCD_PDEBUGLS0STAT_SPEC>;
#[doc = "core0 pdebug status register"]
pub mod core_0_rcd_pdebugls0stat;
#[doc = "CORE_0_RCD_PDEBUGLS0ADDR register accessor: an alias for `Reg<CORE_0_RCD_PDEBUGLS0ADDR_SPEC>`"]
pub type CORE_0_RCD_PDEBUGLS0ADDR =
    crate::Reg<core_0_rcd_pdebugls0addr::CORE_0_RCD_PDEBUGLS0ADDR_SPEC>;
#[doc = "core0 pdebug status register"]
pub mod core_0_rcd_pdebugls0addr;
#[doc = "CORE_0_RCD_PDEBUGLS0DATA register accessor: an alias for `Reg<CORE_0_RCD_PDEBUGLS0DATA_SPEC>`"]
pub type CORE_0_RCD_PDEBUGLS0DATA =
    crate::Reg<core_0_rcd_pdebugls0data::CORE_0_RCD_PDEBUGLS0DATA_SPEC>;
#[doc = "core0 pdebug status register"]
pub mod core_0_rcd_pdebugls0data;
#[doc = "CORE_0_RCD_SP register accessor: an alias for `Reg<CORE_0_RCD_SP_SPEC>`"]
pub type CORE_0_RCD_SP = crate::Reg<core_0_rcd_sp::CORE_0_RCD_SP_SPEC>;
#[doc = "core0 pdebug status register"]
pub mod core_0_rcd_sp;
#[doc = "CORE_0_IRAM0_EXCEPTION_MONITOR_0 register accessor: an alias for `Reg<CORE_0_IRAM0_EXCEPTION_MONITOR_0_SPEC>`"]
pub type CORE_0_IRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<core_0_iram0_exception_monitor_0::CORE_0_IRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "core0 bus busy status regsiter"]
pub mod core_0_iram0_exception_monitor_0;
#[doc = "CORE_0_IRAM0_EXCEPTION_MONITOR_1 register accessor: an alias for `Reg<CORE_0_IRAM0_EXCEPTION_MONITOR_1_SPEC>`"]
pub type CORE_0_IRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<core_0_iram0_exception_monitor_1::CORE_0_IRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "core0 bus busy status regsiter"]
pub mod core_0_iram0_exception_monitor_1;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_0 register accessor: an alias for `Reg<CORE_0_DRAM0_EXCEPTION_MONITOR_0_SPEC>`"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<core_0_dram0_exception_monitor_0::CORE_0_DRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "core0 bus busy status regsiter"]
pub mod core_0_dram0_exception_monitor_0;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_1 register accessor: an alias for `Reg<CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC>`"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<core_0_dram0_exception_monitor_1::CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "core0 bus busy status regsiter"]
pub mod core_0_dram0_exception_monitor_1;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_2 register accessor: an alias for `Reg<CORE_0_DRAM0_EXCEPTION_MONITOR_2_SPEC>`"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_2 =
    crate::Reg<core_0_dram0_exception_monitor_2::CORE_0_DRAM0_EXCEPTION_MONITOR_2_SPEC>;
#[doc = "core0 bus busy status regsiter"]
pub mod core_0_dram0_exception_monitor_2;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_3 register accessor: an alias for `Reg<CORE_0_DRAM0_EXCEPTION_MONITOR_3_SPEC>`"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_3 =
    crate::Reg<core_0_dram0_exception_monitor_3::CORE_0_DRAM0_EXCEPTION_MONITOR_3_SPEC>;
#[doc = "core0 bus busy status regsiter"]
pub mod core_0_dram0_exception_monitor_3;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_4 register accessor: an alias for `Reg<CORE_0_DRAM0_EXCEPTION_MONITOR_4_SPEC>`"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_4 =
    crate::Reg<core_0_dram0_exception_monitor_4::CORE_0_DRAM0_EXCEPTION_MONITOR_4_SPEC>;
#[doc = "core0 bus busy configuration regsiter"]
pub mod core_0_dram0_exception_monitor_4;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_5 register accessor: an alias for `Reg<CORE_0_DRAM0_EXCEPTION_MONITOR_5_SPEC>`"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_5 =
    crate::Reg<core_0_dram0_exception_monitor_5::CORE_0_DRAM0_EXCEPTION_MONITOR_5_SPEC>;
#[doc = "core0 bus busy configuration regsiter"]
pub mod core_0_dram0_exception_monitor_5;
#[doc = "CORE_1_INTERRUPT_ENA register accessor: an alias for `Reg<CORE_1_INTERRUPT_ENA_SPEC>`"]
pub type CORE_1_INTERRUPT_ENA = crate::Reg<core_1_interrupt_ena::CORE_1_INTERRUPT_ENA_SPEC>;
#[doc = "Core1 monitor enable configuration register"]
pub mod core_1_interrupt_ena;
#[doc = "CORE_1_INTERRUPT_RAW register accessor: an alias for `Reg<CORE_1_INTERRUPT_RAW_SPEC>`"]
pub type CORE_1_INTERRUPT_RAW = crate::Reg<core_1_interrupt_raw::CORE_1_INTERRUPT_RAW_SPEC>;
#[doc = "Core1 monitor interrupt status register"]
pub mod core_1_interrupt_raw;
#[doc = "CORE_1_INTERRUPT_RLS register accessor: an alias for `Reg<CORE_1_INTERRUPT_RLS_SPEC>`"]
pub type CORE_1_INTERRUPT_RLS = crate::Reg<core_1_interrupt_rls::CORE_1_INTERRUPT_RLS_SPEC>;
#[doc = "Core1 monitor interrupt enable register"]
pub mod core_1_interrupt_rls;
#[doc = "CORE_1_INTERRUPT_CLR register accessor: an alias for `Reg<CORE_1_INTERRUPT_CLR_SPEC>`"]
pub type CORE_1_INTERRUPT_CLR = crate::Reg<core_1_interrupt_clr::CORE_1_INTERRUPT_CLR_SPEC>;
#[doc = "Core1 monitor interrupt clr register"]
pub mod core_1_interrupt_clr;
#[doc = "CORE_1_AREA_DRAM0_0_MIN register accessor: an alias for `Reg<CORE_1_AREA_DRAM0_0_MIN_SPEC>`"]
pub type CORE_1_AREA_DRAM0_0_MIN =
    crate::Reg<core_1_area_dram0_0_min::CORE_1_AREA_DRAM0_0_MIN_SPEC>;
#[doc = "Core1 dram0 region0 addr configuration register"]
pub mod core_1_area_dram0_0_min;
#[doc = "CORE_1_AREA_DRAM0_0_MAX register accessor: an alias for `Reg<CORE_1_AREA_DRAM0_0_MAX_SPEC>`"]
pub type CORE_1_AREA_DRAM0_0_MAX =
    crate::Reg<core_1_area_dram0_0_max::CORE_1_AREA_DRAM0_0_MAX_SPEC>;
#[doc = "Core1 dram0 region0 addr configuration register"]
pub mod core_1_area_dram0_0_max;
#[doc = "CORE_1_AREA_DRAM0_1_MIN register accessor: an alias for `Reg<CORE_1_AREA_DRAM0_1_MIN_SPEC>`"]
pub type CORE_1_AREA_DRAM0_1_MIN =
    crate::Reg<core_1_area_dram0_1_min::CORE_1_AREA_DRAM0_1_MIN_SPEC>;
#[doc = "Core1 dram0 region1 addr configuration register"]
pub mod core_1_area_dram0_1_min;
#[doc = "CORE_1_AREA_DRAM0_1_MAX register accessor: an alias for `Reg<CORE_1_AREA_DRAM0_1_MAX_SPEC>`"]
pub type CORE_1_AREA_DRAM0_1_MAX =
    crate::Reg<core_1_area_dram0_1_max::CORE_1_AREA_DRAM0_1_MAX_SPEC>;
#[doc = "Core1 dram0 region1 addr configuration register"]
pub mod core_1_area_dram0_1_max;
#[doc = "CORE_1_AREA_PIF_0_MIN register accessor: an alias for `Reg<CORE_1_AREA_PIF_0_MIN_SPEC>`"]
pub type CORE_1_AREA_PIF_0_MIN = crate::Reg<core_1_area_pif_0_min::CORE_1_AREA_PIF_0_MIN_SPEC>;
#[doc = "Core1 PIF region0 addr configuration register"]
pub mod core_1_area_pif_0_min;
#[doc = "CORE_1_AREA_PIF_0_MAX register accessor: an alias for `Reg<CORE_1_AREA_PIF_0_MAX_SPEC>`"]
pub type CORE_1_AREA_PIF_0_MAX = crate::Reg<core_1_area_pif_0_max::CORE_1_AREA_PIF_0_MAX_SPEC>;
#[doc = "Core1 PIF region0 addr configuration register"]
pub mod core_1_area_pif_0_max;
#[doc = "CORE_1_AREA_PIF_1_MIN register accessor: an alias for `Reg<CORE_1_AREA_PIF_1_MIN_SPEC>`"]
pub type CORE_1_AREA_PIF_1_MIN = crate::Reg<core_1_area_pif_1_min::CORE_1_AREA_PIF_1_MIN_SPEC>;
#[doc = "Core1 PIF region1 addr configuration register"]
pub mod core_1_area_pif_1_min;
#[doc = "CORE_1_AREA_PIF_1_MAX register accessor: an alias for `Reg<CORE_1_AREA_PIF_1_MAX_SPEC>`"]
pub type CORE_1_AREA_PIF_1_MAX = crate::Reg<core_1_area_pif_1_max::CORE_1_AREA_PIF_1_MAX_SPEC>;
#[doc = "Core1 PIF region1 addr configuration register"]
pub mod core_1_area_pif_1_max;
#[doc = "CORE_1_AREA_PC register accessor: an alias for `Reg<CORE_1_AREA_PC_SPEC>`"]
pub type CORE_1_AREA_PC = crate::Reg<core_1_area_pc::CORE_1_AREA_PC_SPEC>;
#[doc = "Core1 area sp status register"]
pub mod core_1_area_pc;
#[doc = "CORE_1_AREA_SP register accessor: an alias for `Reg<CORE_1_AREA_SP_SPEC>`"]
pub type CORE_1_AREA_SP = crate::Reg<core_1_area_sp::CORE_1_AREA_SP_SPEC>;
#[doc = "Core1 area pc status register"]
pub mod core_1_area_sp;
#[doc = "CORE_1_SP_UNSTABLE register accessor: an alias for `Reg<CORE_1_SP_UNSTABLE_SPEC>`"]
pub type CORE_1_SP_UNSTABLE = crate::Reg<core_1_sp_unstable::CORE_1_SP_UNSTABLE_SPEC>;
#[doc = "Core1 sp unstable configuration register"]
pub mod core_1_sp_unstable;
#[doc = "CORE_1_SP_MIN register accessor: an alias for `Reg<CORE_1_SP_MIN_SPEC>`"]
pub type CORE_1_SP_MIN = crate::Reg<core_1_sp_min::CORE_1_SP_MIN_SPEC>;
#[doc = "Core1 sp region configuration regsiter"]
pub mod core_1_sp_min;
#[doc = "CORE_1_SP_MAX register accessor: an alias for `Reg<CORE_1_SP_MAX_SPEC>`"]
pub type CORE_1_SP_MAX = crate::Reg<core_1_sp_max::CORE_1_SP_MAX_SPEC>;
#[doc = "Core1 sp region configuration regsiter"]
pub mod core_1_sp_max;
#[doc = "CORE_1_SP_PC register accessor: an alias for `Reg<CORE_1_SP_PC_SPEC>`"]
pub type CORE_1_SP_PC = crate::Reg<core_1_sp_pc::CORE_1_SP_PC_SPEC>;
#[doc = "Core1 sp pc status register"]
pub mod core_1_sp_pc;
#[doc = "CORE_1_RCD_PDEBUGENABLE register accessor: an alias for `Reg<CORE_1_RCD_PDEBUGENABLE_SPEC>`"]
pub type CORE_1_RCD_PDEBUGENABLE =
    crate::Reg<core_1_rcd_pdebugenable::CORE_1_RCD_PDEBUGENABLE_SPEC>;
#[doc = "Core1 pdebug configuration register"]
pub mod core_1_rcd_pdebugenable;
#[doc = "CORE_1_RCD_RECORDING register accessor: an alias for `Reg<CORE_1_RCD_RECORDING_SPEC>`"]
pub type CORE_1_RCD_RECORDING = crate::Reg<core_1_rcd_recording::CORE_1_RCD_RECORDING_SPEC>;
#[doc = "Core1 pdebug status register"]
pub mod core_1_rcd_recording;
#[doc = "CORE_1_RCD_PDEBUGINST register accessor: an alias for `Reg<CORE_1_RCD_PDEBUGINST_SPEC>`"]
pub type CORE_1_RCD_PDEBUGINST = crate::Reg<core_1_rcd_pdebuginst::CORE_1_RCD_PDEBUGINST_SPEC>;
#[doc = "Core1 pdebug status register"]
pub mod core_1_rcd_pdebuginst;
#[doc = "CORE_1_RCD_PDEBUGSTATUS register accessor: an alias for `Reg<CORE_1_RCD_PDEBUGSTATUS_SPEC>`"]
pub type CORE_1_RCD_PDEBUGSTATUS =
    crate::Reg<core_1_rcd_pdebugstatus::CORE_1_RCD_PDEBUGSTATUS_SPEC>;
#[doc = "Core1 pdebug status register"]
pub mod core_1_rcd_pdebugstatus;
#[doc = "CORE_1_RCD_PDEBUGDATA register accessor: an alias for `Reg<CORE_1_RCD_PDEBUGDATA_SPEC>`"]
pub type CORE_1_RCD_PDEBUGDATA = crate::Reg<core_1_rcd_pdebugdata::CORE_1_RCD_PDEBUGDATA_SPEC>;
#[doc = "Core1 pdebug status register"]
pub mod core_1_rcd_pdebugdata;
#[doc = "CORE_1_RCD_PDEBUGPC register accessor: an alias for `Reg<CORE_1_RCD_PDEBUGPC_SPEC>`"]
pub type CORE_1_RCD_PDEBUGPC = crate::Reg<core_1_rcd_pdebugpc::CORE_1_RCD_PDEBUGPC_SPEC>;
#[doc = "Core1 pdebug status register"]
pub mod core_1_rcd_pdebugpc;
#[doc = "CORE_1_RCD_PDEBUGLS0STAT register accessor: an alias for `Reg<CORE_1_RCD_PDEBUGLS0STAT_SPEC>`"]
pub type CORE_1_RCD_PDEBUGLS0STAT =
    crate::Reg<core_1_rcd_pdebugls0stat::CORE_1_RCD_PDEBUGLS0STAT_SPEC>;
#[doc = "Core1 pdebug status register"]
pub mod core_1_rcd_pdebugls0stat;
#[doc = "CORE_1_RCD_PDEBUGLS0ADDR register accessor: an alias for `Reg<CORE_1_RCD_PDEBUGLS0ADDR_SPEC>`"]
pub type CORE_1_RCD_PDEBUGLS0ADDR =
    crate::Reg<core_1_rcd_pdebugls0addr::CORE_1_RCD_PDEBUGLS0ADDR_SPEC>;
#[doc = "Core1 pdebug status register"]
pub mod core_1_rcd_pdebugls0addr;
#[doc = "CORE_1_RCD_PDEBUGLS0DATA register accessor: an alias for `Reg<CORE_1_RCD_PDEBUGLS0DATA_SPEC>`"]
pub type CORE_1_RCD_PDEBUGLS0DATA =
    crate::Reg<core_1_rcd_pdebugls0data::CORE_1_RCD_PDEBUGLS0DATA_SPEC>;
#[doc = "Core1 pdebug status register"]
pub mod core_1_rcd_pdebugls0data;
#[doc = "CORE_1_RCD_SP register accessor: an alias for `Reg<CORE_1_RCD_SP_SPEC>`"]
pub type CORE_1_RCD_SP = crate::Reg<core_1_rcd_sp::CORE_1_RCD_SP_SPEC>;
#[doc = "Core1 pdebug status register"]
pub mod core_1_rcd_sp;
#[doc = "CORE_1_IRAM0_EXCEPTION_MONITOR_0 register accessor: an alias for `Reg<CORE_1_IRAM0_EXCEPTION_MONITOR_0_SPEC>`"]
pub type CORE_1_IRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<core_1_iram0_exception_monitor_0::CORE_1_IRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "Core1 bus busy status regsiter"]
pub mod core_1_iram0_exception_monitor_0;
#[doc = "CORE_1_IRAM0_EXCEPTION_MONITOR_1 register accessor: an alias for `Reg<CORE_1_IRAM0_EXCEPTION_MONITOR_1_SPEC>`"]
pub type CORE_1_IRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<core_1_iram0_exception_monitor_1::CORE_1_IRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "Core1 bus busy status regsiter"]
pub mod core_1_iram0_exception_monitor_1;
#[doc = "CORE_1_DRAM0_EXCEPTION_MONITOR_0 register accessor: an alias for `Reg<CORE_1_DRAM0_EXCEPTION_MONITOR_0_SPEC>`"]
pub type CORE_1_DRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<core_1_dram0_exception_monitor_0::CORE_1_DRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "Core1 bus busy status regsiter"]
pub mod core_1_dram0_exception_monitor_0;
#[doc = "CORE_1_DRAM0_EXCEPTION_MONITOR_1 register accessor: an alias for `Reg<CORE_1_DRAM0_EXCEPTION_MONITOR_1_SPEC>`"]
pub type CORE_1_DRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<core_1_dram0_exception_monitor_1::CORE_1_DRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "Core1 bus busy status regsiter"]
pub mod core_1_dram0_exception_monitor_1;
#[doc = "CORE_1_DRAM0_EXCEPTION_MONITOR_2 register accessor: an alias for `Reg<CORE_1_DRAM0_EXCEPTION_MONITOR_2_SPEC>`"]
pub type CORE_1_DRAM0_EXCEPTION_MONITOR_2 =
    crate::Reg<core_1_dram0_exception_monitor_2::CORE_1_DRAM0_EXCEPTION_MONITOR_2_SPEC>;
#[doc = "Core1 bus busy status regsiter"]
pub mod core_1_dram0_exception_monitor_2;
#[doc = "CORE_1_DRAM0_EXCEPTION_MONITOR_3 register accessor: an alias for `Reg<CORE_1_DRAM0_EXCEPTION_MONITOR_3_SPEC>`"]
pub type CORE_1_DRAM0_EXCEPTION_MONITOR_3 =
    crate::Reg<core_1_dram0_exception_monitor_3::CORE_1_DRAM0_EXCEPTION_MONITOR_3_SPEC>;
#[doc = "Core1 bus busy status regsiter"]
pub mod core_1_dram0_exception_monitor_3;
#[doc = "CORE_1_DRAM0_EXCEPTION_MONITOR_4 register accessor: an alias for `Reg<CORE_1_DRAM0_EXCEPTION_MONITOR_4_SPEC>`"]
pub type CORE_1_DRAM0_EXCEPTION_MONITOR_4 =
    crate::Reg<core_1_dram0_exception_monitor_4::CORE_1_DRAM0_EXCEPTION_MONITOR_4_SPEC>;
#[doc = "Core1 bus busy status regsiter"]
pub mod core_1_dram0_exception_monitor_4;
#[doc = "CORE_1_DRAM0_EXCEPTION_MONITOR_5 register accessor: an alias for `Reg<CORE_1_DRAM0_EXCEPTION_MONITOR_5_SPEC>`"]
pub type CORE_1_DRAM0_EXCEPTION_MONITOR_5 =
    crate::Reg<core_1_dram0_exception_monitor_5::CORE_1_DRAM0_EXCEPTION_MONITOR_5_SPEC>;
#[doc = "Core1 bus busy status regsiter"]
pub mod core_1_dram0_exception_monitor_5;
#[doc = "CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 register accessor: an alias for `Reg<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC>`"]
pub type CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<core_x_iram0_dram0_exception_monitor_0::CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "bus busy configuration register"]
pub mod core_x_iram0_dram0_exception_monitor_0;
#[doc = "CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1 register accessor: an alias for `Reg<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC>`"]
pub type CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<core_x_iram0_dram0_exception_monitor_1::CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "bus busy configuration register"]
pub mod core_x_iram0_dram0_exception_monitor_1;
#[doc = "LOG_SETTING register accessor: an alias for `Reg<LOG_SETTING_SPEC>`"]
pub type LOG_SETTING = crate::Reg<log_setting::LOG_SETTING_SPEC>;
#[doc = "log set register"]
pub mod log_setting;
#[doc = "LOG_DATA_0 register accessor: an alias for `Reg<LOG_DATA_0_SPEC>`"]
pub type LOG_DATA_0 = crate::Reg<log_data_0::LOG_DATA_0_SPEC>;
#[doc = "log check data register"]
pub mod log_data_0;
#[doc = "LOG_DATA_1 register accessor: an alias for `Reg<LOG_DATA_1_SPEC>`"]
pub type LOG_DATA_1 = crate::Reg<log_data_1::LOG_DATA_1_SPEC>;
#[doc = "log check data register"]
pub mod log_data_1;
#[doc = "LOG_DATA_2 register accessor: an alias for `Reg<LOG_DATA_2_SPEC>`"]
pub type LOG_DATA_2 = crate::Reg<log_data_2::LOG_DATA_2_SPEC>;
#[doc = "log check data register"]
pub mod log_data_2;
#[doc = "LOG_DATA_3 register accessor: an alias for `Reg<LOG_DATA_3_SPEC>`"]
pub type LOG_DATA_3 = crate::Reg<log_data_3::LOG_DATA_3_SPEC>;
#[doc = "log check data register"]
pub mod log_data_3;
#[doc = "LOG_DATA_MASK register accessor: an alias for `Reg<LOG_DATA_MASK_SPEC>`"]
pub type LOG_DATA_MASK = crate::Reg<log_data_mask::LOG_DATA_MASK_SPEC>;
#[doc = "log check data mask register"]
pub mod log_data_mask;
#[doc = "LOG_MIN register accessor: an alias for `Reg<LOG_MIN_SPEC>`"]
pub type LOG_MIN = crate::Reg<log_min::LOG_MIN_SPEC>;
#[doc = "log check region configuration register"]
pub mod log_min;
#[doc = "LOG_MAX register accessor: an alias for `Reg<LOG_MAX_SPEC>`"]
pub type LOG_MAX = crate::Reg<log_max::LOG_MAX_SPEC>;
#[doc = "log check region configuration register"]
pub mod log_max;
#[doc = "LOG_MEM_START register accessor: an alias for `Reg<LOG_MEM_START_SPEC>`"]
pub type LOG_MEM_START = crate::Reg<log_mem_start::LOG_MEM_START_SPEC>;
#[doc = "log mem region configuration register"]
pub mod log_mem_start;
#[doc = "LOG_MEM_END register accessor: an alias for `Reg<LOG_MEM_END_SPEC>`"]
pub type LOG_MEM_END = crate::Reg<log_mem_end::LOG_MEM_END_SPEC>;
#[doc = "log mem region configuration register"]
pub mod log_mem_end;
#[doc = "LOG_MEM_WRITING_ADDR register accessor: an alias for `Reg<LOG_MEM_WRITING_ADDR_SPEC>`"]
pub type LOG_MEM_WRITING_ADDR = crate::Reg<log_mem_writing_addr::LOG_MEM_WRITING_ADDR_SPEC>;
#[doc = "log mem addr status register"]
pub mod log_mem_writing_addr;
#[doc = "LOG_MEM_FULL_FLAG register accessor: an alias for `Reg<LOG_MEM_FULL_FLAG_SPEC>`"]
pub type LOG_MEM_FULL_FLAG = crate::Reg<log_mem_full_flag::LOG_MEM_FULL_FLAG_SPEC>;
#[doc = "log mem status register"]
pub mod log_mem_full_flag;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "version register"]
pub mod date;
