#[doc = "Register `CORE_0_INTERRUPT_ENA` reader"]
pub struct R(crate::R<CORE_0_INTERRUPT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_INTERRUPT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_INTERRUPT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_INTERRUPT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_INTERRUPT_ENA` writer"]
pub struct W(crate::W<CORE_0_INTERRUPT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_INTERRUPT_ENA_SPEC>;
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
impl From<crate::W<CORE_0_INTERRUPT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_INTERRUPT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_0_RD_ENA` reader - Core0 dram0 area0 read monitor enable"]
pub struct CORE_0_AREA_DRAM0_0_RD_ENA_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_DRAM0_0_RD_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_DRAM0_0_RD_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_DRAM0_0_RD_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_0_RD_ENA` writer - Core0 dram0 area0 read monitor enable"]
pub struct CORE_0_AREA_DRAM0_0_RD_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_AREA_DRAM0_0_RD_ENA_W<'a> {
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
#[doc = "Field `CORE_0_AREA_DRAM0_0_WR_ENA` reader - Core0 dram0 area0 write monitor enable"]
pub struct CORE_0_AREA_DRAM0_0_WR_ENA_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_DRAM0_0_WR_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_DRAM0_0_WR_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_DRAM0_0_WR_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_0_WR_ENA` writer - Core0 dram0 area0 write monitor enable"]
pub struct CORE_0_AREA_DRAM0_0_WR_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_AREA_DRAM0_0_WR_ENA_W<'a> {
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
#[doc = "Field `CORE_0_AREA_DRAM0_1_RD_ENA` reader - Core0 dram0 area1 read monitor enable"]
pub struct CORE_0_AREA_DRAM0_1_RD_ENA_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_DRAM0_1_RD_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_DRAM0_1_RD_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_DRAM0_1_RD_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_1_RD_ENA` writer - Core0 dram0 area1 read monitor enable"]
pub struct CORE_0_AREA_DRAM0_1_RD_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_AREA_DRAM0_1_RD_ENA_W<'a> {
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
#[doc = "Field `CORE_0_AREA_DRAM0_1_WR_ENA` reader - Core0 dram0 area1 write monitor enable"]
pub struct CORE_0_AREA_DRAM0_1_WR_ENA_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_DRAM0_1_WR_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_DRAM0_1_WR_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_DRAM0_1_WR_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_1_WR_ENA` writer - Core0 dram0 area1 write monitor enable"]
pub struct CORE_0_AREA_DRAM0_1_WR_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_AREA_DRAM0_1_WR_ENA_W<'a> {
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
#[doc = "Field `CORE_0_AREA_PIF_0_RD_ENA` reader - Core0 PIF area0 read monitor enable"]
pub struct CORE_0_AREA_PIF_0_RD_ENA_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_PIF_0_RD_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_PIF_0_RD_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_PIF_0_RD_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_PIF_0_RD_ENA` writer - Core0 PIF area0 read monitor enable"]
pub struct CORE_0_AREA_PIF_0_RD_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_AREA_PIF_0_RD_ENA_W<'a> {
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
#[doc = "Field `CORE_0_AREA_PIF_0_WR_ENA` reader - Core0 PIF area0 write monitor enable"]
pub struct CORE_0_AREA_PIF_0_WR_ENA_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_PIF_0_WR_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_PIF_0_WR_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_PIF_0_WR_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_PIF_0_WR_ENA` writer - Core0 PIF area0 write monitor enable"]
pub struct CORE_0_AREA_PIF_0_WR_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_AREA_PIF_0_WR_ENA_W<'a> {
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
#[doc = "Field `CORE_0_AREA_PIF_1_RD_ENA` reader - Core0 PIF area1 read monitor enable"]
pub struct CORE_0_AREA_PIF_1_RD_ENA_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_PIF_1_RD_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_PIF_1_RD_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_PIF_1_RD_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_PIF_1_RD_ENA` writer - Core0 PIF area1 read monitor enable"]
pub struct CORE_0_AREA_PIF_1_RD_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_AREA_PIF_1_RD_ENA_W<'a> {
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
#[doc = "Field `CORE_0_AREA_PIF_1_WR_ENA` reader - Core0 PIF area1 write monitor enable"]
pub struct CORE_0_AREA_PIF_1_WR_ENA_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_PIF_1_WR_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_PIF_1_WR_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_PIF_1_WR_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_PIF_1_WR_ENA` writer - Core0 PIF area1 write monitor enable"]
pub struct CORE_0_AREA_PIF_1_WR_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_AREA_PIF_1_WR_ENA_W<'a> {
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
#[doc = "Field `CORE_0_SP_SPILL_MIN_ENA` reader - Core0 stackpoint overflow monitor enable"]
pub struct CORE_0_SP_SPILL_MIN_ENA_R(crate::FieldReader<bool, bool>);
impl CORE_0_SP_SPILL_MIN_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_SP_SPILL_MIN_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_SP_SPILL_MIN_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_SP_SPILL_MIN_ENA` writer - Core0 stackpoint overflow monitor enable"]
pub struct CORE_0_SP_SPILL_MIN_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_SP_SPILL_MIN_ENA_W<'a> {
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
#[doc = "Field `CORE_0_SP_SPILL_MAX_ENA` reader - Core0 stackpoint underflow monitor enable"]
pub struct CORE_0_SP_SPILL_MAX_ENA_R(crate::FieldReader<bool, bool>);
impl CORE_0_SP_SPILL_MAX_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_SP_SPILL_MAX_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_SP_SPILL_MAX_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_SP_SPILL_MAX_ENA` writer - Core0 stackpoint underflow monitor enable"]
pub struct CORE_0_SP_SPILL_MAX_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_SP_SPILL_MAX_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CORE_0_IRAM0_EXCEPTION_MONITOR_ENA` reader - IBUS busy monitor enable"]
pub struct CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_R(crate::FieldReader<bool, bool>);
impl CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_IRAM0_EXCEPTION_MONITOR_ENA` writer - IBUS busy monitor enable"]
pub struct CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `CORE_0_DRAM0_EXCEPTION_MONITOR_ENA` reader - DBUS busy monitor enbale"]
pub struct CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_R(crate::FieldReader<bool, bool>);
impl CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_DRAM0_EXCEPTION_MONITOR_ENA` writer - DBUS busy monitor enbale"]
pub struct CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Core0 dram0 area0 read monitor enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_rd_ena(&self) -> CORE_0_AREA_DRAM0_0_RD_ENA_R {
        CORE_0_AREA_DRAM0_0_RD_ENA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Core0 dram0 area0 write monitor enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_wr_ena(&self) -> CORE_0_AREA_DRAM0_0_WR_ENA_R {
        CORE_0_AREA_DRAM0_0_WR_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Core0 dram0 area1 read monitor enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_rd_ena(&self) -> CORE_0_AREA_DRAM0_1_RD_ENA_R {
        CORE_0_AREA_DRAM0_1_RD_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Core0 dram0 area1 write monitor enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_wr_ena(&self) -> CORE_0_AREA_DRAM0_1_WR_ENA_R {
        CORE_0_AREA_DRAM0_1_WR_ENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Core0 PIF area0 read monitor enable"]
    #[inline(always)]
    pub fn core_0_area_pif_0_rd_ena(&self) -> CORE_0_AREA_PIF_0_RD_ENA_R {
        CORE_0_AREA_PIF_0_RD_ENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Core0 PIF area0 write monitor enable"]
    #[inline(always)]
    pub fn core_0_area_pif_0_wr_ena(&self) -> CORE_0_AREA_PIF_0_WR_ENA_R {
        CORE_0_AREA_PIF_0_WR_ENA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Core0 PIF area1 read monitor enable"]
    #[inline(always)]
    pub fn core_0_area_pif_1_rd_ena(&self) -> CORE_0_AREA_PIF_1_RD_ENA_R {
        CORE_0_AREA_PIF_1_RD_ENA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Core0 PIF area1 write monitor enable"]
    #[inline(always)]
    pub fn core_0_area_pif_1_wr_ena(&self) -> CORE_0_AREA_PIF_1_WR_ENA_R {
        CORE_0_AREA_PIF_1_WR_ENA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Core0 stackpoint overflow monitor enable"]
    #[inline(always)]
    pub fn core_0_sp_spill_min_ena(&self) -> CORE_0_SP_SPILL_MIN_ENA_R {
        CORE_0_SP_SPILL_MIN_ENA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Core0 stackpoint underflow monitor enable"]
    #[inline(always)]
    pub fn core_0_sp_spill_max_ena(&self) -> CORE_0_SP_SPILL_MAX_ENA_R {
        CORE_0_SP_SPILL_MAX_ENA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - IBUS busy monitor enable"]
    #[inline(always)]
    pub fn core_0_iram0_exception_monitor_ena(&self) -> CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_R {
        CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DBUS busy monitor enbale"]
    #[inline(always)]
    pub fn core_0_dram0_exception_monitor_ena(&self) -> CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_R {
        CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Core0 dram0 area0 read monitor enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_rd_ena(&mut self) -> CORE_0_AREA_DRAM0_0_RD_ENA_W {
        CORE_0_AREA_DRAM0_0_RD_ENA_W { w: self }
    }
    #[doc = "Bit 1 - Core0 dram0 area0 write monitor enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_wr_ena(&mut self) -> CORE_0_AREA_DRAM0_0_WR_ENA_W {
        CORE_0_AREA_DRAM0_0_WR_ENA_W { w: self }
    }
    #[doc = "Bit 2 - Core0 dram0 area1 read monitor enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_rd_ena(&mut self) -> CORE_0_AREA_DRAM0_1_RD_ENA_W {
        CORE_0_AREA_DRAM0_1_RD_ENA_W { w: self }
    }
    #[doc = "Bit 3 - Core0 dram0 area1 write monitor enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_wr_ena(&mut self) -> CORE_0_AREA_DRAM0_1_WR_ENA_W {
        CORE_0_AREA_DRAM0_1_WR_ENA_W { w: self }
    }
    #[doc = "Bit 4 - Core0 PIF area0 read monitor enable"]
    #[inline(always)]
    pub fn core_0_area_pif_0_rd_ena(&mut self) -> CORE_0_AREA_PIF_0_RD_ENA_W {
        CORE_0_AREA_PIF_0_RD_ENA_W { w: self }
    }
    #[doc = "Bit 5 - Core0 PIF area0 write monitor enable"]
    #[inline(always)]
    pub fn core_0_area_pif_0_wr_ena(&mut self) -> CORE_0_AREA_PIF_0_WR_ENA_W {
        CORE_0_AREA_PIF_0_WR_ENA_W { w: self }
    }
    #[doc = "Bit 6 - Core0 PIF area1 read monitor enable"]
    #[inline(always)]
    pub fn core_0_area_pif_1_rd_ena(&mut self) -> CORE_0_AREA_PIF_1_RD_ENA_W {
        CORE_0_AREA_PIF_1_RD_ENA_W { w: self }
    }
    #[doc = "Bit 7 - Core0 PIF area1 write monitor enable"]
    #[inline(always)]
    pub fn core_0_area_pif_1_wr_ena(&mut self) -> CORE_0_AREA_PIF_1_WR_ENA_W {
        CORE_0_AREA_PIF_1_WR_ENA_W { w: self }
    }
    #[doc = "Bit 8 - Core0 stackpoint overflow monitor enable"]
    #[inline(always)]
    pub fn core_0_sp_spill_min_ena(&mut self) -> CORE_0_SP_SPILL_MIN_ENA_W {
        CORE_0_SP_SPILL_MIN_ENA_W { w: self }
    }
    #[doc = "Bit 9 - Core0 stackpoint underflow monitor enable"]
    #[inline(always)]
    pub fn core_0_sp_spill_max_ena(&mut self) -> CORE_0_SP_SPILL_MAX_ENA_W {
        CORE_0_SP_SPILL_MAX_ENA_W { w: self }
    }
    #[doc = "Bit 10 - IBUS busy monitor enable"]
    #[inline(always)]
    pub fn core_0_iram0_exception_monitor_ena(&mut self) -> CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_W {
        CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_W { w: self }
    }
    #[doc = "Bit 11 - DBUS busy monitor enbale"]
    #[inline(always)]
    pub fn core_0_dram0_exception_monitor_ena(&mut self) -> CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_W {
        CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "core0 monitor enable configuration register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_interrupt_ena]
(index.html) module"]
pub struct CORE_0_INTERRUPT_ENA_SPEC;
impl crate::RegisterSpec for CORE_0_INTERRUPT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_interrupt_ena::R]
(R) reader structure"]
impl crate::Readable for CORE_0_INTERRUPT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_interrupt_ena::W]
(W) writer structure"]
impl crate::Writable for CORE_0_INTERRUPT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_0_INTERRUPT_ENA to value 0"]
impl crate::Resettable for CORE_0_INTERRUPT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
