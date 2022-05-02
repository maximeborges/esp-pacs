#[doc = "Register `CORE_1_DRAM0_EXCEPTION_MONITOR_2` reader"]
pub struct R(crate::R<CORE_1_DRAM0_EXCEPTION_MONITOR_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_DRAM0_EXCEPTION_MONITOR_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_DRAM0_EXCEPTION_MONITOR_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_DRAM0_EXCEPTION_MONITOR_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_1_DRAM0_RECORDING_PC_0` reader - The first dram0's PC status when trigger DRAM busy interrupt"]
pub struct CORE_1_DRAM0_RECORDING_PC_0_R(crate::FieldReader<u32>);
impl CORE_1_DRAM0_RECORDING_PC_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CORE_1_DRAM0_RECORDING_PC_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_DRAM0_RECORDING_PC_0_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - The first dram0's PC status when trigger DRAM busy interrupt"]
    #[inline(always)]
    pub fn core_1_dram0_recording_pc_0(&self) -> CORE_1_DRAM0_RECORDING_PC_0_R {
        CORE_1_DRAM0_RECORDING_PC_0_R::new(self.bits)
    }
}
#[doc = "Core1 bus busy status regsiter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_dram0_exception_monitor_2](index.html) module"]
pub struct CORE_1_DRAM0_EXCEPTION_MONITOR_2_SPEC;
impl crate::RegisterSpec for CORE_1_DRAM0_EXCEPTION_MONITOR_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_dram0_exception_monitor_2::R](R) reader structure"]
impl crate::Readable for CORE_1_DRAM0_EXCEPTION_MONITOR_2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_1_DRAM0_EXCEPTION_MONITOR_2 to value 0xffff_ffff"]
impl crate::Resettable for CORE_1_DRAM0_EXCEPTION_MONITOR_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
