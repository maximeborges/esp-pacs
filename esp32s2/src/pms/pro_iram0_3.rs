#[doc = "Register `PRO_IRAM0_3` reader"]
pub struct R(crate::R<PRO_IRAM0_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_IRAM0_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_IRAM0_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_IRAM0_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_IRAM0_3` writer"]
pub struct W(crate::W<PRO_IRAM0_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_IRAM0_3_SPEC>;
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
impl From<crate::W<PRO_IRAM0_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_IRAM0_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_IRAM0_RTCFAST_SPLTADDR` reader - Configure the split address of RTC FAST for IBUS access."]
pub type PRO_IRAM0_RTCFAST_SPLTADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRO_IRAM0_RTCFAST_SPLTADDR` writer - Configure the split address of RTC FAST for IBUS access."]
pub type PRO_IRAM0_RTCFAST_SPLTADDR_W<'a> =
    crate::FieldWriter<'a, u32, PRO_IRAM0_3_SPEC, u16, u16, 11, 0>;
#[doc = "Field `PRO_IRAM0_RTCFAST_L_F` reader - Setting to 1 grants IBUS permission to fetch RTC FAST low address region."]
pub type PRO_IRAM0_RTCFAST_L_F_R = crate::BitReader<bool>;
#[doc = "Field `PRO_IRAM0_RTCFAST_L_F` writer - Setting to 1 grants IBUS permission to fetch RTC FAST low address region."]
pub type PRO_IRAM0_RTCFAST_L_F_W<'a> = crate::BitWriter<'a, u32, PRO_IRAM0_3_SPEC, bool, 11>;
#[doc = "Field `PRO_IRAM0_RTCFAST_L_R` reader - Setting to 1 grants IBUS permission to read RTC FAST low address region."]
pub type PRO_IRAM0_RTCFAST_L_R_R = crate::BitReader<bool>;
#[doc = "Field `PRO_IRAM0_RTCFAST_L_R` writer - Setting to 1 grants IBUS permission to read RTC FAST low address region."]
pub type PRO_IRAM0_RTCFAST_L_R_W<'a> = crate::BitWriter<'a, u32, PRO_IRAM0_3_SPEC, bool, 12>;
#[doc = "Field `PRO_IRAM0_RTCFAST_L_W` reader - Setting to 1 grants IBUS permission to write RTC FAST low address region."]
pub type PRO_IRAM0_RTCFAST_L_W_R = crate::BitReader<bool>;
#[doc = "Field `PRO_IRAM0_RTCFAST_L_W` writer - Setting to 1 grants IBUS permission to write RTC FAST low address region."]
pub type PRO_IRAM0_RTCFAST_L_W_W<'a> = crate::BitWriter<'a, u32, PRO_IRAM0_3_SPEC, bool, 13>;
#[doc = "Field `PRO_IRAM0_RTCFAST_H_F` reader - Setting to 1 grants IBUS permission to fetch RTC FAST high address region."]
pub type PRO_IRAM0_RTCFAST_H_F_R = crate::BitReader<bool>;
#[doc = "Field `PRO_IRAM0_RTCFAST_H_F` writer - Setting to 1 grants IBUS permission to fetch RTC FAST high address region."]
pub type PRO_IRAM0_RTCFAST_H_F_W<'a> = crate::BitWriter<'a, u32, PRO_IRAM0_3_SPEC, bool, 14>;
#[doc = "Field `PRO_IRAM0_RTCFAST_H_R` reader - Setting to 1 grants IBUS permission to read RTC FAST high address region."]
pub type PRO_IRAM0_RTCFAST_H_R_R = crate::BitReader<bool>;
#[doc = "Field `PRO_IRAM0_RTCFAST_H_R` writer - Setting to 1 grants IBUS permission to read RTC FAST high address region."]
pub type PRO_IRAM0_RTCFAST_H_R_W<'a> = crate::BitWriter<'a, u32, PRO_IRAM0_3_SPEC, bool, 15>;
#[doc = "Field `PRO_IRAM0_RTCFAST_H_W` reader - Setting to 1 grants IBUS permission to write RTC FAST high address region."]
pub type PRO_IRAM0_RTCFAST_H_W_R = crate::BitReader<bool>;
#[doc = "Field `PRO_IRAM0_RTCFAST_H_W` writer - Setting to 1 grants IBUS permission to write RTC FAST high address region."]
pub type PRO_IRAM0_RTCFAST_H_W_W<'a> = crate::BitWriter<'a, u32, PRO_IRAM0_3_SPEC, bool, 16>;
impl R {
    #[doc = "Bits 0:10 - Configure the split address of RTC FAST for IBUS access."]
    #[inline(always)]
    pub fn pro_iram0_rtcfast_spltaddr(&self) -> PRO_IRAM0_RTCFAST_SPLTADDR_R {
        PRO_IRAM0_RTCFAST_SPLTADDR_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Setting to 1 grants IBUS permission to fetch RTC FAST low address region."]
    #[inline(always)]
    pub fn pro_iram0_rtcfast_l_f(&self) -> PRO_IRAM0_RTCFAST_L_F_R {
        PRO_IRAM0_RTCFAST_L_F_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Setting to 1 grants IBUS permission to read RTC FAST low address region."]
    #[inline(always)]
    pub fn pro_iram0_rtcfast_l_r(&self) -> PRO_IRAM0_RTCFAST_L_R_R {
        PRO_IRAM0_RTCFAST_L_R_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Setting to 1 grants IBUS permission to write RTC FAST low address region."]
    #[inline(always)]
    pub fn pro_iram0_rtcfast_l_w(&self) -> PRO_IRAM0_RTCFAST_L_W_R {
        PRO_IRAM0_RTCFAST_L_W_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Setting to 1 grants IBUS permission to fetch RTC FAST high address region."]
    #[inline(always)]
    pub fn pro_iram0_rtcfast_h_f(&self) -> PRO_IRAM0_RTCFAST_H_F_R {
        PRO_IRAM0_RTCFAST_H_F_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Setting to 1 grants IBUS permission to read RTC FAST high address region."]
    #[inline(always)]
    pub fn pro_iram0_rtcfast_h_r(&self) -> PRO_IRAM0_RTCFAST_H_R_R {
        PRO_IRAM0_RTCFAST_H_R_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Setting to 1 grants IBUS permission to write RTC FAST high address region."]
    #[inline(always)]
    pub fn pro_iram0_rtcfast_h_w(&self) -> PRO_IRAM0_RTCFAST_H_W_R {
        PRO_IRAM0_RTCFAST_H_W_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Configure the split address of RTC FAST for IBUS access."]
    #[inline(always)]
    pub fn pro_iram0_rtcfast_spltaddr(&mut self) -> PRO_IRAM0_RTCFAST_SPLTADDR_W {
        PRO_IRAM0_RTCFAST_SPLTADDR_W::new(self)
    }
    #[doc = "Bit 11 - Setting to 1 grants IBUS permission to fetch RTC FAST low address region."]
    #[inline(always)]
    pub fn pro_iram0_rtcfast_l_f(&mut self) -> PRO_IRAM0_RTCFAST_L_F_W {
        PRO_IRAM0_RTCFAST_L_F_W::new(self)
    }
    #[doc = "Bit 12 - Setting to 1 grants IBUS permission to read RTC FAST low address region."]
    #[inline(always)]
    pub fn pro_iram0_rtcfast_l_r(&mut self) -> PRO_IRAM0_RTCFAST_L_R_W {
        PRO_IRAM0_RTCFAST_L_R_W::new(self)
    }
    #[doc = "Bit 13 - Setting to 1 grants IBUS permission to write RTC FAST low address region."]
    #[inline(always)]
    pub fn pro_iram0_rtcfast_l_w(&mut self) -> PRO_IRAM0_RTCFAST_L_W_W {
        PRO_IRAM0_RTCFAST_L_W_W::new(self)
    }
    #[doc = "Bit 14 - Setting to 1 grants IBUS permission to fetch RTC FAST high address region."]
    #[inline(always)]
    pub fn pro_iram0_rtcfast_h_f(&mut self) -> PRO_IRAM0_RTCFAST_H_F_W {
        PRO_IRAM0_RTCFAST_H_F_W::new(self)
    }
    #[doc = "Bit 15 - Setting to 1 grants IBUS permission to read RTC FAST high address region."]
    #[inline(always)]
    pub fn pro_iram0_rtcfast_h_r(&mut self) -> PRO_IRAM0_RTCFAST_H_R_W {
        PRO_IRAM0_RTCFAST_H_R_W::new(self)
    }
    #[doc = "Bit 16 - Setting to 1 grants IBUS permission to write RTC FAST high address region."]
    #[inline(always)]
    pub fn pro_iram0_rtcfast_h_w(&mut self) -> PRO_IRAM0_RTCFAST_H_W_W {
        PRO_IRAM0_RTCFAST_H_W_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IBUS permission control register 3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_iram0_3](index.html) module"]
pub struct PRO_IRAM0_3_SPEC;
impl crate::RegisterSpec for PRO_IRAM0_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_iram0_3::R](R) reader structure"]
impl crate::Readable for PRO_IRAM0_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_iram0_3::W](W) writer structure"]
impl crate::Writable for PRO_IRAM0_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_IRAM0_3 to value 0x0001_f800"]
impl crate::Resettable for PRO_IRAM0_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_f800
    }
}
