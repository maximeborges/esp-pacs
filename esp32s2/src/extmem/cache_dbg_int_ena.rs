#[doc = "Register `CACHE_DBG_INT_ENA` reader"]
pub struct R(crate::R<CACHE_DBG_INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_DBG_INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_DBG_INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_DBG_INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_DBG_INT_ENA` writer"]
pub struct W(crate::W<CACHE_DBG_INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_DBG_INT_ENA_SPEC>;
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
impl From<crate::W<CACHE_DBG_INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_DBG_INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHE_DBG_EN` reader - The bit is used to activate the cache track function. 1: enable, 0: disable."]
pub struct CACHE_DBG_EN_R(crate::FieldReader<bool>);
impl CACHE_DBG_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_DBG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_DBG_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_DBG_EN` writer - The bit is used to activate the cache track function. 1: enable, 0: disable."]
pub struct CACHE_DBG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_DBG_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `IBUS_ACS_MSK_IC_INT_ENA` reader - The bit is used to enable interrupt by cpu access icache while the corresponding ibus is disabled which include speculative access."]
pub struct IBUS_ACS_MSK_IC_INT_ENA_R(crate::FieldReader<bool>);
impl IBUS_ACS_MSK_IC_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IBUS_ACS_MSK_IC_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBUS_ACS_MSK_IC_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBUS_ACS_MSK_IC_INT_ENA` writer - The bit is used to enable interrupt by cpu access icache while the corresponding ibus is disabled which include speculative access."]
pub struct IBUS_ACS_MSK_IC_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> IBUS_ACS_MSK_IC_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `IBUS_CNT_OVF_INT_ENA` reader - The bit is used to enable interrupt by ibus counter overflow."]
pub struct IBUS_CNT_OVF_INT_ENA_R(crate::FieldReader<bool>);
impl IBUS_CNT_OVF_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IBUS_CNT_OVF_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBUS_CNT_OVF_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBUS_CNT_OVF_INT_ENA` writer - The bit is used to enable interrupt by ibus counter overflow."]
pub struct IBUS_CNT_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> IBUS_CNT_OVF_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `IC_SYNC_SIZE_FAULT_INT_ENA` reader - The bit is used to enable interrupt by manual sync configurations fault."]
pub struct IC_SYNC_SIZE_FAULT_INT_ENA_R(crate::FieldReader<bool>);
impl IC_SYNC_SIZE_FAULT_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IC_SYNC_SIZE_FAULT_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC_SYNC_SIZE_FAULT_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC_SYNC_SIZE_FAULT_INT_ENA` writer - The bit is used to enable interrupt by manual sync configurations fault."]
pub struct IC_SYNC_SIZE_FAULT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> IC_SYNC_SIZE_FAULT_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `IC_PRELOAD_SIZE_FAULT_INT_ENA` reader - The bit is used to enable interrupt by manual pre-load configurations fault."]
pub struct IC_PRELOAD_SIZE_FAULT_INT_ENA_R(crate::FieldReader<bool>);
impl IC_PRELOAD_SIZE_FAULT_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IC_PRELOAD_SIZE_FAULT_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC_PRELOAD_SIZE_FAULT_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC_PRELOAD_SIZE_FAULT_INT_ENA` writer - The bit is used to enable interrupt by manual pre-load configurations fault."]
pub struct IC_PRELOAD_SIZE_FAULT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> IC_PRELOAD_SIZE_FAULT_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `ICACHE_REJECT_INT_ENA` reader - The bit is used to enable interrupt by authentication fail."]
pub struct ICACHE_REJECT_INT_ENA_R(crate::FieldReader<bool>);
impl ICACHE_REJECT_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ICACHE_REJECT_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_REJECT_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_REJECT_INT_ENA` writer - The bit is used to enable interrupt by authentication fail."]
pub struct ICACHE_REJECT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHE_REJECT_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `ICACHE_SET_PRELOAD_ILG_INT_ENA` reader - The bit is used to enable interrupt by illegal writing preload registers of icache while icache is busy to issue lock,sync and pre-load operations."]
pub struct ICACHE_SET_PRELOAD_ILG_INT_ENA_R(crate::FieldReader<bool>);
impl ICACHE_SET_PRELOAD_ILG_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ICACHE_SET_PRELOAD_ILG_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_SET_PRELOAD_ILG_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_SET_PRELOAD_ILG_INT_ENA` writer - The bit is used to enable interrupt by illegal writing preload registers of icache while icache is busy to issue lock,sync and pre-load operations."]
pub struct ICACHE_SET_PRELOAD_ILG_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHE_SET_PRELOAD_ILG_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `ICACHE_SET_SYNC_ILG_INT_ENA` reader - The bit is used to enable interrupt by illegal writing sync registers of icache while icache is busy to issue lock,sync and pre-load operations."]
pub struct ICACHE_SET_SYNC_ILG_INT_ENA_R(crate::FieldReader<bool>);
impl ICACHE_SET_SYNC_ILG_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ICACHE_SET_SYNC_ILG_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_SET_SYNC_ILG_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_SET_SYNC_ILG_INT_ENA` writer - The bit is used to enable interrupt by illegal writing sync registers of icache while icache is busy to issue lock,sync and pre-load operations."]
pub struct ICACHE_SET_SYNC_ILG_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHE_SET_SYNC_ILG_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `ICACHE_SET_LOCK_ILG_INT_ENA` reader - The bit is used to enable interrupt by illegal writing lock registers of icache while icache is busy to issue lock,sync or pre-load operations."]
pub struct ICACHE_SET_LOCK_ILG_INT_ENA_R(crate::FieldReader<bool>);
impl ICACHE_SET_LOCK_ILG_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ICACHE_SET_LOCK_ILG_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_SET_LOCK_ILG_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_SET_LOCK_ILG_INT_ENA` writer - The bit is used to enable interrupt by illegal writing lock registers of icache while icache is busy to issue lock,sync or pre-load operations."]
pub struct ICACHE_SET_LOCK_ILG_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHE_SET_LOCK_ILG_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "Field `DBUS_ACS_MSK_DC_INT_ENA` reader - The bit is used to enable interrupt by cpu access dcache while the corresponding dbus is disabled which include speculative access."]
pub struct DBUS_ACS_MSK_DC_INT_ENA_R(crate::FieldReader<bool>);
impl DBUS_ACS_MSK_DC_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBUS_ACS_MSK_DC_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBUS_ACS_MSK_DC_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBUS_ACS_MSK_DC_INT_ENA` writer - The bit is used to enable interrupt by cpu access dcache while the corresponding dbus is disabled which include speculative access."]
pub struct DBUS_ACS_MSK_DC_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DBUS_ACS_MSK_DC_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "Field `DBUS_CNT_OVF_INT_ENA` reader - The bit is used to enable interrupt by dbus counter overflow."]
pub struct DBUS_CNT_OVF_INT_ENA_R(crate::FieldReader<bool>);
impl DBUS_CNT_OVF_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBUS_CNT_OVF_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBUS_CNT_OVF_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBUS_CNT_OVF_INT_ENA` writer - The bit is used to enable interrupt by dbus counter overflow."]
pub struct DBUS_CNT_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DBUS_CNT_OVF_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Field `DC_SYNC_SIZE_FAULT_INT_ENA` reader - The bit is used to enable interrupt by manual sync configurations fault."]
pub struct DC_SYNC_SIZE_FAULT_INT_ENA_R(crate::FieldReader<bool>);
impl DC_SYNC_SIZE_FAULT_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DC_SYNC_SIZE_FAULT_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC_SYNC_SIZE_FAULT_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DC_SYNC_SIZE_FAULT_INT_ENA` writer - The bit is used to enable interrupt by manual sync configurations fault."]
pub struct DC_SYNC_SIZE_FAULT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_SYNC_SIZE_FAULT_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
#[doc = "Field `DC_PRELOAD_SIZE_FAULT_INT_ENA` reader - The bit is used to enable interrupt by manual pre-load configurations fault."]
pub struct DC_PRELOAD_SIZE_FAULT_INT_ENA_R(crate::FieldReader<bool>);
impl DC_PRELOAD_SIZE_FAULT_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DC_PRELOAD_SIZE_FAULT_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC_PRELOAD_SIZE_FAULT_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DC_PRELOAD_SIZE_FAULT_INT_ENA` writer - The bit is used to enable interrupt by manual pre-load configurations fault."]
pub struct DC_PRELOAD_SIZE_FAULT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_PRELOAD_SIZE_FAULT_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `DCACHE_WRITE_FLASH_INT_ENA` reader - The bit is used to enable interrupt by dcache trying to write flash."]
pub struct DCACHE_WRITE_FLASH_INT_ENA_R(crate::FieldReader<bool>);
impl DCACHE_WRITE_FLASH_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCACHE_WRITE_FLASH_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_WRITE_FLASH_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_WRITE_FLASH_INT_ENA` writer - The bit is used to enable interrupt by dcache trying to write flash."]
pub struct DCACHE_WRITE_FLASH_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_WRITE_FLASH_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u32 & 1) << 14);
        self.w
    }
}
#[doc = "Field `DCACHE_REJECT_INT_ENA` reader - The bit is used to enable interrupt by authentication fail."]
pub struct DCACHE_REJECT_INT_ENA_R(crate::FieldReader<bool>);
impl DCACHE_REJECT_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCACHE_REJECT_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_REJECT_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_REJECT_INT_ENA` writer - The bit is used to enable interrupt by authentication fail."]
pub struct DCACHE_REJECT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_REJECT_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "Field `DCACHE_SET_PRELOAD_ILG_INT_ENA` reader - The bit is used to enable interrupt by illegal writing preload registers of dcache while dcache is busy to issue lock,sync and pre-load operations."]
pub struct DCACHE_SET_PRELOAD_ILG_INT_ENA_R(crate::FieldReader<bool>);
impl DCACHE_SET_PRELOAD_ILG_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCACHE_SET_PRELOAD_ILG_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_SET_PRELOAD_ILG_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_SET_PRELOAD_ILG_INT_ENA` writer - The bit is used to enable interrupt by illegal writing preload registers of dcache while dcache is busy to issue lock,sync and pre-load operations."]
pub struct DCACHE_SET_PRELOAD_ILG_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_SET_PRELOAD_ILG_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "Field `DCACHE_SET_SYNC_ILG_INT_ENA` reader - The bit is used to enable interrupt by illegal writing sync registers of dcache while dcache is busy to issue lock,sync and pre-load operations."]
pub struct DCACHE_SET_SYNC_ILG_INT_ENA_R(crate::FieldReader<bool>);
impl DCACHE_SET_SYNC_ILG_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCACHE_SET_SYNC_ILG_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_SET_SYNC_ILG_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_SET_SYNC_ILG_INT_ENA` writer - The bit is used to enable interrupt by illegal writing sync registers of dcache while dcache is busy to issue lock,sync and pre-load operations."]
pub struct DCACHE_SET_SYNC_ILG_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_SET_SYNC_ILG_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u32 & 1) << 17);
        self.w
    }
}
#[doc = "Field `DCACHE_SET_LOCK_ILG_INT_ENA` reader - The bit is used to enable interrupt by illegal writing lock registers of dcache while dcache is busy to issue lock,sync or pre-load operations."]
pub struct DCACHE_SET_LOCK_ILG_INT_ENA_R(crate::FieldReader<bool>);
impl DCACHE_SET_LOCK_ILG_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCACHE_SET_LOCK_ILG_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_SET_LOCK_ILG_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_SET_LOCK_ILG_INT_ENA` writer - The bit is used to enable interrupt by illegal writing lock registers of dcache while dcache is busy to issue lock,sync or pre-load operations."]
pub struct DCACHE_SET_LOCK_ILG_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_SET_LOCK_ILG_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 18)) | ((value as u32 & 1) << 18);
        self.w
    }
}
#[doc = "Field `MMU_ENTRY_FAULT_INT_ENA` reader - The bit is used to enable interrupt by mmu entry fault."]
pub struct MMU_ENTRY_FAULT_INT_ENA_R(crate::FieldReader<bool>);
impl MMU_ENTRY_FAULT_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MMU_ENTRY_FAULT_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMU_ENTRY_FAULT_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMU_ENTRY_FAULT_INT_ENA` writer - The bit is used to enable interrupt by mmu entry fault."]
pub struct MMU_ENTRY_FAULT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> MMU_ENTRY_FAULT_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 19)) | ((value as u32 & 1) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - The bit is used to activate the cache track function. 1: enable, 0: disable."]
    #[inline(always)]
    pub fn cache_dbg_en(&self) -> CACHE_DBG_EN_R {
        CACHE_DBG_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to enable interrupt by cpu access icache while the corresponding ibus is disabled which include speculative access."]
    #[inline(always)]
    pub fn ibus_acs_msk_ic_int_ena(&self) -> IBUS_ACS_MSK_IC_INT_ENA_R {
        IBUS_ACS_MSK_IC_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The bit is used to enable interrupt by ibus counter overflow."]
    #[inline(always)]
    pub fn ibus_cnt_ovf_int_ena(&self) -> IBUS_CNT_OVF_INT_ENA_R {
        IBUS_CNT_OVF_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to enable interrupt by manual sync configurations fault."]
    #[inline(always)]
    pub fn ic_sync_size_fault_int_ena(&self) -> IC_SYNC_SIZE_FAULT_INT_ENA_R {
        IC_SYNC_SIZE_FAULT_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The bit is used to enable interrupt by manual pre-load configurations fault."]
    #[inline(always)]
    pub fn ic_preload_size_fault_int_ena(&self) -> IC_PRELOAD_SIZE_FAULT_INT_ENA_R {
        IC_PRELOAD_SIZE_FAULT_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The bit is used to enable interrupt by authentication fail."]
    #[inline(always)]
    pub fn icache_reject_int_ena(&self) -> ICACHE_REJECT_INT_ENA_R {
        ICACHE_REJECT_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The bit is used to enable interrupt by illegal writing preload registers of icache while icache is busy to issue lock,sync and pre-load operations."]
    #[inline(always)]
    pub fn icache_set_preload_ilg_int_ena(&self) -> ICACHE_SET_PRELOAD_ILG_INT_ENA_R {
        ICACHE_SET_PRELOAD_ILG_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The bit is used to enable interrupt by illegal writing sync registers of icache while icache is busy to issue lock,sync and pre-load operations."]
    #[inline(always)]
    pub fn icache_set_sync_ilg_int_ena(&self) -> ICACHE_SET_SYNC_ILG_INT_ENA_R {
        ICACHE_SET_SYNC_ILG_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The bit is used to enable interrupt by illegal writing lock registers of icache while icache is busy to issue lock,sync or pre-load operations."]
    #[inline(always)]
    pub fn icache_set_lock_ilg_int_ena(&self) -> ICACHE_SET_LOCK_ILG_INT_ENA_R {
        ICACHE_SET_LOCK_ILG_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The bit is used to enable interrupt by cpu access dcache while the corresponding dbus is disabled which include speculative access."]
    #[inline(always)]
    pub fn dbus_acs_msk_dc_int_ena(&self) -> DBUS_ACS_MSK_DC_INT_ENA_R {
        DBUS_ACS_MSK_DC_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The bit is used to enable interrupt by dbus counter overflow."]
    #[inline(always)]
    pub fn dbus_cnt_ovf_int_ena(&self) -> DBUS_CNT_OVF_INT_ENA_R {
        DBUS_CNT_OVF_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The bit is used to enable interrupt by manual sync configurations fault."]
    #[inline(always)]
    pub fn dc_sync_size_fault_int_ena(&self) -> DC_SYNC_SIZE_FAULT_INT_ENA_R {
        DC_SYNC_SIZE_FAULT_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The bit is used to enable interrupt by manual pre-load configurations fault."]
    #[inline(always)]
    pub fn dc_preload_size_fault_int_ena(&self) -> DC_PRELOAD_SIZE_FAULT_INT_ENA_R {
        DC_PRELOAD_SIZE_FAULT_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The bit is used to enable interrupt by dcache trying to write flash."]
    #[inline(always)]
    pub fn dcache_write_flash_int_ena(&self) -> DCACHE_WRITE_FLASH_INT_ENA_R {
        DCACHE_WRITE_FLASH_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The bit is used to enable interrupt by authentication fail."]
    #[inline(always)]
    pub fn dcache_reject_int_ena(&self) -> DCACHE_REJECT_INT_ENA_R {
        DCACHE_REJECT_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The bit is used to enable interrupt by illegal writing preload registers of dcache while dcache is busy to issue lock,sync and pre-load operations."]
    #[inline(always)]
    pub fn dcache_set_preload_ilg_int_ena(&self) -> DCACHE_SET_PRELOAD_ILG_INT_ENA_R {
        DCACHE_SET_PRELOAD_ILG_INT_ENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The bit is used to enable interrupt by illegal writing sync registers of dcache while dcache is busy to issue lock,sync and pre-load operations."]
    #[inline(always)]
    pub fn dcache_set_sync_ilg_int_ena(&self) -> DCACHE_SET_SYNC_ILG_INT_ENA_R {
        DCACHE_SET_SYNC_ILG_INT_ENA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The bit is used to enable interrupt by illegal writing lock registers of dcache while dcache is busy to issue lock,sync or pre-load operations."]
    #[inline(always)]
    pub fn dcache_set_lock_ilg_int_ena(&self) -> DCACHE_SET_LOCK_ILG_INT_ENA_R {
        DCACHE_SET_LOCK_ILG_INT_ENA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The bit is used to enable interrupt by mmu entry fault."]
    #[inline(always)]
    pub fn mmu_entry_fault_int_ena(&self) -> MMU_ENTRY_FAULT_INT_ENA_R {
        MMU_ENTRY_FAULT_INT_ENA_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to activate the cache track function. 1: enable, 0: disable."]
    #[inline(always)]
    pub fn cache_dbg_en(&mut self) -> CACHE_DBG_EN_W {
        CACHE_DBG_EN_W { w: self }
    }
    #[doc = "Bit 2 - The bit is used to enable interrupt by cpu access icache while the corresponding ibus is disabled which include speculative access."]
    #[inline(always)]
    pub fn ibus_acs_msk_ic_int_ena(&mut self) -> IBUS_ACS_MSK_IC_INT_ENA_W {
        IBUS_ACS_MSK_IC_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3 - The bit is used to enable interrupt by ibus counter overflow."]
    #[inline(always)]
    pub fn ibus_cnt_ovf_int_ena(&mut self) -> IBUS_CNT_OVF_INT_ENA_W {
        IBUS_CNT_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4 - The bit is used to enable interrupt by manual sync configurations fault."]
    #[inline(always)]
    pub fn ic_sync_size_fault_int_ena(&mut self) -> IC_SYNC_SIZE_FAULT_INT_ENA_W {
        IC_SYNC_SIZE_FAULT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5 - The bit is used to enable interrupt by manual pre-load configurations fault."]
    #[inline(always)]
    pub fn ic_preload_size_fault_int_ena(&mut self) -> IC_PRELOAD_SIZE_FAULT_INT_ENA_W {
        IC_PRELOAD_SIZE_FAULT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6 - The bit is used to enable interrupt by authentication fail."]
    #[inline(always)]
    pub fn icache_reject_int_ena(&mut self) -> ICACHE_REJECT_INT_ENA_W {
        ICACHE_REJECT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7 - The bit is used to enable interrupt by illegal writing preload registers of icache while icache is busy to issue lock,sync and pre-load operations."]
    #[inline(always)]
    pub fn icache_set_preload_ilg_int_ena(&mut self) -> ICACHE_SET_PRELOAD_ILG_INT_ENA_W {
        ICACHE_SET_PRELOAD_ILG_INT_ENA_W { w: self }
    }
    #[doc = "Bit 8 - The bit is used to enable interrupt by illegal writing sync registers of icache while icache is busy to issue lock,sync and pre-load operations."]
    #[inline(always)]
    pub fn icache_set_sync_ilg_int_ena(&mut self) -> ICACHE_SET_SYNC_ILG_INT_ENA_W {
        ICACHE_SET_SYNC_ILG_INT_ENA_W { w: self }
    }
    #[doc = "Bit 9 - The bit is used to enable interrupt by illegal writing lock registers of icache while icache is busy to issue lock,sync or pre-load operations."]
    #[inline(always)]
    pub fn icache_set_lock_ilg_int_ena(&mut self) -> ICACHE_SET_LOCK_ILG_INT_ENA_W {
        ICACHE_SET_LOCK_ILG_INT_ENA_W { w: self }
    }
    #[doc = "Bit 10 - The bit is used to enable interrupt by cpu access dcache while the corresponding dbus is disabled which include speculative access."]
    #[inline(always)]
    pub fn dbus_acs_msk_dc_int_ena(&mut self) -> DBUS_ACS_MSK_DC_INT_ENA_W {
        DBUS_ACS_MSK_DC_INT_ENA_W { w: self }
    }
    #[doc = "Bit 11 - The bit is used to enable interrupt by dbus counter overflow."]
    #[inline(always)]
    pub fn dbus_cnt_ovf_int_ena(&mut self) -> DBUS_CNT_OVF_INT_ENA_W {
        DBUS_CNT_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 12 - The bit is used to enable interrupt by manual sync configurations fault."]
    #[inline(always)]
    pub fn dc_sync_size_fault_int_ena(&mut self) -> DC_SYNC_SIZE_FAULT_INT_ENA_W {
        DC_SYNC_SIZE_FAULT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 13 - The bit is used to enable interrupt by manual pre-load configurations fault."]
    #[inline(always)]
    pub fn dc_preload_size_fault_int_ena(&mut self) -> DC_PRELOAD_SIZE_FAULT_INT_ENA_W {
        DC_PRELOAD_SIZE_FAULT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 14 - The bit is used to enable interrupt by dcache trying to write flash."]
    #[inline(always)]
    pub fn dcache_write_flash_int_ena(&mut self) -> DCACHE_WRITE_FLASH_INT_ENA_W {
        DCACHE_WRITE_FLASH_INT_ENA_W { w: self }
    }
    #[doc = "Bit 15 - The bit is used to enable interrupt by authentication fail."]
    #[inline(always)]
    pub fn dcache_reject_int_ena(&mut self) -> DCACHE_REJECT_INT_ENA_W {
        DCACHE_REJECT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 16 - The bit is used to enable interrupt by illegal writing preload registers of dcache while dcache is busy to issue lock,sync and pre-load operations."]
    #[inline(always)]
    pub fn dcache_set_preload_ilg_int_ena(&mut self) -> DCACHE_SET_PRELOAD_ILG_INT_ENA_W {
        DCACHE_SET_PRELOAD_ILG_INT_ENA_W { w: self }
    }
    #[doc = "Bit 17 - The bit is used to enable interrupt by illegal writing sync registers of dcache while dcache is busy to issue lock,sync and pre-load operations."]
    #[inline(always)]
    pub fn dcache_set_sync_ilg_int_ena(&mut self) -> DCACHE_SET_SYNC_ILG_INT_ENA_W {
        DCACHE_SET_SYNC_ILG_INT_ENA_W { w: self }
    }
    #[doc = "Bit 18 - The bit is used to enable interrupt by illegal writing lock registers of dcache while dcache is busy to issue lock,sync or pre-load operations."]
    #[inline(always)]
    pub fn dcache_set_lock_ilg_int_ena(&mut self) -> DCACHE_SET_LOCK_ILG_INT_ENA_W {
        DCACHE_SET_LOCK_ILG_INT_ENA_W { w: self }
    }
    #[doc = "Bit 19 - The bit is used to enable interrupt by mmu entry fault."]
    #[inline(always)]
    pub fn mmu_entry_fault_int_ena(&mut self) -> MMU_ENTRY_FAULT_INT_ENA_W {
        MMU_ENTRY_FAULT_INT_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_dbg_int_ena](index.html) module"]
pub struct CACHE_DBG_INT_ENA_SPEC;
impl crate::RegisterSpec for CACHE_DBG_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_dbg_int_ena::R](R) reader structure"]
impl crate::Readable for CACHE_DBG_INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_dbg_int_ena::W](W) writer structure"]
impl crate::Writable for CACHE_DBG_INT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE_DBG_INT_ENA to value 0x01"]
impl crate::Resettable for CACHE_DBG_INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
