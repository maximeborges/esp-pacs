#[doc = "Register `CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1` reader"]
pub struct R(crate::R<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1` writer"]
pub struct W(crate::W<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC>;
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
impl From<crate::W<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1` reader - non busy cycle,for example: when cycle=100 and cycle=10,it means that in 100 cycle, if busy access success time less than 10, it will trigger interrutpt"]
pub type CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1` writer - non busy cycle,for example: when cycle=100 and cycle=10,it means that in 100 cycle, if busy access success time less than 10, it will trigger interrutpt"]
pub type CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_W<'a> =
    crate::FieldWriter<'a, u32, CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC, u32, u32, 20, 0>;
impl R {
    #[doc = "Bits 0:19 - non busy cycle,for example: when cycle=100 and cycle=10,it means that in 100 cycle, if busy access success time less than 10, it will trigger interrutpt"]
    #[inline(always)]
    pub fn core_x_iram0_dram0_limit_cycle_1(&self) -> CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_R {
        CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - non busy cycle,for example: when cycle=100 and cycle=10,it means that in 100 cycle, if busy access success time less than 10, it will trigger interrutpt"]
    #[inline(always)]
    pub fn core_x_iram0_dram0_limit_cycle_1(&mut self) -> CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_W {
        CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "bus busy configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_x_iram0_dram0_exception_monitor_1](index.html) module"]
pub struct CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC;
impl crate::RegisterSpec for CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_x_iram0_dram0_exception_monitor_1::R](R) reader structure"]
impl crate::Readable for CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_x_iram0_dram0_exception_monitor_1::W](W) writer structure"]
impl crate::Writable for CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1 to value 0x000f_ffff"]
impl crate::Resettable for CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000f_ffff
    }
}
