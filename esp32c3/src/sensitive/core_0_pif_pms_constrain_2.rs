#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_2` reader"]
pub struct R(crate::R<CORE_0_PIF_PMS_CONSTRAIN_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_PIF_PMS_CONSTRAIN_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_PIF_PMS_CONSTRAIN_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_PIF_PMS_CONSTRAIN_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_2` writer"]
pub struct W(crate::W<CORE_0_PIF_PMS_CONSTRAIN_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_PIF_PMS_CONSTRAIN_2_SPEC>;
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
impl From<crate::W<CORE_0_PIF_PMS_CONSTRAIN_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_PIF_PMS_CONSTRAIN_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT` reader - core_0_pif_pms_constrain_world_0_bt"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT` writer - core_0_pif_pms_constrain_world_0_bt"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_W<'a> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_2_SPEC, u8, u8, 2, 0>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT0` reader - core_0_pif_pms_constrain_world_0_i2c_ext0"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT0` writer - core_0_pif_pms_constrain_world_0_i2c_ext0"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT0_W<'a> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_2_SPEC, u8, u8, 2, 4>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UHCI0` reader - core_0_pif_pms_constrain_world_0_uhci0"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UHCI0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UHCI0` writer - core_0_pif_pms_constrain_world_0_uhci0"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UHCI0_W<'a> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_2_SPEC, u8, u8, 2, 6>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RMT` reader - core_0_pif_pms_constrain_world_0_rmt"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RMT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RMT` writer - core_0_pif_pms_constrain_world_0_rmt"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RMT_W<'a> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_2_SPEC, u8, u8, 2, 10>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LEDC` reader - core_0_pif_pms_constrain_world_0_ledc"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LEDC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LEDC` writer - core_0_pif_pms_constrain_world_0_ledc"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LEDC_W<'a> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_2_SPEC, u8, u8, 2, 16>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BB` reader - core_0_pif_pms_constrain_world_0_bb"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BB` writer - core_0_pif_pms_constrain_world_0_bb"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BB_W<'a> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_2_SPEC, u8, u8, 2, 22>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP` reader - core_0_pif_pms_constrain_world_0_timergroup"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP` writer - core_0_pif_pms_constrain_world_0_timergroup"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP_W<'a> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_2_SPEC, u8, u8, 2, 26>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP1` reader - core_0_pif_pms_constrain_world_0_timergroup1"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP1` writer - core_0_pif_pms_constrain_world_0_timergroup1"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP1_W<'a> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_2_SPEC, u8, u8, 2, 28>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTIMER` reader - core_0_pif_pms_constrain_world_0_systimer"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTIMER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTIMER` writer - core_0_pif_pms_constrain_world_0_systimer"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTIMER_W<'a> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_2_SPEC, u8, u8, 2, 30>;
impl R {
    #[doc = "Bits 0:1 - core_0_pif_pms_constrain_world_0_bt"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_bt(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - core_0_pif_pms_constrain_world_0_i2c_ext0"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_i2c_ext0(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT0_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT0_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - core_0_pif_pms_constrain_world_0_uhci0"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_uhci0(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UHCI0_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UHCI0_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 10:11 - core_0_pif_pms_constrain_world_0_rmt"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_rmt(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RMT_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RMT_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 16:17 - core_0_pif_pms_constrain_world_0_ledc"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_ledc(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LEDC_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LEDC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 22:23 - core_0_pif_pms_constrain_world_0_bb"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_bb(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BB_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BB_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 26:27 - core_0_pif_pms_constrain_world_0_timergroup"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_timergroup(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - core_0_pif_pms_constrain_world_0_timergroup1"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_timergroup1(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP1_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP1_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - core_0_pif_pms_constrain_world_0_systimer"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_systimer(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTIMER_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTIMER_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - core_0_pif_pms_constrain_world_0_bt"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_bt(&mut self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_W::new(self)
    }
    #[doc = "Bits 4:5 - core_0_pif_pms_constrain_world_0_i2c_ext0"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_i2c_ext0(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT0_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT0_W::new(self)
    }
    #[doc = "Bits 6:7 - core_0_pif_pms_constrain_world_0_uhci0"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_uhci0(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UHCI0_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UHCI0_W::new(self)
    }
    #[doc = "Bits 10:11 - core_0_pif_pms_constrain_world_0_rmt"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_rmt(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RMT_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RMT_W::new(self)
    }
    #[doc = "Bits 16:17 - core_0_pif_pms_constrain_world_0_ledc"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_ledc(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LEDC_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LEDC_W::new(self)
    }
    #[doc = "Bits 22:23 - core_0_pif_pms_constrain_world_0_bb"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_bb(&mut self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BB_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BB_W::new(self)
    }
    #[doc = "Bits 26:27 - core_0_pif_pms_constrain_world_0_timergroup"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_timergroup(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP_W::new(self)
    }
    #[doc = "Bits 28:29 - core_0_pif_pms_constrain_world_0_timergroup1"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_timergroup1(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP1_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP1_W::new(self)
    }
    #[doc = "Bits 30:31 - core_0_pif_pms_constrain_world_0_systimer"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_systimer(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTIMER_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTIMER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_pif_pms_constrain_2](index.html) module"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_2_SPEC;
impl crate::RegisterSpec for CORE_0_PIF_PMS_CONSTRAIN_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_pif_pms_constrain_2::R](R) reader structure"]
impl crate::Readable for CORE_0_PIF_PMS_CONSTRAIN_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_pif_pms_constrain_2::W](W) writer structure"]
impl crate::Writable for CORE_0_PIF_PMS_CONSTRAIN_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_0_PIF_PMS_CONSTRAIN_2 to value 0xfcc3_0cf3"]
impl crate::Resettable for CORE_0_PIF_PMS_CONSTRAIN_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfcc3_0cf3
    }
}
