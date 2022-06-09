#[doc = "Register `TX_SIM` reader"]
pub struct R(crate::R<TX_SIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_SIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_SIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_SIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_SIM` writer"]
pub struct W(crate::W<TX_SIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_SIM_SPEC>;
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
impl From<crate::W<TX_SIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_SIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_SIM_CH0` reader - reg_rmt_tx_sim_ch0."]
pub type TX_SIM_CH0_R = crate::BitReader<bool>;
#[doc = "Field `TX_SIM_CH0` writer - reg_rmt_tx_sim_ch0."]
pub type TX_SIM_CH0_W<'a> = crate::BitWriter<'a, u32, TX_SIM_SPEC, bool, 0>;
#[doc = "Field `TX_SIM_CH1` reader - reg_rmt_tx_sim_ch1."]
pub type TX_SIM_CH1_R = crate::BitReader<bool>;
#[doc = "Field `TX_SIM_CH1` writer - reg_rmt_tx_sim_ch1."]
pub type TX_SIM_CH1_W<'a> = crate::BitWriter<'a, u32, TX_SIM_SPEC, bool, 1>;
#[doc = "Field `TX_SIM_EN` reader - reg_rmt_tx_sim_en."]
pub type TX_SIM_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_SIM_EN` writer - reg_rmt_tx_sim_en."]
pub type TX_SIM_EN_W<'a> = crate::BitWriter<'a, u32, TX_SIM_SPEC, bool, 2>;
impl R {
    #[doc = "Bit 0 - reg_rmt_tx_sim_ch0."]
    #[inline(always)]
    pub fn tx_sim_ch0(&self) -> TX_SIM_CH0_R {
        TX_SIM_CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_rmt_tx_sim_ch1."]
    #[inline(always)]
    pub fn tx_sim_ch1(&self) -> TX_SIM_CH1_R {
        TX_SIM_CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - reg_rmt_tx_sim_en."]
    #[inline(always)]
    pub fn tx_sim_en(&self) -> TX_SIM_EN_R {
        TX_SIM_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - reg_rmt_tx_sim_ch0."]
    #[inline(always)]
    pub fn tx_sim_ch0(&mut self) -> TX_SIM_CH0_W {
        TX_SIM_CH0_W::new(self)
    }
    #[doc = "Bit 1 - reg_rmt_tx_sim_ch1."]
    #[inline(always)]
    pub fn tx_sim_ch1(&mut self) -> TX_SIM_CH1_W {
        TX_SIM_CH1_W::new(self)
    }
    #[doc = "Bit 2 - reg_rmt_tx_sim_en."]
    #[inline(always)]
    pub fn tx_sim_en(&mut self) -> TX_SIM_EN_W {
        TX_SIM_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RMT_TX_SIM_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_sim](index.html) module"]
pub struct TX_SIM_SPEC;
impl crate::RegisterSpec for TX_SIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_sim::R](R) reader structure"]
impl crate::Readable for TX_SIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_sim::W](W) writer structure"]
impl crate::Writable for TX_SIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_SIM to value 0"]
impl crate::Resettable for TX_SIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
