#[doc = "Register `APB_DAC_CTRL` reader"]
pub struct R(crate::R<APB_DAC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_DAC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_DAC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_DAC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_DAC_CTRL` writer"]
pub struct W(crate::W<APB_DAC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_DAC_CTRL_SPEC>;
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
impl From<crate::W<APB_DAC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_DAC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAC_TIMER_TARGET` reader - Set DAC timer target."]
pub struct DAC_TIMER_TARGET_R(crate::FieldReader<u16, u16>);
impl DAC_TIMER_TARGET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DAC_TIMER_TARGET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_TIMER_TARGET_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_TIMER_TARGET` writer - Set DAC timer target."]
pub struct DAC_TIMER_TARGET_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_TIMER_TARGET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `DAC_TIMER_EN` reader - Enable read dac data."]
pub struct DAC_TIMER_EN_R(crate::FieldReader<bool, bool>);
impl DAC_TIMER_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAC_TIMER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_TIMER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_TIMER_EN` writer - Enable read dac data."]
pub struct DAC_TIMER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_TIMER_EN_W<'a> {
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
#[doc = "Field `APB_DAC_ALTER_MODE` reader - Enable DAC alter mode."]
pub struct APB_DAC_ALTER_MODE_R(crate::FieldReader<bool, bool>);
impl APB_DAC_ALTER_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_DAC_ALTER_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_DAC_ALTER_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_DAC_ALTER_MODE` writer - Enable DAC alter mode."]
pub struct APB_DAC_ALTER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_DAC_ALTER_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `APB_DAC_TRANS` reader - Enable DMA_DAC."]
pub struct APB_DAC_TRANS_R(crate::FieldReader<bool, bool>);
impl APB_DAC_TRANS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_DAC_TRANS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_DAC_TRANS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_DAC_TRANS` writer - Enable DMA_DAC."]
pub struct APB_DAC_TRANS_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_DAC_TRANS_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u32 & 1) << 14);
        self.w
    }
}
#[doc = "Field `DAC_RESET_FIFO` reader - Reset DIG DAC FIFO."]
pub struct DAC_RESET_FIFO_R(crate::FieldReader<bool, bool>);
impl DAC_RESET_FIFO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAC_RESET_FIFO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_RESET_FIFO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_RESET_FIFO` writer - Reset DIG DAC FIFO."]
pub struct DAC_RESET_FIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_RESET_FIFO_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "Field `APB_DAC_RST` reader - Reset DIG DAC by software."]
pub struct APB_DAC_RST_R(crate::FieldReader<bool, bool>);
impl APB_DAC_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_DAC_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_DAC_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_DAC_RST` writer - Reset DIG DAC by software."]
pub struct APB_DAC_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_DAC_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Set DAC timer target."]
    #[inline(always)]
    pub fn dac_timer_target(&self) -> DAC_TIMER_TARGET_R {
        DAC_TIMER_TARGET_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Enable read dac data."]
    #[inline(always)]
    pub fn dac_timer_en(&self) -> DAC_TIMER_EN_R {
        DAC_TIMER_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable DAC alter mode."]
    #[inline(always)]
    pub fn apb_dac_alter_mode(&self) -> APB_DAC_ALTER_MODE_R {
        APB_DAC_ALTER_MODE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable DMA_DAC."]
    #[inline(always)]
    pub fn apb_dac_trans(&self) -> APB_DAC_TRANS_R {
        APB_DAC_TRANS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reset DIG DAC FIFO."]
    #[inline(always)]
    pub fn dac_reset_fifo(&self) -> DAC_RESET_FIFO_R {
        DAC_RESET_FIFO_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset DIG DAC by software."]
    #[inline(always)]
    pub fn apb_dac_rst(&self) -> APB_DAC_RST_R {
        APB_DAC_RST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Set DAC timer target."]
    #[inline(always)]
    pub fn dac_timer_target(&mut self) -> DAC_TIMER_TARGET_W {
        DAC_TIMER_TARGET_W { w: self }
    }
    #[doc = "Bit 12 - Enable read dac data."]
    #[inline(always)]
    pub fn dac_timer_en(&mut self) -> DAC_TIMER_EN_W {
        DAC_TIMER_EN_W { w: self }
    }
    #[doc = "Bit 13 - Enable DAC alter mode."]
    #[inline(always)]
    pub fn apb_dac_alter_mode(&mut self) -> APB_DAC_ALTER_MODE_W {
        APB_DAC_ALTER_MODE_W { w: self }
    }
    #[doc = "Bit 14 - Enable DMA_DAC."]
    #[inline(always)]
    pub fn apb_dac_trans(&mut self) -> APB_DAC_TRANS_W {
        APB_DAC_TRANS_W { w: self }
    }
    #[doc = "Bit 15 - Reset DIG DAC FIFO."]
    #[inline(always)]
    pub fn dac_reset_fifo(&mut self) -> DAC_RESET_FIFO_W {
        DAC_RESET_FIFO_W { w: self }
    }
    #[doc = "Bit 16 - Reset DIG DAC by software."]
    #[inline(always)]
    pub fn apb_dac_rst(&mut self) -> APB_DAC_RST_W {
        APB_DAC_RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure DAC settings\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_dac_ctrl]
(index.html) module"]
pub struct APB_DAC_CTRL_SPEC;
impl crate::RegisterSpec for APB_DAC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_dac_ctrl::R]
(R) reader structure"]
impl crate::Readable for APB_DAC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_dac_ctrl::W]
(W) writer structure"]
impl crate::Writable for APB_DAC_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB_DAC_CTRL to value 0x2064"]
impl crate::Resettable for APB_DAC_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2064
    }
}
