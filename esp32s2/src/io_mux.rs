#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock output configuration register"]
    pub pin_ctrl: crate::Reg<pin_ctrl::PIN_CTRL_SPEC>,
    #[doc = "0x04 - Configuration register for pin GPIO0"]
    pub gpio0: crate::Reg<gpio0::GPIO0_SPEC>,
    #[doc = "0x08 - Configuration register for pin GPIO1"]
    pub gpio1: crate::Reg<gpio1::GPIO1_SPEC>,
    #[doc = "0x0c - Configuration register for pin GPIO2"]
    pub gpio2: crate::Reg<gpio2::GPIO2_SPEC>,
    #[doc = "0x10 - Configuration register for pin GPIO3"]
    pub gpio3: crate::Reg<gpio3::GPIO3_SPEC>,
    #[doc = "0x14 - Configuration register for pin GPIO4"]
    pub gpio4: crate::Reg<gpio4::GPIO4_SPEC>,
    #[doc = "0x18 - Configuration register for pin GPIO5"]
    pub gpio5: crate::Reg<gpio5::GPIO5_SPEC>,
    #[doc = "0x1c - Configuration register for pin GPIO6"]
    pub gpio6: crate::Reg<gpio6::GPIO6_SPEC>,
    #[doc = "0x20 - Configuration register for pin GPIO7"]
    pub gpio7: crate::Reg<gpio7::GPIO7_SPEC>,
    #[doc = "0x24 - Configuration register for pin GPIO8"]
    pub gpio8: crate::Reg<gpio8::GPIO8_SPEC>,
    #[doc = "0x28 - Configuration register for pin GPIO9"]
    pub gpio9: crate::Reg<gpio9::GPIO9_SPEC>,
    #[doc = "0x2c - Configuration register for pin GPIO10"]
    pub gpio10: crate::Reg<gpio10::GPIO10_SPEC>,
    #[doc = "0x30 - Configuration register for pin GPIO11"]
    pub gpio11: crate::Reg<gpio11::GPIO11_SPEC>,
    #[doc = "0x34 - Configuration register for pin GPIO12"]
    pub gpio12: crate::Reg<gpio12::GPIO12_SPEC>,
    #[doc = "0x38 - Configuration register for pin GPIO13"]
    pub gpio13: crate::Reg<gpio13::GPIO13_SPEC>,
    #[doc = "0x3c - Configuration register for pin GPIO14"]
    pub gpio14: crate::Reg<gpio14::GPIO14_SPEC>,
    #[doc = "0x40 - Configuration register for pad GPIO15"]
    pub gpio15: crate::Reg<gpio15::GPIO15_SPEC>,
    #[doc = "0x44 - Configuration register for pad GPIO16"]
    pub gpio16: crate::Reg<gpio16::GPIO16_SPEC>,
    #[doc = "0x48 - Configuration register for pad GPIO17"]
    pub gpio17: crate::Reg<gpio17::GPIO17_SPEC>,
    #[doc = "0x4c - Configuration register for pad GPIO18"]
    pub gpio18: crate::Reg<gpio18::GPIO18_SPEC>,
    #[doc = "0x50 - Configuration register for pin GPIO19"]
    pub gpio19: crate::Reg<gpio19::GPIO19_SPEC>,
    #[doc = "0x54 - Configuration register for pin GPIO20"]
    pub gpio20: crate::Reg<gpio20::GPIO20_SPEC>,
    #[doc = "0x58 - Configuration register for pin GPIO21"]
    pub gpio21: crate::Reg<gpio21::GPIO21_SPEC>,
    _reserved23: [u8; 0x10],
    #[doc = "0x6c - Configuration register for pad GPIO26"]
    pub gpio26: crate::Reg<gpio26::GPIO26_SPEC>,
    #[doc = "0x70 - Configuration register for pad GPIO27"]
    pub gpio27: crate::Reg<gpio27::GPIO27_SPEC>,
    #[doc = "0x74 - Configuration register for pad GPIO28"]
    pub gpio28: crate::Reg<gpio28::GPIO28_SPEC>,
    #[doc = "0x78 - Configuration register for pad GPIO29"]
    pub gpio29: crate::Reg<gpio29::GPIO29_SPEC>,
    #[doc = "0x7c - Configuration register for pad GPIO30"]
    pub gpio30: crate::Reg<gpio30::GPIO30_SPEC>,
    #[doc = "0x80 - Configuration register for pad GPIO31"]
    pub gpio31: crate::Reg<gpio31::GPIO31_SPEC>,
    #[doc = "0x84 - Configuration register for pad GPIO32"]
    pub gpio32: crate::Reg<gpio32::GPIO32_SPEC>,
    #[doc = "0x88 - Configuration register for pin GPIO33"]
    pub gpio33: crate::Reg<gpio33::GPIO33_SPEC>,
    #[doc = "0x8c - Configuration register for pin GPIO34"]
    pub gpio34: crate::Reg<gpio34::GPIO34_SPEC>,
    #[doc = "0x90 - Configuration register for pin GPIO35"]
    pub gpio35: crate::Reg<gpio35::GPIO35_SPEC>,
    #[doc = "0x94 - Configuration register for pin GPIO36"]
    pub gpio36: crate::Reg<gpio36::GPIO36_SPEC>,
    #[doc = "0x98 - Configuration register for pin GPIO37"]
    pub gpio37: crate::Reg<gpio37::GPIO37_SPEC>,
    #[doc = "0x9c - Configuration register for pin GPIO38"]
    pub gpio38: crate::Reg<gpio38::GPIO38_SPEC>,
    #[doc = "0xa0 - Configuration register for pad GPIO39"]
    pub gpio39: crate::Reg<gpio39::GPIO39_SPEC>,
    #[doc = "0xa4 - Configuration register for pad GPIO40"]
    pub gpio40: crate::Reg<gpio40::GPIO40_SPEC>,
    #[doc = "0xa8 - Configuration register for pad GPIO41"]
    pub gpio41: crate::Reg<gpio41::GPIO41_SPEC>,
    #[doc = "0xac - Configuration register for pad GPIO42"]
    pub gpio42: crate::Reg<gpio42::GPIO42_SPEC>,
    #[doc = "0xb0 - Configuration register for pad GPIO43"]
    pub gpio43: crate::Reg<gpio43::GPIO43_SPEC>,
    #[doc = "0xb4 - Configuration register for pad GPIO44"]
    pub gpio44: crate::Reg<gpio44::GPIO44_SPEC>,
    #[doc = "0xb8 - Configuration register for pin GPIO45"]
    pub gpio45: crate::Reg<gpio45::GPIO45_SPEC>,
    #[doc = "0xbc - Configuration register for pin GPIO46"]
    pub gpio46: crate::Reg<gpio46::GPIO46_SPEC>,
    _reserved44: [u8; 0x3c],
    #[doc = "0xfc - Version control register"]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "PIN_CTRL register accessor: an alias for `Reg<PIN_CTRL_SPEC>`"]
