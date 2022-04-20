#[doc = "Register `MEM_CONF` reader"]
pub struct R(crate::R<MEM_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEM_CONF` writer"]
pub struct W(crate::W<MEM_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEM_CONF_SPEC>;
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
impl From<crate::W<MEM_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEM_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_SIZE` reader - This register is used to configure the amount of mem allocated for receive-FIFO. The default number is 128 bytes."]
pub struct RX_SIZE_R(crate::FieldReader<u8, u8>);
impl RX_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_SIZE` writer - This register is used to configure the amount of mem allocated for receive-FIFO. The default number is 128 bytes."]
pub struct RX_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 1)) | ((value as u32 & 7) << 1);
        self.w
    }
}
#[doc = "Field `TX_SIZE` reader - This register is used to configure the amount of mem allocated for transmit-FIFO. The default number is 128 bytes."]
pub struct TX_SIZE_R(crate::FieldReader<u8, u8>);
impl TX_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_SIZE` writer - This register is used to configure the amount of mem allocated for transmit-FIFO. The default number is 128 bytes."]
pub struct TX_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 4)) | ((value as u32 & 7) << 4);
        self.w
    }
}
#[doc = "Field `RX_FLOW_THRHD` reader - This register is used to configure the maximum amount of data that can be received when hardware flow control works."]
pub struct RX_FLOW_THRHD_R(crate::FieldReader<u16, u16>);
impl RX_FLOW_THRHD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RX_FLOW_THRHD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FLOW_THRHD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FLOW_THRHD` writer - This register is used to configure the maximum amount of data that can be received when hardware flow control works."]
pub struct RX_FLOW_THRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FLOW_THRHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 7)) | ((value as u32 & 0x03ff) << 7);
        self.w
    }
}
#[doc = "Field `RX_TOUT_THRHD` reader - This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1."]
pub struct RX_TOUT_THRHD_R(crate::FieldReader<u16, u16>);
impl RX_TOUT_THRHD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RX_TOUT_THRHD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_TOUT_THRHD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_TOUT_THRHD` writer - This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1."]
pub struct RX_TOUT_THRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TOUT_THRHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 17)) | ((value as u32 & 0x03ff) << 17);
        self.w
    }
}
#[doc = "Field `MEM_FORCE_PD` reader - Set this bit to force power down UART memory."]
pub struct MEM_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl MEM_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_FORCE_PD` writer - Set this bit to force power down UART memory."]
pub struct MEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_FORCE_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 27)) | ((value as u32 & 1) << 27);
        self.w
    }
}
#[doc = "Field `MEM_FORCE_PU` reader - Set this bit to force power up UART memory."]
pub struct MEM_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl MEM_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_FORCE_PU` writer - Set this bit to force power up UART memory."]
pub struct MEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_FORCE_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 28)) | ((value as u32 & 1) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:3 - This register is used to configure the amount of mem allocated for receive-FIFO. The default number is 128 bytes."]
    #[inline(always)]
    pub fn rx_size(&self) -> RX_SIZE_R {
        RX_SIZE_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:6 - This register is used to configure the amount of mem allocated for transmit-FIFO. The default number is 128 bytes."]
    #[inline(always)]
    pub fn tx_size(&self) -> TX_SIZE_R {
        TX_SIZE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:16 - This register is used to configure the maximum amount of data that can be received when hardware flow control works."]
    #[inline(always)]
    pub fn rx_flow_thrhd(&self) -> RX_FLOW_THRHD_R {
        RX_FLOW_THRHD_R::new(((self.bits >> 7) & 0x03ff) as u16)
    }
    #[doc = "Bits 17:26 - This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1."]
    #[inline(always)]
    pub fn rx_tout_thrhd(&self) -> RX_TOUT_THRHD_R {
        RX_TOUT_THRHD_R::new(((self.bits >> 17) & 0x03ff) as u16)
    }
    #[doc = "Bit 27 - Set this bit to force power down UART memory."]
    #[inline(always)]
    pub fn mem_force_pd(&self) -> MEM_FORCE_PD_R {
        MEM_FORCE_PD_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Set this bit to force power up UART memory."]
    #[inline(always)]
    pub fn mem_force_pu(&self) -> MEM_FORCE_PU_R {
        MEM_FORCE_PU_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:3 - This register is used to configure the amount of mem allocated for receive-FIFO. The default number is 128 bytes."]
    #[inline(always)]
    pub fn rx_size(&mut self) -> RX_SIZE_W {
        RX_SIZE_W { w: self }
    }
    #[doc = "Bits 4:6 - This register is used to configure the amount of mem allocated for transmit-FIFO. The default number is 128 bytes."]
    #[inline(always)]
    pub fn tx_size(&mut self) -> TX_SIZE_W {
        TX_SIZE_W { w: self }
    }
    #[doc = "Bits 7:16 - This register is used to configure the maximum amount of data that can be received when hardware flow control works."]
    #[inline(always)]
    pub fn rx_flow_thrhd(&mut self) -> RX_FLOW_THRHD_W {
        RX_FLOW_THRHD_W { w: self }
    }
    #[doc = "Bits 17:26 - This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1."]
    #[inline(always)]
    pub fn rx_tout_thrhd(&mut self) -> RX_TOUT_THRHD_W {
        RX_TOUT_THRHD_W { w: self }
    }
    #[doc = "Bit 27 - Set this bit to force power down UART memory."]
    #[inline(always)]
    pub fn mem_force_pd(&mut self) -> MEM_FORCE_PD_W {
        MEM_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 28 - Set this bit to force power up UART memory."]
    #[inline(always)]
    pub fn mem_force_pu(&mut self) -> MEM_FORCE_PU_W {
        MEM_FORCE_PU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART threshold and allocation configuration\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_conf]
(index.html) module"]
pub struct MEM_CONF_SPEC;
impl crate::RegisterSpec for MEM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_conf::R]
(R) reader structure"]
impl crate::Readable for MEM_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_conf::W]
(W) writer structure"]
impl crate::Writable for MEM_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEM_CONF to value 0x0014_0012"]
impl crate::Resettable for MEM_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0014_0012
    }
}
