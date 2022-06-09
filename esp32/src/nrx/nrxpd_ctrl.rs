#[doc = "Register `NRXPD_CTRL` reader"]
pub struct R(crate::R<NRXPD_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NRXPD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NRXPD_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NRXPD_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NRXPD_CTRL` writer"]
pub struct W(crate::W<NRXPD_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NRXPD_CTRL_SPEC>;
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
impl From<crate::W<NRXPD_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NRXPD_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEMAP_FORCE_PD` reader - "]
pub type DEMAP_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `DEMAP_FORCE_PD` writer - "]
pub type DEMAP_FORCE_PD_W<'a> = crate::BitWriter<'a, u32, NRXPD_CTRL_SPEC, bool, 0>;
#[doc = "Field `DEMAP_FORCE_PU` reader - "]
pub type DEMAP_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `DEMAP_FORCE_PU` writer - "]
pub type DEMAP_FORCE_PU_W<'a> = crate::BitWriter<'a, u32, NRXPD_CTRL_SPEC, bool, 1>;
#[doc = "Field `VIT_FORCE_PD` reader - "]
pub type VIT_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `VIT_FORCE_PD` writer - "]
pub type VIT_FORCE_PD_W<'a> = crate::BitWriter<'a, u32, NRXPD_CTRL_SPEC, bool, 2>;
#[doc = "Field `VIT_FORCE_PU` reader - "]
pub type VIT_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `VIT_FORCE_PU` writer - "]
pub type VIT_FORCE_PU_W<'a> = crate::BitWriter<'a, u32, NRXPD_CTRL_SPEC, bool, 3>;
#[doc = "Field `RX_ROT_FORCE_PD` reader - "]
pub type RX_ROT_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `RX_ROT_FORCE_PD` writer - "]
pub type RX_ROT_FORCE_PD_W<'a> = crate::BitWriter<'a, u32, NRXPD_CTRL_SPEC, bool, 4>;
#[doc = "Field `RX_ROT_FORCE_PU` reader - "]
pub type RX_ROT_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `RX_ROT_FORCE_PU` writer - "]
pub type RX_ROT_FORCE_PU_W<'a> = crate::BitWriter<'a, u32, NRXPD_CTRL_SPEC, bool, 5>;
#[doc = "Field `CHAN_EST_FORCE_PD` reader - "]
pub type CHAN_EST_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `CHAN_EST_FORCE_PD` writer - "]
pub type CHAN_EST_FORCE_PD_W<'a> = crate::BitWriter<'a, u32, NRXPD_CTRL_SPEC, bool, 6>;
#[doc = "Field `CHAN_EST_FORCE_PU` reader - "]
pub type CHAN_EST_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `CHAN_EST_FORCE_PU` writer - "]
pub type CHAN_EST_FORCE_PU_W<'a> = crate::BitWriter<'a, u32, NRXPD_CTRL_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn demap_force_pd(&self) -> DEMAP_FORCE_PD_R {
        DEMAP_FORCE_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn demap_force_pu(&self) -> DEMAP_FORCE_PU_R {
        DEMAP_FORCE_PU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn vit_force_pd(&self) -> VIT_FORCE_PD_R {
        VIT_FORCE_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn vit_force_pu(&self) -> VIT_FORCE_PU_R {
        VIT_FORCE_PU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rx_rot_force_pd(&self) -> RX_ROT_FORCE_PD_R {
        RX_ROT_FORCE_PD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_rot_force_pu(&self) -> RX_ROT_FORCE_PU_R {
        RX_ROT_FORCE_PU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn chan_est_force_pd(&self) -> CHAN_EST_FORCE_PD_R {
        CHAN_EST_FORCE_PD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn chan_est_force_pu(&self) -> CHAN_EST_FORCE_PU_R {
        CHAN_EST_FORCE_PU_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn demap_force_pd(&mut self) -> DEMAP_FORCE_PD_W {
        DEMAP_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn demap_force_pu(&mut self) -> DEMAP_FORCE_PU_W {
        DEMAP_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn vit_force_pd(&mut self) -> VIT_FORCE_PD_W {
        VIT_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn vit_force_pu(&mut self) -> VIT_FORCE_PU_W {
        VIT_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rx_rot_force_pd(&mut self) -> RX_ROT_FORCE_PD_W {
        RX_ROT_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_rot_force_pu(&mut self) -> RX_ROT_FORCE_PU_W {
        RX_ROT_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn chan_est_force_pd(&mut self) -> CHAN_EST_FORCE_PD_W {
        CHAN_EST_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn chan_est_force_pu(&mut self) -> CHAN_EST_FORCE_PU_W {
        CHAN_EST_FORCE_PU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WiFi RX control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nrxpd_ctrl](index.html) module"]
pub struct NRXPD_CTRL_SPEC;
impl crate::RegisterSpec for NRXPD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nrxpd_ctrl::R](R) reader structure"]
impl crate::Readable for NRXPD_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nrxpd_ctrl::W](W) writer structure"]
impl crate::Writable for NRXPD_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NRXPD_CTRL to value 0"]
impl crate::Resettable for NRXPD_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
