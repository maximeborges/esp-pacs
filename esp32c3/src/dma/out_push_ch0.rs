#[doc = "Register `OUT_PUSH_CH0` reader"]
pub struct R(crate::R<OUT_PUSH_CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_PUSH_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_PUSH_CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_PUSH_CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_PUSH_CH0` writer"]
pub struct W(crate::W<OUT_PUSH_CH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_PUSH_CH0_SPEC>;
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
impl From<crate::W<OUT_PUSH_CH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_PUSH_CH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTFIFO_WDATA_CH0` reader - This register stores the data that need to be pushed into DMA FIFO."]
pub struct OUTFIFO_WDATA_CH0_R(crate::FieldReader<u16, u16>);
impl OUTFIFO_WDATA_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        OUTFIFO_WDATA_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTFIFO_WDATA_CH0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTFIFO_WDATA_CH0` writer - This register stores the data that need to be pushed into DMA FIFO."]
pub struct OUTFIFO_WDATA_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTFIFO_WDATA_CH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
#[doc = "Field `OUTFIFO_PUSH_CH0` reader - Set this bit to push data into DMA FIFO."]
pub struct OUTFIFO_PUSH_CH0_R(crate::FieldReader<bool, bool>);
impl OUTFIFO_PUSH_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTFIFO_PUSH_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTFIFO_PUSH_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTFIFO_PUSH_CH0` writer - Set this bit to push data into DMA FIFO."]
pub struct OUTFIFO_PUSH_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTFIFO_PUSH_CH0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - This register stores the data that need to be pushed into DMA FIFO."]
    #[inline(always)]
    pub fn outfifo_wdata_ch0(&self) -> OUTFIFO_WDATA_CH0_R {
        OUTFIFO_WDATA_CH0_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - Set this bit to push data into DMA FIFO."]
    #[inline(always)]
    pub fn outfifo_push_ch0(&self) -> OUTFIFO_PUSH_CH0_R {
        OUTFIFO_PUSH_CH0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - This register stores the data that need to be pushed into DMA FIFO."]
    #[inline(always)]
    pub fn outfifo_wdata_ch0(&mut self) -> OUTFIFO_WDATA_CH0_W {
        OUTFIFO_WDATA_CH0_W { w: self }
    }
    #[doc = "Bit 9 - Set this bit to push data into DMA FIFO."]
    #[inline(always)]
    pub fn outfifo_push_ch0(&mut self) -> OUTFIFO_PUSH_CH0_W {
        OUTFIFO_PUSH_CH0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_OUT_PUSH_CH0_REG.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_push_ch0]
(index.html) module"]
pub struct OUT_PUSH_CH0_SPEC;
impl crate::RegisterSpec for OUT_PUSH_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_push_ch0::R]
(R) reader structure"]
impl crate::Readable for OUT_PUSH_CH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_push_ch0::W]
(W) writer structure"]
impl crate::Writable for OUT_PUSH_CH0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT_PUSH_CH0 to value 0"]
impl crate::Resettable for OUT_PUSH_CH0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
