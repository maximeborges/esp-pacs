#[doc = "Register `CACHE_ILG_INT_CLR` writer"]
pub struct W(crate::W<CACHE_ILG_INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_ILG_INT_CLR_SPEC>;
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
impl From<crate::W<CACHE_ILG_INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_ILG_INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICACHE_SYNC_OP_FAULT_INT_CLR` writer - The bit is used to clear interrupt by sync configurations fault."]
pub struct ICACHE_SYNC_OP_FAULT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHE_SYNC_OP_FAULT_INT_CLR_W<'a> {
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
#[doc = "Field `ICACHE_PRELOAD_OP_FAULT_INT_CLR` writer - The bit is used to clear interrupt by preload configurations fault."]
pub struct ICACHE_PRELOAD_OP_FAULT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHE_PRELOAD_OP_FAULT_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to clear interrupt by sync configurations fault."]
    #[inline(always)]
    pub fn icache_sync_op_fault_int_clr(&mut self) -> ICACHE_SYNC_OP_FAULT_INT_CLR_W {
        ICACHE_SYNC_OP_FAULT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1 - The bit is used to clear interrupt by preload configurations fault."]
    #[inline(always)]
    pub fn icache_preload_op_fault_int_clr(&mut self) -> ICACHE_PRELOAD_OP_FAULT_INT_CLR_W {
        ICACHE_PRELOAD_OP_FAULT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5 - The bit is used to clear interrupt by mmu entry fault."]
    #[inline(always)]
    pub fn mmu_entry_fault_int_clr(&mut self) -> MMU_ENTRY_FAULT_INT_CLR_W {
        MMU_ENTRY_FAULT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7 - The bit is used to clear interrupt by ibus counter overflow."]
    #[inline(always)]
    pub fn ibus_cnt_ovf_int_clr(&mut self) -> IBUS_CNT_OVF_INT_CLR_W {
        IBUS_CNT_OVF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 8 - The bit is used to clear interrupt by dbus counter overflow."]
    #[inline(always)]
    pub fn dbus_cnt_ovf_int_clr(&mut self) -> DBUS_CNT_OVF_INT_CLR_W {
        DBUS_CNT_OVF_INT_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This description will be updated in the near future.\n\nThis register you can [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_ilg_int_clr]
(index.html) module"]
pub struct CACHE_ILG_INT_CLR_SPEC;
impl crate::RegisterSpec for CACHE_ILG_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cache_ilg_int_clr::W]
(W) writer structure"]
impl crate::Writable for CACHE_ILG_INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE_ILG_INT_CLR to value 0"]
impl crate::Resettable for CACHE_ILG_INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
