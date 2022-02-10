#[doc = "Register `IN_CONF1_CH%s` reader"]
pub struct R(crate::R<IN_CONF1_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_CONF1_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_CONF1_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_CONF1_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN_CONF1_CH%s` writer"]
pub struct W(crate::W<IN_CONF1_CH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN_CONF1_CH_SPEC>;
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
impl From<crate::W<IN_CONF1_CH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN_CONF1_CH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_INFIFO_FULL_THRS_CH` reader - This register is used to generate the INFIFO_FULL_WM_INT interrupt when Rx channel 0 received byte number in Rx FIFO is up to the value of the register."]
pub struct DMA_INFIFO_FULL_THRS_CH_R(crate::FieldReader<u16, u16>);
impl DMA_INFIFO_FULL_THRS_CH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DMA_INFIFO_FULL_THRS_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_INFIFO_FULL_THRS_CH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_INFIFO_FULL_THRS_CH` writer - This register is used to generate the INFIFO_FULL_WM_INT interrupt when Rx channel 0 received byte number in Rx FIFO is up to the value of the register."]
pub struct DMA_INFIFO_FULL_THRS_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_INFIFO_FULL_THRS_CH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `IN_CHECK_OWNER_CH` reader - Set this bit to enable checking the owner attribute of the link descriptor."]
pub struct IN_CHECK_OWNER_CH_R(crate::FieldReader<bool, bool>);
impl IN_CHECK_OWNER_CH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_CHECK_OWNER_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_CHECK_OWNER_CH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_CHECK_OWNER_CH` writer - Set this bit to enable checking the owner attribute of the link descriptor."]
pub struct IN_CHECK_OWNER_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_CHECK_OWNER_CH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `IN_EXT_MEM_BK_SIZE_CH` reader - Block size of Rx channel 0 when DMA access external SRAM. 0: 16 bytes 1: 32 bytes 2/3:reserved"]
pub struct IN_EXT_MEM_BK_SIZE_CH_R(crate::FieldReader<u8, u8>);
impl IN_EXT_MEM_BK_SIZE_CH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IN_EXT_MEM_BK_SIZE_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_EXT_MEM_BK_SIZE_CH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_EXT_MEM_BK_SIZE_CH` writer - Block size of Rx channel 0 when DMA access external SRAM. 0: 16 bytes 1: 32 bytes 2/3:reserved"]
pub struct IN_EXT_MEM_BK_SIZE_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_EXT_MEM_BK_SIZE_CH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - This register is used to generate the INFIFO_FULL_WM_INT interrupt when Rx channel 0 received byte number in Rx FIFO is up to the value of the register."]
    #[inline(always)]
    pub fn dma_infifo_full_thrs_ch(&self) -> DMA_INFIFO_FULL_THRS_CH_R {
        DMA_INFIFO_FULL_THRS_CH_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn in_check_owner_ch(&self) -> IN_CHECK_OWNER_CH_R {
        IN_CHECK_OWNER_CH_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - Block size of Rx channel 0 when DMA access external SRAM. 0: 16 bytes 1: 32 bytes 2/3:reserved"]
    #[inline(always)]
    pub fn in_ext_mem_bk_size_ch(&self) -> IN_EXT_MEM_BK_SIZE_CH_R {
        IN_EXT_MEM_BK_SIZE_CH_R::new(((self.bits >> 13) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - This register is used to generate the INFIFO_FULL_WM_INT interrupt when Rx channel 0 received byte number in Rx FIFO is up to the value of the register."]
    #[inline(always)]
    pub fn dma_infifo_full_thrs_ch(&mut self) -> DMA_INFIFO_FULL_THRS_CH_W {
        DMA_INFIFO_FULL_THRS_CH_W { w: self }
    }
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn in_check_owner_ch(&mut self) -> IN_CHECK_OWNER_CH_W {
        IN_CHECK_OWNER_CH_W { w: self }
    }
    #[doc = "Bits 13:14 - Block size of Rx channel 0 when DMA access external SRAM. 0: 16 bytes 1: 32 bytes 2/3:reserved"]
    #[inline(always)]
    pub fn in_ext_mem_bk_size_ch(&mut self) -> IN_EXT_MEM_BK_SIZE_CH_W {
        IN_EXT_MEM_BK_SIZE_CH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure 1 register of Rx channel 0\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_conf1_ch]
(index.html) module"]
pub struct IN_CONF1_CH_SPEC;
impl crate::RegisterSpec for IN_CONF1_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_conf1_ch::R]
(R) reader structure"]
impl crate::Readable for IN_CONF1_CH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in_conf1_ch::W]
(W) writer structure"]
impl crate::Writable for IN_CONF1_CH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IN_CONF1_CH%s to value 0x0c"]
impl crate::Resettable for IN_CONF1_CH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c
    }
}