pub type PIN_CTRL = crate::Reg<pin_ctrl::PIN_CTRL_SPEC>;
#[doc = "Clock output configuration register"]
pub mod pin_ctrl;
#[doc = "GPIO0 register accessor: an alias for `Reg<GPIO0_SPEC>`"]
pub type GPIO0 = crate::Reg<gpio0::GPIO0_SPEC>;
#[doc = "Configuration register for pin GPIO0"]
pub mod gpio0;
#[doc = "GPIO1 register accessor: an alias for `Reg<GPIO1_SPEC>`"]
pub type GPIO1 = crate::Reg<gpio1::GPIO1_SPEC>;
#[doc = "Configuration register for pin GPIO1"]
pub mod gpio1;
#[doc = "GPIO2 register accessor: an alias for `Reg<GPIO2_SPEC>`"]
pub type GPIO2 = crate::Reg<gpio2::GPIO2_SPEC>;
#[doc = "Configuration register for pin GPIO2"]
pub mod gpio2;
#[doc = "GPIO3 register accessor: an alias for `Reg<GPIO3_SPEC>`"]
pub type GPIO3 = crate::Reg<gpio3::GPIO3_SPEC>;
#[doc = "Configuration register for pin GPIO3"]
pub mod gpio3;
#[doc = "GPIO4 register accessor: an alias for `Reg<GPIO4_SPEC>`"]
pub type GPIO4 = crate::Reg<gpio4::GPIO4_SPEC>;
#[doc = "Configuration register for pin GPIO4"]
pub mod gpio4;
#[doc = "GPIO5 register accessor: an alias for `Reg<GPIO5_SPEC>`"]
pub type GPIO5 = crate::Reg<gpio5::GPIO5_SPEC>;
#[doc = "Configuration register for pin GPIO5"]
pub mod gpio5;
#[doc = "GPIO6 register accessor: an alias for `Reg<GPIO6_SPEC>`"]
pub type GPIO6 = crate::Reg<gpio6::GPIO6_SPEC>;
#[doc = "Configuration register for pin GPIO6"]
pub mod gpio6;
#[doc = "GPIO7 register accessor: an alias for `Reg<GPIO7_SPEC>`"]
pub type GPIO7 = crate::Reg<gpio7::GPIO7_SPEC>;
#[doc = "Configuration register for pin GPIO7"]
pub mod gpio7;
#[doc = "GPIO8 register accessor: an alias for `Reg<GPIO8_SPEC>`"]
pub type GPIO8 = crate::Reg<gpio8::GPIO8_SPEC>;
#[doc = "Configuration register for pin GPIO8"]
pub mod gpio8;
#[doc = "GPIO9 register accessor: an alias for `Reg<GPIO9_SPEC>`"]
pub type GPIO9 = crate::Reg<gpio9::GPIO9_SPEC>;
#[doc = "Configuration register for pin GPIO9"]
pub mod gpio9;
#[doc = "GPIO10 register accessor: an alias for `Reg<GPIO10_SPEC>`"]
pub type GPIO10 = crate::Reg<gpio10::GPIO10_SPEC>;
#[doc = "Configuration register for pin GPIO10"]
pub mod gpio10;
#[doc = "GPIO11 register accessor: an alias for `Reg<GPIO11_SPEC>`"]
pub type GPIO11 = crate::Reg<gpio11::GPIO11_SPEC>;
#[doc = "Configuration register for pin GPIO11"]
pub mod gpio11;
#[doc = "GPIO12 register accessor: an alias for `Reg<GPIO12_SPEC>`"]
pub type GPIO12 = crate::Reg<gpio12::GPIO12_SPEC>;
#[doc = "Configuration register for pin GPIO12"]
pub mod gpio12;
#[doc = "GPIO13 register accessor: an alias for `Reg<GPIO13_SPEC>`"]
pub type GPIO13 = crate::Reg<gpio13::GPIO13_SPEC>;
#[doc = "Configuration register for pin GPIO13"]
pub mod gpio13;
#[doc = "GPIO14 register accessor: an alias for `Reg<GPIO14_SPEC>`"]
pub type GPIO14 = crate::Reg<gpio14::GPIO14_SPEC>;
#[doc = "Configuration register for pin GPIO14"]
pub mod gpio14;
#[doc = "GPIO15 register accessor: an alias for `Reg<GPIO15_SPEC>`"]
pub type GPIO15 = crate::Reg<gpio15::GPIO15_SPEC>;
#[doc = "Configuration register for pad GPIO15"]
pub mod gpio15;
#[doc = "GPIO16 register accessor: an alias for `Reg<GPIO16_SPEC>`"]
pub type GPIO16 = crate::Reg<gpio16::GPIO16_SPEC>;
#[doc = "Configuration register for pad GPIO16"]
pub mod gpio16;
#[doc = "GPIO17 register accessor: an alias for `Reg<GPIO17_SPEC>`"]
pub type GPIO17 = crate::Reg<gpio17::GPIO17_SPEC>;
#[doc = "Configuration register for pad GPIO17"]
pub mod gpio17;
#[doc = "GPIO18 register accessor: an alias for `Reg<GPIO18_SPEC>`"]
pub type GPIO18 = crate::Reg<gpio18::GPIO18_SPEC>;
#[doc = "Configuration register for pad GPIO18"]
pub mod gpio18;
#[doc = "GPIO19 register accessor: an alias for `Reg<GPIO19_SPEC>`"]
pub type GPIO19 = crate::Reg<gpio19::GPIO19_SPEC>;
#[doc = "Configuration register for pin GPIO19"]
pub mod gpio19;
#[doc = "GPIO20 register accessor: an alias for `Reg<GPIO20_SPEC>`"]
pub type GPIO20 = crate::Reg<gpio20::GPIO20_SPEC>;
#[doc = "Configuration register for pin GPIO20"]
pub mod gpio20;
#[doc = "GPIO21 register accessor: an alias for `Reg<GPIO21_SPEC>`"]
pub type GPIO21 = crate::Reg<gpio21::GPIO21_SPEC>;
#[doc = "Configuration register for pin GPIO21"]
pub mod gpio21;
#[doc = "GPIO26 register accessor: an alias for `Reg<GPIO26_SPEC>`"]
pub type GPIO26 = crate::Reg<gpio26::GPIO26_SPEC>;
#[doc = "Configuration register for pad GPIO26"]
pub mod gpio26;
#[doc = "GPIO27 register accessor: an alias for `Reg<GPIO27_SPEC>`"]
pub type GPIO27 = crate::Reg<gpio27::GPIO27_SPEC>;
#[doc = "Configuration register for pad GPIO27"]
pub mod gpio27;
#[doc = "GPIO28 register accessor: an alias for `Reg<GPIO28_SPEC>`"]
pub type GPIO28 = crate::Reg<gpio28::GPIO28_SPEC>;
#[doc = "Configuration register for pad GPIO28"]
pub mod gpio28;
#[doc = "GPIO29 register accessor: an alias for `Reg<GPIO29_SPEC>`"]
pub type GPIO29 = crate::Reg<gpio29::GPIO29_SPEC>;
#[doc = "Configuration register for pad GPIO29"]
pub mod gpio29;
#[doc = "GPIO30 register accessor: an alias for `Reg<GPIO30_SPEC>`"]
pub type GPIO30 = crate::Reg<gpio30::GPIO30_SPEC>;
#[doc = "Configuration register for pad GPIO30"]
pub mod gpio30;
#[doc = "GPIO31 register accessor: an alias for `Reg<GPIO31_SPEC>`"]
pub type GPIO31 = crate::Reg<gpio31::GPIO31_SPEC>;
#[doc = "Configuration register for pad GPIO31"]
pub mod gpio31;
#[doc = "GPIO32 register accessor: an alias for `Reg<GPIO32_SPEC>`"]
pub type GPIO32 = crate::Reg<gpio32::GPIO32_SPEC>;
#[doc = "Configuration register for pad GPIO32"]
pub mod gpio32;
#[doc = "GPIO33 register accessor: an alias for `Reg<GPIO33_SPEC>`"]
pub type GPIO33 = crate::Reg<gpio33::GPIO33_SPEC>;
#[doc = "Configuration register for pin GPIO33"]
pub mod gpio33;
#[doc = "GPIO34 register accessor: an alias for `Reg<GPIO34_SPEC>`"]
pub type GPIO34 = crate::Reg<gpio34::GPIO34_SPEC>;
#[doc = "Configuration register for pin GPIO34"]
pub mod gpio34;
#[doc = "GPIO35 register accessor: an alias for `Reg<GPIO35_SPEC>`"]
pub type GPIO35 = crate::Reg<gpio35::GPIO35_SPEC>;
#[doc = "Configuration register for pin GPIO35"]
pub mod gpio35;
#[doc = "GPIO36 register accessor: an alias for `Reg<GPIO36_SPEC>`"]
pub type GPIO36 = crate::Reg<gpio36::GPIO36_SPEC>;
#[doc = "Configuration register for pin GPIO36"]
pub mod gpio36;
#[doc = "GPIO37 register accessor: an alias for `Reg<GPIO37_SPEC>`"]
pub type GPIO37 = crate::Reg<gpio37::GPIO37_SPEC>;
#[doc = "Configuration register for pin GPIO37"]
pub mod gpio37;
#[doc = "GPIO38 register accessor: an alias for `Reg<GPIO38_SPEC>`"]
pub type GPIO38 = crate::Reg<gpio38::GPIO38_SPEC>;
#[doc = "Configuration register for pin GPIO38"]
pub mod gpio38;
#[doc = "GPIO39 register accessor: an alias for `Reg<GPIO39_SPEC>`"]
pub type GPIO39 = crate::Reg<gpio39::GPIO39_SPEC>;
#[doc = "Configuration register for pad GPIO39"]
pub mod gpio39;
#[doc = "GPIO40 register accessor: an alias for `Reg<GPIO40_SPEC>`"]
pub type GPIO40 = crate::Reg<gpio40::GPIO40_SPEC>;
#[doc = "Configuration register for pad GPIO40"]
pub mod gpio40;
#[doc = "GPIO41 register accessor: an alias for `Reg<GPIO41_SPEC>`"]
pub type GPIO41 = crate::Reg<gpio41::GPIO41_SPEC>;
#[doc = "Configuration register for pad GPIO41"]
pub mod gpio41;
#[doc = "GPIO42 register accessor: an alias for `Reg<GPIO42_SPEC>`"]
pub type GPIO42 = crate::Reg<gpio42::GPIO42_SPEC>;
#[doc = "Configuration register for pad GPIO42"]
pub mod gpio42;
#[doc = "GPIO43 register accessor: an alias for `Reg<GPIO43_SPEC>`"]
pub type GPIO43 = crate::Reg<gpio43::GPIO43_SPEC>;
#[doc = "Configuration register for pad GPIO43"]
pub mod gpio43;
#[doc = "GPIO44 register accessor: an alias for `Reg<GPIO44_SPEC>`"]
pub type GPIO44 = crate::Reg<gpio44::GPIO44_SPEC>;
#[doc = "Configuration register for pad GPIO44"]
pub mod gpio44;
#[doc = "GPIO45 register accessor: an alias for `Reg<GPIO45_SPEC>`"]
pub type GPIO45 = crate::Reg<gpio45::GPIO45_SPEC>;
#[doc = "Configuration register for pin GPIO45"]
pub mod gpio45;
#[doc = "GPIO46 register accessor: an alias for `Reg<GPIO46_SPEC>`"]
pub type GPIO46 = crate::Reg<gpio46::GPIO46_SPEC>;
#[doc = "Configuration register for pin GPIO46"]
pub mod gpio46;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
