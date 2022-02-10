#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    #[doc = "0x100..0x140 - Plaintext register %s"]
    pub plain_: [crate::Reg<plain_::PLAIN__SPEC>; 16],
    #[doc = "0x140 - Configures the size of target memory space"]
    pub linesize: crate::Reg<linesize::LINESIZE_SPEC>,
    #[doc = "0x144 - Configures the type of the external memory"]
    pub destination: crate::Reg<destination::DESTINATION_SPEC>,
    #[doc = "0x148 - Physical address"]
    pub physical_address: crate::Reg<physical_address::PHYSICAL_ADDRESS_SPEC>,
    #[doc = "0x14c - Activates AES algorithm"]
    pub trigger: crate::Reg<trigger::TRIGGER_SPEC>,
    #[doc = "0x150 - Release control"]
    pub release: crate::Reg<release::RELEASE_SPEC>,
    #[doc = "0x154 - Destroys control"]
    pub destroy: crate::Reg<destroy::DESTROY_SPEC>,
    #[doc = "0x158 - Status register"]
    pub state: crate::Reg<state::STATE_SPEC>,
    #[doc = "0x15c - Version control register"]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "PLAIN_ register accessor: an alias for `Reg<PLAIN__SPEC>`"]
pub type PLAIN_ = crate::Reg<plain_::PLAIN__SPEC>;
#[doc = "Plaintext register %s"]
pub mod plain_;
#[doc = "LINESIZE register accessor: an alias for `Reg<LINESIZE_SPEC>`"]
pub type LINESIZE = crate::Reg<linesize::LINESIZE_SPEC>;
#[doc = "Configures the size of target memory space"]
pub mod linesize;
#[doc = "DESTINATION register accessor: an alias for `Reg<DESTINATION_SPEC>`"]
pub type DESTINATION = crate::Reg<destination::DESTINATION_SPEC>;
#[doc = "Configures the type of the external memory"]
pub mod destination;
#[doc = "PHYSICAL_ADDRESS register accessor: an alias for `Reg<PHYSICAL_ADDRESS_SPEC>`"]
pub type PHYSICAL_ADDRESS = crate::Reg<physical_address::PHYSICAL_ADDRESS_SPEC>;
#[doc = "Physical address"]
pub mod physical_address;
#[doc = "TRIGGER register accessor: an alias for `Reg<TRIGGER_SPEC>`"]
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
#[doc = "Activates AES algorithm"]
pub mod trigger;
#[doc = "RELEASE register accessor: an alias for `Reg<RELEASE_SPEC>`"]
pub type RELEASE = crate::Reg<release::RELEASE_SPEC>;
#[doc = "Release control"]
pub mod release;
#[doc = "DESTROY register accessor: an alias for `Reg<DESTROY_SPEC>`"]
pub type DESTROY = crate::Reg<destroy::DESTROY_SPEC>;
#[doc = "Destroys control"]
pub mod destroy;
#[doc = "STATE register accessor: an alias for `Reg<STATE_SPEC>`"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "Status register"]
pub mod state;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
