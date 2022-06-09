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
pub type OUT_CHECK_OWNER_CH_R = crate::BitReader<bool>;
#[doc = "Field `OUT_CHECK_OWNER_CH` writer - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type OUT_CHECK_OWNER_CH_W<'a> = crate::BitWriter<'a, u32, OUT_CONF1_CH_SPEC, bool, 12>;
#[doc = "Field `OUT_EXT_MEM_BK_SIZE_CH` reader - Block size of Tx channel 0 when DMA access external SRAM. 0: 16 bytes 1: 32 bytes 2/3:reserved"]
pub type OUT_EXT_MEM_BK_SIZE_CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUT_EXT_MEM_BK_SIZE_CH` writer - Block size of Tx channel 0 when DMA access external SRAM. 0: 16 bytes 1: 32 bytes 2/3:reserved"]
pub type OUT_EXT_MEM_BK_SIZE_CH_W<'a> =
    crate::FieldWriter<'a, u32, OUT_CONF1_CH_SPEC, u8, u8, 2, 13>;
impl R {
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn out_check_owner_ch(&self) -> OUT_CHECK_OWNER_CH_R {
        OUT_CHECK_OWNER_CH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Block size of Tx channel 0 when DMA access external SRAM. 0: 16 bytes 1: 32 bytes 2/3:reserved"]
    #[inline(always)]
    pub fn out_ext_mem_bk_size_ch(&self) -> OUT_EXT_MEM_BK_SIZE_CH_R {
        OUT_EXT_MEM_BK_SIZE_CH_R::new(((self.bits >> 13) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn out_check_owner_ch(&mut self) -> OUT_CHECK_OWNER_CH_W {
        OUT_CHECK_OWNER_CH_W::new(self)
    }
    #[doc = "Bits 13:14 - Block size of Tx channel 0 when DMA access external SRAM. 0: 16 bytes 1: 32 bytes 2/3:reserved"]
    #[inline(always)]
    pub fn out_ext_mem_bk_size_ch(&mut self) -> OUT_EXT_MEM_BK_SIZE_CH_W {
        OUT_EXT_MEM_BK_SIZE_CH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure 1 register of Tx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_conf1_ch](index.html) module"]
pub struct OUT_CONF1_CH_SPEC;
impl crate::RegisterSpec for OUT_CONF1_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_conf1_ch::R](R) reader structure"]
impl crate::Readable for OUT_CONF1_CH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_conf1_ch::W](W) writer structure"]
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
