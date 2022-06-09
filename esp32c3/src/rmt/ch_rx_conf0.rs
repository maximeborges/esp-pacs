#[doc = "Register `CH%s_RX_CONF0` reader"]
pub struct R(crate::R<CH_RX_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_RX_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_RX_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_RX_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%s_RX_CONF0` writer"]
pub struct W(crate::W<CH_RX_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_RX_CONF0_SPEC>;
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
impl From<crate::W<CH_RX_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_RX_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV_CNT` reader - reg_div_cnt_ch2."]
pub type DIV_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV_CNT` writer - reg_div_cnt_ch2."]
pub type DIV_CNT_W<'a> = crate::FieldWriter<'a, u32, CH_RX_CONF0_SPEC, u8, u8, 8, 0>;
#[doc = "Field `IDLE_THRES` reader - reg_idle_thres_ch2."]
pub type IDLE_THRES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IDLE_THRES` writer - reg_idle_thres_ch2."]
pub type IDLE_THRES_W<'a> = crate::FieldWriter<'a, u32, CH_RX_CONF0_SPEC, u16, u16, 15, 8>;
#[doc = "Field `MEM_SIZE` reader - reg_mem_size_ch2."]
pub type MEM_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEM_SIZE` writer - reg_mem_size_ch2."]
pub type MEM_SIZE_W<'a> = crate::FieldWriter<'a, u32, CH_RX_CONF0_SPEC, u8, u8, 3, 23>;
#[doc = "Field `CARRIER_EN` reader - reg_carrier_en_ch2."]
pub type CARRIER_EN_R = crate::BitReader<bool>;
#[doc = "Field `CARRIER_EN` writer - reg_carrier_en_ch2."]
pub type CARRIER_EN_W<'a> = crate::BitWriter<'a, u32, CH_RX_CONF0_SPEC, bool, 28>;
#[doc = "Field `CARRIER_OUT_LV` reader - reg_carrier_out_lv_ch2."]
pub type CARRIER_OUT_LV_R = crate::BitReader<bool>;
#[doc = "Field `CARRIER_OUT_LV` writer - reg_carrier_out_lv_ch2."]
pub type CARRIER_OUT_LV_W<'a> = crate::BitWriter<'a, u32, CH_RX_CONF0_SPEC, bool, 29>;
impl R {
    #[doc = "Bits 0:7 - reg_div_cnt_ch2."]
    #[inline(always)]
    pub fn div_cnt(&self) -> DIV_CNT_R {
        DIV_CNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:22 - reg_idle_thres_ch2."]
    #[inline(always)]
    pub fn idle_thres(&self) -> IDLE_THRES_R {
        IDLE_THRES_R::new(((self.bits >> 8) & 0x7fff) as u16)
    }
    #[doc = "Bits 23:25 - reg_mem_size_ch2."]
    #[inline(always)]
    pub fn mem_size(&self) -> MEM_SIZE_R {
        MEM_SIZE_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 28 - reg_carrier_en_ch2."]
    #[inline(always)]
    pub fn carrier_en(&self) -> CARRIER_EN_R {
        CARRIER_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - reg_carrier_out_lv_ch2."]
    #[inline(always)]
    pub fn carrier_out_lv(&self) -> CARRIER_OUT_LV_R {
        CARRIER_OUT_LV_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - reg_div_cnt_ch2."]
    #[inline(always)]
    pub fn div_cnt(&mut self) -> DIV_CNT_W {
        DIV_CNT_W::new(self)
    }
    #[doc = "Bits 8:22 - reg_idle_thres_ch2."]
    #[inline(always)]
    pub fn idle_thres(&mut self) -> IDLE_THRES_W {
        IDLE_THRES_W::new(self)
    }
    #[doc = "Bits 23:25 - reg_mem_size_ch2."]
    #[inline(always)]
    pub fn mem_size(&mut self) -> MEM_SIZE_W {
        MEM_SIZE_W::new(self)
    }
    #[doc = "Bit 28 - reg_carrier_en_ch2."]
    #[inline(always)]
    pub fn carrier_en(&mut self) -> CARRIER_EN_W {
        CARRIER_EN_W::new(self)
    }
    #[doc = "Bit 29 - reg_carrier_out_lv_ch2."]
    #[inline(always)]
    pub fn carrier_out_lv(&mut self) -> CARRIER_OUT_LV_W {
        CARRIER_OUT_LV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RMT_CH2CONF0_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_rx_conf0](index.html) module"]
pub struct CH_RX_CONF0_SPEC;
impl crate::RegisterSpec for CH_RX_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_rx_conf0::R](R) reader structure"]
impl crate::Readable for CH_RX_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_rx_conf0::W](W) writer structure"]
impl crate::Writable for CH_RX_CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH%s_RX_CONF0 to value 0x30ff_ff02"]
impl crate::Resettable for CH_RX_CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x30ff_ff02
    }
}
