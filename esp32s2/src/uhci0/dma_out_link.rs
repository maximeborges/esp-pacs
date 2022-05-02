#[doc = "Register `DMA_OUT_LINK` reader"]
pub struct R(crate::R<DMA_OUT_LINK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_OUT_LINK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_OUT_LINK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_OUT_LINK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_OUT_LINK` writer"]
pub struct W(crate::W<DMA_OUT_LINK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_OUT_LINK_SPEC>;
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
impl From<crate::W<DMA_OUT_LINK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_OUT_LINK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTLINK_ADDR` reader - This register is used to specify the least significant 20 bits of the first transmit descriptor's address."]
pub struct OUTLINK_ADDR_R(crate::FieldReader<u32>);
impl OUTLINK_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        OUTLINK_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTLINK_ADDR_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTLINK_ADDR` writer - This register is used to specify the least significant 20 bits of the first transmit descriptor's address."]
pub struct OUTLINK_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTLINK_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
#[doc = "Field `OUTLINK_STOP` reader - Set this bit to stop dealing with the transmit descriptor."]
pub struct OUTLINK_STOP_R(crate::FieldReader<bool>);
impl OUTLINK_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTLINK_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTLINK_STOP_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTLINK_STOP` writer - Set this bit to stop dealing with the transmit descriptor."]
pub struct OUTLINK_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTLINK_STOP_W<'a> {
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
#[doc = "Field `OUTLINK_START` reader - Set this bit to start a new transmit descriptor."]
pub struct OUTLINK_START_R(crate::FieldReader<bool>);
impl OUTLINK_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTLINK_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTLINK_START_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTLINK_START` writer - Set this bit to start a new transmit descriptor."]
pub struct OUTLINK_START_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTLINK_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 29)) | ((value as u32 & 1) << 29);
        self.w
    }
}
#[doc = "Field `OUTLINK_RESTART` reader - Set this bit to restart the transmit descriptor from the last address."]
pub struct OUTLINK_RESTART_R(crate::FieldReader<bool>);
impl OUTLINK_RESTART_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTLINK_RESTART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTLINK_RESTART_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTLINK_RESTART` writer - Set this bit to restart the transmit descriptor from the last address."]
pub struct OUTLINK_RESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTLINK_RESTART_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "Field `OUTLINK_PARK` reader - 1: the transmit descriptor's FSM is in idle state. 0: the transmit descriptor's FSM is working."]
pub struct OUTLINK_PARK_R(crate::FieldReader<bool>);
impl OUTLINK_PARK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTLINK_PARK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTLINK_PARK_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:19 - This register is used to specify the least significant 20 bits of the first transmit descriptor's address."]
    #[inline(always)]
    pub fn outlink_addr(&self) -> OUTLINK_ADDR_R {
        OUTLINK_ADDR_R::new((self.bits & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 28 - Set this bit to stop dealing with the transmit descriptor."]
    #[inline(always)]
    pub fn outlink_stop(&self) -> OUTLINK_STOP_R {
        OUTLINK_STOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Set this bit to start a new transmit descriptor."]
    #[inline(always)]
    pub fn outlink_start(&self) -> OUTLINK_START_R {
        OUTLINK_START_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Set this bit to restart the transmit descriptor from the last address."]
    #[inline(always)]
    pub fn outlink_restart(&self) -> OUTLINK_RESTART_R {
        OUTLINK_RESTART_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: the transmit descriptor's FSM is in idle state. 0: the transmit descriptor's FSM is working."]
    #[inline(always)]
    pub fn outlink_park(&self) -> OUTLINK_PARK_R {
        OUTLINK_PARK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - This register is used to specify the least significant 20 bits of the first transmit descriptor's address."]
    #[inline(always)]
    pub fn outlink_addr(&mut self) -> OUTLINK_ADDR_W {
        OUTLINK_ADDR_W { w: self }
    }
    #[doc = "Bit 28 - Set this bit to stop dealing with the transmit descriptor."]
    #[inline(always)]
    pub fn outlink_stop(&mut self) -> OUTLINK_STOP_W {
        OUTLINK_STOP_W { w: self }
    }
    #[doc = "Bit 29 - Set this bit to start a new transmit descriptor."]
    #[inline(always)]
    pub fn outlink_start(&mut self) -> OUTLINK_START_W {
        OUTLINK_START_W { w: self }
    }
    #[doc = "Bit 30 - Set this bit to restart the transmit descriptor from the last address."]
    #[inline(always)]
    pub fn outlink_restart(&mut self) -> OUTLINK_RESTART_W {
        OUTLINK_RESTART_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Link descriptor address and control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_link](index.html) module"]
pub struct DMA_OUT_LINK_SPEC;
impl crate::RegisterSpec for DMA_OUT_LINK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_out_link::R](R) reader structure"]
impl crate::Readable for DMA_OUT_LINK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_out_link::W](W) writer structure"]
impl crate::Writable for DMA_OUT_LINK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_OUT_LINK to value 0"]
impl crate::Resettable for DMA_OUT_LINK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
