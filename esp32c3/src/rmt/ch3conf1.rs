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
#[doc = "Field `RX_EN` reader - reg_rx_en_ch3."]
pub type RX_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_EN` writer - reg_rx_en_ch3."]
pub type RX_EN_W<'a> = crate::BitWriter<'a, u32, CH3CONF1_SPEC, bool, 0>;
#[doc = "Field `MEM_WR_RST` writer - reg_mem_wr_rst_ch3."]
pub type MEM_WR_RST_W<'a> = crate::BitWriter<'a, u32, CH3CONF1_SPEC, bool, 1>;
#[doc = "Field `APB_MEM_RST` writer - reg_apb_mem_rst_ch3."]
pub type APB_MEM_RST_W<'a> = crate::BitWriter<'a, u32, CH3CONF1_SPEC, bool, 2>;
#[doc = "Field `MEM_OWNER` reader - reg_mem_owner_ch3."]
pub type MEM_OWNER_R = crate::BitReader<bool>;
#[doc = "Field `MEM_OWNER` writer - reg_mem_owner_ch3."]
pub type MEM_OWNER_W<'a> = crate::BitWriter<'a, u32, CH3CONF1_SPEC, bool, 3>;
#[doc = "Field `RX_FILTER_EN` reader - reg_rx_filter_en_ch3."]
pub type RX_FILTER_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_FILTER_EN` writer - reg_rx_filter_en_ch3."]
pub type RX_FILTER_EN_W<'a> = crate::BitWriter<'a, u32, CH3CONF1_SPEC, bool, 4>;
#[doc = "Field `RX_FILTER_THRES` reader - reg_rx_filter_thres_ch3."]
pub type RX_FILTER_THRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_FILTER_THRES` writer - reg_rx_filter_thres_ch3."]
pub type RX_FILTER_THRES_W<'a> = crate::FieldWriter<'a, u32, CH3CONF1_SPEC, u8, u8, 8, 5>;
#[doc = "Field `MEM_RX_WRAP_EN` reader - reg_mem_rx_wrap_en_ch3."]
pub type MEM_RX_WRAP_EN_R = crate::BitReader<bool>;
#[doc = "Field `MEM_RX_WRAP_EN` writer - reg_mem_rx_wrap_en_ch3."]
pub type MEM_RX_WRAP_EN_W<'a> = crate::BitWriter<'a, u32, CH3CONF1_SPEC, bool, 13>;
#[doc = "Field `AFIFO_RST` writer - reg_afifo_rst_ch3."]
pub type AFIFO_RST_W<'a> = crate::BitWriter<'a, u32, CH3CONF1_SPEC, bool, 14>;
#[doc = "Field `CONF_UPDATE` writer - reg_conf_update_ch3."]
pub type CONF_UPDATE_W<'a> = crate::BitWriter<'a, u32, CH3CONF1_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 0 - reg_rx_en_ch3."]
    #[inline(always)]
    pub fn rx_en(&self) -> RX_EN_R {
        RX_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - reg_mem_owner_ch3."]
    #[inline(always)]
    pub fn mem_owner(&self) -> MEM_OWNER_R {
        MEM_OWNER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reg_rx_filter_en_ch3."]
    #[inline(always)]
    pub fn rx_filter_en(&self) -> RX_FILTER_EN_R {
        RX_FILTER_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:12 - reg_rx_filter_thres_ch3."]
    #[inline(always)]
    pub fn rx_filter_thres(&self) -> RX_FILTER_THRES_R {
        RX_FILTER_THRES_R::new(((self.bits >> 5) & 0xff) as u8)
    }
    #[doc = "Bit 13 - reg_mem_rx_wrap_en_ch3."]
    #[inline(always)]
    pub fn mem_rx_wrap_en(&self) -> MEM_RX_WRAP_EN_R {
        MEM_RX_WRAP_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - reg_rx_en_ch3."]
    #[inline(always)]
    pub fn rx_en(&mut self) -> RX_EN_W {
        RX_EN_W::new(self)
    }
    #[doc = "Bit 1 - reg_mem_wr_rst_ch3."]
    #[inline(always)]
    pub fn mem_wr_rst(&mut self) -> MEM_WR_RST_W {
        MEM_WR_RST_W::new(self)
    }
    #[doc = "Bit 2 - reg_apb_mem_rst_ch3."]
    #[inline(always)]
    pub fn apb_mem_rst(&mut self) -> APB_MEM_RST_W {
        APB_MEM_RST_W::new(self)
    }
    #[doc = "Bit 3 - reg_mem_owner_ch3."]
    #[inline(always)]
    pub fn mem_owner(&mut self) -> MEM_OWNER_W {
        MEM_OWNER_W::new(self)
    }
    #[doc = "Bit 4 - reg_rx_filter_en_ch3."]
    #[inline(always)]
    pub fn rx_filter_en(&mut self) -> RX_FILTER_EN_W {
        RX_FILTER_EN_W::new(self)
    }
    #[doc = "Bits 5:12 - reg_rx_filter_thres_ch3."]
    #[inline(always)]
    pub fn rx_filter_thres(&mut self) -> RX_FILTER_THRES_W {
        RX_FILTER_THRES_W::new(self)
    }
    #[doc = "Bit 13 - reg_mem_rx_wrap_en_ch3."]
    #[inline(always)]
    pub fn mem_rx_wrap_en(&mut self) -> MEM_RX_WRAP_EN_W {
        MEM_RX_WRAP_EN_W::new(self)
    }
    #[doc = "Bit 14 - reg_afifo_rst_ch3."]
    #[inline(always)]
    pub fn afifo_rst(&mut self) -> AFIFO_RST_W {
        AFIFO_RST_W::new(self)
    }
    #[doc = "Bit 15 - reg_conf_update_ch3."]
    #[inline(always)]
    pub fn conf_update(&mut self) -> CONF_UPDATE_W {
        CONF_UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RMT_CH3CONF1_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3conf1](index.html) module"]
pub struct CH3CONF1_SPEC;
impl crate::RegisterSpec for CH3CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch3conf1::R](R) reader structure"]
impl crate::Readable for CH3CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch3conf1::W](W) writer structure"]
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
