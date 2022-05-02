#[doc = "Register `CONF0` reader"]
pub struct R(crate::R<CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF0` writer"]
pub struct W(crate::W<CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_RST` reader - Write 1, then write 0 to this bit to reset decode state machine."]
pub struct TX_RST_R(crate::FieldReader<bool>);
impl TX_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_RST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_RST` writer - Write 1, then write 0 to this bit to reset decode state machine."]
pub struct TX_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `RX_RST` reader - Write 1, then write 0 to this bit to reset encode state machine."]
pub struct RX_RST_R(crate::FieldReader<bool>);
impl RX_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_RST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_RST` writer - Write 1, then write 0 to this bit to reset encode state machine."]
pub struct RX_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `UART0_CE` reader - Set this bit to link up HCI and UART0."]
pub struct UART0_CE_R(crate::FieldReader<bool>);
impl UART0_CE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART0_CE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART0_CE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART0_CE` writer - Set this bit to link up HCI and UART0."]
pub struct UART0_CE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0_CE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `UART1_CE` reader - Set this bit to link up HCI and UART1."]
pub struct UART1_CE_R(crate::FieldReader<bool>);
impl UART1_CE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART1_CE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART1_CE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART1_CE` writer - Set this bit to link up HCI and UART1."]
pub struct UART1_CE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_CE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `UART2_CE` reader - Set this bit to link up HCI and UART2."]
pub struct UART2_CE_R(crate::FieldReader<bool>);
impl UART2_CE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART2_CE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART2_CE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART2_CE` writer - Set this bit to link up HCI and UART2."]
pub struct UART2_CE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2_CE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `SEPER_EN` reader - Set this bit to separate the data frame using a special char."]
pub struct SEPER_EN_R(crate::FieldReader<bool>);
impl SEPER_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEPER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEPER_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEPER_EN` writer - Set this bit to separate the data frame using a special char."]
pub struct SEPER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SEPER_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `HEAD_EN` reader - Set this bit to encode the data packet with a formatting header."]
pub struct HEAD_EN_R(crate::FieldReader<bool>);
impl HEAD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HEAD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HEAD_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HEAD_EN` writer - Set this bit to encode the data packet with a formatting header."]
pub struct HEAD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HEAD_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `CRC_REC_EN` reader - Set this bit to enable UHCI to receive the 16 bit CRC."]
pub struct CRC_REC_EN_R(crate::FieldReader<bool>);
impl CRC_REC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRC_REC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_REC_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_REC_EN` writer - Set this bit to enable UHCI to receive the 16 bit CRC."]
pub struct CRC_REC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_REC_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `UART_IDLE_EOF_EN` reader - If this bit is set to 1, UHCI will end the payload receiving process when UART has been in idle state."]
pub struct UART_IDLE_EOF_EN_R(crate::FieldReader<bool>);
impl UART_IDLE_EOF_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_IDLE_EOF_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_IDLE_EOF_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_IDLE_EOF_EN` writer - If this bit is set to 1, UHCI will end the payload receiving process when UART has been in idle state."]
pub struct UART_IDLE_EOF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_IDLE_EOF_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `LEN_EOF_EN` reader - If this bit is set to 1, UHCI decoder receiving payload data is end when the receiving byte count has reached the specified value. The value is payload length indicated by UHCI packet header when UHCI_HEAD_EN is 1 or the value is configuration value when UHCI_HEAD_EN is 0. If this bit is set to 0, UHCI decoder receiving payload data is end when 0xc0 is received."]
pub struct LEN_EOF_EN_R(crate::FieldReader<bool>);
impl LEN_EOF_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LEN_EOF_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEN_EOF_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEN_EOF_EN` writer - If this bit is set to 1, UHCI decoder receiving payload data is end when the receiving byte count has reached the specified value. The value is payload length indicated by UHCI packet header when UHCI_HEAD_EN is 1 or the value is configuration value when UHCI_HEAD_EN is 0. If this bit is set to 0, UHCI decoder receiving payload data is end when 0xc0 is received."]
pub struct LEN_EOF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEN_EOF_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "Field `ENCODE_CRC_EN` reader - Set this bit to enable data integrity checking by appending a 16 bit CCITT-CRC to end of the payload."]
pub struct ENCODE_CRC_EN_R(crate::FieldReader<bool>);
impl ENCODE_CRC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENCODE_CRC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENCODE_CRC_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENCODE_CRC_EN` writer - Set this bit to enable data integrity checking by appending a 16 bit CCITT-CRC to end of the payload."]
pub struct ENCODE_CRC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCODE_CRC_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "Field `CLK_EN` reader - 1'b1: Force clock on for register. 1'b0: Support clock only when application writes registers."]
pub struct CLK_EN_R(crate::FieldReader<bool>);
impl CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_EN` writer - 1'b1: Force clock on for register. 1'b0: Support clock only when application writes registers."]
pub struct CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Field `UART_RX_BRK_EOF_EN` reader - If this bit is set to 1, UHCI will end payload receive process when NULL frame is received by UART."]
pub struct UART_RX_BRK_EOF_EN_R(crate::FieldReader<bool>);
impl UART_RX_BRK_EOF_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_RX_BRK_EOF_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_RX_BRK_EOF_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_RX_BRK_EOF_EN` writer - If this bit is set to 1, UHCI will end payload receive process when NULL frame is received by UART."]
pub struct UART_RX_BRK_EOF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RX_BRK_EOF_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write 1, then write 0 to this bit to reset decode state machine."]
    #[inline(always)]
    pub fn tx_rst(&self) -> TX_RST_R {
        TX_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1, then write 0 to this bit to reset encode state machine."]
    #[inline(always)]
    pub fn rx_rst(&self) -> RX_RST_R {
        RX_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to link up HCI and UART0."]
    #[inline(always)]
    pub fn uart0_ce(&self) -> UART0_CE_R {
        UART0_CE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to link up HCI and UART1."]
    #[inline(always)]
    pub fn uart1_ce(&self) -> UART1_CE_R {
        UART1_CE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to link up HCI and UART2."]
    #[inline(always)]
    pub fn uart2_ce(&self) -> UART2_CE_R {
        UART2_CE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to separate the data frame using a special char."]
    #[inline(always)]
    pub fn seper_en(&self) -> SEPER_EN_R {
        SEPER_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to encode the data packet with a formatting header."]
    #[inline(always)]
    pub fn head_en(&self) -> HEAD_EN_R {
        HEAD_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to enable UHCI to receive the 16 bit CRC."]
    #[inline(always)]
    pub fn crc_rec_en(&self) -> CRC_REC_EN_R {
        CRC_REC_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - If this bit is set to 1, UHCI will end the payload receiving process when UART has been in idle state."]
    #[inline(always)]
    pub fn uart_idle_eof_en(&self) -> UART_IDLE_EOF_EN_R {
        UART_IDLE_EOF_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - If this bit is set to 1, UHCI decoder receiving payload data is end when the receiving byte count has reached the specified value. The value is payload length indicated by UHCI packet header when UHCI_HEAD_EN is 1 or the value is configuration value when UHCI_HEAD_EN is 0. If this bit is set to 0, UHCI decoder receiving payload data is end when 0xc0 is received."]
    #[inline(always)]
    pub fn len_eof_en(&self) -> LEN_EOF_EN_R {
        LEN_EOF_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set this bit to enable data integrity checking by appending a 16 bit CCITT-CRC to end of the payload."]
    #[inline(always)]
    pub fn encode_crc_en(&self) -> ENCODE_CRC_EN_R {
        ENCODE_CRC_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1'b1: Force clock on for register. 1'b0: Support clock only when application writes registers."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - If this bit is set to 1, UHCI will end payload receive process when NULL frame is received by UART."]
    #[inline(always)]
    pub fn uart_rx_brk_eof_en(&self) -> UART_RX_BRK_EOF_EN_R {
        UART_RX_BRK_EOF_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1, then write 0 to this bit to reset decode state machine."]
    #[inline(always)]
    pub fn tx_rst(&mut self) -> TX_RST_W {
        TX_RST_W { w: self }
    }
    #[doc = "Bit 1 - Write 1, then write 0 to this bit to reset encode state machine."]
    #[inline(always)]
    pub fn rx_rst(&mut self) -> RX_RST_W {
        RX_RST_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to link up HCI and UART0."]
    #[inline(always)]
    pub fn uart0_ce(&mut self) -> UART0_CE_W {
        UART0_CE_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to link up HCI and UART1."]
    #[inline(always)]
    pub fn uart1_ce(&mut self) -> UART1_CE_W {
        UART1_CE_W { w: self }
    }
    #[doc = "Bit 4 - Set this bit to link up HCI and UART2."]
    #[inline(always)]
    pub fn uart2_ce(&mut self) -> UART2_CE_W {
        UART2_CE_W { w: self }
    }
    #[doc = "Bit 5 - Set this bit to separate the data frame using a special char."]
    #[inline(always)]
    pub fn seper_en(&mut self) -> SEPER_EN_W {
        SEPER_EN_W { w: self }
    }
    #[doc = "Bit 6 - Set this bit to encode the data packet with a formatting header."]
    #[inline(always)]
    pub fn head_en(&mut self) -> HEAD_EN_W {
        HEAD_EN_W { w: self }
    }
    #[doc = "Bit 7 - Set this bit to enable UHCI to receive the 16 bit CRC."]
    #[inline(always)]
    pub fn crc_rec_en(&mut self) -> CRC_REC_EN_W {
        CRC_REC_EN_W { w: self }
    }
    #[doc = "Bit 8 - If this bit is set to 1, UHCI will end the payload receiving process when UART has been in idle state."]
    #[inline(always)]
    pub fn uart_idle_eof_en(&mut self) -> UART_IDLE_EOF_EN_W {
        UART_IDLE_EOF_EN_W { w: self }
    }
    #[doc = "Bit 9 - If this bit is set to 1, UHCI decoder receiving payload data is end when the receiving byte count has reached the specified value. The value is payload length indicated by UHCI packet header when UHCI_HEAD_EN is 1 or the value is configuration value when UHCI_HEAD_EN is 0. If this bit is set to 0, UHCI decoder receiving payload data is end when 0xc0 is received."]
    #[inline(always)]
    pub fn len_eof_en(&mut self) -> LEN_EOF_EN_W {
        LEN_EOF_EN_W { w: self }
    }
    #[doc = "Bit 10 - Set this bit to enable data integrity checking by appending a 16 bit CCITT-CRC to end of the payload."]
    #[inline(always)]
    pub fn encode_crc_en(&mut self) -> ENCODE_CRC_EN_W {
        ENCODE_CRC_EN_W { w: self }
    }
    #[doc = "Bit 11 - 1'b1: Force clock on for register. 1'b0: Support clock only when application writes registers."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W { w: self }
    }
    #[doc = "Bit 12 - If this bit is set to 1, UHCI will end payload receive process when NULL frame is received by UART."]
    #[inline(always)]
    pub fn uart_rx_brk_eof_en(&mut self) -> UART_RX_BRK_EOF_EN_W {
        UART_RX_BRK_EOF_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UHCI configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf0](index.html) module"]
pub struct CONF0_SPEC;
impl crate::RegisterSpec for CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf0::R](R) reader structure"]
impl crate::Readable for CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf0::W](W) writer structure"]
impl crate::Writable for CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONF0 to value 0x06e0"]
impl crate::Resettable for CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x06e0
    }
}
