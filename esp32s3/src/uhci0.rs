#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UHCI configuration register"]
    pub conf0: crate::Reg<conf0::CONF0_SPEC>,
    #[doc = "0x04 - Raw interrupt status"]
    pub int_raw: crate::Reg<int_raw::INT_RAW_SPEC>,
    #[doc = "0x08 - Masked interrupt status"]
    pub int_st: crate::Reg<int_st::INT_ST_SPEC>,
    #[doc = "0x0c - Interrupt enable bits"]
    pub int_ena: crate::Reg<int_ena::INT_ENA_SPEC>,
    #[doc = "0x10 - Interrupt clear bits"]
    pub int_clr: crate::Reg<int_clr::INT_CLR_SPEC>,
    #[doc = "0x14 - Software interrupt trigger source"]
    pub app_int_set: crate::Reg<app_int_set::APP_INT_SET_SPEC>,
    #[doc = "0x18 - UHCI configuration register"]
    pub conf1: crate::Reg<conf1::CONF1_SPEC>,
    #[doc = "0x1c - UHCI receive status"]
    pub state0: crate::Reg<state0::STATE0_SPEC>,
    #[doc = "0x20 - UHCI transmit status"]
    pub state1: crate::Reg<state1::STATE1_SPEC>,
    #[doc = "0x24 - Escape character configuration"]
    pub escape_conf: crate::Reg<escape_conf::ESCAPE_CONF_SPEC>,
    #[doc = "0x28 - Timeout configuration"]
    pub hung_conf: crate::Reg<hung_conf::HUNG_CONF_SPEC>,
    #[doc = "0x2c - UHCI ACK number configuration"]
    pub ack_num: crate::Reg<ack_num::ACK_NUM_SPEC>,
    #[doc = "0x30 - UHCI packet header register"]
    pub rx_head: crate::Reg<rx_head::RX_HEAD_SPEC>,
    #[doc = "0x34 - UHCI quick send configuration register"]
    pub quick_sent: crate::Reg<quick_sent::QUICK_SENT_SPEC>,
    #[doc = "0x38 - Q0_WORD0 quick_sent register"]
    pub reg_q0_word0: crate::Reg<reg_q0_word0::REG_Q0_WORD0_SPEC>,
    #[doc = "0x3c - Q0_WORD1 quick_sent register"]
    pub reg_q0_word1: crate::Reg<reg_q0_word1::REG_Q0_WORD1_SPEC>,
    #[doc = "0x40 - Q1_WORD0 quick_sent register"]
    pub reg_q1_word0: crate::Reg<reg_q1_word0::REG_Q1_WORD0_SPEC>,
    #[doc = "0x44 - Q1_WORD1 quick_sent register"]
    pub reg_q1_word1: crate::Reg<reg_q1_word1::REG_Q1_WORD1_SPEC>,
    #[doc = "0x48 - Q2_WORD0 quick_sent register"]
    pub reg_q2_word0: crate::Reg<reg_q2_word0::REG_Q2_WORD0_SPEC>,
    #[doc = "0x4c - Q2_WORD1 quick_sent register"]
    pub reg_q2_word1: crate::Reg<reg_q2_word1::REG_Q2_WORD1_SPEC>,
    #[doc = "0x50 - Q3_WORD0 quick_sent register"]
    pub reg_q3_word0: crate::Reg<reg_q3_word0::REG_Q3_WORD0_SPEC>,
    #[doc = "0x54 - Q3_WORD1 quick_sent register"]
    pub reg_q3_word1: crate::Reg<reg_q3_word1::REG_Q3_WORD1_SPEC>,
    #[doc = "0x58 - Q4_WORD0 quick_sent register"]
    pub reg_q4_word0: crate::Reg<reg_q4_word0::REG_Q4_WORD0_SPEC>,
    #[doc = "0x5c - Q4_WORD1 quick_sent register"]
    pub reg_q4_word1: crate::Reg<reg_q4_word1::REG_Q4_WORD1_SPEC>,
    #[doc = "0x60 - Q5_WORD0 quick_sent register"]
    pub reg_q5_word0: crate::Reg<reg_q5_word0::REG_Q5_WORD0_SPEC>,
    #[doc = "0x64 - Q5_WORD1 quick_sent register"]
    pub reg_q5_word1: crate::Reg<reg_q5_word1::REG_Q5_WORD1_SPEC>,
    #[doc = "0x68 - Q6_WORD0 quick_sent register"]
    pub reg_q6_word0: crate::Reg<reg_q6_word0::REG_Q6_WORD0_SPEC>,
    #[doc = "0x6c - Q6_WORD1 quick_sent register"]
    pub reg_q6_word1: crate::Reg<reg_q6_word1::REG_Q6_WORD1_SPEC>,
    #[doc = "0x70 - Escape sequence configuration register 0"]
    pub esc_conf0: crate::Reg<esc_conf0::ESC_CONF0_SPEC>,
    #[doc = "0x74 - Escape sequence configuration register 1"]
    pub esc_conf1: crate::Reg<esc_conf1::ESC_CONF1_SPEC>,
    #[doc = "0x78 - Escape sequence configuration register 2"]
    pub esc_conf2: crate::Reg<esc_conf2::ESC_CONF2_SPEC>,
    #[doc = "0x7c - Escape sequence configuration register 3"]
    pub esc_conf3: crate::Reg<esc_conf3::ESC_CONF3_SPEC>,
    #[doc = "0x80 - Configure register for packet length"]
    pub pkt_thres: crate::Reg<pkt_thres::PKT_THRES_SPEC>,
    #[doc = "0x84 - UHCI version control register"]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "CONF0 register accessor: an alias for `Reg<CONF0_SPEC>`"]
