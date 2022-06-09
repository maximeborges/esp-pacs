#[doc = "Register `GEN2_B` reader"]
pub struct R(crate::R<GEN2_B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GEN2_B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GEN2_B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GEN2_B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GEN2_B` writer"]
pub struct W(crate::W<GEN2_B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GEN2_B_SPEC>;
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
impl From<crate::W<GEN2_B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GEN2_B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UTEZ` reader - Action on PWM2B triggered by event TEZ when timer increasing"]
pub type UTEZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UTEZ` writer - Action on PWM2B triggered by event TEZ when timer increasing"]
pub type UTEZ_W<'a> = crate::FieldWriter<'a, u32, GEN2_B_SPEC, u8, u8, 2, 0>;
#[doc = "Field `UTEP` reader - Action on PWM2B triggered by event TEP when timer increasing"]
pub type UTEP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UTEP` writer - Action on PWM2B triggered by event TEP when timer increasing"]
pub type UTEP_W<'a> = crate::FieldWriter<'a, u32, GEN2_B_SPEC, u8, u8, 2, 2>;
#[doc = "Field `UTEA` reader - Action on PWM2B triggered by event TEA when timer increasing"]
pub type UTEA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UTEA` writer - Action on PWM2B triggered by event TEA when timer increasing"]
pub type UTEA_W<'a> = crate::FieldWriter<'a, u32, GEN2_B_SPEC, u8, u8, 2, 4>;
#[doc = "Field `UTEB` reader - Action on PWM2B triggered by event TEB when timer increasing"]
pub type UTEB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UTEB` writer - Action on PWM2B triggered by event TEB when timer increasing"]
pub type UTEB_W<'a> = crate::FieldWriter<'a, u32, GEN2_B_SPEC, u8, u8, 2, 6>;
#[doc = "Field `UT0` reader - Action on PWM2B triggered by event_t0 when timer increasing"]
pub type UT0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UT0` writer - Action on PWM2B triggered by event_t0 when timer increasing"]
pub type UT0_W<'a> = crate::FieldWriter<'a, u32, GEN2_B_SPEC, u8, u8, 2, 8>;
#[doc = "Field `UT1` reader - Action on PWM2B triggered by event_t1 when timer increasing"]
pub type UT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UT1` writer - Action on PWM2B triggered by event_t1 when timer increasing"]
pub type UT1_W<'a> = crate::FieldWriter<'a, u32, GEN2_B_SPEC, u8, u8, 2, 10>;
#[doc = "Field `DTEZ` reader - Action on PWM2B triggered by event TEZ when timer decreasing"]
pub type DTEZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTEZ` writer - Action on PWM2B triggered by event TEZ when timer decreasing"]
pub type DTEZ_W<'a> = crate::FieldWriter<'a, u32, GEN2_B_SPEC, u8, u8, 2, 12>;
#[doc = "Field `DTEP` reader - Action on PWM2B triggered by event TEP when timer decreasing"]
pub type DTEP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTEP` writer - Action on PWM2B triggered by event TEP when timer decreasing"]
pub type DTEP_W<'a> = crate::FieldWriter<'a, u32, GEN2_B_SPEC, u8, u8, 2, 14>;
#[doc = "Field `DTEA` reader - Action on PWM2B triggered by event TEA when timer decreasing"]
pub type DTEA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTEA` writer - Action on PWM2B triggered by event TEA when timer decreasing"]
pub type DTEA_W<'a> = crate::FieldWriter<'a, u32, GEN2_B_SPEC, u8, u8, 2, 16>;
#[doc = "Field `DTEB` reader - Action on PWM2B triggered by event TEB when timer decreasing"]
pub type DTEB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTEB` writer - Action on PWM2B triggered by event TEB when timer decreasing"]
pub type DTEB_W<'a> = crate::FieldWriter<'a, u32, GEN2_B_SPEC, u8, u8, 2, 18>;
#[doc = "Field `DT0` reader - Action on PWM2B triggered by event_t0 when timer decreasing"]
pub type DT0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DT0` writer - Action on PWM2B triggered by event_t0 when timer decreasing"]
pub type DT0_W<'a> = crate::FieldWriter<'a, u32, GEN2_B_SPEC, u8, u8, 2, 20>;
#[doc = "Field `DT1` reader - Action on PWM2B triggered by event_t1 when timer decreasing. 0: no change, 1: low, 2: high, 3: toggle"]
pub type DT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DT1` writer - Action on PWM2B triggered by event_t1 when timer decreasing. 0: no change, 1: low, 2: high, 3: toggle"]
pub type DT1_W<'a> = crate::FieldWriter<'a, u32, GEN2_B_SPEC, u8, u8, 2, 22>;
impl R {
    #[doc = "Bits 0:1 - Action on PWM2B triggered by event TEZ when timer increasing"]
    #[inline(always)]
    pub fn utez(&self) -> UTEZ_R {
        UTEZ_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Action on PWM2B triggered by event TEP when timer increasing"]
    #[inline(always)]
    pub fn utep(&self) -> UTEP_R {
        UTEP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Action on PWM2B triggered by event TEA when timer increasing"]
    #[inline(always)]
    pub fn utea(&self) -> UTEA_R {
        UTEA_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Action on PWM2B triggered by event TEB when timer increasing"]
    #[inline(always)]
    pub fn uteb(&self) -> UTEB_R {
        UTEB_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Action on PWM2B triggered by event_t0 when timer increasing"]
    #[inline(always)]
    pub fn ut0(&self) -> UT0_R {
        UT0_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Action on PWM2B triggered by event_t1 when timer increasing"]
    #[inline(always)]
    pub fn ut1(&self) -> UT1_R {
        UT1_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Action on PWM2B triggered by event TEZ when timer decreasing"]
    #[inline(always)]
    pub fn dtez(&self) -> DTEZ_R {
        DTEZ_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Action on PWM2B triggered by event TEP when timer decreasing"]
    #[inline(always)]
    pub fn dtep(&self) -> DTEP_R {
        DTEP_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Action on PWM2B triggered by event TEA when timer decreasing"]
    #[inline(always)]
    pub fn dtea(&self) -> DTEA_R {
        DTEA_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Action on PWM2B triggered by event TEB when timer decreasing"]
    #[inline(always)]
    pub fn dteb(&self) -> DTEB_R {
        DTEB_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Action on PWM2B triggered by event_t0 when timer decreasing"]
    #[inline(always)]
    pub fn dt0(&self) -> DT0_R {
        DT0_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Action on PWM2B triggered by event_t1 when timer decreasing. 0: no change, 1: low, 2: high, 3: toggle"]
    #[inline(always)]
    pub fn dt1(&self) -> DT1_R {
        DT1_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Action on PWM2B triggered by event TEZ when timer increasing"]
    #[inline(always)]
    pub fn utez(&mut self) -> UTEZ_W {
        UTEZ_W::new(self)
    }
    #[doc = "Bits 2:3 - Action on PWM2B triggered by event TEP when timer increasing"]
    #[inline(always)]
    pub fn utep(&mut self) -> UTEP_W {
        UTEP_W::new(self)
    }
    #[doc = "Bits 4:5 - Action on PWM2B triggered by event TEA when timer increasing"]
    #[inline(always)]
    pub fn utea(&mut self) -> UTEA_W {
        UTEA_W::new(self)
    }
    #[doc = "Bits 6:7 - Action on PWM2B triggered by event TEB when timer increasing"]
    #[inline(always)]
    pub fn uteb(&mut self) -> UTEB_W {
        UTEB_W::new(self)
    }
    #[doc = "Bits 8:9 - Action on PWM2B triggered by event_t0 when timer increasing"]
    #[inline(always)]
    pub fn ut0(&mut self) -> UT0_W {
        UT0_W::new(self)
    }
    #[doc = "Bits 10:11 - Action on PWM2B triggered by event_t1 when timer increasing"]
    #[inline(always)]
    pub fn ut1(&mut self) -> UT1_W {
        UT1_W::new(self)
    }
    #[doc = "Bits 12:13 - Action on PWM2B triggered by event TEZ when timer decreasing"]
    #[inline(always)]
    pub fn dtez(&mut self) -> DTEZ_W {
        DTEZ_W::new(self)
    }
    #[doc = "Bits 14:15 - Action on PWM2B triggered by event TEP when timer decreasing"]
    #[inline(always)]
    pub fn dtep(&mut self) -> DTEP_W {
        DTEP_W::new(self)
    }
    #[doc = "Bits 16:17 - Action on PWM2B triggered by event TEA when timer decreasing"]
    #[inline(always)]
    pub fn dtea(&mut self) -> DTEA_W {
        DTEA_W::new(self)
    }
    #[doc = "Bits 18:19 - Action on PWM2B triggered by event TEB when timer decreasing"]
    #[inline(always)]
    pub fn dteb(&mut self) -> DTEB_W {
        DTEB_W::new(self)
    }
    #[doc = "Bits 20:21 - Action on PWM2B triggered by event_t0 when timer decreasing"]
    #[inline(always)]
    pub fn dt0(&mut self) -> DT0_W {
        DT0_W::new(self)
    }
    #[doc = "Bits 22:23 - Action on PWM2B triggered by event_t1 when timer decreasing. 0: no change, 1: low, 2: high, 3: toggle"]
    #[inline(always)]
    pub fn dt1(&mut self) -> DT1_W {
        DT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Actions triggered by events on PWM2B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gen2_b](index.html) module"]
pub struct GEN2_B_SPEC;
impl crate::RegisterSpec for GEN2_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gen2_b::R](R) reader structure"]
impl crate::Readable for GEN2_B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gen2_b::W](W) writer structure"]
impl crate::Writable for GEN2_B_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GEN2_B to value 0"]
impl crate::Resettable for GEN2_B_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
