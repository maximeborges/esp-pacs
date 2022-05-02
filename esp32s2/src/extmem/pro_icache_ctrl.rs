#[doc = "Register `PRO_ICACHE_CTRL` reader"]
pub struct R(crate::R<PRO_ICACHE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_ICACHE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_ICACHE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_ICACHE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_ICACHE_CTRL` writer"]
pub struct W(crate::W<PRO_ICACHE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_ICACHE_CTRL_SPEC>;
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
impl From<crate::W<PRO_ICACHE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_ICACHE_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_ICACHE_ENABLE` reader - The bit is used to activate the data cache. 0: disable, 1: enable"]
pub struct PRO_ICACHE_ENABLE_R(crate::FieldReader<bool>);
impl PRO_ICACHE_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_ICACHE_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_ICACHE_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_ICACHE_ENABLE` writer - The bit is used to activate the data cache. 0: disable, 1: enable"]
pub struct PRO_ICACHE_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_ICACHE_ENABLE_W<'a> {
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
#[doc = "Field `PRO_ICACHE_SETSIZE_MODE` reader - The bit is used to configure cache memory size.0: 8KB, 1: 16KB"]
pub struct PRO_ICACHE_SETSIZE_MODE_R(crate::FieldReader<bool>);
impl PRO_ICACHE_SETSIZE_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_ICACHE_SETSIZE_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_ICACHE_SETSIZE_MODE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_ICACHE_SETSIZE_MODE` writer - The bit is used to configure cache memory size.0: 8KB, 1: 16KB"]
pub struct PRO_ICACHE_SETSIZE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_ICACHE_SETSIZE_MODE_W<'a> {
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
#[doc = "Field `PRO_ICACHE_BLOCKSIZE_MODE` reader - The bit is used to configure cache block size.0: 16 bytes, 1: 32 bytes"]
pub struct PRO_ICACHE_BLOCKSIZE_MODE_R(crate::FieldReader<bool>);
impl PRO_ICACHE_BLOCKSIZE_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_ICACHE_BLOCKSIZE_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_ICACHE_BLOCKSIZE_MODE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_ICACHE_BLOCKSIZE_MODE` writer - The bit is used to configure cache block size.0: 16 bytes, 1: 32 bytes"]
pub struct PRO_ICACHE_BLOCKSIZE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_ICACHE_BLOCKSIZE_MODE_W<'a> {
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
#[doc = "Field `PRO_ICACHE_INVALIDATE_ENA` reader - The bit is used to enable invalidate operation. It will be cleared by hardware after invalidate operation done."]
pub struct PRO_ICACHE_INVALIDATE_ENA_R(crate::FieldReader<bool>);
impl PRO_ICACHE_INVALIDATE_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_ICACHE_INVALIDATE_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_ICACHE_INVALIDATE_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_ICACHE_INVALIDATE_ENA` writer - The bit is used to enable invalidate operation. It will be cleared by hardware after invalidate operation done."]
pub struct PRO_ICACHE_INVALIDATE_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_ICACHE_INVALIDATE_ENA_W<'a> {
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
#[doc = "Field `PRO_ICACHE_INVALIDATE_DONE` reader - The bit is used to indicate invalidate operation is finished."]
pub struct PRO_ICACHE_INVALIDATE_DONE_R(crate::FieldReader<bool>);
impl PRO_ICACHE_INVALIDATE_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_ICACHE_INVALIDATE_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_ICACHE_INVALIDATE_DONE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_ICACHE_LOCK0_EN` reader - The bit is used to enable pre-lock operation which is combined with PRO_ICACHE_LOCK0_ADDR_REG and PRO_ICACHE_LOCK0_SIZE_REG."]
pub struct PRO_ICACHE_LOCK0_EN_R(crate::FieldReader<bool>);
impl PRO_ICACHE_LOCK0_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_ICACHE_LOCK0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_ICACHE_LOCK0_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_ICACHE_LOCK0_EN` writer - The bit is used to enable pre-lock operation which is combined with PRO_ICACHE_LOCK0_ADDR_REG and PRO_ICACHE_LOCK0_SIZE_REG."]
pub struct PRO_ICACHE_LOCK0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_ICACHE_LOCK0_EN_W<'a> {
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
#[doc = "Field `PRO_ICACHE_LOCK1_EN` reader - The bit is used to enable pre-lock operation which is combined with PRO_ICACHE_LOCK1_ADDR_REG and PRO_ICACHE_LOCK1_SIZE_REG."]
pub struct PRO_ICACHE_LOCK1_EN_R(crate::FieldReader<bool>);
impl PRO_ICACHE_LOCK1_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_ICACHE_LOCK1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_ICACHE_LOCK1_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_ICACHE_LOCK1_EN` writer - The bit is used to enable pre-lock operation which is combined with PRO_ICACHE_LOCK1_ADDR_REG and PRO_ICACHE_LOCK1_SIZE_REG."]
pub struct PRO_ICACHE_LOCK1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_ICACHE_LOCK1_EN_W<'a> {
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
#[doc = "Field `PRO_ICACHE_AUTOLOAD_ENA` reader - The bit is used to enable and disable conditional-preload operation. It is combined with pre_dcache_autoload_done. 1: enable, 0: disable."]
pub struct PRO_ICACHE_AUTOLOAD_ENA_R(crate::FieldReader<bool>);
impl PRO_ICACHE_AUTOLOAD_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_ICACHE_AUTOLOAD_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_ICACHE_AUTOLOAD_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_ICACHE_AUTOLOAD_ENA` writer - The bit is used to enable and disable conditional-preload operation. It is combined with pre_dcache_autoload_done. 1: enable, 0: disable."]
pub struct PRO_ICACHE_AUTOLOAD_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_ICACHE_AUTOLOAD_ENA_W<'a> {
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
#[doc = "Field `PRO_ICACHE_AUTOLOAD_DONE` reader - The bit is used to indicate conditional-preload operation is finished."]
pub struct PRO_ICACHE_AUTOLOAD_DONE_R(crate::FieldReader<bool>);
impl PRO_ICACHE_AUTOLOAD_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_ICACHE_AUTOLOAD_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_ICACHE_AUTOLOAD_DONE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_ICACHE_PRELOAD_ENA` reader - The bit is used to enable preload operation. It will be cleared by hardware after preload operation done."]
pub struct PRO_ICACHE_PRELOAD_ENA_R(crate::FieldReader<bool>);
impl PRO_ICACHE_PRELOAD_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_ICACHE_PRELOAD_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_ICACHE_PRELOAD_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_ICACHE_PRELOAD_ENA` writer - The bit is used to enable preload operation. It will be cleared by hardware after preload operation done."]
pub struct PRO_ICACHE_PRELOAD_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_ICACHE_PRELOAD_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 20)) | ((value as u32 & 1) << 20);
        self.w
    }
}
#[doc = "Field `PRO_ICACHE_PRELOAD_DONE` reader - The bit is used to indicate preload operation is finished."]
pub struct PRO_ICACHE_PRELOAD_DONE_R(crate::FieldReader<bool>);
impl PRO_ICACHE_PRELOAD_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_ICACHE_PRELOAD_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_ICACHE_PRELOAD_DONE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_ICACHE_UNLOCK_ENA` reader - The bit is used to enable unlock operation. It will be cleared by hardware after unlock operation done."]
pub struct PRO_ICACHE_UNLOCK_ENA_R(crate::FieldReader<bool>);
impl PRO_ICACHE_UNLOCK_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_ICACHE_UNLOCK_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_ICACHE_UNLOCK_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_ICACHE_UNLOCK_ENA` writer - The bit is used to enable unlock operation. It will be cleared by hardware after unlock operation done."]
pub struct PRO_ICACHE_UNLOCK_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_ICACHE_UNLOCK_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 22)) | ((value as u32 & 1) << 22);
        self.w
    }
}
#[doc = "Field `PRO_ICACHE_UNLOCK_DONE` reader - The bit is used to indicate unlock operation is finished."]
pub struct PRO_ICACHE_UNLOCK_DONE_R(crate::FieldReader<bool>);
impl PRO_ICACHE_UNLOCK_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_ICACHE_UNLOCK_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_ICACHE_UNLOCK_DONE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_ICACHE_LOCK_ENA` reader - The bit is used to enable lock operation. It will be cleared by hardware after lock operation done."]
pub struct PRO_ICACHE_LOCK_ENA_R(crate::FieldReader<bool>);
impl PRO_ICACHE_LOCK_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_ICACHE_LOCK_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_ICACHE_LOCK_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_ICACHE_LOCK_ENA` writer - The bit is used to enable lock operation. It will be cleared by hardware after lock operation done."]
pub struct PRO_ICACHE_LOCK_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_ICACHE_LOCK_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 24)) | ((value as u32 & 1) << 24);
        self.w
    }
}
#[doc = "Field `PRO_ICACHE_LOCK_DONE` reader - The bit is used to indicate lock operation is finished."]
pub struct PRO_ICACHE_LOCK_DONE_R(crate::FieldReader<bool>);
impl PRO_ICACHE_LOCK_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_ICACHE_LOCK_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_ICACHE_LOCK_DONE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - The bit is used to activate the data cache. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn pro_icache_enable(&self) -> PRO_ICACHE_ENABLE_R {
        PRO_ICACHE_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to configure cache memory size.0: 8KB, 1: 16KB"]
    #[inline(always)]
    pub fn pro_icache_setsize_mode(&self) -> PRO_ICACHE_SETSIZE_MODE_R {
        PRO_ICACHE_SETSIZE_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The bit is used to configure cache block size.0: 16 bytes, 1: 32 bytes"]
    #[inline(always)]
    pub fn pro_icache_blocksize_mode(&self) -> PRO_ICACHE_BLOCKSIZE_MODE_R {
        PRO_ICACHE_BLOCKSIZE_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - The bit is used to enable invalidate operation. It will be cleared by hardware after invalidate operation done."]
    #[inline(always)]
    pub fn pro_icache_invalidate_ena(&self) -> PRO_ICACHE_INVALIDATE_ENA_R {
        PRO_ICACHE_INVALIDATE_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The bit is used to indicate invalidate operation is finished."]
    #[inline(always)]
    pub fn pro_icache_invalidate_done(&self) -> PRO_ICACHE_INVALIDATE_DONE_R {
        PRO_ICACHE_INVALIDATE_DONE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - The bit is used to enable pre-lock operation which is combined with PRO_ICACHE_LOCK0_ADDR_REG and PRO_ICACHE_LOCK0_SIZE_REG."]
    #[inline(always)]
    pub fn pro_icache_lock0_en(&self) -> PRO_ICACHE_LOCK0_EN_R {
        PRO_ICACHE_LOCK0_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The bit is used to enable pre-lock operation which is combined with PRO_ICACHE_LOCK1_ADDR_REG and PRO_ICACHE_LOCK1_SIZE_REG."]
    #[inline(always)]
    pub fn pro_icache_lock1_en(&self) -> PRO_ICACHE_LOCK1_EN_R {
        PRO_ICACHE_LOCK1_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - The bit is used to enable and disable conditional-preload operation. It is combined with pre_dcache_autoload_done. 1: enable, 0: disable."]
    #[inline(always)]
    pub fn pro_icache_autoload_ena(&self) -> PRO_ICACHE_AUTOLOAD_ENA_R {
        PRO_ICACHE_AUTOLOAD_ENA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The bit is used to indicate conditional-preload operation is finished."]
    #[inline(always)]
    pub fn pro_icache_autoload_done(&self) -> PRO_ICACHE_AUTOLOAD_DONE_R {
        PRO_ICACHE_AUTOLOAD_DONE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The bit is used to enable preload operation. It will be cleared by hardware after preload operation done."]
    #[inline(always)]
    pub fn pro_icache_preload_ena(&self) -> PRO_ICACHE_PRELOAD_ENA_R {
        PRO_ICACHE_PRELOAD_ENA_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The bit is used to indicate preload operation is finished."]
    #[inline(always)]
    pub fn pro_icache_preload_done(&self) -> PRO_ICACHE_PRELOAD_DONE_R {
        PRO_ICACHE_PRELOAD_DONE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The bit is used to enable unlock operation. It will be cleared by hardware after unlock operation done."]
    #[inline(always)]
    pub fn pro_icache_unlock_ena(&self) -> PRO_ICACHE_UNLOCK_ENA_R {
        PRO_ICACHE_UNLOCK_ENA_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The bit is used to indicate unlock operation is finished."]
    #[inline(always)]
    pub fn pro_icache_unlock_done(&self) -> PRO_ICACHE_UNLOCK_DONE_R {
        PRO_ICACHE_UNLOCK_DONE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - The bit is used to enable lock operation. It will be cleared by hardware after lock operation done."]
    #[inline(always)]
    pub fn pro_icache_lock_ena(&self) -> PRO_ICACHE_LOCK_ENA_R {
        PRO_ICACHE_LOCK_ENA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - The bit is used to indicate lock operation is finished."]
    #[inline(always)]
    pub fn pro_icache_lock_done(&self) -> PRO_ICACHE_LOCK_DONE_R {
        PRO_ICACHE_LOCK_DONE_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to activate the data cache. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn pro_icache_enable(&mut self) -> PRO_ICACHE_ENABLE_W {
        PRO_ICACHE_ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - The bit is used to configure cache memory size.0: 8KB, 1: 16KB"]
    #[inline(always)]
    pub fn pro_icache_setsize_mode(&mut self) -> PRO_ICACHE_SETSIZE_MODE_W {
        PRO_ICACHE_SETSIZE_MODE_W { w: self }
    }
    #[doc = "Bit 3 - The bit is used to configure cache block size.0: 16 bytes, 1: 32 bytes"]
    #[inline(always)]
    pub fn pro_icache_blocksize_mode(&mut self) -> PRO_ICACHE_BLOCKSIZE_MODE_W {
        PRO_ICACHE_BLOCKSIZE_MODE_W { w: self }
    }
    #[doc = "Bit 8 - The bit is used to enable invalidate operation. It will be cleared by hardware after invalidate operation done."]
    #[inline(always)]
    pub fn pro_icache_invalidate_ena(&mut self) -> PRO_ICACHE_INVALIDATE_ENA_W {
        PRO_ICACHE_INVALIDATE_ENA_W { w: self }
    }
    #[doc = "Bit 14 - The bit is used to enable pre-lock operation which is combined with PRO_ICACHE_LOCK0_ADDR_REG and PRO_ICACHE_LOCK0_SIZE_REG."]
    #[inline(always)]
    pub fn pro_icache_lock0_en(&mut self) -> PRO_ICACHE_LOCK0_EN_W {
        PRO_ICACHE_LOCK0_EN_W { w: self }
    }
    #[doc = "Bit 15 - The bit is used to enable pre-lock operation which is combined with PRO_ICACHE_LOCK1_ADDR_REG and PRO_ICACHE_LOCK1_SIZE_REG."]
    #[inline(always)]
    pub fn pro_icache_lock1_en(&mut self) -> PRO_ICACHE_LOCK1_EN_W {
        PRO_ICACHE_LOCK1_EN_W { w: self }
    }
    #[doc = "Bit 18 - The bit is used to enable and disable conditional-preload operation. It is combined with pre_dcache_autoload_done. 1: enable, 0: disable."]
    #[inline(always)]
    pub fn pro_icache_autoload_ena(&mut self) -> PRO_ICACHE_AUTOLOAD_ENA_W {
        PRO_ICACHE_AUTOLOAD_ENA_W { w: self }
    }
    #[doc = "Bit 20 - The bit is used to enable preload operation. It will be cleared by hardware after preload operation done."]
    #[inline(always)]
    pub fn pro_icache_preload_ena(&mut self) -> PRO_ICACHE_PRELOAD_ENA_W {
        PRO_ICACHE_PRELOAD_ENA_W { w: self }
    }
    #[doc = "Bit 22 - The bit is used to enable unlock operation. It will be cleared by hardware after unlock operation done."]
    #[inline(always)]
    pub fn pro_icache_unlock_ena(&mut self) -> PRO_ICACHE_UNLOCK_ENA_W {
        PRO_ICACHE_UNLOCK_ENA_W { w: self }
    }
    #[doc = "Bit 24 - The bit is used to enable lock operation. It will be cleared by hardware after lock operation done."]
    #[inline(always)]
    pub fn pro_icache_lock_ena(&mut self) -> PRO_ICACHE_LOCK_ENA_W {
        PRO_ICACHE_LOCK_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_icache_ctrl](index.html) module"]
pub struct PRO_ICACHE_CTRL_SPEC;
impl crate::RegisterSpec for PRO_ICACHE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_icache_ctrl::R](R) reader structure"]
impl crate::Readable for PRO_ICACHE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_icache_ctrl::W](W) writer structure"]
impl crate::Writable for PRO_ICACHE_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_ICACHE_CTRL to value 0x0100"]
impl crate::Resettable for PRO_ICACHE_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}
