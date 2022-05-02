#[doc = "Register `DIG_ISO` reader"]
pub struct R(crate::R<DIG_ISO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIG_ISO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIG_ISO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIG_ISO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIG_ISO` writer"]
pub struct W(crate::W<DIG_ISO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIG_ISO_SPEC>;
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
impl From<crate::W<DIG_ISO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIG_ISO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCE_OFF` reader - No public"]
pub struct FORCE_OFF_R(crate::FieldReader<bool>);
impl FORCE_OFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_OFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_OFF_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCE_OFF` writer - No public"]
pub struct FORCE_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_OFF_W<'a> {
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
#[doc = "Field `FORCE_ON` reader - No public"]
pub struct FORCE_ON_R(crate::FieldReader<bool>);
impl FORCE_ON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_ON_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCE_ON` writer - No public"]
pub struct FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_ON_W<'a> {
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
#[doc = "Field `DG_PAD_AUTOHOLD` reader - read only register to indicate digital pad auto-hold status"]
pub struct DG_PAD_AUTOHOLD_R(crate::FieldReader<bool>);
impl DG_PAD_AUTOHOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DG_PAD_AUTOHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_PAD_AUTOHOLD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLR_DG_PAD_AUTOHOLD` writer - wtite only register to clear digital pad auto-hold"]
pub struct CLR_DG_PAD_AUTOHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_DG_PAD_AUTOHOLD_W<'a> {
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
#[doc = "Field `DG_PAD_AUTOHOLD_EN` reader - digital pad enable auto-hold"]
pub struct DG_PAD_AUTOHOLD_EN_R(crate::FieldReader<bool>);
impl DG_PAD_AUTOHOLD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DG_PAD_AUTOHOLD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_PAD_AUTOHOLD_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_PAD_AUTOHOLD_EN` writer - digital pad enable auto-hold"]
pub struct DG_PAD_AUTOHOLD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_PAD_AUTOHOLD_EN_W<'a> {
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
#[doc = "Field `DG_PAD_FORCE_NOISO` reader - digital pad force no ISO"]
pub struct DG_PAD_FORCE_NOISO_R(crate::FieldReader<bool>);
impl DG_PAD_FORCE_NOISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DG_PAD_FORCE_NOISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_PAD_FORCE_NOISO_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_PAD_FORCE_NOISO` writer - digital pad force no ISO"]
pub struct DG_PAD_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_PAD_FORCE_NOISO_W<'a> {
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
#[doc = "Field `DG_PAD_FORCE_ISO` reader - digital pad force ISO"]
pub struct DG_PAD_FORCE_ISO_R(crate::FieldReader<bool>);
impl DG_PAD_FORCE_ISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DG_PAD_FORCE_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_PAD_FORCE_ISO_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_PAD_FORCE_ISO` writer - digital pad force ISO"]
pub struct DG_PAD_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_PAD_FORCE_ISO_W<'a> {
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
#[doc = "Field `DG_PAD_FORCE_UNHOLD` reader - digital pad force un-hold"]
pub struct DG_PAD_FORCE_UNHOLD_R(crate::FieldReader<bool>);
impl DG_PAD_FORCE_UNHOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DG_PAD_FORCE_UNHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_PAD_FORCE_UNHOLD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_PAD_FORCE_UNHOLD` writer - digital pad force un-hold"]
pub struct DG_PAD_FORCE_UNHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_PAD_FORCE_UNHOLD_W<'a> {
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
#[doc = "Field `DG_PAD_FORCE_HOLD` reader - digital pad force hold"]
pub struct DG_PAD_FORCE_HOLD_R(crate::FieldReader<bool>);
impl DG_PAD_FORCE_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DG_PAD_FORCE_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_PAD_FORCE_HOLD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_PAD_FORCE_HOLD` writer - digital pad force hold"]
pub struct DG_PAD_FORCE_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_PAD_FORCE_HOLD_W<'a> {
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
#[doc = "Field `BT_FORCE_ISO` reader - internal SRAM 2 force ISO"]
pub struct BT_FORCE_ISO_R(crate::FieldReader<bool>);
impl BT_FORCE_ISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BT_FORCE_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BT_FORCE_ISO_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BT_FORCE_ISO` writer - internal SRAM 2 force ISO"]
pub struct BT_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> BT_FORCE_ISO_W<'a> {
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
#[doc = "Field `BT_FORCE_NOISO` reader - internal SRAM 2 force no ISO"]
pub struct BT_FORCE_NOISO_R(crate::FieldReader<bool>);
impl BT_FORCE_NOISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BT_FORCE_NOISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BT_FORCE_NOISO_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BT_FORCE_NOISO` writer - internal SRAM 2 force no ISO"]
pub struct BT_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> BT_FORCE_NOISO_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 23)) | ((value as u32 & 1) << 23);
        self.w
    }
}
#[doc = "Field `DG_PERI_FORCE_ISO` reader - internal SRAM 3 force ISO"]
pub struct DG_PERI_FORCE_ISO_R(crate::FieldReader<bool>);
impl DG_PERI_FORCE_ISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DG_PERI_FORCE_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_PERI_FORCE_ISO_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_PERI_FORCE_ISO` writer - internal SRAM 3 force ISO"]
pub struct DG_PERI_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_PERI_FORCE_ISO_W<'a> {
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
#[doc = "Field `DG_PERI_FORCE_NOISO` reader - internal SRAM 3 force no ISO"]
pub struct DG_PERI_FORCE_NOISO_R(crate::FieldReader<bool>);
impl DG_PERI_FORCE_NOISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DG_PERI_FORCE_NOISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_PERI_FORCE_NOISO_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_PERI_FORCE_NOISO` writer - internal SRAM 3 force no ISO"]
pub struct DG_PERI_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_PERI_FORCE_NOISO_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 25)) | ((value as u32 & 1) << 25);
        self.w
    }
}
#[doc = "Field `CPU_TOP_FORCE_ISO` reader - internal SRAM 4 force ISO"]
pub struct CPU_TOP_FORCE_ISO_R(crate::FieldReader<bool>);
impl CPU_TOP_FORCE_ISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPU_TOP_FORCE_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_TOP_FORCE_ISO_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_TOP_FORCE_ISO` writer - internal SRAM 4 force ISO"]
pub struct CPU_TOP_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_TOP_FORCE_ISO_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 26)) | ((value as u32 & 1) << 26);
        self.w
    }
}
#[doc = "Field `CPU_TOP_FORCE_NOISO` reader - internal SRAM 4 force no ISO"]
pub struct CPU_TOP_FORCE_NOISO_R(crate::FieldReader<bool>);
impl CPU_TOP_FORCE_NOISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPU_TOP_FORCE_NOISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_TOP_FORCE_NOISO_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_TOP_FORCE_NOISO` writer - internal SRAM 4 force no ISO"]
pub struct CPU_TOP_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_TOP_FORCE_NOISO_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 27)) | ((value as u32 & 1) << 27);
        self.w
    }
}
#[doc = "Field `WIFI_FORCE_ISO` reader - wifi force ISO"]
pub struct WIFI_FORCE_ISO_R(crate::FieldReader<bool>);
impl WIFI_FORCE_ISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WIFI_FORCE_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFI_FORCE_ISO_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIFI_FORCE_ISO` writer - wifi force ISO"]
pub struct WIFI_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_FORCE_ISO_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 28)) | ((value as u32 & 1) << 28);
        self.w
    }
}
#[doc = "Field `WIFI_FORCE_NOISO` reader - wifi force no ISO"]
pub struct WIFI_FORCE_NOISO_R(crate::FieldReader<bool>);
impl WIFI_FORCE_NOISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WIFI_FORCE_NOISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFI_FORCE_NOISO_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIFI_FORCE_NOISO` writer - wifi force no ISO"]
pub struct WIFI_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_FORCE_NOISO_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 29)) | ((value as u32 & 1) << 29);
        self.w
    }
}
#[doc = "Field `DG_WRAP_FORCE_ISO` reader - digital core force ISO"]
pub struct DG_WRAP_FORCE_ISO_R(crate::FieldReader<bool>);
impl DG_WRAP_FORCE_ISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DG_WRAP_FORCE_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_WRAP_FORCE_ISO_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_WRAP_FORCE_ISO` writer - digital core force ISO"]
pub struct DG_WRAP_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_WRAP_FORCE_ISO_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "Field `DG_WRAP_FORCE_NOISO` reader - digita core force no ISO"]
pub struct DG_WRAP_FORCE_NOISO_R(crate::FieldReader<bool>);
impl DG_WRAP_FORCE_NOISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DG_WRAP_FORCE_NOISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_WRAP_FORCE_NOISO_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_WRAP_FORCE_NOISO` writer - digita core force no ISO"]
pub struct DG_WRAP_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_WRAP_FORCE_NOISO_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - No public"]
    #[inline(always)]
    pub fn force_off(&self) -> FORCE_OFF_R {
        FORCE_OFF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - No public"]
    #[inline(always)]
    pub fn force_on(&self) -> FORCE_ON_R {
        FORCE_ON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - read only register to indicate digital pad auto-hold status"]
    #[inline(always)]
    pub fn dg_pad_autohold(&self) -> DG_PAD_AUTOHOLD_R {
        DG_PAD_AUTOHOLD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - digital pad enable auto-hold"]
    #[inline(always)]
    pub fn dg_pad_autohold_en(&self) -> DG_PAD_AUTOHOLD_EN_R {
        DG_PAD_AUTOHOLD_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - digital pad force no ISO"]
    #[inline(always)]
    pub fn dg_pad_force_noiso(&self) -> DG_PAD_FORCE_NOISO_R {
        DG_PAD_FORCE_NOISO_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - digital pad force ISO"]
    #[inline(always)]
    pub fn dg_pad_force_iso(&self) -> DG_PAD_FORCE_ISO_R {
        DG_PAD_FORCE_ISO_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - digital pad force un-hold"]
    #[inline(always)]
    pub fn dg_pad_force_unhold(&self) -> DG_PAD_FORCE_UNHOLD_R {
        DG_PAD_FORCE_UNHOLD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - digital pad force hold"]
    #[inline(always)]
    pub fn dg_pad_force_hold(&self) -> DG_PAD_FORCE_HOLD_R {
        DG_PAD_FORCE_HOLD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 22 - internal SRAM 2 force ISO"]
    #[inline(always)]
    pub fn bt_force_iso(&self) -> BT_FORCE_ISO_R {
        BT_FORCE_ISO_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - internal SRAM 2 force no ISO"]
    #[inline(always)]
    pub fn bt_force_noiso(&self) -> BT_FORCE_NOISO_R {
        BT_FORCE_NOISO_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - internal SRAM 3 force ISO"]
    #[inline(always)]
    pub fn dg_peri_force_iso(&self) -> DG_PERI_FORCE_ISO_R {
        DG_PERI_FORCE_ISO_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - internal SRAM 3 force no ISO"]
    #[inline(always)]
    pub fn dg_peri_force_noiso(&self) -> DG_PERI_FORCE_NOISO_R {
        DG_PERI_FORCE_NOISO_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - internal SRAM 4 force ISO"]
    #[inline(always)]
    pub fn cpu_top_force_iso(&self) -> CPU_TOP_FORCE_ISO_R {
        CPU_TOP_FORCE_ISO_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - internal SRAM 4 force no ISO"]
    #[inline(always)]
    pub fn cpu_top_force_noiso(&self) -> CPU_TOP_FORCE_NOISO_R {
        CPU_TOP_FORCE_NOISO_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - wifi force ISO"]
    #[inline(always)]
    pub fn wifi_force_iso(&self) -> WIFI_FORCE_ISO_R {
        WIFI_FORCE_ISO_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - wifi force no ISO"]
    #[inline(always)]
    pub fn wifi_force_noiso(&self) -> WIFI_FORCE_NOISO_R {
        WIFI_FORCE_NOISO_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - digital core force ISO"]
    #[inline(always)]
    pub fn dg_wrap_force_iso(&self) -> DG_WRAP_FORCE_ISO_R {
        DG_WRAP_FORCE_ISO_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - digita core force no ISO"]
    #[inline(always)]
    pub fn dg_wrap_force_noiso(&self) -> DG_WRAP_FORCE_NOISO_R {
        DG_WRAP_FORCE_NOISO_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - No public"]
    #[inline(always)]
    pub fn force_off(&mut self) -> FORCE_OFF_W {
        FORCE_OFF_W { w: self }
    }
    #[doc = "Bit 8 - No public"]
    #[inline(always)]
    pub fn force_on(&mut self) -> FORCE_ON_W {
        FORCE_ON_W { w: self }
    }
    #[doc = "Bit 10 - wtite only register to clear digital pad auto-hold"]
    #[inline(always)]
    pub fn clr_dg_pad_autohold(&mut self) -> CLR_DG_PAD_AUTOHOLD_W {
        CLR_DG_PAD_AUTOHOLD_W { w: self }
    }
    #[doc = "Bit 11 - digital pad enable auto-hold"]
    #[inline(always)]
    pub fn dg_pad_autohold_en(&mut self) -> DG_PAD_AUTOHOLD_EN_W {
        DG_PAD_AUTOHOLD_EN_W { w: self }
    }
    #[doc = "Bit 12 - digital pad force no ISO"]
    #[inline(always)]
    pub fn dg_pad_force_noiso(&mut self) -> DG_PAD_FORCE_NOISO_W {
        DG_PAD_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 13 - digital pad force ISO"]
    #[inline(always)]
    pub fn dg_pad_force_iso(&mut self) -> DG_PAD_FORCE_ISO_W {
        DG_PAD_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 14 - digital pad force un-hold"]
    #[inline(always)]
    pub fn dg_pad_force_unhold(&mut self) -> DG_PAD_FORCE_UNHOLD_W {
        DG_PAD_FORCE_UNHOLD_W { w: self }
    }
    #[doc = "Bit 15 - digital pad force hold"]
    #[inline(always)]
    pub fn dg_pad_force_hold(&mut self) -> DG_PAD_FORCE_HOLD_W {
        DG_PAD_FORCE_HOLD_W { w: self }
    }
    #[doc = "Bit 22 - internal SRAM 2 force ISO"]
    #[inline(always)]
    pub fn bt_force_iso(&mut self) -> BT_FORCE_ISO_W {
        BT_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 23 - internal SRAM 2 force no ISO"]
    #[inline(always)]
    pub fn bt_force_noiso(&mut self) -> BT_FORCE_NOISO_W {
        BT_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 24 - internal SRAM 3 force ISO"]
    #[inline(always)]
    pub fn dg_peri_force_iso(&mut self) -> DG_PERI_FORCE_ISO_W {
        DG_PERI_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 25 - internal SRAM 3 force no ISO"]
    #[inline(always)]
    pub fn dg_peri_force_noiso(&mut self) -> DG_PERI_FORCE_NOISO_W {
        DG_PERI_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 26 - internal SRAM 4 force ISO"]
    #[inline(always)]
    pub fn cpu_top_force_iso(&mut self) -> CPU_TOP_FORCE_ISO_W {
        CPU_TOP_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 27 - internal SRAM 4 force no ISO"]
    #[inline(always)]
    pub fn cpu_top_force_noiso(&mut self) -> CPU_TOP_FORCE_NOISO_W {
        CPU_TOP_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 28 - wifi force ISO"]
    #[inline(always)]
    pub fn wifi_force_iso(&mut self) -> WIFI_FORCE_ISO_W {
        WIFI_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 29 - wifi force no ISO"]
    #[inline(always)]
    pub fn wifi_force_noiso(&mut self) -> WIFI_FORCE_NOISO_W {
        WIFI_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 30 - digital core force ISO"]
    #[inline(always)]
    pub fn dg_wrap_force_iso(&mut self) -> DG_WRAP_FORCE_ISO_W {
        DG_WRAP_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 31 - digita core force no ISO"]
    #[inline(always)]
    pub fn dg_wrap_force_noiso(&mut self) -> DG_WRAP_FORCE_NOISO_W {
        DG_WRAP_FORCE_NOISO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "congigure digital power isolation\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dig_iso](index.html) module"]
pub struct DIG_ISO_SPEC;
impl crate::RegisterSpec for DIG_ISO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dig_iso::R](R) reader structure"]
impl crate::Readable for DIG_ISO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dig_iso::W](W) writer structure"]
impl crate::Writable for DIG_ISO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIG_ISO to value 0xaa80_5080"]
impl crate::Resettable for DIG_ISO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xaa80_5080
    }
}
