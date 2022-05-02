#[doc = "Register `CORE_0_PIF_PMS_MONITOR_5` reader"]
pub struct R(crate::R<CORE_0_PIF_PMS_MONITOR_5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_PIF_PMS_MONITOR_5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_PIF_PMS_MONITOR_5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_PIF_PMS_MONITOR_5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_INTR` reader - Record core0 unsupported access type interrupt state."]
pub struct CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_INTR_R(crate::FieldReader<bool>);
impl CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_INTR_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HSIZE` reader - Record access type when core0 initiate unsupported access type."]
pub struct CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HSIZE_R(crate::FieldReader<u8>);
impl CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HSIZE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HWORLD` reader - Record world information when core0 initiate unsupported access type."]
pub struct CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HWORLD_R(crate::FieldReader<u8>);
impl CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HWORLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HWORLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HWORLD_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Record core0 unsupported access type interrupt state."]
    #[inline(always)]
    pub fn core_0_pif_pms_monitor_nonword_violate_intr(
        &self,
    ) -> CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_INTR_R {
        CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_INTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Record access type when core0 initiate unsupported access type."]
    #[inline(always)]
    pub fn core_0_pif_pms_monitor_nonword_violate_status_hsize(
        &self,
    ) -> CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HSIZE_R {
        CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HSIZE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Record world information when core0 initiate unsupported access type."]
    #[inline(always)]
    pub fn core_0_pif_pms_monitor_nonword_violate_status_hworld(
        &self,
    ) -> CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HWORLD_R {
        CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HWORLD_R::new(((self.bits >> 3) & 3) as u8)
    }
}
#[doc = "Core0 permission report register 5.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_pif_pms_monitor_5](index.html) module"]
pub struct CORE_0_PIF_PMS_MONITOR_5_SPEC;
impl crate::RegisterSpec for CORE_0_PIF_PMS_MONITOR_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_pif_pms_monitor_5::R](R) reader structure"]
impl crate::Readable for CORE_0_PIF_PMS_MONITOR_5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_0_PIF_PMS_MONITOR_5 to value 0"]
impl crate::Resettable for CORE_0_PIF_PMS_MONITOR_5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
