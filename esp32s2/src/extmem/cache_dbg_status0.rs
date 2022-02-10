#[doc = "Register `CACHE_DBG_STATUS0` reader"]
pub struct R(crate::R<CACHE_DBG_STATUS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_DBG_STATUS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_DBG_STATUS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_DBG_STATUS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IBUS0_ACS_MSK_ICACHE_ST` reader - The bit is used to indicate interrupt by cpu access icache while the ibus0 is disabled or icache is disabled which include speculative access."]
pub struct IBUS0_ACS_MSK_ICACHE_ST_R(crate::FieldReader<bool, bool>);
impl IBUS0_ACS_MSK_ICACHE_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IBUS0_ACS_MSK_ICACHE_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBUS0_ACS_MSK_ICACHE_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBUS1_ACS_MSK_ICACHE_ST` reader - The bit is used to indicate interrupt by cpu access icache while the ibus1 is disabled or icache is disabled which include speculative access."]
pub struct IBUS1_ACS_MSK_ICACHE_ST_R(crate::FieldReader<bool, bool>);
impl IBUS1_ACS_MSK_ICACHE_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IBUS1_ACS_MSK_ICACHE_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBUS1_ACS_MSK_ICACHE_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBUS2_ACS_MSK_ICACHE_ST` reader - The bit is used to indicate interrupt by cpu access icache while the ibus2 is disabled or icache is disabled which include speculative access."]
pub struct IBUS2_ACS_MSK_ICACHE_ST_R(crate::FieldReader<bool, bool>);
impl IBUS2_ACS_MSK_ICACHE_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IBUS2_ACS_MSK_ICACHE_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBUS2_ACS_MSK_ICACHE_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBUS0_ACS_CNT_OVF_ST` reader - The bit is used to indicate interrupt by ibus0 counter overflow."]
pub struct IBUS0_ACS_CNT_OVF_ST_R(crate::FieldReader<bool, bool>);
impl IBUS0_ACS_CNT_OVF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IBUS0_ACS_CNT_OVF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBUS0_ACS_CNT_OVF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBUS1_ACS_CNT_OVF_ST` reader - The bit is used to indicate interrupt by ibus1 counter overflow."]
pub struct IBUS1_ACS_CNT_OVF_ST_R(crate::FieldReader<bool, bool>);
impl IBUS1_ACS_CNT_OVF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IBUS1_ACS_CNT_OVF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBUS1_ACS_CNT_OVF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBUS2_ACS_CNT_OVF_ST` reader - The bit is used to indicate interrupt by ibus2 counter overflow."]
pub struct IBUS2_ACS_CNT_OVF_ST_R(crate::FieldReader<bool, bool>);
impl IBUS2_ACS_CNT_OVF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IBUS2_ACS_CNT_OVF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBUS2_ACS_CNT_OVF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBUS0_ACS_MISS_CNT_OVF_ST` reader - The bit is used to indicate interrupt by ibus0 miss counter overflow."]
pub struct IBUS0_ACS_MISS_CNT_OVF_ST_R(crate::FieldReader<bool, bool>);
impl IBUS0_ACS_MISS_CNT_OVF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IBUS0_ACS_MISS_CNT_OVF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBUS0_ACS_MISS_CNT_OVF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBUS1_ACS_MISS_CNT_OVF_ST` reader - The bit is used to indicate interrupt by ibus1 miss counter overflow."]
pub struct IBUS1_ACS_MISS_CNT_OVF_ST_R(crate::FieldReader<bool, bool>);
impl IBUS1_ACS_MISS_CNT_OVF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IBUS1_ACS_MISS_CNT_OVF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBUS1_ACS_MISS_CNT_OVF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBUS2_ACS_MISS_CNT_OVF_ST` reader - The bit is used to indicate interrupt by ibus2 miss counter overflow."]
pub struct IBUS2_ACS_MISS_CNT_OVF_ST_R(crate::FieldReader<bool, bool>);
impl IBUS2_ACS_MISS_CNT_OVF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IBUS2_ACS_MISS_CNT_OVF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBUS2_ACS_MISS_CNT_OVF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBUS0_ABANDON_CNT_OVF_ST` reader - The bit is used to indicate interrupt by ibus0 abandon counter overflow."]
pub struct IBUS0_ABANDON_CNT_OVF_ST_R(crate::FieldReader<bool, bool>);
impl IBUS0_ABANDON_CNT_OVF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IBUS0_ABANDON_CNT_OVF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBUS0_ABANDON_CNT_OVF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBUS1_ABANDON_CNT_OVF_ST` reader - The bit is used to indicate interrupt by ibus1 abandon counter overflow."]
pub struct IBUS1_ABANDON_CNT_OVF_ST_R(crate::FieldReader<bool, bool>);
impl IBUS1_ABANDON_CNT_OVF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IBUS1_ABANDON_CNT_OVF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBUS1_ABANDON_CNT_OVF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBUS2_ABANDON_CNT_OVF_ST` reader - The bit is used to indicate interrupt by ibus2 abandon counter overflow."]
pub struct IBUS2_ABANDON_CNT_OVF_ST_R(crate::FieldReader<bool, bool>);
impl IBUS2_ABANDON_CNT_OVF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IBUS2_ABANDON_CNT_OVF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBUS2_ABANDON_CNT_OVF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC_PRELOAD_MISS_CNT_OVF_ST` reader - The bit is used to indicate interrupt by pre-load miss counter overflow."]
pub struct IC_PRELOAD_MISS_CNT_OVF_ST_R(crate::FieldReader<bool, bool>);
impl IC_PRELOAD_MISS_CNT_OVF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IC_PRELOAD_MISS_CNT_OVF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC_PRELOAD_MISS_CNT_OVF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC_PRELOAD_CNT_OVF_ST` reader - The bit is used to indicate interrupt by pre-load counter overflow."]
pub struct IC_PRELOAD_CNT_OVF_ST_R(crate::FieldReader<bool, bool>);
impl IC_PRELOAD_CNT_OVF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IC_PRELOAD_CNT_OVF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC_PRELOAD_CNT_OVF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC_SYNC_SIZE_FAULT_ST` reader - The bit is used to indicate interrupt by manual sync configurations fault."]
pub struct IC_SYNC_SIZE_FAULT_ST_R(crate::FieldReader<bool, bool>);
impl IC_SYNC_SIZE_FAULT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IC_SYNC_SIZE_FAULT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC_SYNC_SIZE_FAULT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC_PRELOAD_SIZE_FAULT_ST` reader - The bit is used to indicate interrupt by manual pre-load configurations fault."]
pub struct IC_PRELOAD_SIZE_FAULT_ST_R(crate::FieldReader<bool, bool>);
impl IC_PRELOAD_SIZE_FAULT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IC_PRELOAD_SIZE_FAULT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC_PRELOAD_SIZE_FAULT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_REJECT_ST` reader - The bit is used to indicate interrupt by authentication fail."]
pub struct ICACHE_REJECT_ST_R(crate::FieldReader<bool, bool>);
impl ICACHE_REJECT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ICACHE_REJECT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_REJECT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_SET_PRELOAD_ILG_ST` reader - The bit is used to indicate interrupt by illegal writing preload registers of icache while icache is busy to issue lock,sync and pre-load operations."]
pub struct ICACHE_SET_PRELOAD_ILG_ST_R(crate::FieldReader<bool, bool>);
impl ICACHE_SET_PRELOAD_ILG_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ICACHE_SET_PRELOAD_ILG_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_SET_PRELOAD_ILG_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_SET_SYNC_ILG_ST` reader - The bit is used to indicate interrupt by illegal writing sync registers of icache while icache is busy to issue lock,sync and pre-load operations."]
pub struct ICACHE_SET_SYNC_ILG_ST_R(crate::FieldReader<bool, bool>);
impl ICACHE_SET_SYNC_ILG_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ICACHE_SET_SYNC_ILG_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_SET_SYNC_ILG_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_SET_LOCK_ILG_ST` reader - The bit is used to indicate interrupt by illegal writing lock registers of icache while icache is busy to issue lock,sync or pre-load operations."]
pub struct ICACHE_SET_LOCK_ILG_ST_R(crate::FieldReader<bool, bool>);
impl ICACHE_SET_LOCK_ILG_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ICACHE_SET_LOCK_ILG_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_SET_LOCK_ILG_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - The bit is used to indicate interrupt by cpu access icache while the ibus0 is disabled or icache is disabled which include speculative access."]
    #[inline(always)]
    pub fn ibus0_acs_msk_icache_st(&self) -> IBUS0_ACS_MSK_ICACHE_ST_R {
        IBUS0_ACS_MSK_ICACHE_ST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The bit is used to indicate interrupt by cpu access icache while the ibus1 is disabled or icache is disabled which include speculative access."]
    #[inline(always)]
    pub fn ibus1_acs_msk_icache_st(&self) -> IBUS1_ACS_MSK_ICACHE_ST_R {
        IBUS1_ACS_MSK_ICACHE_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The bit is used to indicate interrupt by cpu access icache while the ibus2 is disabled or icache is disabled which include speculative access."]
    #[inline(always)]
    pub fn ibus2_acs_msk_icache_st(&self) -> IBUS2_ACS_MSK_ICACHE_ST_R {
        IBUS2_ACS_MSK_ICACHE_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The bit is used to indicate interrupt by ibus0 counter overflow."]
    #[inline(always)]
    pub fn ibus0_acs_cnt_ovf_st(&self) -> IBUS0_ACS_CNT_OVF_ST_R {
        IBUS0_ACS_CNT_OVF_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - The bit is used to indicate interrupt by ibus1 counter overflow."]
    #[inline(always)]
    pub fn ibus1_acs_cnt_ovf_st(&self) -> IBUS1_ACS_CNT_OVF_ST_R {
        IBUS1_ACS_CNT_OVF_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The bit is used to indicate interrupt by ibus2 counter overflow."]
    #[inline(always)]
    pub fn ibus2_acs_cnt_ovf_st(&self) -> IBUS2_ACS_CNT_OVF_ST_R {
        IBUS2_ACS_CNT_OVF_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - The bit is used to indicate interrupt by ibus0 miss counter overflow."]
    #[inline(always)]
    pub fn ibus0_acs_miss_cnt_ovf_st(&self) -> IBUS0_ACS_MISS_CNT_OVF_ST_R {
        IBUS0_ACS_MISS_CNT_OVF_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - The bit is used to indicate interrupt by ibus1 miss counter overflow."]
    #[inline(always)]
    pub fn ibus1_acs_miss_cnt_ovf_st(&self) -> IBUS1_ACS_MISS_CNT_OVF_ST_R {
        IBUS1_ACS_MISS_CNT_OVF_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - The bit is used to indicate interrupt by ibus2 miss counter overflow."]
    #[inline(always)]
    pub fn ibus2_acs_miss_cnt_ovf_st(&self) -> IBUS2_ACS_MISS_CNT_OVF_ST_R {
        IBUS2_ACS_MISS_CNT_OVF_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - The bit is used to indicate interrupt by ibus0 abandon counter overflow."]
    #[inline(always)]
    pub fn ibus0_abandon_cnt_ovf_st(&self) -> IBUS0_ABANDON_CNT_OVF_ST_R {
        IBUS0_ABANDON_CNT_OVF_ST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - The bit is used to indicate interrupt by ibus1 abandon counter overflow."]
    #[inline(always)]
    pub fn ibus1_abandon_cnt_ovf_st(&self) -> IBUS1_ABANDON_CNT_OVF_ST_R {
        IBUS1_ABANDON_CNT_OVF_ST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - The bit is used to indicate interrupt by ibus2 abandon counter overflow."]
    #[inline(always)]
    pub fn ibus2_abandon_cnt_ovf_st(&self) -> IBUS2_ABANDON_CNT_OVF_ST_R {
        IBUS2_ABANDON_CNT_OVF_ST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - The bit is used to indicate interrupt by pre-load miss counter overflow."]
    #[inline(always)]
    pub fn ic_preload_miss_cnt_ovf_st(&self) -> IC_PRELOAD_MISS_CNT_OVF_ST_R {
        IC_PRELOAD_MISS_CNT_OVF_ST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - The bit is used to indicate interrupt by pre-load counter overflow."]
    #[inline(always)]
    pub fn ic_preload_cnt_ovf_st(&self) -> IC_PRELOAD_CNT_OVF_ST_R {
        IC_PRELOAD_CNT_OVF_ST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - The bit is used to indicate interrupt by manual sync configurations fault."]
    #[inline(always)]
    pub fn ic_sync_size_fault_st(&self) -> IC_SYNC_SIZE_FAULT_ST_R {
        IC_SYNC_SIZE_FAULT_ST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - The bit is used to indicate interrupt by manual pre-load configurations fault."]
    #[inline(always)]
    pub fn ic_preload_size_fault_st(&self) -> IC_PRELOAD_SIZE_FAULT_ST_R {
        IC_PRELOAD_SIZE_FAULT_ST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - The bit is used to indicate interrupt by authentication fail."]
    #[inline(always)]
    pub fn icache_reject_st(&self) -> ICACHE_REJECT_ST_R {
        ICACHE_REJECT_ST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - The bit is used to indicate interrupt by illegal writing preload registers of icache while icache is busy to issue lock,sync and pre-load operations."]
    #[inline(always)]
    pub fn icache_set_preload_ilg_st(&self) -> ICACHE_SET_PRELOAD_ILG_ST_R {
        ICACHE_SET_PRELOAD_ILG_ST_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - The bit is used to indicate interrupt by illegal writing sync registers of icache while icache is busy to issue lock,sync and pre-load operations."]
    #[inline(always)]
    pub fn icache_set_sync_ilg_st(&self) -> ICACHE_SET_SYNC_ILG_ST_R {
        ICACHE_SET_SYNC_ILG_ST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - The bit is used to indicate interrupt by illegal writing lock registers of icache while icache is busy to issue lock,sync or pre-load operations."]
    #[inline(always)]
    pub fn icache_set_lock_ilg_st(&self) -> ICACHE_SET_LOCK_ILG_ST_R {
        ICACHE_SET_LOCK_ILG_ST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
#[doc = "register description\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_dbg_status0]
(index.html) module"]
pub struct CACHE_DBG_STATUS0_SPEC;
impl crate::RegisterSpec for CACHE_DBG_STATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_dbg_status0::R]
(R) reader structure"]
impl crate::Readable for CACHE_DBG_STATUS0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CACHE_DBG_STATUS0 to value 0"]
impl crate::Resettable for CACHE_DBG_STATUS0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