pub type CONF0 = crate::Reg<conf0::CONF0_SPEC>;
#[doc = "UHCI configuration register"]
pub mod conf0;
#[doc = "INT_RAW register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Raw interrupt status"]
pub mod int_raw;
#[doc = "INT_ST register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Masked interrupt status"]
pub mod int_st;
#[doc = "INT_ENA register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt enable bits"]
pub mod int_ena;
#[doc = "INT_CLR register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt clear bits"]
pub mod int_clr;
#[doc = "APP_INT_SET register accessor: an alias for `Reg<APP_INT_SET_SPEC>`"]
pub type APP_INT_SET = crate::Reg<app_int_set::APP_INT_SET_SPEC>;
#[doc = "Software interrupt trigger source"]
pub mod app_int_set;
#[doc = "CONF1 register accessor: an alias for `Reg<CONF1_SPEC>`"]
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
#[doc = "UHCI configuration register"]
pub mod conf1;
#[doc = "STATE0 register accessor: an alias for `Reg<STATE0_SPEC>`"]
pub type STATE0 = crate::Reg<state0::STATE0_SPEC>;
#[doc = "UHCI receive status"]
pub mod state0;
#[doc = "STATE1 register accessor: an alias for `Reg<STATE1_SPEC>`"]
pub type STATE1 = crate::Reg<state1::STATE1_SPEC>;
#[doc = "UHCI transmit status"]
pub mod state1;
#[doc = "ESCAPE_CONF register accessor: an alias for `Reg<ESCAPE_CONF_SPEC>`"]
pub type ESCAPE_CONF = crate::Reg<escape_conf::ESCAPE_CONF_SPEC>;
#[doc = "Escape character configuration"]
pub mod escape_conf;
#[doc = "HUNG_CONF register accessor: an alias for `Reg<HUNG_CONF_SPEC>`"]
pub type HUNG_CONF = crate::Reg<hung_conf::HUNG_CONF_SPEC>;
#[doc = "Timeout configuration"]
pub mod hung_conf;
#[doc = "ACK_NUM register accessor: an alias for `Reg<ACK_NUM_SPEC>`"]
pub type ACK_NUM = crate::Reg<ack_num::ACK_NUM_SPEC>;
#[doc = "UHCI ACK number configuration"]
pub mod ack_num;
#[doc = "RX_HEAD register accessor: an alias for `Reg<RX_HEAD_SPEC>`"]
pub type RX_HEAD = crate::Reg<rx_head::RX_HEAD_SPEC>;
#[doc = "UHCI packet header register"]
pub mod rx_head;
#[doc = "QUICK_SENT register accessor: an alias for `Reg<QUICK_SENT_SPEC>`"]
pub type QUICK_SENT = crate::Reg<quick_sent::QUICK_SENT_SPEC>;
#[doc = "UHCI quick send configuration register"]
pub mod quick_sent;
#[doc = "REG_Q0_WORD0 register accessor: an alias for `Reg<REG_Q0_WORD0_SPEC>`"]
pub type REG_Q0_WORD0 = crate::Reg<reg_q0_word0::REG_Q0_WORD0_SPEC>;
#[doc = "Q0_WORD0 quick_sent register"]
pub mod reg_q0_word0;
#[doc = "REG_Q0_WORD1 register accessor: an alias for `Reg<REG_Q0_WORD1_SPEC>`"]
pub type REG_Q0_WORD1 = crate::Reg<reg_q0_word1::REG_Q0_WORD1_SPEC>;
#[doc = "Q0_WORD1 quick_sent register"]
pub mod reg_q0_word1;
#[doc = "REG_Q1_WORD0 register accessor: an alias for `Reg<REG_Q1_WORD0_SPEC>`"]
pub type REG_Q1_WORD0 = crate::Reg<reg_q1_word0::REG_Q1_WORD0_SPEC>;
#[doc = "Q1_WORD0 quick_sent register"]
pub mod reg_q1_word0;
#[doc = "REG_Q1_WORD1 register accessor: an alias for `Reg<REG_Q1_WORD1_SPEC>`"]
pub type REG_Q1_WORD1 = crate::Reg<reg_q1_word1::REG_Q1_WORD1_SPEC>;
#[doc = "Q1_WORD1 quick_sent register"]
pub mod reg_q1_word1;
#[doc = "REG_Q2_WORD0 register accessor: an alias for `Reg<REG_Q2_WORD0_SPEC>`"]
pub type REG_Q2_WORD0 = crate::Reg<reg_q2_word0::REG_Q2_WORD0_SPEC>;
#[doc = "Q2_WORD0 quick_sent register"]
pub mod reg_q2_word0;
#[doc = "REG_Q2_WORD1 register accessor: an alias for `Reg<REG_Q2_WORD1_SPEC>`"]
pub type REG_Q2_WORD1 = crate::Reg<reg_q2_word1::REG_Q2_WORD1_SPEC>;
#[doc = "Q2_WORD1 quick_sent register"]
pub mod reg_q2_word1;
#[doc = "REG_Q3_WORD0 register accessor: an alias for `Reg<REG_Q3_WORD0_SPEC>`"]
pub type REG_Q3_WORD0 = crate::Reg<reg_q3_word0::REG_Q3_WORD0_SPEC>;
#[doc = "Q3_WORD0 quick_sent register"]
pub mod reg_q3_word0;
#[doc = "REG_Q3_WORD1 register accessor: an alias for `Reg<REG_Q3_WORD1_SPEC>`"]
pub type REG_Q3_WORD1 = crate::Reg<reg_q3_word1::REG_Q3_WORD1_SPEC>;
#[doc = "Q3_WORD1 quick_sent register"]
pub mod reg_q3_word1;
#[doc = "REG_Q4_WORD0 register accessor: an alias for `Reg<REG_Q4_WORD0_SPEC>`"]
pub type REG_Q4_WORD0 = crate::Reg<reg_q4_word0::REG_Q4_WORD0_SPEC>;
#[doc = "Q4_WORD0 quick_sent register"]
pub mod reg_q4_word0;
#[doc = "REG_Q4_WORD1 register accessor: an alias for `Reg<REG_Q4_WORD1_SPEC>`"]
pub type REG_Q4_WORD1 = crate::Reg<reg_q4_word1::REG_Q4_WORD1_SPEC>;
#[doc = "Q4_WORD1 quick_sent register"]
pub mod reg_q4_word1;
#[doc = "REG_Q5_WORD0 register accessor: an alias for `Reg<REG_Q5_WORD0_SPEC>`"]
pub type REG_Q5_WORD0 = crate::Reg<reg_q5_word0::REG_Q5_WORD0_SPEC>;
#[doc = "Q5_WORD0 quick_sent register"]
pub mod reg_q5_word0;
#[doc = "REG_Q5_WORD1 register accessor: an alias for `Reg<REG_Q5_WORD1_SPEC>`"]
pub type REG_Q5_WORD1 = crate::Reg<reg_q5_word1::REG_Q5_WORD1_SPEC>;
#[doc = "Q5_WORD1 quick_sent register"]
pub mod reg_q5_word1;
#[doc = "REG_Q6_WORD0 register accessor: an alias for `Reg<REG_Q6_WORD0_SPEC>`"]
pub type REG_Q6_WORD0 = crate::Reg<reg_q6_word0::REG_Q6_WORD0_SPEC>;
#[doc = "Q6_WORD0 quick_sent register"]
pub mod reg_q6_word0;
#[doc = "REG_Q6_WORD1 register accessor: an alias for `Reg<REG_Q6_WORD1_SPEC>`"]
pub type REG_Q6_WORD1 = crate::Reg<reg_q6_word1::REG_Q6_WORD1_SPEC>;
#[doc = "Q6_WORD1 quick_sent register"]
pub mod reg_q6_word1;
#[doc = "ESC_CONF0 register accessor: an alias for `Reg<ESC_CONF0_SPEC>`"]
pub type ESC_CONF0 = crate::Reg<esc_conf0::ESC_CONF0_SPEC>;
#[doc = "Escape sequence configuration register 0"]
pub mod esc_conf0;
#[doc = "ESC_CONF1 register accessor: an alias for `Reg<ESC_CONF1_SPEC>`"]
pub type ESC_CONF1 = crate::Reg<esc_conf1::ESC_CONF1_SPEC>;
#[doc = "Escape sequence configuration register 1"]
pub mod esc_conf1;
#[doc = "ESC_CONF2 register accessor: an alias for `Reg<ESC_CONF2_SPEC>`"]
pub type ESC_CONF2 = crate::Reg<esc_conf2::ESC_CONF2_SPEC>;
#[doc = "Escape sequence configuration register 2"]
pub mod esc_conf2;
#[doc = "ESC_CONF3 register accessor: an alias for `Reg<ESC_CONF3_SPEC>`"]
pub type ESC_CONF3 = crate::Reg<esc_conf3::ESC_CONF3_SPEC>;
#[doc = "Escape sequence configuration register 3"]
pub mod esc_conf3;
#[doc = "PKT_THRES register accessor: an alias for `Reg<PKT_THRES_SPEC>`"]
pub type PKT_THRES = crate::Reg<pkt_thres::PKT_THRES_SPEC>;
#[doc = "Configure register for packet length"]
pub mod pkt_thres;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "UHCI version control register"]
pub mod date;
