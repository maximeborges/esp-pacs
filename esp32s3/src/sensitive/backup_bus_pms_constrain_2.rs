#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_2` reader"]
pub struct R(crate::R<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_2` writer"]
pub struct W(crate::W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC>;
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
impl From<crate::W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BT` reader - BackUp access bt permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_BT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BT` writer - BackUp access bt permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_BT_W<'a> =
    crate::FieldWriter<'a, u32, BACKUP_BUS_PMS_CONSTRAIN_2_SPEC, u8, u8, 2, 0>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0` reader - BackUp access i2c_ext0 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0` writer - BackUp access i2c_ext0 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_W<'a> =
    crate::FieldWriter<'a, u32, BACKUP_BUS_PMS_CONSTRAIN_2_SPEC, u8, u8, 2, 4>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_UHCI0` reader - BackUp access uhci0 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_UHCI0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_UHCI0` writer - BackUp access uhci0 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_UHCI0_W<'a> =
    crate::FieldWriter<'a, u32, BACKUP_BUS_PMS_CONSTRAIN_2_SPEC, u8, u8, 2, 6>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SLCHOST` reader - BackUp access slchost permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_SLCHOST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SLCHOST` writer - BackUp access slchost permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_SLCHOST_W<'a> =
    crate::FieldWriter<'a, u32, BACKUP_BUS_PMS_CONSTRAIN_2_SPEC, u8, u8, 2, 8>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RMT` reader - BackUp access rmt permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_RMT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RMT` writer - BackUp access rmt permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_RMT_W<'a> =
    crate::FieldWriter<'a, u32, BACKUP_BUS_PMS_CONSTRAIN_2_SPEC, u8, u8, 2, 10>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_PCNT` reader - BackUp access pcnt permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_PCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_PCNT` writer - BackUp access pcnt permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_PCNT_W<'a> =
    crate::FieldWriter<'a, u32, BACKUP_BUS_PMS_CONSTRAIN_2_SPEC, u8, u8, 2, 12>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SLC` reader - BackUp access slc permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_SLC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SLC` writer - BackUp access slc permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_SLC_W<'a> =
    crate::FieldWriter<'a, u32, BACKUP_BUS_PMS_CONSTRAIN_2_SPEC, u8, u8, 2, 14>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_LEDC` reader - BackUp access ledc permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_LEDC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_LEDC` writer - BackUp access ledc permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_LEDC_W<'a> =
    crate::FieldWriter<'a, u32, BACKUP_BUS_PMS_CONSTRAIN_2_SPEC, u8, u8, 2, 16>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BACKUP` reader - BackUp access backup permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_BACKUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BACKUP` writer - BackUp access backup permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_BACKUP_W<'a> =
    crate::FieldWriter<'a, u32, BACKUP_BUS_PMS_CONSTRAIN_2_SPEC, u8, u8, 2, 18>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BB` reader - BackUp access bb permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_BB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BB` writer - BackUp access bb permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_BB_W<'a> =
    crate::FieldWriter<'a, u32, BACKUP_BUS_PMS_CONSTRAIN_2_SPEC, u8, u8, 2, 22>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_PWM0` reader - BackUp access pwm0 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_PWM0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_PWM0` writer - BackUp access pwm0 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_PWM0_W<'a> =
    crate::FieldWriter<'a, u32, BACKUP_BUS_PMS_CONSTRAIN_2_SPEC, u8, u8, 2, 24>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP` reader - BackUp access timergroup permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP` writer - BackUp access timergroup permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_W<'a> =
    crate::FieldWriter<'a, u32, BACKUP_BUS_PMS_CONSTRAIN_2_SPEC, u8, u8, 2, 26>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1` reader - BackUp access timergroup1 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1` writer - BackUp access timergroup1 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_W<'a> =
    crate::FieldWriter<'a, u32, BACKUP_BUS_PMS_CONSTRAIN_2_SPEC, u8, u8, 2, 28>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER` reader - BackUp access systimer permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER` writer - BackUp access systimer permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_W<'a> =
    crate::FieldWriter<'a, u32, BACKUP_BUS_PMS_CONSTRAIN_2_SPEC, u8, u8, 2, 30>;
impl R {
    #[doc = "Bits 0:1 - BackUp access bt permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_bt(&self) -> BACKUP_BUS_PMS_CONSTRAIN_BT_R {
        BACKUP_BUS_PMS_CONSTRAIN_BT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - BackUp access i2c_ext0 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_i2c_ext0(&self) -> BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_R {
        BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - BackUp access uhci0 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_uhci0(&self) -> BACKUP_BUS_PMS_CONSTRAIN_UHCI0_R {
        BACKUP_BUS_PMS_CONSTRAIN_UHCI0_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - BackUp access slchost permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_slchost(&self) -> BACKUP_BUS_PMS_CONSTRAIN_SLCHOST_R {
        BACKUP_BUS_PMS_CONSTRAIN_SLCHOST_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - BackUp access rmt permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_rmt(&self) -> BACKUP_BUS_PMS_CONSTRAIN_RMT_R {
        BACKUP_BUS_PMS_CONSTRAIN_RMT_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - BackUp access pcnt permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_pcnt(&self) -> BACKUP_BUS_PMS_CONSTRAIN_PCNT_R {
        BACKUP_BUS_PMS_CONSTRAIN_PCNT_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - BackUp access slc permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_slc(&self) -> BACKUP_BUS_PMS_CONSTRAIN_SLC_R {
        BACKUP_BUS_PMS_CONSTRAIN_SLC_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - BackUp access ledc permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_ledc(&self) -> BACKUP_BUS_PMS_CONSTRAIN_LEDC_R {
        BACKUP_BUS_PMS_CONSTRAIN_LEDC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - BackUp access backup permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_backup(&self) -> BACKUP_BUS_PMS_CONSTRAIN_BACKUP_R {
        BACKUP_BUS_PMS_CONSTRAIN_BACKUP_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 22:23 - BackUp access bb permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_bb(&self) -> BACKUP_BUS_PMS_CONSTRAIN_BB_R {
        BACKUP_BUS_PMS_CONSTRAIN_BB_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - BackUp access pwm0 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_pwm0(&self) -> BACKUP_BUS_PMS_CONSTRAIN_PWM0_R {
        BACKUP_BUS_PMS_CONSTRAIN_PWM0_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - BackUp access timergroup permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_timergroup(&self) -> BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_R {
        BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - BackUp access timergroup1 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_timergroup1(&self) -> BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_R {
        BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - BackUp access systimer permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_systimer(&self) -> BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_R {
        BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - BackUp access bt permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_bt(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_BT_W {
        BACKUP_BUS_PMS_CONSTRAIN_BT_W::new(self)
    }
    #[doc = "Bits 4:5 - BackUp access i2c_ext0 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_i2c_ext0(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_W {
        BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_W::new(self)
    }
    #[doc = "Bits 6:7 - BackUp access uhci0 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_uhci0(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_UHCI0_W {
        BACKUP_BUS_PMS_CONSTRAIN_UHCI0_W::new(self)
    }
    #[doc = "Bits 8:9 - BackUp access slchost permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_slchost(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_SLCHOST_W {
        BACKUP_BUS_PMS_CONSTRAIN_SLCHOST_W::new(self)
    }
    #[doc = "Bits 10:11 - BackUp access rmt permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_rmt(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_RMT_W {
        BACKUP_BUS_PMS_CONSTRAIN_RMT_W::new(self)
    }
    #[doc = "Bits 12:13 - BackUp access pcnt permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_pcnt(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_PCNT_W {
        BACKUP_BUS_PMS_CONSTRAIN_PCNT_W::new(self)
    }
    #[doc = "Bits 14:15 - BackUp access slc permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_slc(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_SLC_W {
        BACKUP_BUS_PMS_CONSTRAIN_SLC_W::new(self)
    }
    #[doc = "Bits 16:17 - BackUp access ledc permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_ledc(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_LEDC_W {
        BACKUP_BUS_PMS_CONSTRAIN_LEDC_W::new(self)
    }
    #[doc = "Bits 18:19 - BackUp access backup permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_backup(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_BACKUP_W {
        BACKUP_BUS_PMS_CONSTRAIN_BACKUP_W::new(self)
    }
    #[doc = "Bits 22:23 - BackUp access bb permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_bb(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_BB_W {
        BACKUP_BUS_PMS_CONSTRAIN_BB_W::new(self)
    }
    #[doc = "Bits 24:25 - BackUp access pwm0 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_pwm0(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_PWM0_W {
        BACKUP_BUS_PMS_CONSTRAIN_PWM0_W::new(self)
    }
    #[doc = "Bits 26:27 - BackUp access timergroup permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_timergroup(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_W {
        BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_W::new(self)
    }
    #[doc = "Bits 28:29 - BackUp access timergroup1 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_timergroup1(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_W {
        BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_W::new(self)
    }
    #[doc = "Bits 30:31 - BackUp access systimer permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_systimer(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_W {
        BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BackUp access peripherals permission configuration register 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backup_bus_pms_constrain_2](index.html) module"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_2_SPEC;
impl crate::RegisterSpec for BACKUP_BUS_PMS_CONSTRAIN_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [backup_bus_pms_constrain_2::R](R) reader structure"]
impl crate::Readable for BACKUP_BUS_PMS_CONSTRAIN_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [backup_bus_pms_constrain_2::W](W) writer structure"]
impl crate::Writable for BACKUP_BUS_PMS_CONSTRAIN_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BACKUP_BUS_PMS_CONSTRAIN_2 to value 0xffcf_fff3"]
impl crate::Resettable for BACKUP_BUS_PMS_CONSTRAIN_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffcf_fff3
    }
}
