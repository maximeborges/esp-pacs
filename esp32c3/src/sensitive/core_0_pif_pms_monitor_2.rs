#[doc = "Register `CORE_0_PIF_PMS_MONITOR_2` reader"]
pub struct R(crate::R<CORE_0_PIF_PMS_MONITOR_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_PIF_PMS_MONITOR_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_PIF_PMS_MONITOR_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_PIF_PMS_MONITOR_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR` reader - core_0_pif_pms_monitor_violate_intr"]
pub struct CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_R(crate::FieldReader<bool, bool>);
impl CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HPORT_0` reader - core_0_pif_pms_monitor_violate_status_hport_0"]
pub struct CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HPORT_0_R(crate::FieldReader<bool, bool>);
impl CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HPORT_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HPORT_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HPORT_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HSIZE` reader - core_0_pif_pms_monitor_violate_status_hsize"]
pub struct CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HSIZE_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HWRITE` reader - core_0_pif_pms_monitor_violate_status_hwrite"]
pub struct CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HWRITE_R(crate::FieldReader<bool, bool>);
impl CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HWRITE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HWRITE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HWRITE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HWORLD` reader - core_0_pif_pms_monitor_violate_status_hworld"]
pub struct CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HWORLD_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HWORLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HWORLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HWORLD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - core_0_pif_pms_monitor_violate_intr"]
    #[inline(always)]
    pub fn core_0_pif_pms_monitor_violate_intr(&self) -> CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_R {
        CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - core_0_pif_pms_monitor_violate_status_hport_0"]
    #[inline(always)]
    pub fn core_0_pif_pms_monitor_violate_status_hport_0(
        &self,
    ) -> CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HPORT_0_R {
        CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HPORT_0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - core_0_pif_pms_monitor_violate_status_hsize"]
    #[inline(always)]
    pub fn core_0_pif_pms_monitor_violate_status_hsize(
        &self,
    ) -> CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HSIZE_R {
        CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HSIZE_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 5 - core_0_pif_pms_monitor_violate_status_hwrite"]
    #[inline(always)]
    pub fn core_0_pif_pms_monitor_violate_status_hwrite(
        &self,
    ) -> CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HWRITE_R {
        CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HWRITE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - core_0_pif_pms_monitor_violate_status_hworld"]
    #[inline(always)]
    pub fn core_0_pif_pms_monitor_violate_status_hworld(
        &self,
    ) -> CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HWORLD_R {
        CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HWORLD_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_2_REG\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_pif_pms_monitor_2]
(index.html) module"]
pub struct CORE_0_PIF_PMS_MONITOR_2_SPEC;
impl crate::RegisterSpec for CORE_0_PIF_PMS_MONITOR_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_pif_pms_monitor_2::R]
(R) reader structure"]
impl crate::Readable for CORE_0_PIF_PMS_MONITOR_2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_0_PIF_PMS_MONITOR_2 to value 0"]
impl crate::Resettable for CORE_0_PIF_PMS_MONITOR_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
