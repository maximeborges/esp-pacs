#[doc = "Register `CACHE_DBG_STATUS1` reader"]
pub struct R(crate::R<CACHE_DBG_STATUS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_DBG_STATUS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_DBG_STATUS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_DBG_STATUS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DBUS0_ACS_MSK_DCACHE_ST` reader - The bit is used to indicate interrupt by cpu access dcache while the dbus0 is disabled or dcache is disabled which include speculative access."]
pub struct DBUS0_ACS_MSK_DCACHE_ST_R(crate::FieldReader<bool, bool>);
impl DBUS0_ACS_MSK_DCACHE_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBUS0_ACS_MSK_DCACHE_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBUS0_ACS_MSK_DCACHE_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBUS1_ACS_MSK_DCACHE_ST` reader - The bit is used to indicate interrupt by cpu access dcache while the dbus1 is disabled or dcache is disabled which include speculative access."]
pub struct DBUS1_ACS_MSK_DCACHE_ST_R(crate::FieldReader<bool, bool>);
impl DBUS1_ACS_MSK_DCACHE_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBUS1_ACS_MSK_DCACHE_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBUS1_ACS_MSK_DCACHE_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBUS2_ACS_MSK_DCACHE_ST` reader - The bit is used to indicate interrupt by cpu access dcache while the dbus2 is disabled or dcache is disabled which include speculative access."]
pub struct DBUS2_ACS_MSK_DCACHE_ST_R(crate::FieldReader<bool, bool>);
impl DBUS2_ACS_MSK_DCACHE_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBUS2_ACS_MSK_DCACHE_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBUS2_ACS_MSK_DCACHE_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBUS0_ACS_CNT_OVF_ST` reader - The bit is used to indicate interrupt by dbus0 counter overflow."]
pub struct DBUS0_ACS_CNT_OVF_ST_R(crate::FieldReader<bool, bool>);
impl DBUS0_ACS_CNT_OVF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBUS0_ACS_CNT_OVF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBUS0_ACS_CNT_OVF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBUS1_ACS_CNT_OVF_ST` reader - The bit is used to indicate interrupt by dbus1 counter overflow."]
pub struct DBUS1_ACS_CNT_OVF_ST_R(crate::FieldReader<bool, bool>);
impl DBUS1_ACS_CNT_OVF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBUS1_ACS_CNT_OVF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBUS1_ACS_CNT_OVF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBUS2_ACS_CNT_OVF_ST` reader - The bit is used to indicate interrupt by dbus2 counter overflow."]
pub struct DBUS2_ACS_CNT_OVF_ST_R(crate::FieldReader<bool, bool>);
impl DBUS2_ACS_CNT_OVF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBUS2_ACS_CNT_OVF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBUS2_ACS_CNT_OVF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBUS0_ACS_MISS_CNT_OVF_ST` reader - The bit is used to indicate interrupt by dbus0 miss counter overflow."]
pub struct DBUS0_ACS_MISS_CNT_OVF_ST_R(crate::FieldReader<bool, bool>);
impl DBUS0_ACS_MISS_CNT_OVF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBUS0_ACS_MISS_CNT_OVF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBUS0_ACS_MISS_CNT_OVF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBUS1_ACS_MISS_CNT_OVF_ST` reader - The bit is used to indicate interrupt by dbus1 miss counter overflow."]
pub struct DBUS1_ACS_MISS_CNT_OVF_ST_R(crate::FieldReader<bool, bool>);
impl DBUS1_ACS_MISS_CNT_OVF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBUS1_ACS_MISS_CNT_OVF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBUS1_ACS_MISS_CNT_OVF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBUS2_ACS_MISS_CNT_OVF_ST` reader - The bit is used to indicate interrupt by dbus2 miss counter overflow."]
pub struct DBUS2_ACS_MISS_CNT_OVF_ST_R(crate::FieldReader<bool, bool>);
impl DBUS2_ACS_MISS_CNT_OVF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBUS2_ACS_MISS_CNT_OVF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBUS2_ACS_MISS_CNT_OVF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBUS0_ACS_WB_CNT_OVF_ST` reader - The bit is used to indicate interrupt by dbus0 eviction counter overflow."]
pub struct DBUS0_ACS_WB_CNT_OVF_ST_R(crate::FieldReader<bool, bool>);
impl DBUS0_ACS_WB_CNT_OVF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBUS0_ACS_WB_CNT_OVF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBUS0_ACS_WB_CNT_OVF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBUS1_ACS_WB_CNT_OVF_ST` reader - The bit is used to indicate interrupt by dbus1 eviction counter overflow."]
pub struct DBUS1_ACS_WB_CNT_OVF_ST_R(crate::FieldReader<bool, bool>);
impl DBUS1_ACS_WB_CNT_OVF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBUS1_ACS_WB_CNT_OVF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBUS1_ACS_WB_CNT_OVF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBUS2_ACS_WB_CNT_OVF_ST` reader - The bit is used to indicate interrupt by dbus2 eviction counter overflow."]
pub struct DBUS2_ACS_WB_CNT_OVF_ST_R(crate::FieldReader<bool, bool>);
impl DBUS2_ACS_WB_CNT_OVF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBUS2_ACS_WB_CNT_OVF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBUS2_ACS_WB_CNT_OVF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBUS0_ABANDON_CNT_OVF_ST` reader - The bit is used to indicate interrupt by dbus0 abandon counter overflow."]
pub struct DBUS0_ABANDON_CNT_OVF_ST_R(crate::FieldReader<bool, bool>);
impl DBUS0_ABANDON_CNT_OVF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBUS0_ABANDON_CNT_OVF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBUS0_ABANDON_CNT_OVF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBUS1_ABANDON_CNT_OVF_ST` reader - The bit is used to indicate interrupt by dbus1 abandon counter overflow."]
pub struct DBUS1_ABANDON_CNT_OVF_ST_R(crate::FieldReader<bool, bool>);
impl DBUS1_ABANDON_CNT_OVF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBUS1_ABANDON_CNT_OVF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBUS1_ABANDON_CNT_OVF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBUS2_ABANDON_CNT_OVF_ST` reader - The bit is used to indicate interrupt by dbus2 abandon counter overflow."]
pub struct DBUS2_ABANDON_CNT_OVF_ST_R(crate::FieldReader<bool, bool>);
impl DBUS2_ABANDON_CNT_OVF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBUS2_ABANDON_CNT_OVF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBUS2_ABANDON_CNT_OVF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DC_PRELOAD_MISS_CNT_OVF_ST` reader - The bit is used to indicate interrupt by pre-load miss counter overflow."]
pub struct DC_PRELOAD_MISS_CNT_OVF_ST_R(crate::FieldReader<bool, bool>);
impl DC_PRELOAD_MISS_CNT_OVF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DC_PRELOAD_MISS_CNT_OVF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC_PRELOAD_MISS_CNT_OVF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DC_PRELOAD_EVICT_CNT_OVF_ST` reader - The bit is used to indicate interrupt by pre-load eviction counter overflow."]
pub struct DC_PRELOAD_EVICT_CNT_OVF_ST_R(crate::FieldReader<bool, bool>);
impl DC_PRELOAD_EVICT_CNT_OVF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DC_PRELOAD_EVICT_CNT_OVF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC_PRELOAD_EVICT_CNT_OVF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DC_PRELOAD_CNT_OVF_ST` reader - The bit is used to indicate interrupt by pre-load counter overflow."]
pub struct DC_PRELOAD_CNT_OVF_ST_R(crate::FieldReader<bool, bool>);
impl DC_PRELOAD_CNT_OVF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DC_PRELOAD_CNT_OVF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC_PRELOAD_CNT_OVF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DC_SYNC_SIZE_FAULT_ST` reader - The bit is used to indicate interrupt by manual sync configurations fault."]
pub struct DC_SYNC_SIZE_FAULT_ST_R(crate::FieldReader<bool, bool>);
impl DC_SYNC_SIZE_FAULT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DC_SYNC_SIZE_FAULT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC_SYNC_SIZE_FAULT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DC_PRELOAD_SIZE_FAULT_ST` reader - The bit is used to indicate interrupt by manual pre-load configurations fault."]
pub struct DC_PRELOAD_SIZE_FAULT_ST_R(crate::FieldReader<bool, bool>);
impl DC_PRELOAD_SIZE_FAULT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DC_PRELOAD_SIZE_FAULT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC_PRELOAD_SIZE_FAULT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_WRITE_FLASH_ST` reader - The bit is used to indicate interrupt by dcache trying to write flash."]
pub struct DCACHE_WRITE_FLASH_ST_R(crate::FieldReader<bool, bool>);
impl DCACHE_WRITE_FLASH_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCACHE_WRITE_FLASH_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_WRITE_FLASH_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_REJECT_ST` reader - The bit is used to indicate interrupt by authentication fail."]
pub struct DCACHE_REJECT_ST_R(crate::FieldReader<bool, bool>);
impl DCACHE_REJECT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCACHE_REJECT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_REJECT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_SET_PRELOAD_ILG_ST` reader - The bit is used to indicate interrupt by illegal writing preload registers of icache while icache is busy to issue lock,sync and pre-load operations."]
pub struct DCACHE_SET_PRELOAD_ILG_ST_R(crate::FieldReader<bool, bool>);
impl DCACHE_SET_PRELOAD_ILG_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCACHE_SET_PRELOAD_ILG_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_SET_PRELOAD_ILG_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_SET_SYNC_ILG_ST` reader - The bit is used to indicate interrupt by illegal writing sync registers of icache while icache is busy to issue lock,sync and pre-load operations."]
pub struct DCACHE_SET_SYNC_ILG_ST_R(crate::FieldReader<bool, bool>);
impl DCACHE_SET_SYNC_ILG_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCACHE_SET_SYNC_ILG_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_SET_SYNC_ILG_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_SET_LOCK_ILG_ST` reader - The bit is used to indicate interrupt by illegal writing lock registers of icache while icache is busy to issue lock,sync or pre-load operations."]
pub struct DCACHE_SET_LOCK_ILG_ST_R(crate::FieldReader<bool, bool>);
impl DCACHE_SET_LOCK_ILG_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCACHE_SET_LOCK_ILG_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_SET_LOCK_ILG_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMU_ENTRY_FAULT_ST` reader - The bit is used to indicate interrupt by mmu entry fault."]
pub struct MMU_ENTRY_FAULT_ST_R(crate::FieldReader<bool, bool>);
impl MMU_ENTRY_FAULT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MMU_ENTRY_FAULT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMU_ENTRY_FAULT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - The bit is used to indicate interrupt by cpu access dcache while the dbus0 is disabled or dcache is disabled which include speculative access."]
    #[inline(always)]
    pub fn dbus0_acs_msk_dcache_st(&self) -> DBUS0_ACS_MSK_DCACHE_ST_R {
        DBUS0_ACS_MSK_DCACHE_ST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The bit is used to indicate interrupt by cpu access dcache while the dbus1 is disabled or dcache is disabled which include speculative access."]
    #[inline(always)]
    pub fn dbus1_acs_msk_dcache_st(&self) -> DBUS1_ACS_MSK_DCACHE_ST_R {
        DBUS1_ACS_MSK_DCACHE_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The bit is used to indicate interrupt by cpu access dcache while the dbus2 is disabled or dcache is disabled which include speculative access."]
    #[inline(always)]
    pub fn dbus2_acs_msk_dcache_st(&self) -> DBUS2_ACS_MSK_DCACHE_ST_R {
        DBUS2_ACS_MSK_DCACHE_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The bit is used to indicate interrupt by dbus0 counter overflow."]
    #[inline(always)]
    pub fn dbus0_acs_cnt_ovf_st(&self) -> DBUS0_ACS_CNT_OVF_ST_R {
        DBUS0_ACS_CNT_OVF_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - The bit is used to indicate interrupt by dbus1 counter overflow."]
    #[inline(always)]
    pub fn dbus1_acs_cnt_ovf_st(&self) -> DBUS1_ACS_CNT_OVF_ST_R {
        DBUS1_ACS_CNT_OVF_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The bit is used to indicate interrupt by dbus2 counter overflow."]
    #[inline(always)]
    pub fn dbus2_acs_cnt_ovf_st(&self) -> DBUS2_ACS_CNT_OVF_ST_R {
        DBUS2_ACS_CNT_OVF_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - The bit is used to indicate interrupt by dbus0 miss counter overflow."]
    #[inline(always)]
    pub fn dbus0_acs_miss_cnt_ovf_st(&self) -> DBUS0_ACS_MISS_CNT_OVF_ST_R {
        DBUS0_ACS_MISS_CNT_OVF_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - The bit is used to indicate interrupt by dbus1 miss counter overflow."]
    #[inline(always)]
    pub fn dbus1_acs_miss_cnt_ovf_st(&self) -> DBUS1_ACS_MISS_CNT_OVF_ST_R {
        DBUS1_ACS_MISS_CNT_OVF_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - The bit is used to indicate interrupt by dbus2 miss counter overflow."]
    #[inline(always)]
    pub fn dbus2_acs_miss_cnt_ovf_st(&self) -> DBUS2_ACS_MISS_CNT_OVF_ST_R {
        DBUS2_ACS_MISS_CNT_OVF_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - The bit is used to indicate interrupt by dbus0 eviction counter overflow."]
    #[inline(always)]
    pub fn dbus0_acs_wb_cnt_ovf_st(&self) -> DBUS0_ACS_WB_CNT_OVF_ST_R {
        DBUS0_ACS_WB_CNT_OVF_ST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - The bit is used to indicate interrupt by dbus1 eviction counter overflow."]
    #[inline(always)]
    pub fn dbus1_acs_wb_cnt_ovf_st(&self) -> DBUS1_ACS_WB_CNT_OVF_ST_R {
        DBUS1_ACS_WB_CNT_OVF_ST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - The bit is used to indicate interrupt by dbus2 eviction counter overflow."]
    #[inline(always)]
    pub fn dbus2_acs_wb_cnt_ovf_st(&self) -> DBUS2_ACS_WB_CNT_OVF_ST_R {
        DBUS2_ACS_WB_CNT_OVF_ST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - The bit is used to indicate interrupt by dbus0 abandon counter overflow."]
    #[inline(always)]
    pub fn dbus0_abandon_cnt_ovf_st(&self) -> DBUS0_ABANDON_CNT_OVF_ST_R {
        DBUS0_ABANDON_CNT_OVF_ST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - The bit is used to indicate interrupt by dbus1 abandon counter overflow."]
    #[inline(always)]
    pub fn dbus1_abandon_cnt_ovf_st(&self) -> DBUS1_ABANDON_CNT_OVF_ST_R {
        DBUS1_ABANDON_CNT_OVF_ST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - The bit is used to indicate interrupt by dbus2 abandon counter overflow."]
    #[inline(always)]
    pub fn dbus2_abandon_cnt_ovf_st(&self) -> DBUS2_ABANDON_CNT_OVF_ST_R {
        DBUS2_ABANDON_CNT_OVF_ST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - The bit is used to indicate interrupt by pre-load miss counter overflow."]
    #[inline(always)]
    pub fn dc_preload_miss_cnt_ovf_st(&self) -> DC_PRELOAD_MISS_CNT_OVF_ST_R {
        DC_PRELOAD_MISS_CNT_OVF_ST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - The bit is used to indicate interrupt by pre-load eviction counter overflow."]
    #[inline(always)]
    pub fn dc_preload_evict_cnt_ovf_st(&self) -> DC_PRELOAD_EVICT_CNT_OVF_ST_R {
        DC_PRELOAD_EVICT_CNT_OVF_ST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - The bit is used to indicate interrupt by pre-load counter overflow."]
    #[inline(always)]
    pub fn dc_preload_cnt_ovf_st(&self) -> DC_PRELOAD_CNT_OVF_ST_R {
        DC_PRELOAD_CNT_OVF_ST_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - The bit is used to indicate interrupt by manual sync configurations fault."]
    #[inline(always)]
    pub fn dc_sync_size_fault_st(&self) -> DC_SYNC_SIZE_FAULT_ST_R {
        DC_SYNC_SIZE_FAULT_ST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - The bit is used to indicate interrupt by manual pre-load configurations fault."]
    #[inline(always)]
    pub fn dc_preload_size_fault_st(&self) -> DC_PRELOAD_SIZE_FAULT_ST_R {
        DC_PRELOAD_SIZE_FAULT_ST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - The bit is used to indicate interrupt by dcache trying to write flash."]
    #[inline(always)]
    pub fn dcache_write_flash_st(&self) -> DCACHE_WRITE_FLASH_ST_R {
        DCACHE_WRITE_FLASH_ST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - The bit is used to indicate interrupt by authentication fail."]
    #[inline(always)]
    pub fn dcache_reject_st(&self) -> DCACHE_REJECT_ST_R {
        DCACHE_REJECT_ST_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - The bit is used to indicate interrupt by illegal writing preload registers of icache while icache is busy to issue lock,sync and pre-load operations."]
    #[inline(always)]
    pub fn dcache_set_preload_ilg_st(&self) -> DCACHE_SET_PRELOAD_ILG_ST_R {
        DCACHE_SET_PRELOAD_ILG_ST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - The bit is used to indicate interrupt by illegal writing sync registers of icache while icache is busy to issue lock,sync and pre-load operations."]
    #[inline(always)]
    pub fn dcache_set_sync_ilg_st(&self) -> DCACHE_SET_SYNC_ILG_ST_R {
        DCACHE_SET_SYNC_ILG_ST_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - The bit is used to indicate interrupt by illegal writing lock registers of icache while icache is busy to issue lock,sync or pre-load operations."]
    #[inline(always)]
    pub fn dcache_set_lock_ilg_st(&self) -> DCACHE_SET_LOCK_ILG_ST_R {
        DCACHE_SET_LOCK_ILG_ST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - The bit is used to indicate interrupt by mmu entry fault."]
    #[inline(always)]
    pub fn mmu_entry_fault_st(&self) -> MMU_ENTRY_FAULT_ST_R {
        MMU_ENTRY_FAULT_ST_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
#[doc = "register description\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_dbg_status1]
(index.html) module"]
pub struct CACHE_DBG_STATUS1_SPEC;
impl crate::RegisterSpec for CACHE_DBG_STATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_dbg_status1::R]
(R) reader structure"]
impl crate::Readable for CACHE_DBG_STATUS1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CACHE_DBG_STATUS1 to value 0"]
impl crate::Resettable for CACHE_DBG_STATUS1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
