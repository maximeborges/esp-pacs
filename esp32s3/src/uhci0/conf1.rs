#[doc = "Register `CONF1` reader"]
pub struct R(crate::R<CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF1` writer"]
pub struct W(crate::W<CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF1_SPEC>;
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
impl From<crate::W<CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHECK_SUM_EN` reader - This is the enable bit to check header checksum when UHCI receives a data packet."]
pub struct CHECK_SUM_EN_R(crate::FieldReader<bool, bool>);
impl CHECK_SUM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHECK_SUM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHECK_SUM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHECK_SUM_EN` writer - This is the enable bit to check header checksum when UHCI receives a data packet."]
pub struct CHECK_SUM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHECK_SUM_EN_W<'a> {
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
#[doc = "Field `CHECK_SEQ_EN` reader - This is the enable bit to check sequence number when UHCI receives a data packet."]
pub struct CHECK_SEQ_EN_R(crate::FieldReader<bool, bool>);
impl CHECK_SEQ_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHECK_SEQ_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHECK_SEQ_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHECK_SEQ_EN` writer - This is the enable bit to check sequence number when UHCI receives a data packet."]
pub struct CHECK_SEQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHECK_SEQ_EN_W<'a> {
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
#[doc = "Field `CRC_DISABLE` reader - Set this bit to support CRC calculation. Data Integrity Check Present bit in UHCI packet frame should be 1."]
pub struct CRC_DISABLE_R(crate::FieldReader<bool, bool>);
impl CRC_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRC_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_DISABLE` writer - Set this bit to support CRC calculation. Data Integrity Check Present bit in UHCI packet frame should be 1."]
pub struct CRC_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_DISABLE_W<'a> {
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
#[doc = "Field `SAVE_HEAD` reader - Set this bit to save the packet header when HCI receives a data packet."]
pub struct SAVE_HEAD_R(crate::FieldReader<bool, bool>);
impl SAVE_HEAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAVE_HEAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAVE_HEAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAVE_HEAD` writer - Set this bit to save the packet header when HCI receives a data packet."]
pub struct SAVE_HEAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SAVE_HEAD_W<'a> {
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
#[doc = "Field `TX_CHECK_SUM_RE` reader - Set this bit to encode the data packet with a checksum."]
pub struct TX_CHECK_SUM_RE_R(crate::FieldReader<bool, bool>);
impl TX_CHECK_SUM_RE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_CHECK_SUM_RE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_CHECK_SUM_RE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CHECK_SUM_RE` writer - Set this bit to encode the data packet with a checksum."]
pub struct TX_CHECK_SUM_RE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CHECK_SUM_RE_W<'a> {
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
#[doc = "Field `TX_ACK_NUM_RE` reader - Set this bit to encode the data packet with an acknowledgment when a reliable packet is to be transmit."]
pub struct TX_ACK_NUM_RE_R(crate::FieldReader<bool, bool>);
impl TX_ACK_NUM_RE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_ACK_NUM_RE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_ACK_NUM_RE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_ACK_NUM_RE` writer - Set this bit to encode the data packet with an acknowledgment when a reliable packet is to be transmit."]
pub struct TX_ACK_NUM_RE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ACK_NUM_RE_W<'a> {
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
#[doc = "Field `WAIT_SW_START` reader - The uhci-encoder will jump to ST_SW_WAIT status if this register is set to 1."]
pub struct WAIT_SW_START_R(crate::FieldReader<bool, bool>);
impl WAIT_SW_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAIT_SW_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAIT_SW_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAIT_SW_START` writer - The uhci-encoder will jump to ST_SW_WAIT status if this register is set to 1."]
pub struct WAIT_SW_START_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_SW_START_W<'a> {
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
#[doc = "Field `SW_START` reader - If current UHCI_ENCODE_STATE is ST_SW_WAIT, the UHCI will start to send data packet out when this bit is set to 1."]
pub struct SW_START_R(crate::FieldReader<bool, bool>);
impl SW_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SW_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_START` writer - If current UHCI_ENCODE_STATE is ST_SW_WAIT, the UHCI will start to send data packet out when this bit is set to 1."]
pub struct SW_START_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_START_W<'a> {
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
impl R {
    #[doc = "Bit 0 - This is the enable bit to check header checksum when UHCI receives a data packet."]
    #[inline(always)]
    pub fn check_sum_en(&self) -> CHECK_SUM_EN_R {
        CHECK_SUM_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the enable bit to check sequence number when UHCI receives a data packet."]
    #[inline(always)]
    pub fn check_seq_en(&self) -> CHECK_SEQ_EN_R {
        CHECK_SEQ_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to support CRC calculation. Data Integrity Check Present bit in UHCI packet frame should be 1."]
    #[inline(always)]
    pub fn crc_disable(&self) -> CRC_DISABLE_R {
        CRC_DISABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to save the packet header when HCI receives a data packet."]
    #[inline(always)]
    pub fn save_head(&self) -> SAVE_HEAD_R {
        SAVE_HEAD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to encode the data packet with a checksum."]
    #[inline(always)]
    pub fn tx_check_sum_re(&self) -> TX_CHECK_SUM_RE_R {
        TX_CHECK_SUM_RE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to encode the data packet with an acknowledgment when a reliable packet is to be transmit."]
    #[inline(always)]
    pub fn tx_ack_num_re(&self) -> TX_ACK_NUM_RE_R {
        TX_ACK_NUM_RE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - The uhci-encoder will jump to ST_SW_WAIT status if this register is set to 1."]
    #[inline(always)]
    pub fn wait_sw_start(&self) -> WAIT_SW_START_R {
        WAIT_SW_START_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - If current UHCI_ENCODE_STATE is ST_SW_WAIT, the UHCI will start to send data packet out when this bit is set to 1."]
    #[inline(always)]
    pub fn sw_start(&self) -> SW_START_R {
        SW_START_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This is the enable bit to check header checksum when UHCI receives a data packet."]
    #[inline(always)]
    pub fn check_sum_en(&mut self) -> CHECK_SUM_EN_W {
        CHECK_SUM_EN_W { w: self }
    }
    #[doc = "Bit 1 - This is the enable bit to check sequence number when UHCI receives a data packet."]
    #[inline(always)]
    pub fn check_seq_en(&mut self) -> CHECK_SEQ_EN_W {
        CHECK_SEQ_EN_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to support CRC calculation. Data Integrity Check Present bit in UHCI packet frame should be 1."]
    #[inline(always)]
    pub fn crc_disable(&mut self) -> CRC_DISABLE_W {
        CRC_DISABLE_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to save the packet header when HCI receives a data packet."]
    #[inline(always)]
    pub fn save_head(&mut self) -> SAVE_HEAD_W {
        SAVE_HEAD_W { w: self }
    }
    #[doc = "Bit 4 - Set this bit to encode the data packet with a checksum."]
    #[inline(always)]
    pub fn tx_check_sum_re(&mut self) -> TX_CHECK_SUM_RE_W {
        TX_CHECK_SUM_RE_W { w: self }
    }
    #[doc = "Bit 5 - Set this bit to encode the data packet with an acknowledgment when a reliable packet is to be transmit."]
    #[inline(always)]
    pub fn tx_ack_num_re(&mut self) -> TX_ACK_NUM_RE_W {
        TX_ACK_NUM_RE_W { w: self }
    }
    #[doc = "Bit 7 - The uhci-encoder will jump to ST_SW_WAIT status if this register is set to 1."]
    #[inline(always)]
    pub fn wait_sw_start(&mut self) -> WAIT_SW_START_W {
        WAIT_SW_START_W { w: self }
    }
    #[doc = "Bit 8 - If current UHCI_ENCODE_STATE is ST_SW_WAIT, the UHCI will start to send data packet out when this bit is set to 1."]
    #[inline(always)]
    pub fn sw_start(&mut self) -> SW_START_W {
        SW_START_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UHCI configuration register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf1]
(index.html) module"]
pub struct CONF1_SPEC;
impl crate::RegisterSpec for CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf1::R]
(R) reader structure"]
impl crate::Readable for CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf1::W]
(W) writer structure"]
impl crate::Writable for CONF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONF1 to value 0x33"]
impl crate::Resettable for CONF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x33
    }
}
