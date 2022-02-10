#[doc = "Register `OUT_CONF1_CH%s` reader"]
pub struct R(crate::R<OUT_CONF1_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_CONF1_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_CONF1_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_CONF1_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_CONF1_CH%s` writer"]
pub struct W(crate::W<OUT_CONF1_CH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_CONF1_CH_SPEC>;
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
impl From<crate::W<OUT_CONF1_CH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_CONF1_CH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT_CHECK_OWNER_CH` reader - Set this bit to enable checking the owner attribute of the link descriptor."]
pub struct OUT_CHECK_OWNER_CH_R(crate::FieldReader<bool, bool>);
impl OUT_CHECK_OWNER_CH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_CHECK_OWNER_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_CHECK_OWNER_CH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_CHECK_OWNER_CH` writer - Set this bit to enable checking the owner attribute of the link descriptor."]
pub struct OUT_CHECK_OWNER_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_CHECK_OWNER_CH_W<'a> {
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
#[doc = "Field `OUT_EXT_MEM_BK_SIZE_CH` reader - Block size of Tx channel 0 when DMA access external SRAM. 0: 16 bytes 1: 32 bytes 2/3:reserved"]
pub struct OUT_EXT_MEM_BK_SIZE_CH_R(crate::FieldReader<u8, u8>);
impl OUT_EXT_MEM_BK_SIZE_CH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OUT_EXT_MEM_BK_SIZE_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_EXT_MEM_BK_SIZE_CH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_EXT_MEM_BK_SIZE_CH` writer - Block size of Tx channel 0 when DMA access external SRAM. 0: 16 bytes 1: 32 bytes 2/3:reserved"]
pub struct OUT_EXT_MEM_BK_SIZE_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_EXT_MEM_BK_SIZE_CH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn out_check_owner_ch(&self) -> OUT_CHECK_OWNER_CH_R {
        OUT_CHECK_OWNER_CH_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - Block size of Tx channel 0 when DMA access external SRAM. 0: 16 bytes 1: 32 bytes 2/3:reserved"]
    #[inline(always)]
    pub fn out_ext_mem_bk_size_ch(&self) -> OUT_EXT_MEM_BK_SIZE_CH_R {
        OUT_EXT_MEM_BK_SIZE_CH_R::new(((self.bits >> 13) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn out_check_owner_ch(&mut self) -> OUT_CHECK_OWNER_CH_W {
        OUT_CHECK_OWNER_CH_W { w: self }
    }
    #[doc = "Bits 13:14 - Block size of Tx channel 0 when DMA access external SRAM. 0: 16 bytes 1: 32 bytes 2/3:reserved"]
    #[inline(always)]
    pub fn out_ext_mem_bk_size_ch(&mut self) -> OUT_EXT_MEM_BK_SIZE_CH_W {
        OUT_EXT_MEM_BK_SIZE_CH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure 1 register of Tx channel 0\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_conf1_ch]
(index.html) module"]
pub struct OUT_CONF1_CH_SPEC;
impl crate::RegisterSpec for OUT_CONF1_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_conf1_ch::R]
(R) reader structure"]
impl crate::Readable for OUT_CONF1_CH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_conf1_ch::W]
(W) writer structure"]
impl crate::Writable for OUT_CONF1_CH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT_CONF1_CH%s to value 0"]
impl crate::Resettable for OUT_CONF1_CH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
