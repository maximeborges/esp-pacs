#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x10 - The memory that stores M"]
    pub m_mem: [crate::Reg<m_mem::M_MEM_SPEC>; 16],
    _reserved1: [u8; 0x01f0],
    #[doc = "0x200..0x210 - The memory that stores Z"]
    pub z_mem: [crate::Reg<z_mem::Z_MEM_SPEC>; 16],
    _reserved2: [u8; 0x01f0],
    #[doc = "0x400..0x410 - The memory that stores Y"]
    pub y_mem: [crate::Reg<y_mem::Y_MEM_SPEC>; 16],
    _reserved3: [u8; 0x01f0],
    #[doc = "0x600..0x610 - The memory that stores X"]
    pub x_mem: [crate::Reg<x_mem::X_MEM_SPEC>; 16],
    _reserved4: [u8; 0x01f0],
    #[doc = "0x800 - RSA M_prime register"]
    pub m_prime: crate::Reg<m_prime::M_PRIME_SPEC>,
    #[doc = "0x804 - RSA mode register"]
    pub mode: crate::Reg<mode::MODE_SPEC>,
    #[doc = "0x808 - RSA query clean register"]
    pub query_clean: crate::Reg<query_clean::QUERY_CLEAN_SPEC>,
    #[doc = "0x80c - RSA modular exponentiation trigger register."]
    pub set_start_modexp: crate::Reg<set_start_modexp::SET_START_MODEXP_SPEC>,
    #[doc = "0x810 - RSA modular multiplication trigger register."]
    pub set_start_modmult: crate::Reg<set_start_modmult::SET_START_MODMULT_SPEC>,
    #[doc = "0x814 - RSA normal multiplication trigger register."]
    pub set_start_mult: crate::Reg<set_start_mult::SET_START_MULT_SPEC>,
    #[doc = "0x818 - RSA query idle register"]
    pub query_idle: crate::Reg<query_idle::QUERY_IDLE_SPEC>,
    #[doc = "0x81c - RSA interrupt clear register"]
    pub int_clr: crate::Reg<int_clr::INT_CLR_SPEC>,
    #[doc = "0x820 - RSA constant time option register"]
    pub constant_time: crate::Reg<constant_time::CONSTANT_TIME_SPEC>,
    #[doc = "0x824 - RSA search option"]
    pub search_enable: crate::Reg<search_enable::SEARCH_ENABLE_SPEC>,
    #[doc = "0x828 - RSA search position configure register"]
    pub search_pos: crate::Reg<search_pos::SEARCH_POS_SPEC>,
    #[doc = "0x82c - RSA interrupt enable register"]
    pub int_ena: crate::Reg<int_ena::INT_ENA_SPEC>,
    #[doc = "0x830 - RSA version control register"]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "M_MEM register accessor: an alias for `Reg<M_MEM_SPEC>`"]
pub type M_MEM = crate::Reg<m_mem::M_MEM_SPEC>;
#[doc = "The memory that stores M"]
pub mod m_mem;
#[doc = "Z_MEM register accessor: an alias for `Reg<Z_MEM_SPEC>`"]
pub type Z_MEM = crate::Reg<z_mem::Z_MEM_SPEC>;
#[doc = "The memory that stores Z"]
pub mod z_mem;
#[doc = "Y_MEM register accessor: an alias for `Reg<Y_MEM_SPEC>`"]
pub type Y_MEM = crate::Reg<y_mem::Y_MEM_SPEC>;
#[doc = "The memory that stores Y"]
pub mod y_mem;
#[doc = "X_MEM register accessor: an alias for `Reg<X_MEM_SPEC>`"]
pub type X_MEM = crate::Reg<x_mem::X_MEM_SPEC>;
#[doc = "The memory that stores X"]
pub mod x_mem;
#[doc = "M_PRIME register accessor: an alias for `Reg<M_PRIME_SPEC>`"]
pub type M_PRIME = crate::Reg<m_prime::M_PRIME_SPEC>;
#[doc = "RSA M_prime register"]
pub mod m_prime;
#[doc = "MODE register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "RSA mode register"]
pub mod mode;
#[doc = "QUERY_CLEAN register accessor: an alias for `Reg<QUERY_CLEAN_SPEC>`"]
pub type QUERY_CLEAN = crate::Reg<query_clean::QUERY_CLEAN_SPEC>;
#[doc = "RSA query clean register"]
pub mod query_clean;
#[doc = "SET_START_MODEXP register accessor: an alias for `Reg<SET_START_MODEXP_SPEC>`"]
pub type SET_START_MODEXP = crate::Reg<set_start_modexp::SET_START_MODEXP_SPEC>;
#[doc = "RSA modular exponentiation trigger register."]
pub mod set_start_modexp;
#[doc = "SET_START_MODMULT register accessor: an alias for `Reg<SET_START_MODMULT_SPEC>`"]
pub type SET_START_MODMULT = crate::Reg<set_start_modmult::SET_START_MODMULT_SPEC>;
#[doc = "RSA modular multiplication trigger register."]
pub mod set_start_modmult;
#[doc = "SET_START_MULT register accessor: an alias for `Reg<SET_START_MULT_SPEC>`"]
pub type SET_START_MULT = crate::Reg<set_start_mult::SET_START_MULT_SPEC>;
#[doc = "RSA normal multiplication trigger register."]
pub mod set_start_mult;
#[doc = "QUERY_IDLE register accessor: an alias for `Reg<QUERY_IDLE_SPEC>`"]
pub type QUERY_IDLE = crate::Reg<query_idle::QUERY_IDLE_SPEC>;
#[doc = "RSA query idle register"]
pub mod query_idle;
#[doc = "INT_CLR register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "RSA interrupt clear register"]
pub mod int_clr;
#[doc = "CONSTANT_TIME register accessor: an alias for `Reg<CONSTANT_TIME_SPEC>`"]
pub type CONSTANT_TIME = crate::Reg<constant_time::CONSTANT_TIME_SPEC>;
#[doc = "RSA constant time option register"]
pub mod constant_time;
#[doc = "SEARCH_ENABLE register accessor: an alias for `Reg<SEARCH_ENABLE_SPEC>`"]
pub type SEARCH_ENABLE = crate::Reg<search_enable::SEARCH_ENABLE_SPEC>;
#[doc = "RSA search option"]
pub mod search_enable;
#[doc = "SEARCH_POS register accessor: an alias for `Reg<SEARCH_POS_SPEC>`"]
pub type SEARCH_POS = crate::Reg<search_pos::SEARCH_POS_SPEC>;
#[doc = "RSA search position configure register"]
pub mod search_pos;
#[doc = "INT_ENA register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "RSA interrupt enable register"]
pub mod int_ena;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "RSA version control register"]
pub mod date;
