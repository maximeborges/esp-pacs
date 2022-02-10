#[doc = "Register `CH3CONF1` reader"]
pub struct R(crate::R<CH3CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH3CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH3CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH3CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH3CONF1` writer"]
pub struct W(crate::W<CH3CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH3CONF1_SPEC>;
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
impl From<crate::W<CH3CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH3CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_EN_CH3` reader - reg_rx_en_ch3."]
pub struct RX_EN_CH3_R(crate::FieldReader<bool, bool>);
impl RX_EN_CH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_EN_CH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_EN_CH3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_EN_CH3` writer - reg_rx_en_ch3."]
pub struct RX_EN_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_EN_CH3_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `MEM_WR_RST_CH3` writer - reg_mem_wr_rst_ch3."]
pub struct MEM_WR_RST_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_WR_RST_CH3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `APB_MEM_RST_CH3` writer - reg_apb_mem_rst_ch3."]
pub struct APB_MEM_RST_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_MEM_RST_CH3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `MEM_OWNER_CH3` reader - reg_mem_owner_ch3."]
pub struct MEM_OWNER_CH3_R(crate::FieldReader<bool, bool>);
impl MEM_OWNER_CH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM_OWNER_CH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_OWNER_CH3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_OWNER_CH3` writer - reg_mem_owner_ch3."]
pub struct MEM_OWNER_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_OWNER_CH3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `RX_FILTER_EN_CH3` reader - reg_rx_filter_en_ch3."]
pub struct RX_FILTER_EN_CH3_R(crate::FieldReader<bool, bool>);
impl RX_FILTER_EN_CH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FILTER_EN_CH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FILTER_EN_CH3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FILTER_EN_CH3` writer - reg_rx_filter_en_ch3."]
pub struct RX_FILTER_EN_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FILTER_EN_CH3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `RX_FILTER_THRES_CH3` reader - reg_rx_filter_thres_ch3."]
pub struct RX_FILTER_THRES_CH3_R(crate::FieldReader<u8, u8>);
impl RX_FILTER_THRES_CH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_FILTER_THRES_CH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FILTER_THRES_CH3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FILTER_THRES_CH3` writer - reg_rx_filter_thres_ch3."]
pub struct RX_FILTER_THRES_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FILTER_THRES_CH3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 5)) | ((value as u32 & 0xff) << 5);
        self.w
    }
}
#[doc = "Field `MEM_RX_WRAP_EN_CH3` reader - reg_mem_rx_wrap_en_ch3."]
pub struct MEM_RX_WRAP_EN_CH3_R(crate::FieldReader<bool, bool>);
impl MEM_RX_WRAP_EN_CH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM_RX_WRAP_EN_CH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_RX_WRAP_EN_CH3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_RX_WRAP_EN_CH3` writer - reg_mem_rx_wrap_en_ch3."]
pub struct MEM_RX_WRAP_EN_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_RX_WRAP_EN_CH3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `AFIFO_RST_CH3` writer - reg_afifo_rst_ch3."]
pub struct AFIFO_RST_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> AFIFO_RST_CH3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `CONF_UPDATE_CH3` writer - reg_conf_update_ch3."]
pub struct CONF_UPDATE_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> CONF_UPDATE_CH3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - reg_rx_en_ch3."]
    #[inline(always)]
    pub fn rx_en_ch3(&self) -> RX_EN_CH3_R {
        RX_EN_CH3_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - reg_mem_owner_ch3."]
    #[inline(always)]
    pub fn mem_owner_ch3(&self) -> MEM_OWNER_CH3_R {
        MEM_OWNER_CH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - reg_rx_filter_en_ch3."]
    #[inline(always)]
    pub fn rx_filter_en_ch3(&self) -> RX_FILTER_EN_CH3_R {
        RX_FILTER_EN_CH3_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:12 - reg_rx_filter_thres_ch3."]
    #[inline(always)]
    pub fn rx_filter_thres_ch3(&self) -> RX_FILTER_THRES_CH3_R {
        RX_FILTER_THRES_CH3_R::new(((self.bits >> 5) & 0xff) as u8)
    }
    #[doc = "Bit 13 - reg_mem_rx_wrap_en_ch3."]
    #[inline(always)]
    pub fn mem_rx_wrap_en_ch3(&self) -> MEM_RX_WRAP_EN_CH3_R {
        MEM_RX_WRAP_EN_CH3_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - reg_rx_en_ch3."]
    #[inline(always)]
    pub fn rx_en_ch3(&mut self) -> RX_EN_CH3_W {
        RX_EN_CH3_W { w: self }
    }
    #[doc = "Bit 1 - reg_mem_wr_rst_ch3."]
    #[inline(always)]
    pub fn mem_wr_rst_ch3(&mut self) -> MEM_WR_RST_CH3_W {
        MEM_WR_RST_CH3_W { w: self }
    }
    #[doc = "Bit 2 - reg_apb_mem_rst_ch3."]
    #[inline(always)]
    pub fn apb_mem_rst_ch3(&mut self) -> APB_MEM_RST_CH3_W {
        APB_MEM_RST_CH3_W { w: self }
    }
    #[doc = "Bit 3 - reg_mem_owner_ch3."]
    #[inline(always)]
    pub fn mem_owner_ch3(&mut self) -> MEM_OWNER_CH3_W {
        MEM_OWNER_CH3_W { w: self }
    }
    #[doc = "Bit 4 - reg_rx_filter_en_ch3."]
    #[inline(always)]
    pub fn rx_filter_en_ch3(&mut self) -> RX_FILTER_EN_CH3_W {
        RX_FILTER_EN_CH3_W { w: self }
    }
    #[doc = "Bits 5:12 - reg_rx_filter_thres_ch3."]
    #[inline(always)]
    pub fn rx_filter_thres_ch3(&mut self) -> RX_FILTER_THRES_CH3_W {
        RX_FILTER_THRES_CH3_W { w: self }
    }
    #[doc = "Bit 13 - reg_mem_rx_wrap_en_ch3."]
    #[inline(always)]
    pub fn mem_rx_wrap_en_ch3(&mut self) -> MEM_RX_WRAP_EN_CH3_W {
        MEM_RX_WRAP_EN_CH3_W { w: self }
    }
    #[doc = "Bit 14 - reg_afifo_rst_ch3."]
    #[inline(always)]
    pub fn afifo_rst_ch3(&mut self) -> AFIFO_RST_CH3_W {
        AFIFO_RST_CH3_W { w: self }
    }
    #[doc = "Bit 15 - reg_conf_update_ch3."]
    #[inline(always)]
    pub fn conf_update_ch3(&mut self) -> CONF_UPDATE_CH3_W {
        CONF_UPDATE_CH3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RMT_CH3CONF1_REG.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3conf1]
(index.html) module"]
pub struct CH3CONF1_SPEC;
impl crate::RegisterSpec for CH3CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch3conf1::R]
(R) reader structure"]
impl crate::Readable for CH3CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch3conf1::W]
(W) writer structure"]
impl crate::Writable for CH3CONF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH3CONF1 to value 0x01e8"]
impl crate::Resettable for CH3CONF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01e8
    }
}
