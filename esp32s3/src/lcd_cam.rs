#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LCD clock register"]
    pub lcd_clock: crate::Reg<lcd_clock::LCD_CLOCK_SPEC>,
    #[doc = "0x04 - Camera configuration register"]
    pub cam_ctrl: crate::Reg<cam_ctrl::CAM_CTRL_SPEC>,
    #[doc = "0x08 - Camera configuration register"]
    pub cam_ctrl1: crate::Reg<cam_ctrl1::CAM_CTRL1_SPEC>,
    #[doc = "0x0c - Camera configuration register"]
    pub cam_rgb_yuv: crate::Reg<cam_rgb_yuv::CAM_RGB_YUV_SPEC>,
    #[doc = "0x10 - LCD configuration register"]
    pub lcd_rgb_yuv: crate::Reg<lcd_rgb_yuv::LCD_RGB_YUV_SPEC>,
    #[doc = "0x14 - LCD configuration register"]
    pub lcd_user: crate::Reg<lcd_user::LCD_USER_SPEC>,
    #[doc = "0x18 - LCD configuration register"]
    pub lcd_misc: crate::Reg<lcd_misc::LCD_MISC_SPEC>,
    #[doc = "0x1c - LCD configuration register"]
    pub lcd_ctrl: crate::Reg<lcd_ctrl::LCD_CTRL_SPEC>,
    #[doc = "0x20 - LCD configuration register"]
    pub lcd_ctrl1: crate::Reg<lcd_ctrl1::LCD_CTRL1_SPEC>,
    #[doc = "0x24 - LCD configuration register"]
    pub lcd_ctrl2: crate::Reg<lcd_ctrl2::LCD_CTRL2_SPEC>,
    #[doc = "0x28 - LCD configuration register"]
    pub lcd_cmd_val: crate::Reg<lcd_cmd_val::LCD_CMD_VAL_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x30 - LCD configuration register"]
    pub lcd_dly_mode: crate::Reg<lcd_dly_mode::LCD_DLY_MODE_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x38 - LCD configuration register"]
    pub lcd_data_dout_mode: crate::Reg<lcd_data_dout_mode::LCD_DATA_DOUT_MODE_SPEC>,
    _reserved13: [u8; 0x28],
    #[doc = "0x64 - LCD_camera DMA inturrupt enable register"]
    pub lc_dma_int_ena: crate::Reg<lc_dma_int_ena::LC_DMA_INT_ENA_SPEC>,
    #[doc = "0x68 - LCD_camera DMA raw inturrupt status register"]
    pub lc_dma_int_raw: crate::Reg<lc_dma_int_raw::LC_DMA_INT_RAW_SPEC>,
    #[doc = "0x6c - LCD_camera DMA masked inturrupt status register"]
    pub lc_dma_int_st: crate::Reg<lc_dma_int_st::LC_DMA_INT_ST_SPEC>,
    #[doc = "0x70 - LCD_camera DMA inturrupt clear register"]
    pub lc_dma_int_clr: crate::Reg<lc_dma_int_clr::LC_DMA_INT_CLR_SPEC>,
    _reserved17: [u8; 0x88],
    #[doc = "0xfc - Version register"]
    pub lc_reg_date: crate::Reg<lc_reg_date::LC_REG_DATE_SPEC>,
}
#[doc = "LCD_CLOCK register accessor: an alias for `Reg<LCD_CLOCK_SPEC>`"]
pub type LCD_CLOCK = crate::Reg<lcd_clock::LCD_CLOCK_SPEC>;
#[doc = "LCD clock register"]
pub mod lcd_clock;
#[doc = "CAM_CTRL register accessor: an alias for `Reg<CAM_CTRL_SPEC>`"]
pub type CAM_CTRL = crate::Reg<cam_ctrl::CAM_CTRL_SPEC>;
#[doc = "Camera configuration register"]
pub mod cam_ctrl;
#[doc = "CAM_CTRL1 register accessor: an alias for `Reg<CAM_CTRL1_SPEC>`"]
pub type CAM_CTRL1 = crate::Reg<cam_ctrl1::CAM_CTRL1_SPEC>;
#[doc = "Camera configuration register"]
pub mod cam_ctrl1;
#[doc = "CAM_RGB_YUV register accessor: an alias for `Reg<CAM_RGB_YUV_SPEC>`"]
pub type CAM_RGB_YUV = crate::Reg<cam_rgb_yuv::CAM_RGB_YUV_SPEC>;
#[doc = "Camera configuration register"]
pub mod cam_rgb_yuv;
#[doc = "LCD_RGB_YUV register accessor: an alias for `Reg<LCD_RGB_YUV_SPEC>`"]
pub type LCD_RGB_YUV = crate::Reg<lcd_rgb_yuv::LCD_RGB_YUV_SPEC>;
#[doc = "LCD configuration register"]
pub mod lcd_rgb_yuv;
#[doc = "LCD_USER register accessor: an alias for `Reg<LCD_USER_SPEC>`"]
pub type LCD_USER = crate::Reg<lcd_user::LCD_USER_SPEC>;
#[doc = "LCD configuration register"]
pub mod lcd_user;
#[doc = "LCD_MISC register accessor: an alias for `Reg<LCD_MISC_SPEC>`"]
pub type LCD_MISC = crate::Reg<lcd_misc::LCD_MISC_SPEC>;
#[doc = "LCD configuration register"]
pub mod lcd_misc;
#[doc = "LCD_CTRL register accessor: an alias for `Reg<LCD_CTRL_SPEC>`"]
pub type LCD_CTRL = crate::Reg<lcd_ctrl::LCD_CTRL_SPEC>;
#[doc = "LCD configuration register"]
pub mod lcd_ctrl;
#[doc = "LCD_CTRL1 register accessor: an alias for `Reg<LCD_CTRL1_SPEC>`"]
pub type LCD_CTRL1 = crate::Reg<lcd_ctrl1::LCD_CTRL1_SPEC>;
#[doc = "LCD configuration register"]
pub mod lcd_ctrl1;
#[doc = "LCD_CTRL2 register accessor: an alias for `Reg<LCD_CTRL2_SPEC>`"]
pub type LCD_CTRL2 = crate::Reg<lcd_ctrl2::LCD_CTRL2_SPEC>;
#[doc = "LCD configuration register"]
pub mod lcd_ctrl2;
#[doc = "LCD_CMD_VAL register accessor: an alias for `Reg<LCD_CMD_VAL_SPEC>`"]
pub type LCD_CMD_VAL = crate::Reg<lcd_cmd_val::LCD_CMD_VAL_SPEC>;
#[doc = "LCD configuration register"]
pub mod lcd_cmd_val;
#[doc = "LCD_DLY_MODE register accessor: an alias for `Reg<LCD_DLY_MODE_SPEC>`"]
pub type LCD_DLY_MODE = crate::Reg<lcd_dly_mode::LCD_DLY_MODE_SPEC>;
#[doc = "LCD configuration register"]
pub mod lcd_dly_mode;
#[doc = "LCD_DATA_DOUT_MODE register accessor: an alias for `Reg<LCD_DATA_DOUT_MODE_SPEC>`"]
pub type LCD_DATA_DOUT_MODE = crate::Reg<lcd_data_dout_mode::LCD_DATA_DOUT_MODE_SPEC>;
#[doc = "LCD configuration register"]
pub mod lcd_data_dout_mode;
#[doc = "LC_DMA_INT_ENA register accessor: an alias for `Reg<LC_DMA_INT_ENA_SPEC>`"]
pub type LC_DMA_INT_ENA = crate::Reg<lc_dma_int_ena::LC_DMA_INT_ENA_SPEC>;
#[doc = "LCD_camera DMA inturrupt enable register"]
pub mod lc_dma_int_ena;
#[doc = "LC_DMA_INT_RAW register accessor: an alias for `Reg<LC_DMA_INT_RAW_SPEC>`"]
pub type LC_DMA_INT_RAW = crate::Reg<lc_dma_int_raw::LC_DMA_INT_RAW_SPEC>;
#[doc = "LCD_camera DMA raw inturrupt status register"]
pub mod lc_dma_int_raw;
#[doc = "LC_DMA_INT_ST register accessor: an alias for `Reg<LC_DMA_INT_ST_SPEC>`"]
pub type LC_DMA_INT_ST = crate::Reg<lc_dma_int_st::LC_DMA_INT_ST_SPEC>;
#[doc = "LCD_camera DMA masked inturrupt status register"]
pub mod lc_dma_int_st;
#[doc = "LC_DMA_INT_CLR register accessor: an alias for `Reg<LC_DMA_INT_CLR_SPEC>`"]
pub type LC_DMA_INT_CLR = crate::Reg<lc_dma_int_clr::LC_DMA_INT_CLR_SPEC>;
#[doc = "LCD_camera DMA inturrupt clear register"]
pub mod lc_dma_int_clr;
#[doc = "LC_REG_DATE register accessor: an alias for `Reg<LC_REG_DATE_SPEC>`"]
pub type LC_REG_DATE = crate::Reg<lc_reg_date::LC_REG_DATE_SPEC>;
#[doc = "Version register"]
pub mod lc_reg_date;
