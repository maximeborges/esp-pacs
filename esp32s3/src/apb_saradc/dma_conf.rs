#[doc = "Register `DMA_CONF` reader"]
pub struct R(crate::R<DMA_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_CONF` writer"]
pub struct W(crate::W<DMA_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CONF_SPEC>;
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
impl From<crate::W<DMA_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APB_ADC_EOF_NUM` reader - the dma_in_suc_eof gen when sample cnt = spi_eof_num"]
pub struct APB_ADC_EOF_NUM_R(crate::FieldReader<u16, u16>);
impl APB_ADC_EOF_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        APB_ADC_EOF_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_ADC_EOF_NUM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_ADC_EOF_NUM` writer - the dma_in_suc_eof gen when sample cnt = spi_eof_num"]
pub struct APB_ADC_EOF_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_ADC_EOF_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `APB_ADC_RESET_FSM` reader - reset_apb_adc_state"]
pub struct APB_ADC_RESET_FSM_R(crate::FieldReader<bool, bool>);
impl APB_ADC_RESET_FSM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_ADC_RESET_FSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_ADC_RESET_FSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_ADC_RESET_FSM` writer - reset_apb_adc_state"]
pub struct APB_ADC_RESET_FSM_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_ADC_RESET_FSM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `APB_ADC_TRANS` reader - enable apb_adc use spi_dma"]
pub struct APB_ADC_TRANS_R(crate::FieldReader<bool, bool>);
impl APB_ADC_TRANS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_ADC_TRANS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_ADC_TRANS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_ADC_TRANS` writer - enable apb_adc use spi_dma"]
pub struct APB_ADC_TRANS_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_ADC_TRANS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - the dma_in_suc_eof gen when sample cnt = spi_eof_num"]
    #[inline(always)]
    pub fn apb_adc_eof_num(&self) -> APB_ADC_EOF_NUM_R {
        APB_ADC_EOF_NUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 30 - reset_apb_adc_state"]
    #[inline(always)]
    pub fn apb_adc_reset_fsm(&self) -> APB_ADC_RESET_FSM_R {
        APB_ADC_RESET_FSM_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - enable apb_adc use spi_dma"]
    #[inline(always)]
    pub fn apb_adc_trans(&self) -> APB_ADC_TRANS_R {
        APB_ADC_TRANS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - the dma_in_suc_eof gen when sample cnt = spi_eof_num"]
    #[inline(always)]
    pub fn apb_adc_eof_num(&mut self) -> APB_ADC_EOF_NUM_W {
        APB_ADC_EOF_NUM_W { w: self }
    }
    #[doc = "Bit 30 - reset_apb_adc_state"]
    #[inline(always)]
    pub fn apb_adc_reset_fsm(&mut self) -> APB_ADC_RESET_FSM_W {
        APB_ADC_RESET_FSM_W { w: self }
    }
    #[doc = "Bit 31 - enable apb_adc use spi_dma"]
    #[inline(always)]
    pub fn apb_adc_trans(&mut self) -> APB_ADC_TRANS_W {
        APB_ADC_TRANS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure apb saradc dma\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_conf]
(index.html) module"]
pub struct DMA_CONF_SPEC;
impl crate::RegisterSpec for DMA_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_conf::R]
(R) reader structure"]
impl crate::Readable for DMA_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_conf::W]
(W) writer structure"]
impl crate::Writable for DMA_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_CONF to value 0xff"]
impl crate::Resettable for DMA_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
