#[doc = "Register `LSCH1_CONF0` reader"]
pub struct R(crate::R<LSCH1_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSCH1_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSCH1_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSCH1_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LSCH1_CONF0` writer"]
pub struct W(crate::W<LSCH1_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LSCH1_CONF0_SPEC>;
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
impl From<crate::W<LSCH1_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LSCH1_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_SEL_LSCH1` reader - reg_timer_sel_lsch1."]
pub type TIMER_SEL_LSCH1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMER_SEL_LSCH1` writer - reg_timer_sel_lsch1."]
pub type TIMER_SEL_LSCH1_W<'a> = crate::FieldWriter<'a, u32, LSCH1_CONF0_SPEC, u8, u8, 2, 0>;
#[doc = "Field `SIG_OUT_EN_LSCH1` reader - reg_sig_out_en_lsch1."]
pub type SIG_OUT_EN_LSCH1_R = crate::BitReader<bool>;
#[doc = "Field `SIG_OUT_EN_LSCH1` writer - reg_sig_out_en_lsch1."]
pub type SIG_OUT_EN_LSCH1_W<'a> = crate::BitWriter<'a, u32, LSCH1_CONF0_SPEC, bool, 2>;
#[doc = "Field `IDLE_LV_LSCH1` reader - reg_idle_lv_lsch1."]
pub type IDLE_LV_LSCH1_R = crate::BitReader<bool>;
#[doc = "Field `IDLE_LV_LSCH1` writer - reg_idle_lv_lsch1."]
pub type IDLE_LV_LSCH1_W<'a> = crate::BitWriter<'a, u32, LSCH1_CONF0_SPEC, bool, 3>;
#[doc = "Field `PARA_UP_LSCH1` writer - reg_para_up_lsch1."]
pub type PARA_UP_LSCH1_W<'a> = crate::BitWriter<'a, u32, LSCH1_CONF0_SPEC, bool, 4>;
#[doc = "Field `OVF_NUM_LSCH1` reader - reg_ovf_num_lsch1."]
pub type OVF_NUM_LSCH1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OVF_NUM_LSCH1` writer - reg_ovf_num_lsch1."]
pub type OVF_NUM_LSCH1_W<'a> = crate::FieldWriter<'a, u32, LSCH1_CONF0_SPEC, u16, u16, 10, 5>;
#[doc = "Field `OVF_CNT_EN_LSCH1` reader - reg_ovf_cnt_en_lsch1."]
pub type OVF_CNT_EN_LSCH1_R = crate::BitReader<bool>;
#[doc = "Field `OVF_CNT_EN_LSCH1` writer - reg_ovf_cnt_en_lsch1."]
pub type OVF_CNT_EN_LSCH1_W<'a> = crate::BitWriter<'a, u32, LSCH1_CONF0_SPEC, bool, 15>;
#[doc = "Field `OVF_CNT_RESET_LSCH1` writer - reg_ovf_cnt_reset_lsch1."]
pub type OVF_CNT_RESET_LSCH1_W<'a> = crate::BitWriter<'a, u32, LSCH1_CONF0_SPEC, bool, 16>;
impl R {
    #[doc = "Bits 0:1 - reg_timer_sel_lsch1."]
    #[inline(always)]
    pub fn timer_sel_lsch1(&self) -> TIMER_SEL_LSCH1_R {
        TIMER_SEL_LSCH1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - reg_sig_out_en_lsch1."]
    #[inline(always)]
    pub fn sig_out_en_lsch1(&self) -> SIG_OUT_EN_LSCH1_R {
        SIG_OUT_EN_LSCH1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reg_idle_lv_lsch1."]
    #[inline(always)]
    pub fn idle_lv_lsch1(&self) -> IDLE_LV_LSCH1_R {
        IDLE_LV_LSCH1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 5:14 - reg_ovf_num_lsch1."]
    #[inline(always)]
    pub fn ovf_num_lsch1(&self) -> OVF_NUM_LSCH1_R {
        OVF_NUM_LSCH1_R::new(((self.bits >> 5) & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - reg_ovf_cnt_en_lsch1."]
    #[inline(always)]
    pub fn ovf_cnt_en_lsch1(&self) -> OVF_CNT_EN_LSCH1_R {
        OVF_CNT_EN_LSCH1_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - reg_timer_sel_lsch1."]
    #[inline(always)]
    pub fn timer_sel_lsch1(&mut self) -> TIMER_SEL_LSCH1_W {
        TIMER_SEL_LSCH1_W::new(self)
    }
    #[doc = "Bit 2 - reg_sig_out_en_lsch1."]
    #[inline(always)]
    pub fn sig_out_en_lsch1(&mut self) -> SIG_OUT_EN_LSCH1_W {
        SIG_OUT_EN_LSCH1_W::new(self)
    }
    #[doc = "Bit 3 - reg_idle_lv_lsch1."]
    #[inline(always)]
    pub fn idle_lv_lsch1(&mut self) -> IDLE_LV_LSCH1_W {
        IDLE_LV_LSCH1_W::new(self)
    }
    #[doc = "Bit 4 - reg_para_up_lsch1."]
    #[inline(always)]
    pub fn para_up_lsch1(&mut self) -> PARA_UP_LSCH1_W {
        PARA_UP_LSCH1_W::new(self)
    }
    #[doc = "Bits 5:14 - reg_ovf_num_lsch1."]
    #[inline(always)]
    pub fn ovf_num_lsch1(&mut self) -> OVF_NUM_LSCH1_W {
        OVF_NUM_LSCH1_W::new(self)
    }
    #[doc = "Bit 15 - reg_ovf_cnt_en_lsch1."]
    #[inline(always)]
    pub fn ovf_cnt_en_lsch1(&mut self) -> OVF_CNT_EN_LSCH1_W {
        OVF_CNT_EN_LSCH1_W::new(self)
    }
    #[doc = "Bit 16 - reg_ovf_cnt_reset_lsch1."]
    #[inline(always)]
    pub fn ovf_cnt_reset_lsch1(&mut self) -> OVF_CNT_RESET_LSCH1_W {
        OVF_CNT_RESET_LSCH1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LEDC_LSCH1_CONF0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch1_conf0](index.html) module"]
pub struct LSCH1_CONF0_SPEC;
impl crate::RegisterSpec for LSCH1_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lsch1_conf0::R](R) reader structure"]
impl crate::Readable for LSCH1_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lsch1_conf0::W](W) writer structure"]
impl crate::Writable for LSCH1_CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LSCH1_CONF0 to value 0"]
impl crate::Resettable for LSCH1_CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
