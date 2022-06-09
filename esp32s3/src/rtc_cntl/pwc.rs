#[doc = "Register `PWC` reader"]
pub struct R(crate::R<PWC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWC` writer"]
pub struct W(crate::W<PWC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWC_SPEC>;
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
impl From<crate::W<PWC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_FASTMEM_FORCE_NOISO` reader - Fast RTC memory force no ISO"]
pub type RTC_FASTMEM_FORCE_NOISO_R = crate::BitReader<bool>;
#[doc = "Field `RTC_FASTMEM_FORCE_NOISO` writer - Fast RTC memory force no ISO"]
pub type RTC_FASTMEM_FORCE_NOISO_W<'a> = crate::BitWriter<'a, u32, PWC_SPEC, bool, 0>;
#[doc = "Field `RTC_FASTMEM_FORCE_ISO` reader - Fast RTC memory force ISO"]
pub type RTC_FASTMEM_FORCE_ISO_R = crate::BitReader<bool>;
#[doc = "Field `RTC_FASTMEM_FORCE_ISO` writer - Fast RTC memory force ISO"]
pub type RTC_FASTMEM_FORCE_ISO_W<'a> = crate::BitWriter<'a, u32, PWC_SPEC, bool, 1>;
#[doc = "Field `RTC_SLOWMEM_FORCE_NOISO` reader - RTC memory force no ISO"]
pub type RTC_SLOWMEM_FORCE_NOISO_R = crate::BitReader<bool>;
#[doc = "Field `RTC_SLOWMEM_FORCE_NOISO` writer - RTC memory force no ISO"]
pub type RTC_SLOWMEM_FORCE_NOISO_W<'a> = crate::BitWriter<'a, u32, PWC_SPEC, bool, 2>;
#[doc = "Field `RTC_SLOWMEM_FORCE_ISO` reader - RTC memory force ISO"]
pub type RTC_SLOWMEM_FORCE_ISO_R = crate::BitReader<bool>;
#[doc = "Field `RTC_SLOWMEM_FORCE_ISO` writer - RTC memory force ISO"]
pub type RTC_SLOWMEM_FORCE_ISO_W<'a> = crate::BitWriter<'a, u32, PWC_SPEC, bool, 3>;
#[doc = "Field `RTC_FORCE_ISO` reader - rtc_peri force ISO"]
pub type RTC_FORCE_ISO_R = crate::BitReader<bool>;
#[doc = "Field `RTC_FORCE_ISO` writer - rtc_peri force ISO"]
pub type RTC_FORCE_ISO_W<'a> = crate::BitWriter<'a, u32, PWC_SPEC, bool, 4>;
#[doc = "Field `RTC_FORCE_NOISO` reader - rtc_peri force no ISO"]
pub type RTC_FORCE_NOISO_R = crate::BitReader<bool>;
#[doc = "Field `RTC_FORCE_NOISO` writer - rtc_peri force no ISO"]
pub type RTC_FORCE_NOISO_W<'a> = crate::BitWriter<'a, u32, PWC_SPEC, bool, 5>;
#[doc = "Field `RTC_FASTMEM_FOLW_CPU` reader - 1: Fast RTC memory PD following CPU, 0: fast RTC memory PD following RTC state machine"]
pub type RTC_FASTMEM_FOLW_CPU_R = crate::BitReader<bool>;
#[doc = "Field `RTC_FASTMEM_FOLW_CPU` writer - 1: Fast RTC memory PD following CPU, 0: fast RTC memory PD following RTC state machine"]
pub type RTC_FASTMEM_FOLW_CPU_W<'a> = crate::BitWriter<'a, u32, PWC_SPEC, bool, 6>;
#[doc = "Field `RTC_FASTMEM_FORCE_LPD` reader - Fast RTC memory force PD"]
pub type RTC_FASTMEM_FORCE_LPD_R = crate::BitReader<bool>;
#[doc = "Field `RTC_FASTMEM_FORCE_LPD` writer - Fast RTC memory force PD"]
pub type RTC_FASTMEM_FORCE_LPD_W<'a> = crate::BitWriter<'a, u32, PWC_SPEC, bool, 7>;
#[doc = "Field `RTC_FASTMEM_FORCE_LPU` reader - Fast RTC memory force no PD"]
pub type RTC_FASTMEM_FORCE_LPU_R = crate::BitReader<bool>;
#[doc = "Field `RTC_FASTMEM_FORCE_LPU` writer - Fast RTC memory force no PD"]
pub type RTC_FASTMEM_FORCE_LPU_W<'a> = crate::BitWriter<'a, u32, PWC_SPEC, bool, 8>;
#[doc = "Field `RTC_SLOWMEM_FOLW_CPU` reader - 1: RTC memory PD following CPU, 0: RTC memory PD following RTC state machine"]
pub type RTC_SLOWMEM_FOLW_CPU_R = crate::BitReader<bool>;
#[doc = "Field `RTC_SLOWMEM_FOLW_CPU` writer - 1: RTC memory PD following CPU, 0: RTC memory PD following RTC state machine"]
pub type RTC_SLOWMEM_FOLW_CPU_W<'a> = crate::BitWriter<'a, u32, PWC_SPEC, bool, 9>;
#[doc = "Field `RTC_SLOWMEM_FORCE_LPD` reader - RTC memory force PD"]
pub type RTC_SLOWMEM_FORCE_LPD_R = crate::BitReader<bool>;
#[doc = "Field `RTC_SLOWMEM_FORCE_LPD` writer - RTC memory force PD"]
pub type RTC_SLOWMEM_FORCE_LPD_W<'a> = crate::BitWriter<'a, u32, PWC_SPEC, bool, 10>;
#[doc = "Field `RTC_SLOWMEM_FORCE_LPU` reader - RTC memory force no PD"]
pub type RTC_SLOWMEM_FORCE_LPU_R = crate::BitReader<bool>;
#[doc = "Field `RTC_SLOWMEM_FORCE_LPU` writer - RTC memory force no PD"]
pub type RTC_SLOWMEM_FORCE_LPU_W<'a> = crate::BitWriter<'a, u32, PWC_SPEC, bool, 11>;
#[doc = "Field `RTC_FORCE_PD` reader - rtc_peri force power down"]
pub type RTC_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `RTC_FORCE_PD` writer - rtc_peri force power down"]
pub type RTC_FORCE_PD_W<'a> = crate::BitWriter<'a, u32, PWC_SPEC, bool, 18>;
#[doc = "Field `RTC_FORCE_PU` reader - rtc_peri force power up"]
pub type RTC_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `RTC_FORCE_PU` writer - rtc_peri force power up"]
pub type RTC_FORCE_PU_W<'a> = crate::BitWriter<'a, u32, PWC_SPEC, bool, 19>;
#[doc = "Field `RTC_PD_EN` reader - enable power down rtc_peri in sleep"]
pub type RTC_PD_EN_R = crate::BitReader<bool>;
#[doc = "Field `RTC_PD_EN` writer - enable power down rtc_peri in sleep"]
pub type RTC_PD_EN_W<'a> = crate::BitWriter<'a, u32, PWC_SPEC, bool, 20>;
#[doc = "Field `RTC_PAD_FORCE_HOLD` reader - rtc pad force hold"]
pub type RTC_PAD_FORCE_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `RTC_PAD_FORCE_HOLD` writer - rtc pad force hold"]
pub type RTC_PAD_FORCE_HOLD_W<'a> = crate::BitWriter<'a, u32, PWC_SPEC, bool, 21>;
impl R {
    #[doc = "Bit 0 - Fast RTC memory force no ISO"]
    #[inline(always)]
    pub fn rtc_fastmem_force_noiso(&self) -> RTC_FASTMEM_FORCE_NOISO_R {
        RTC_FASTMEM_FORCE_NOISO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fast RTC memory force ISO"]
    #[inline(always)]
    pub fn rtc_fastmem_force_iso(&self) -> RTC_FASTMEM_FORCE_ISO_R {
        RTC_FASTMEM_FORCE_ISO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC memory force no ISO"]
    #[inline(always)]
    pub fn rtc_slowmem_force_noiso(&self) -> RTC_SLOWMEM_FORCE_NOISO_R {
        RTC_SLOWMEM_FORCE_NOISO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC memory force ISO"]
    #[inline(always)]
    pub fn rtc_slowmem_force_iso(&self) -> RTC_SLOWMEM_FORCE_ISO_R {
        RTC_SLOWMEM_FORCE_ISO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - rtc_peri force ISO"]
    #[inline(always)]
    pub fn rtc_force_iso(&self) -> RTC_FORCE_ISO_R {
        RTC_FORCE_ISO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - rtc_peri force no ISO"]
    #[inline(always)]
    pub fn rtc_force_noiso(&self) -> RTC_FORCE_NOISO_R {
        RTC_FORCE_NOISO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Fast RTC memory PD following CPU, 0: fast RTC memory PD following RTC state machine"]
    #[inline(always)]
    pub fn rtc_fastmem_folw_cpu(&self) -> RTC_FASTMEM_FOLW_CPU_R {
        RTC_FASTMEM_FOLW_CPU_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Fast RTC memory force PD"]
    #[inline(always)]
    pub fn rtc_fastmem_force_lpd(&self) -> RTC_FASTMEM_FORCE_LPD_R {
        RTC_FASTMEM_FORCE_LPD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Fast RTC memory force no PD"]
    #[inline(always)]
    pub fn rtc_fastmem_force_lpu(&self) -> RTC_FASTMEM_FORCE_LPU_R {
        RTC_FASTMEM_FORCE_LPU_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: RTC memory PD following CPU, 0: RTC memory PD following RTC state machine"]
    #[inline(always)]
    pub fn rtc_slowmem_folw_cpu(&self) -> RTC_SLOWMEM_FOLW_CPU_R {
        RTC_SLOWMEM_FOLW_CPU_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC memory force PD"]
    #[inline(always)]
    pub fn rtc_slowmem_force_lpd(&self) -> RTC_SLOWMEM_FORCE_LPD_R {
        RTC_SLOWMEM_FORCE_LPD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RTC memory force no PD"]
    #[inline(always)]
    pub fn rtc_slowmem_force_lpu(&self) -> RTC_SLOWMEM_FORCE_LPU_R {
        RTC_SLOWMEM_FORCE_LPU_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 18 - rtc_peri force power down"]
    #[inline(always)]
    pub fn rtc_force_pd(&self) -> RTC_FORCE_PD_R {
        RTC_FORCE_PD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - rtc_peri force power up"]
    #[inline(always)]
    pub fn rtc_force_pu(&self) -> RTC_FORCE_PU_R {
        RTC_FORCE_PU_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - enable power down rtc_peri in sleep"]
    #[inline(always)]
    pub fn rtc_pd_en(&self) -> RTC_PD_EN_R {
        RTC_PD_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - rtc pad force hold"]
    #[inline(always)]
    pub fn rtc_pad_force_hold(&self) -> RTC_PAD_FORCE_HOLD_R {
        RTC_PAD_FORCE_HOLD_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fast RTC memory force no ISO"]
    #[inline(always)]
    pub fn rtc_fastmem_force_noiso(&mut self) -> RTC_FASTMEM_FORCE_NOISO_W {
        RTC_FASTMEM_FORCE_NOISO_W::new(self)
    }
    #[doc = "Bit 1 - Fast RTC memory force ISO"]
    #[inline(always)]
    pub fn rtc_fastmem_force_iso(&mut self) -> RTC_FASTMEM_FORCE_ISO_W {
        RTC_FASTMEM_FORCE_ISO_W::new(self)
    }
    #[doc = "Bit 2 - RTC memory force no ISO"]
    #[inline(always)]
    pub fn rtc_slowmem_force_noiso(&mut self) -> RTC_SLOWMEM_FORCE_NOISO_W {
        RTC_SLOWMEM_FORCE_NOISO_W::new(self)
    }
    #[doc = "Bit 3 - RTC memory force ISO"]
    #[inline(always)]
    pub fn rtc_slowmem_force_iso(&mut self) -> RTC_SLOWMEM_FORCE_ISO_W {
        RTC_SLOWMEM_FORCE_ISO_W::new(self)
    }
    #[doc = "Bit 4 - rtc_peri force ISO"]
    #[inline(always)]
    pub fn rtc_force_iso(&mut self) -> RTC_FORCE_ISO_W {
        RTC_FORCE_ISO_W::new(self)
    }
    #[doc = "Bit 5 - rtc_peri force no ISO"]
    #[inline(always)]
    pub fn rtc_force_noiso(&mut self) -> RTC_FORCE_NOISO_W {
        RTC_FORCE_NOISO_W::new(self)
    }
    #[doc = "Bit 6 - 1: Fast RTC memory PD following CPU, 0: fast RTC memory PD following RTC state machine"]
    #[inline(always)]
    pub fn rtc_fastmem_folw_cpu(&mut self) -> RTC_FASTMEM_FOLW_CPU_W {
        RTC_FASTMEM_FOLW_CPU_W::new(self)
    }
    #[doc = "Bit 7 - Fast RTC memory force PD"]
    #[inline(always)]
    pub fn rtc_fastmem_force_lpd(&mut self) -> RTC_FASTMEM_FORCE_LPD_W {
        RTC_FASTMEM_FORCE_LPD_W::new(self)
    }
    #[doc = "Bit 8 - Fast RTC memory force no PD"]
    #[inline(always)]
    pub fn rtc_fastmem_force_lpu(&mut self) -> RTC_FASTMEM_FORCE_LPU_W {
        RTC_FASTMEM_FORCE_LPU_W::new(self)
    }
    #[doc = "Bit 9 - 1: RTC memory PD following CPU, 0: RTC memory PD following RTC state machine"]
    #[inline(always)]
    pub fn rtc_slowmem_folw_cpu(&mut self) -> RTC_SLOWMEM_FOLW_CPU_W {
        RTC_SLOWMEM_FOLW_CPU_W::new(self)
    }
    #[doc = "Bit 10 - RTC memory force PD"]
    #[inline(always)]
    pub fn rtc_slowmem_force_lpd(&mut self) -> RTC_SLOWMEM_FORCE_LPD_W {
        RTC_SLOWMEM_FORCE_LPD_W::new(self)
    }
    #[doc = "Bit 11 - RTC memory force no PD"]
    #[inline(always)]
    pub fn rtc_slowmem_force_lpu(&mut self) -> RTC_SLOWMEM_FORCE_LPU_W {
        RTC_SLOWMEM_FORCE_LPU_W::new(self)
    }
    #[doc = "Bit 18 - rtc_peri force power down"]
    #[inline(always)]
    pub fn rtc_force_pd(&mut self) -> RTC_FORCE_PD_W {
        RTC_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 19 - rtc_peri force power up"]
    #[inline(always)]
    pub fn rtc_force_pu(&mut self) -> RTC_FORCE_PU_W {
        RTC_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 20 - enable power down rtc_peri in sleep"]
    #[inline(always)]
    pub fn rtc_pd_en(&mut self) -> RTC_PD_EN_W {
        RTC_PD_EN_W::new(self)
    }
    #[doc = "Bit 21 - rtc pad force hold"]
    #[inline(always)]
    pub fn rtc_pad_force_hold(&mut self) -> RTC_PAD_FORCE_HOLD_W {
        RTC_PAD_FORCE_HOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure rtc power\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwc](index.html) module"]
pub struct PWC_SPEC;
impl crate::RegisterSpec for PWC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwc::R](R) reader structure"]
impl crate::Readable for PWC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwc::W](W) writer structure"]
impl crate::Writable for PWC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWC to value 0x0925"]
impl crate::Resettable for PWC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0925
    }
}
