#[doc = "Register `CACHE_DBG_INT_CLR` writer"]
pub struct W(crate::W<CACHE_DBG_INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_DBG_INT_CLR_SPEC>;
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
impl From<crate::W<CACHE_DBG_INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_DBG_INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IBUS_ACS_MSK_IC_INT_CLR` writer - The bit is used to clear interrupt by cpu access icache while the corresponding ibus is disabled or icache is disabled which include speculative access."]
pub struct IBUS_ACS_MSK_IC_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> IBUS_ACS_MSK_IC_INT_CLR_W<'a> {
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
#[doc = "Field `IBUS_CNT_OVF_INT_CLR` writer - The bit is used to clear interrupt by ibus counter overflow."]
pub struct IBUS_CNT_OVF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> IBUS_CNT_OVF_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `IC_SYNC_SIZE_FAULT_INT_CLR` writer - The bit is used to clear interrupt by manual sync configurations fault."]
pub struct IC_SYNC_SIZE_FAULT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> IC_SYNC_SIZE_FAULT_INT_CLR_W<'a> {
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
#[doc = "Field `IC_PRELOAD_SIZE_FAULT_INT_CLR` writer - The bit is used to clear interrupt by manual pre-load configurations fault."]
pub struct IC_PRELOAD_SIZE_FAULT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> IC_PRELOAD_SIZE_FAULT_INT_CLR_W<'a> {
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
#[doc = "Field `ICACHE_REJECT_INT_CLR` writer - The bit is used to clear interrupt by authentication fail."]
pub struct ICACHE_REJECT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHE_REJECT_INT_CLR_W<'a> {
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
#[doc = "Field `ICACHE_SET_ILG_INT_CLR` writer - The bit is used to clear interrupt by illegal writing lock registers of icache while icache is busy to issue lock,sync or pre-load operations."]
pub struct ICACHE_SET_ILG_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHE_SET_ILG_INT_CLR_W<'a> {
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
#[doc = "Field `DBUS_ACS_MSK_DC_INT_CLR` writer - The bit is used to clear interrupt by cpu access dcache while the corresponding dbus is disabled or dcache is disabled which include speculative access."]
pub struct DBUS_ACS_MSK_DC_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DBUS_ACS_MSK_DC_INT_CLR_W<'a> {
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
#[doc = "Field `DBUS_CNT_OVF_INT_CLR` writer - The bit is used to clear interrupt by dbus counter overflow."]
pub struct DBUS_CNT_OVF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DBUS_CNT_OVF_INT_CLR_W<'a> {
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
#[doc = "Field `DC_SYNC_SIZE_FAULT_INT_CLR` writer - The bit is used to clear interrupt by manual sync configurations fault."]
pub struct DC_SYNC_SIZE_FAULT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_SYNC_SIZE_FAULT_INT_CLR_W<'a> {
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
#[doc = "Field `DC_PRELOAD_SIZE_FAULT_INT_CLR` writer - The bit is used to clear interrupt by manual pre-load configurations fault."]
pub struct DC_PRELOAD_SIZE_FAULT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_PRELOAD_SIZE_FAULT_INT_CLR_W<'a> {
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
#[doc = "Field `DCACHE_WRITE_FLASH_INT_CLR` writer - The bit is used to clear interrupt by dcache trying to write flash."]
pub struct DCACHE_WRITE_FLASH_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_WRITE_FLASH_INT_CLR_W<'a> {
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
#[doc = "Field `DCACHE_REJECT_INT_CLR` writer - The bit is used to clear interrupt by authentication fail."]
pub struct DCACHE_REJECT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_REJECT_INT_CLR_W<'a> {
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
#[doc = "Field `DCACHE_SET_ILG_INT_CLR` writer - The bit is used to clear interrupt by illegal writing lock registers of dcache while dcache is busy to issue lock,sync or pre-load operations."]
pub struct DCACHE_SET_ILG_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_SET_ILG_INT_CLR_W<'a> {
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
#[doc = "Field `MMU_ENTRY_FAULT_INT_CLR` writer - The bit is used to clear interrupt by mmu entry fault."]
pub struct MMU_ENTRY_FAULT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> MMU_ENTRY_FAULT_INT_CLR_W<'a> {
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
impl W {
    #[doc = "Bit 0 - The bit is used to clear interrupt by cpu access icache while the corresponding ibus is disabled or icache is disabled which include speculative access."]
    #[inline(always)]
    pub fn ibus_acs_msk_ic_int_clr(&mut self) -> IBUS_ACS_MSK_IC_INT_CLR_W {
        IBUS_ACS_MSK_IC_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1 - The bit is used to clear interrupt by ibus counter overflow."]
    #[inline(always)]
    pub fn ibus_cnt_ovf_int_clr(&mut self) -> IBUS_CNT_OVF_INT_CLR_W {
        IBUS_CNT_OVF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2 - The bit is used to clear interrupt by manual sync configurations fault."]
    #[inline(always)]
    pub fn ic_sync_size_fault_int_clr(&mut self) -> IC_SYNC_SIZE_FAULT_INT_CLR_W {
        IC_SYNC_SIZE_FAULT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3 - The bit is used to clear interrupt by manual pre-load configurations fault."]
    #[inline(always)]
    pub fn ic_preload_size_fault_int_clr(&mut self) -> IC_PRELOAD_SIZE_FAULT_INT_CLR_W {
        IC_PRELOAD_SIZE_FAULT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4 - The bit is used to clear interrupt by authentication fail."]
    #[inline(always)]
    pub fn icache_reject_int_clr(&mut self) -> ICACHE_REJECT_INT_CLR_W {
        ICACHE_REJECT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5 - The bit is used to clear interrupt by illegal writing lock registers of icache while icache is busy to issue lock,sync or pre-load operations."]
    #[inline(always)]
    pub fn icache_set_ilg_int_clr(&mut self) -> ICACHE_SET_ILG_INT_CLR_W {
        ICACHE_SET_ILG_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6 - The bit is used to clear interrupt by cpu access dcache while the corresponding dbus is disabled or dcache is disabled which include speculative access."]
    #[inline(always)]
    pub fn dbus_acs_msk_dc_int_clr(&mut self) -> DBUS_ACS_MSK_DC_INT_CLR_W {
        DBUS_ACS_MSK_DC_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7 - The bit is used to clear interrupt by dbus counter overflow."]
    #[inline(always)]
    pub fn dbus_cnt_ovf_int_clr(&mut self) -> DBUS_CNT_OVF_INT_CLR_W {
        DBUS_CNT_OVF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 8 - The bit is used to clear interrupt by manual sync configurations fault."]
    #[inline(always)]
    pub fn dc_sync_size_fault_int_clr(&mut self) -> DC_SYNC_SIZE_FAULT_INT_CLR_W {
        DC_SYNC_SIZE_FAULT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 9 - The bit is used to clear interrupt by manual pre-load configurations fault."]
    #[inline(always)]
    pub fn dc_preload_size_fault_int_clr(&mut self) -> DC_PRELOAD_SIZE_FAULT_INT_CLR_W {
        DC_PRELOAD_SIZE_FAULT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 10 - The bit is used to clear interrupt by dcache trying to write flash."]
    #[inline(always)]
    pub fn dcache_write_flash_int_clr(&mut self) -> DCACHE_WRITE_FLASH_INT_CLR_W {
        DCACHE_WRITE_FLASH_INT_CLR_W { w: self }
    }
    #[doc = "Bit 11 - The bit is used to clear interrupt by authentication fail."]
    #[inline(always)]
    pub fn dcache_reject_int_clr(&mut self) -> DCACHE_REJECT_INT_CLR_W {
        DCACHE_REJECT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 12 - The bit is used to clear interrupt by illegal writing lock registers of dcache while dcache is busy to issue lock,sync or pre-load operations."]
    #[inline(always)]
    pub fn dcache_set_ilg_int_clr(&mut self) -> DCACHE_SET_ILG_INT_CLR_W {
        DCACHE_SET_ILG_INT_CLR_W { w: self }
    }
    #[doc = "Bit 13 - The bit is used to clear interrupt by mmu entry fault."]
    #[inline(always)]
    pub fn mmu_entry_fault_int_clr(&mut self) -> MMU_ENTRY_FAULT_INT_CLR_W {
        MMU_ENTRY_FAULT_INT_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_dbg_int_clr]
(index.html) module"]
pub struct CACHE_DBG_INT_CLR_SPEC;
impl crate::RegisterSpec for CACHE_DBG_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cache_dbg_int_clr::W]
(W) writer structure"]
impl crate::Writable for CACHE_DBG_INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE_DBG_INT_CLR to value 0"]
impl crate::Resettable for CACHE_DBG_INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
