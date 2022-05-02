#[doc = "Register `HOLD` reader"]
pub struct R(crate::R<HOLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOLD` writer"]
pub struct W(crate::W<HOLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOLD_SPEC>;
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
impl From<crate::W<HOLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_HOLD_ENA` reader - This register is for two SPI masters to share the same cs clock and data signals. The bits of one SPI are set, if the other SPI is busy, the SPI will be hold. 1(3): hold at idle phase 2: hold at prepare phase. Can be configured in CONF state."]
pub struct INT_HOLD_ENA_R(crate::FieldReader<u8>);
impl INT_HOLD_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INT_HOLD_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_HOLD_ENA_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_HOLD_ENA` writer - This register is for two SPI masters to share the same cs clock and data signals. The bits of one SPI are set, if the other SPI is busy, the SPI will be hold. 1(3): hold at idle phase 2: hold at prepare phase. Can be configured in CONF state."]
pub struct INT_HOLD_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_HOLD_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `VAL` reader - spi hold output value, which should be used with SPI_HOLD_OUT_EN. Can be configured in CONF state."]
pub struct VAL_R(crate::FieldReader<bool>);
impl VAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL` writer - spi hold output value, which should be used with SPI_HOLD_OUT_EN. Can be configured in CONF state."]
pub struct VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL_W<'a> {
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
#[doc = "Field `OUT_EN` reader - Enable set spi output hold value to spi_hold_reg. It can be used to hold spi state machine with SPI_EXT_HOLD_EN and other usr hold signals. Can be configured in CONF state."]
pub struct OUT_EN_R(crate::FieldReader<bool>);
impl OUT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_EN` writer - Enable set spi output hold value to spi_hold_reg. It can be used to hold spi state machine with SPI_EXT_HOLD_EN and other usr hold signals. Can be configured in CONF state."]
pub struct OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_EN_W<'a> {
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
#[doc = "Field `OUT_TIME` reader - set the hold cycles of output spi_hold signal when SPI_HOLD_OUT_EN is enable. Can be configured in CONF state."]
pub struct OUT_TIME_R(crate::FieldReader<u8>);
impl OUT_TIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OUT_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_TIME_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_TIME` writer - set the hold cycles of output spi_hold signal when SPI_HOLD_OUT_EN is enable. Can be configured in CONF state."]
pub struct OUT_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 4)) | ((value as u32 & 7) << 4);
        self.w
    }
}
#[doc = "Field `DMA_SEG_TRANS_DONE` reader - 1: spi master DMA full-duplex/half-duplex seg-trans ends or slave half-duplex seg-trans ends. And data has been pushed to corresponding memory. 0: seg-trans is not ended or not occurred. Can not be changed by CONF_buf."]
pub struct DMA_SEG_TRANS_DONE_R(crate::FieldReader<bool>);
impl DMA_SEG_TRANS_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_SEG_TRANS_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_SEG_TRANS_DONE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_SEG_TRANS_DONE` writer - 1: spi master DMA full-duplex/half-duplex seg-trans ends or slave half-duplex seg-trans ends. And data has been pushed to corresponding memory. 0: seg-trans is not ended or not occurred. Can not be changed by CONF_buf."]
pub struct DMA_SEG_TRANS_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SEG_TRANS_DONE_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - This register is for two SPI masters to share the same cs clock and data signals. The bits of one SPI are set, if the other SPI is busy, the SPI will be hold. 1(3): hold at idle phase 2: hold at prepare phase. Can be configured in CONF state."]
    #[inline(always)]
    pub fn int_hold_ena(&self) -> INT_HOLD_ENA_R {
        INT_HOLD_ENA_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - spi hold output value, which should be used with SPI_HOLD_OUT_EN. Can be configured in CONF state."]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable set spi output hold value to spi_hold_reg. It can be used to hold spi state machine with SPI_EXT_HOLD_EN and other usr hold signals. Can be configured in CONF state."]
    #[inline(always)]
    pub fn out_en(&self) -> OUT_EN_R {
        OUT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - set the hold cycles of output spi_hold signal when SPI_HOLD_OUT_EN is enable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn out_time(&self) -> OUT_TIME_R {
        OUT_TIME_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - 1: spi master DMA full-duplex/half-duplex seg-trans ends or slave half-duplex seg-trans ends. And data has been pushed to corresponding memory. 0: seg-trans is not ended or not occurred. Can not be changed by CONF_buf."]
    #[inline(always)]
    pub fn dma_seg_trans_done(&self) -> DMA_SEG_TRANS_DONE_R {
        DMA_SEG_TRANS_DONE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - This register is for two SPI masters to share the same cs clock and data signals. The bits of one SPI are set, if the other SPI is busy, the SPI will be hold. 1(3): hold at idle phase 2: hold at prepare phase. Can be configured in CONF state."]
    #[inline(always)]
    pub fn int_hold_ena(&mut self) -> INT_HOLD_ENA_W {
        INT_HOLD_ENA_W { w: self }
    }
    #[doc = "Bit 2 - spi hold output value, which should be used with SPI_HOLD_OUT_EN. Can be configured in CONF state."]
    #[inline(always)]
    pub fn val(&mut self) -> VAL_W {
        VAL_W { w: self }
    }
    #[doc = "Bit 3 - Enable set spi output hold value to spi_hold_reg. It can be used to hold spi state machine with SPI_EXT_HOLD_EN and other usr hold signals. Can be configured in CONF state."]
    #[inline(always)]
    pub fn out_en(&mut self) -> OUT_EN_W {
        OUT_EN_W { w: self }
    }
    #[doc = "Bits 4:6 - set the hold cycles of output spi_hold signal when SPI_HOLD_OUT_EN is enable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn out_time(&mut self) -> OUT_TIME_W {
        OUT_TIME_W { w: self }
    }
    #[doc = "Bit 7 - 1: spi master DMA full-duplex/half-duplex seg-trans ends or slave half-duplex seg-trans ends. And data has been pushed to corresponding memory. 0: seg-trans is not ended or not occurred. Can not be changed by CONF_buf."]
    #[inline(always)]
    pub fn dma_seg_trans_done(&mut self) -> DMA_SEG_TRANS_DONE_W {
        DMA_SEG_TRANS_DONE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI hold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hold](index.html) module"]
pub struct HOLD_SPEC;
impl crate::RegisterSpec for HOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hold::R](R) reader structure"]
impl crate::Readable for HOLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hold::W](W) writer structure"]
impl crate::Writable for HOLD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOLD to value 0"]
impl crate::Resettable for HOLD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
