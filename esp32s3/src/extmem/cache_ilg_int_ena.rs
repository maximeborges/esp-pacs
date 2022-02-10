#[doc = "Register `CACHE_ILG_INT_ENA` reader"]
pub struct R(crate::R<CACHE_ILG_INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_ILG_INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_ILG_INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_ILG_INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_ILG_INT_ENA` writer"]
pub struct W(crate::W<CACHE_ILG_INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_ILG_INT_ENA_SPEC>;
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
impl From<crate::W<CACHE_ILG_INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_ILG_INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICACHE_SYNC_OP_FAULT_INT_ENA` reader - The bit is used to enable interrupt by sync configurations fault."]
pub struct ICACHE_SYNC_OP_FAULT_INT_ENA_R(crate::FieldReader<bool, bool>);
impl ICACHE_SYNC_OP_FAULT_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ICACHE_SYNC_OP_FAULT_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_SYNC_OP_FAULT_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_SYNC_OP_FAULT_INT_ENA` writer - The bit is used to enable interrupt by sync configurations fault."]
pub struct ICACHE_SYNC_OP_FAULT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHE_SYNC_OP_FAULT_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `ICACHE_PRELOAD_OP_FAULT_INT_ENA` reader - The bit is used to enable interrupt by preload configurations fault."]
pub struct ICACHE_PRELOAD_OP_FAULT_INT_ENA_R(crate::FieldReader<bool, bool>);
impl ICACHE_PRELOAD_OP_FAULT_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ICACHE_PRELOAD_OP_FAULT_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_PRELOAD_OP_FAULT_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_PRELOAD_OP_FAULT_INT_ENA` writer - The bit is used to enable interrupt by preload configurations fault."]
pub struct ICACHE_PRELOAD_OP_FAULT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHE_PRELOAD_OP_FAULT_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `DCACHE_SYNC_OP_FAULT_INT_ENA` reader - The bit is used to enable interrupt by sync configurations fault."]
pub struct DCACHE_SYNC_OP_FAULT_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DCACHE_SYNC_OP_FAULT_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCACHE_SYNC_OP_FAULT_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_SYNC_OP_FAULT_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_SYNC_OP_FAULT_INT_ENA` writer - The bit is used to enable interrupt by sync configurations fault."]
pub struct DCACHE_SYNC_OP_FAULT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_SYNC_OP_FAULT_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `DCACHE_PRELOAD_OP_FAULT_INT_ENA` reader - The bit is used to enable interrupt by preload configurations fault."]
pub struct DCACHE_PRELOAD_OP_FAULT_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DCACHE_PRELOAD_OP_FAULT_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCACHE_PRELOAD_OP_FAULT_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_PRELOAD_OP_FAULT_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_PRELOAD_OP_FAULT_INT_ENA` writer - The bit is used to enable interrupt by preload configurations fault."]
pub struct DCACHE_PRELOAD_OP_FAULT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_PRELOAD_OP_FAULT_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `DCACHE_WRITE_FLASH_INT_ENA` reader - The bit is used to enable interrupt by dcache trying to write flash."]
pub struct DCACHE_WRITE_FLASH_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DCACHE_WRITE_FLASH_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCACHE_WRITE_FLASH_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_WRITE_FLASH_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `MMU_ENTRY_FAULT_INT_ENA` reader - The bit is used to enable interrupt by mmu entry fault."]
pub struct MMU_ENTRY_FAULT_INT_ENA_R(crate::FieldReader<bool, bool>);
impl MMU_ENTRY_FAULT_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MMU_ENTRY_FAULT_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMU_ENTRY_FAULT_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `DCACHE_OCCUPY_EXC_INT_ENA` reader - The bit is used to enable interrupt by dcache trying to replace a line whose blocks all have been occupied by occupy-mode."]
pub struct DCACHE_OCCUPY_EXC_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DCACHE_OCCUPY_EXC_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCACHE_OCCUPY_EXC_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_OCCUPY_EXC_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_OCCUPY_EXC_INT_ENA` writer - The bit is used to enable interrupt by dcache trying to replace a line whose blocks all have been occupied by occupy-mode."]
pub struct DCACHE_OCCUPY_EXC_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_OCCUPY_EXC_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `IBUS_CNT_OVF_INT_ENA` reader - The bit is used to enable interrupt by ibus counter overflow."]
pub struct IBUS_CNT_OVF_INT_ENA_R(crate::FieldReader<bool, bool>);
impl IBUS_CNT_OVF_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IBUS_CNT_OVF_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBUS_CNT_OVF_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `DBUS_CNT_OVF_INT_ENA` reader - The bit is used to enable interrupt by dbus counter overflow."]
pub struct DBUS_CNT_OVF_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DBUS_CNT_OVF_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBUS_CNT_OVF_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBUS_CNT_OVF_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - The bit is used to enable interrupt by sync configurations fault."]
    #[inline(always)]
    pub fn icache_sync_op_fault_int_ena(&self) -> ICACHE_SYNC_OP_FAULT_INT_ENA_R {
        ICACHE_SYNC_OP_FAULT_INT_ENA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable interrupt by preload configurations fault."]
    #[inline(always)]
    pub fn icache_preload_op_fault_int_ena(&self) -> ICACHE_PRELOAD_OP_FAULT_INT_ENA_R {
        ICACHE_PRELOAD_OP_FAULT_INT_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The bit is used to enable interrupt by sync configurations fault."]
    #[inline(always)]
    pub fn dcache_sync_op_fault_int_ena(&self) -> DCACHE_SYNC_OP_FAULT_INT_ENA_R {
        DCACHE_SYNC_OP_FAULT_INT_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The bit is used to enable interrupt by preload configurations fault."]
    #[inline(always)]
    pub fn dcache_preload_op_fault_int_ena(&self) -> DCACHE_PRELOAD_OP_FAULT_INT_ENA_R {
        DCACHE_PRELOAD_OP_FAULT_INT_ENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The bit is used to enable interrupt by dcache trying to write flash."]
    #[inline(always)]
    pub fn dcache_write_flash_int_ena(&self) -> DCACHE_WRITE_FLASH_INT_ENA_R {
        DCACHE_WRITE_FLASH_INT_ENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - The bit is used to enable interrupt by mmu entry fault."]
    #[inline(always)]
    pub fn mmu_entry_fault_int_ena(&self) -> MMU_ENTRY_FAULT_INT_ENA_R {
        MMU_ENTRY_FAULT_INT_ENA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The bit is used to enable interrupt by dcache trying to replace a line whose blocks all have been occupied by occupy-mode."]
    #[inline(always)]
    pub fn dcache_occupy_exc_int_ena(&self) -> DCACHE_OCCUPY_EXC_INT_ENA_R {
        DCACHE_OCCUPY_EXC_INT_ENA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The bit is used to enable interrupt by ibus counter overflow."]
    #[inline(always)]
    pub fn ibus_cnt_ovf_int_ena(&self) -> IBUS_CNT_OVF_INT_ENA_R {
        IBUS_CNT_OVF_INT_ENA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - The bit is used to enable interrupt by dbus counter overflow."]
    #[inline(always)]
    pub fn dbus_cnt_ovf_int_ena(&self) -> DBUS_CNT_OVF_INT_ENA_R {
        DBUS_CNT_OVF_INT_ENA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable interrupt by sync configurations fault."]
    #[inline(always)]
    pub fn icache_sync_op_fault_int_ena(&mut self) -> ICACHE_SYNC_OP_FAULT_INT_ENA_W {
        ICACHE_SYNC_OP_FAULT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1 - The bit is used to enable interrupt by preload configurations fault."]
    #[inline(always)]
    pub fn icache_preload_op_fault_int_ena(&mut self) -> ICACHE_PRELOAD_OP_FAULT_INT_ENA_W {
        ICACHE_PRELOAD_OP_FAULT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2 - The bit is used to enable interrupt by sync configurations fault."]
    #[inline(always)]
    pub fn dcache_sync_op_fault_int_ena(&mut self) -> DCACHE_SYNC_OP_FAULT_INT_ENA_W {
        DCACHE_SYNC_OP_FAULT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3 - The bit is used to enable interrupt by preload configurations fault."]
    #[inline(always)]
    pub fn dcache_preload_op_fault_int_ena(&mut self) -> DCACHE_PRELOAD_OP_FAULT_INT_ENA_W {
        DCACHE_PRELOAD_OP_FAULT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4 - The bit is used to enable interrupt by dcache trying to write flash."]
    #[inline(always)]
    pub fn dcache_write_flash_int_ena(&mut self) -> DCACHE_WRITE_FLASH_INT_ENA_W {
        DCACHE_WRITE_FLASH_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5 - The bit is used to enable interrupt by mmu entry fault."]
    #[inline(always)]
    pub fn mmu_entry_fault_int_ena(&mut self) -> MMU_ENTRY_FAULT_INT_ENA_W {
        MMU_ENTRY_FAULT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6 - The bit is used to enable interrupt by dcache trying to replace a line whose blocks all have been occupied by occupy-mode."]
    #[inline(always)]
    pub fn dcache_occupy_exc_int_ena(&mut self) -> DCACHE_OCCUPY_EXC_INT_ENA_W {
        DCACHE_OCCUPY_EXC_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7 - The bit is used to enable interrupt by ibus counter overflow."]
    #[inline(always)]
    pub fn ibus_cnt_ovf_int_ena(&mut self) -> IBUS_CNT_OVF_INT_ENA_W {
        IBUS_CNT_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 8 - The bit is used to enable interrupt by dbus counter overflow."]
    #[inline(always)]
    pub fn dbus_cnt_ovf_int_ena(&mut self) -> DBUS_CNT_OVF_INT_ENA_W {
        DBUS_CNT_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_ilg_int_ena]
(index.html) module"]
pub struct CACHE_ILG_INT_ENA_SPEC;
impl crate::RegisterSpec for CACHE_ILG_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_ilg_int_ena::R]
(R) reader structure"]
impl crate::Readable for CACHE_ILG_INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_ilg_int_ena::W]
(W) writer structure"]
impl crate::Writable for CACHE_ILG_INT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE_ILG_INT_ENA to value 0"]
impl crate::Resettable for CACHE_ILG_INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
