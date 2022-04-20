#[doc = "Register `DIG_PWC` reader"]
pub struct R(crate::R<DIG_PWC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIG_PWC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIG_PWC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIG_PWC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIG_PWC` writer"]
pub struct W(crate::W<DIG_PWC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIG_PWC_SPEC>;
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
impl From<crate::W<DIG_PWC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIG_PWC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSLP_MEM_FORCE_PD` reader - Set this bit to FPD the memories in the digital system in sleep."]
pub struct LSLP_MEM_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl LSLP_MEM_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSLP_MEM_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSLP_MEM_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSLP_MEM_FORCE_PD` writer - Set this bit to FPD the memories in the digital system in sleep."]
pub struct LSLP_MEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> LSLP_MEM_FORCE_PD_W<'a> {
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
#[doc = "Field `LSLP_MEM_FORCE_PU` reader - Set this bit to FPU the memories in the digital system."]
pub struct LSLP_MEM_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl LSLP_MEM_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSLP_MEM_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSLP_MEM_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSLP_MEM_FORCE_PU` writer - Set this bit to FPU the memories in the digital system."]
pub struct LSLP_MEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> LSLP_MEM_FORCE_PU_W<'a> {
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
#[doc = "Field `ROM0_FORCE_PD` reader - ROM force power down"]
pub struct ROM0_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl ROM0_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROM0_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM0_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM0_FORCE_PD` writer - ROM force power down"]
pub struct ROM0_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM0_FORCE_PD_W<'a> {
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
#[doc = "Field `ROM0_FORCE_PU` reader - ROM force power up"]
pub struct ROM0_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl ROM0_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROM0_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM0_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM0_FORCE_PU` writer - ROM force power up"]
pub struct ROM0_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM0_FORCE_PU_W<'a> {
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
#[doc = "Field `INTER_RAM0_FORCE_PD` reader - internal SRAM 0 force power down"]
pub struct INTER_RAM0_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl INTER_RAM0_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTER_RAM0_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTER_RAM0_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTER_RAM0_FORCE_PD` writer - internal SRAM 0 force power down"]
pub struct INTER_RAM0_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> INTER_RAM0_FORCE_PD_W<'a> {
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
#[doc = "Field `INTER_RAM0_FORCE_PU` reader - internal SRAM 0 force power up"]
pub struct INTER_RAM0_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl INTER_RAM0_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTER_RAM0_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTER_RAM0_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTER_RAM0_FORCE_PU` writer - internal SRAM 0 force power up"]
pub struct INTER_RAM0_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> INTER_RAM0_FORCE_PU_W<'a> {
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
#[doc = "Field `INTER_RAM1_FORCE_PD` reader - internal SRAM 1 force power down"]
pub struct INTER_RAM1_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl INTER_RAM1_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTER_RAM1_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTER_RAM1_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTER_RAM1_FORCE_PD` writer - internal SRAM 1 force power down"]
pub struct INTER_RAM1_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> INTER_RAM1_FORCE_PD_W<'a> {
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
#[doc = "Field `INTER_RAM1_FORCE_PU` reader - internal SRAM 1 force power up"]
pub struct INTER_RAM1_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl INTER_RAM1_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTER_RAM1_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTER_RAM1_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTER_RAM1_FORCE_PU` writer - internal SRAM 1 force power up"]
pub struct INTER_RAM1_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> INTER_RAM1_FORCE_PU_W<'a> {
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
#[doc = "Field `INTER_RAM2_FORCE_PD` reader - internal SRAM 2 force power down"]
pub struct INTER_RAM2_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl INTER_RAM2_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTER_RAM2_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTER_RAM2_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTER_RAM2_FORCE_PD` writer - internal SRAM 2 force power down"]
pub struct INTER_RAM2_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> INTER_RAM2_FORCE_PD_W<'a> {
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
#[doc = "Field `INTER_RAM2_FORCE_PU` reader - internal SRAM 2 force power up"]
pub struct INTER_RAM2_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl INTER_RAM2_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTER_RAM2_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTER_RAM2_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTER_RAM2_FORCE_PU` writer - internal SRAM 2 force power up"]
pub struct INTER_RAM2_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> INTER_RAM2_FORCE_PU_W<'a> {
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
#[doc = "Field `INTER_RAM3_FORCE_PD` reader - internal SRAM 3 force power down"]
pub struct INTER_RAM3_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl INTER_RAM3_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTER_RAM3_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTER_RAM3_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTER_RAM3_FORCE_PD` writer - internal SRAM 3 force power down"]
pub struct INTER_RAM3_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> INTER_RAM3_FORCE_PD_W<'a> {
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
#[doc = "Field `INTER_RAM3_FORCE_PU` reader - internal SRAM 3 force power up"]
pub struct INTER_RAM3_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl INTER_RAM3_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTER_RAM3_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTER_RAM3_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTER_RAM3_FORCE_PU` writer - internal SRAM 3 force power up"]
pub struct INTER_RAM3_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> INTER_RAM3_FORCE_PU_W<'a> {
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
#[doc = "Field `INTER_RAM4_FORCE_PD` reader - internal SRAM 4 force power down"]
pub struct INTER_RAM4_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl INTER_RAM4_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTER_RAM4_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTER_RAM4_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTER_RAM4_FORCE_PD` writer - internal SRAM 4 force power down"]
pub struct INTER_RAM4_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> INTER_RAM4_FORCE_PD_W<'a> {
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
#[doc = "Field `INTER_RAM4_FORCE_PU` reader - internal SRAM 4 force power up"]
pub struct INTER_RAM4_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl INTER_RAM4_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTER_RAM4_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTER_RAM4_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTER_RAM4_FORCE_PU` writer - internal SRAM 4 force power up"]
pub struct INTER_RAM4_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> INTER_RAM4_FORCE_PU_W<'a> {
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
#[doc = "Field `WIFI_FORCE_PD` reader - Set this bit to FPD the Wi-Fi circuit."]
pub struct WIFI_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl WIFI_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WIFI_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFI_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIFI_FORCE_PD` writer - Set this bit to FPD the Wi-Fi circuit."]
pub struct WIFI_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_FORCE_PD_W<'a> {
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
#[doc = "Field `WIFI_FORCE_PU` reader - Set this bit to FPU the Wi-Fi circuit."]
pub struct WIFI_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl WIFI_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WIFI_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFI_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIFI_FORCE_PU` writer - Set this bit to FPU the Wi-Fi circuit."]
pub struct WIFI_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_FORCE_PU_W<'a> {
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
#[doc = "Field `DG_WRAP_FORCE_PD` reader - Set this bit to FPD the digital system."]
pub struct DG_WRAP_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl DG_WRAP_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DG_WRAP_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_WRAP_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_WRAP_FORCE_PD` writer - Set this bit to FPD the digital system."]
pub struct DG_WRAP_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_WRAP_FORCE_PD_W<'a> {
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
#[doc = "Field `DG_WRAP_FORCE_PU` reader - Set this bit to FPD the DC-DC convertor in the digital system."]
pub struct DG_WRAP_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl DG_WRAP_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DG_WRAP_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_WRAP_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_WRAP_FORCE_PU` writer - Set this bit to FPD the DC-DC convertor in the digital system."]
pub struct DG_WRAP_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_WRAP_FORCE_PU_W<'a> {
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
#[doc = "Field `DG_DCDC_FORCE_PD` reader - Set this bit to FPD the DC-DC convertor in the digital system."]
pub struct DG_DCDC_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl DG_DCDC_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DG_DCDC_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_DCDC_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_DCDC_FORCE_PD` writer - Set this bit to FPD the DC-DC convertor in the digital system."]
pub struct DG_DCDC_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_DCDC_FORCE_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 21)) | ((value as u32 & 1) << 21);
        self.w
    }
}
#[doc = "Field `DG_DCDC_FORCE_PU` reader - Set this bit to FPU the DC-DC convertor in the digital system."]
pub struct DG_DCDC_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl DG_DCDC_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DG_DCDC_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_DCDC_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_DCDC_FORCE_PU` writer - Set this bit to FPU the DC-DC convertor in the digital system."]
pub struct DG_DCDC_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_DCDC_FORCE_PU_W<'a> {
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
#[doc = "Field `DG_DCDC_PD_EN` reader - Set this bit to enable PD for the DC-DC convertor in the digital system."]
pub struct DG_DCDC_PD_EN_R(crate::FieldReader<bool, bool>);
impl DG_DCDC_PD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DG_DCDC_PD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_DCDC_PD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_DCDC_PD_EN` writer - Set this bit to enable PD for the DC-DC convertor in the digital system."]
pub struct DG_DCDC_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_DCDC_PD_EN_W<'a> {
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
#[doc = "Field `ROM0_PD_EN` reader - enable power down ROM in sleep"]
pub struct ROM0_PD_EN_R(crate::FieldReader<bool, bool>);
impl ROM0_PD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROM0_PD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM0_PD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM0_PD_EN` writer - enable power down ROM in sleep"]
pub struct ROM0_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM0_PD_EN_W<'a> {
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
#[doc = "Field `INTER_RAM0_PD_EN` reader - enable power down internal SRAM 0 in sleep"]
pub struct INTER_RAM0_PD_EN_R(crate::FieldReader<bool, bool>);
impl INTER_RAM0_PD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTER_RAM0_PD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTER_RAM0_PD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTER_RAM0_PD_EN` writer - enable power down internal SRAM 0 in sleep"]
pub struct INTER_RAM0_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTER_RAM0_PD_EN_W<'a> {
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
#[doc = "Field `INTER_RAM1_PD_EN` reader - enable power down internal SRAM 1 in sleep"]
pub struct INTER_RAM1_PD_EN_R(crate::FieldReader<bool, bool>);
impl INTER_RAM1_PD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTER_RAM1_PD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTER_RAM1_PD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTER_RAM1_PD_EN` writer - enable power down internal SRAM 1 in sleep"]
pub struct INTER_RAM1_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTER_RAM1_PD_EN_W<'a> {
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
#[doc = "Field `INTER_RAM2_PD_EN` reader - enable power down internal SRAM 2 in sleep"]
pub struct INTER_RAM2_PD_EN_R(crate::FieldReader<bool, bool>);
impl INTER_RAM2_PD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTER_RAM2_PD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTER_RAM2_PD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTER_RAM2_PD_EN` writer - enable power down internal SRAM 2 in sleep"]
pub struct INTER_RAM2_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTER_RAM2_PD_EN_W<'a> {
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
#[doc = "Field `INTER_RAM3_PD_EN` reader - enable power down internal SRAM 3 in sleep"]
pub struct INTER_RAM3_PD_EN_R(crate::FieldReader<bool, bool>);
impl INTER_RAM3_PD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTER_RAM3_PD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTER_RAM3_PD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTER_RAM3_PD_EN` writer - enable power down internal SRAM 3 in sleep"]
pub struct INTER_RAM3_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTER_RAM3_PD_EN_W<'a> {
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
#[doc = "Field `INTER_RAM4_PD_EN` reader - enable power down internal SRAM 4 in sleep"]
pub struct INTER_RAM4_PD_EN_R(crate::FieldReader<bool, bool>);
impl INTER_RAM4_PD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTER_RAM4_PD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTER_RAM4_PD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTER_RAM4_PD_EN` writer - enable power down internal SRAM 4 in sleep"]
pub struct INTER_RAM4_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTER_RAM4_PD_EN_W<'a> {
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
#[doc = "Field `WIFI_PD_EN` reader - Set this bit to enable PD for the Wi-Fi circuit in sleep."]
pub struct WIFI_PD_EN_R(crate::FieldReader<bool, bool>);
impl WIFI_PD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WIFI_PD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFI_PD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIFI_PD_EN` writer - Set this bit to enable PD for the Wi-Fi circuit in sleep."]
pub struct WIFI_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_PD_EN_W<'a> {
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
#[doc = "Field `DG_WRAP_PD_EN` reader - Set this bit to enable PD for the digital system in sleep."]
pub struct DG_WRAP_PD_EN_R(crate::FieldReader<bool, bool>);
impl DG_WRAP_PD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DG_WRAP_PD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_WRAP_PD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_WRAP_PD_EN` writer - Set this bit to enable PD for the digital system in sleep."]
pub struct DG_WRAP_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_WRAP_PD_EN_W<'a> {
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
    #[doc = "Bit 3 - Set this bit to FPD the memories in the digital system in sleep."]
    #[inline(always)]
    pub fn lslp_mem_force_pd(&self) -> LSLP_MEM_FORCE_PD_R {
        LSLP_MEM_FORCE_PD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to FPU the memories in the digital system."]
    #[inline(always)]
    pub fn lslp_mem_force_pu(&self) -> LSLP_MEM_FORCE_PU_R {
        LSLP_MEM_FORCE_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ROM force power down"]
    #[inline(always)]
    pub fn rom0_force_pd(&self) -> ROM0_FORCE_PD_R {
        ROM0_FORCE_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ROM force power up"]
    #[inline(always)]
    pub fn rom0_force_pu(&self) -> ROM0_FORCE_PU_R {
        ROM0_FORCE_PU_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - internal SRAM 0 force power down"]
    #[inline(always)]
    pub fn inter_ram0_force_pd(&self) -> INTER_RAM0_FORCE_PD_R {
        INTER_RAM0_FORCE_PD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - internal SRAM 0 force power up"]
    #[inline(always)]
    pub fn inter_ram0_force_pu(&self) -> INTER_RAM0_FORCE_PU_R {
        INTER_RAM0_FORCE_PU_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - internal SRAM 1 force power down"]
    #[inline(always)]
    pub fn inter_ram1_force_pd(&self) -> INTER_RAM1_FORCE_PD_R {
        INTER_RAM1_FORCE_PD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - internal SRAM 1 force power up"]
    #[inline(always)]
    pub fn inter_ram1_force_pu(&self) -> INTER_RAM1_FORCE_PU_R {
        INTER_RAM1_FORCE_PU_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - internal SRAM 2 force power down"]
    #[inline(always)]
    pub fn inter_ram2_force_pd(&self) -> INTER_RAM2_FORCE_PD_R {
        INTER_RAM2_FORCE_PD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - internal SRAM 2 force power up"]
    #[inline(always)]
    pub fn inter_ram2_force_pu(&self) -> INTER_RAM2_FORCE_PU_R {
        INTER_RAM2_FORCE_PU_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - internal SRAM 3 force power down"]
    #[inline(always)]
    pub fn inter_ram3_force_pd(&self) -> INTER_RAM3_FORCE_PD_R {
        INTER_RAM3_FORCE_PD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - internal SRAM 3 force power up"]
    #[inline(always)]
    pub fn inter_ram3_force_pu(&self) -> INTER_RAM3_FORCE_PU_R {
        INTER_RAM3_FORCE_PU_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - internal SRAM 4 force power down"]
    #[inline(always)]
    pub fn inter_ram4_force_pd(&self) -> INTER_RAM4_FORCE_PD_R {
        INTER_RAM4_FORCE_PD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - internal SRAM 4 force power up"]
    #[inline(always)]
    pub fn inter_ram4_force_pu(&self) -> INTER_RAM4_FORCE_PU_R {
        INTER_RAM4_FORCE_PU_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Set this bit to FPD the Wi-Fi circuit."]
    #[inline(always)]
    pub fn wifi_force_pd(&self) -> WIFI_FORCE_PD_R {
        WIFI_FORCE_PD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Set this bit to FPU the Wi-Fi circuit."]
    #[inline(always)]
    pub fn wifi_force_pu(&self) -> WIFI_FORCE_PU_R {
        WIFI_FORCE_PU_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Set this bit to FPD the digital system."]
    #[inline(always)]
    pub fn dg_wrap_force_pd(&self) -> DG_WRAP_FORCE_PD_R {
        DG_WRAP_FORCE_PD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Set this bit to FPD the DC-DC convertor in the digital system."]
    #[inline(always)]
    pub fn dg_wrap_force_pu(&self) -> DG_WRAP_FORCE_PU_R {
        DG_WRAP_FORCE_PU_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to FPD the DC-DC convertor in the digital system."]
    #[inline(always)]
    pub fn dg_dcdc_force_pd(&self) -> DG_DCDC_FORCE_PD_R {
        DG_DCDC_FORCE_PD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set this bit to FPU the DC-DC convertor in the digital system."]
    #[inline(always)]
    pub fn dg_dcdc_force_pu(&self) -> DG_DCDC_FORCE_PU_R {
        DG_DCDC_FORCE_PU_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Set this bit to enable PD for the DC-DC convertor in the digital system."]
    #[inline(always)]
    pub fn dg_dcdc_pd_en(&self) -> DG_DCDC_PD_EN_R {
        DG_DCDC_PD_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - enable power down ROM in sleep"]
    #[inline(always)]
    pub fn rom0_pd_en(&self) -> ROM0_PD_EN_R {
        ROM0_PD_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - enable power down internal SRAM 0 in sleep"]
    #[inline(always)]
    pub fn inter_ram0_pd_en(&self) -> INTER_RAM0_PD_EN_R {
        INTER_RAM0_PD_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - enable power down internal SRAM 1 in sleep"]
    #[inline(always)]
    pub fn inter_ram1_pd_en(&self) -> INTER_RAM1_PD_EN_R {
        INTER_RAM1_PD_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - enable power down internal SRAM 2 in sleep"]
    #[inline(always)]
    pub fn inter_ram2_pd_en(&self) -> INTER_RAM2_PD_EN_R {
        INTER_RAM2_PD_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - enable power down internal SRAM 3 in sleep"]
    #[inline(always)]
    pub fn inter_ram3_pd_en(&self) -> INTER_RAM3_PD_EN_R {
        INTER_RAM3_PD_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - enable power down internal SRAM 4 in sleep"]
    #[inline(always)]
    pub fn inter_ram4_pd_en(&self) -> INTER_RAM4_PD_EN_R {
        INTER_RAM4_PD_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Set this bit to enable PD for the Wi-Fi circuit in sleep."]
    #[inline(always)]
    pub fn wifi_pd_en(&self) -> WIFI_PD_EN_R {
        WIFI_PD_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Set this bit to enable PD for the digital system in sleep."]
    #[inline(always)]
    pub fn dg_wrap_pd_en(&self) -> DG_WRAP_PD_EN_R {
        DG_WRAP_PD_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Set this bit to FPD the memories in the digital system in sleep."]
    #[inline(always)]
    pub fn lslp_mem_force_pd(&mut self) -> LSLP_MEM_FORCE_PD_W {
        LSLP_MEM_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 4 - Set this bit to FPU the memories in the digital system."]
    #[inline(always)]
    pub fn lslp_mem_force_pu(&mut self) -> LSLP_MEM_FORCE_PU_W {
        LSLP_MEM_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 5 - ROM force power down"]
    #[inline(always)]
    pub fn rom0_force_pd(&mut self) -> ROM0_FORCE_PD_W {
        ROM0_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 6 - ROM force power up"]
    #[inline(always)]
    pub fn rom0_force_pu(&mut self) -> ROM0_FORCE_PU_W {
        ROM0_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 7 - internal SRAM 0 force power down"]
    #[inline(always)]
    pub fn inter_ram0_force_pd(&mut self) -> INTER_RAM0_FORCE_PD_W {
        INTER_RAM0_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 8 - internal SRAM 0 force power up"]
    #[inline(always)]
    pub fn inter_ram0_force_pu(&mut self) -> INTER_RAM0_FORCE_PU_W {
        INTER_RAM0_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 9 - internal SRAM 1 force power down"]
    #[inline(always)]
    pub fn inter_ram1_force_pd(&mut self) -> INTER_RAM1_FORCE_PD_W {
        INTER_RAM1_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 10 - internal SRAM 1 force power up"]
    #[inline(always)]
    pub fn inter_ram1_force_pu(&mut self) -> INTER_RAM1_FORCE_PU_W {
        INTER_RAM1_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 11 - internal SRAM 2 force power down"]
    #[inline(always)]
    pub fn inter_ram2_force_pd(&mut self) -> INTER_RAM2_FORCE_PD_W {
        INTER_RAM2_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 12 - internal SRAM 2 force power up"]
    #[inline(always)]
    pub fn inter_ram2_force_pu(&mut self) -> INTER_RAM2_FORCE_PU_W {
        INTER_RAM2_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 13 - internal SRAM 3 force power down"]
    #[inline(always)]
    pub fn inter_ram3_force_pd(&mut self) -> INTER_RAM3_FORCE_PD_W {
        INTER_RAM3_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 14 - internal SRAM 3 force power up"]
    #[inline(always)]
    pub fn inter_ram3_force_pu(&mut self) -> INTER_RAM3_FORCE_PU_W {
        INTER_RAM3_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 15 - internal SRAM 4 force power down"]
    #[inline(always)]
    pub fn inter_ram4_force_pd(&mut self) -> INTER_RAM4_FORCE_PD_W {
        INTER_RAM4_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 16 - internal SRAM 4 force power up"]
    #[inline(always)]
    pub fn inter_ram4_force_pu(&mut self) -> INTER_RAM4_FORCE_PU_W {
        INTER_RAM4_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 17 - Set this bit to FPD the Wi-Fi circuit."]
    #[inline(always)]
    pub fn wifi_force_pd(&mut self) -> WIFI_FORCE_PD_W {
        WIFI_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 18 - Set this bit to FPU the Wi-Fi circuit."]
    #[inline(always)]
    pub fn wifi_force_pu(&mut self) -> WIFI_FORCE_PU_W {
        WIFI_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 19 - Set this bit to FPD the digital system."]
    #[inline(always)]
    pub fn dg_wrap_force_pd(&mut self) -> DG_WRAP_FORCE_PD_W {
        DG_WRAP_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 20 - Set this bit to FPD the DC-DC convertor in the digital system."]
    #[inline(always)]
    pub fn dg_wrap_force_pu(&mut self) -> DG_WRAP_FORCE_PU_W {
        DG_WRAP_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 21 - Set this bit to FPD the DC-DC convertor in the digital system."]
    #[inline(always)]
    pub fn dg_dcdc_force_pd(&mut self) -> DG_DCDC_FORCE_PD_W {
        DG_DCDC_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 22 - Set this bit to FPU the DC-DC convertor in the digital system."]
    #[inline(always)]
    pub fn dg_dcdc_force_pu(&mut self) -> DG_DCDC_FORCE_PU_W {
        DG_DCDC_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 23 - Set this bit to enable PD for the DC-DC convertor in the digital system."]
    #[inline(always)]
    pub fn dg_dcdc_pd_en(&mut self) -> DG_DCDC_PD_EN_W {
        DG_DCDC_PD_EN_W { w: self }
    }
    #[doc = "Bit 24 - enable power down ROM in sleep"]
    #[inline(always)]
    pub fn rom0_pd_en(&mut self) -> ROM0_PD_EN_W {
        ROM0_PD_EN_W { w: self }
    }
    #[doc = "Bit 25 - enable power down internal SRAM 0 in sleep"]
    #[inline(always)]
    pub fn inter_ram0_pd_en(&mut self) -> INTER_RAM0_PD_EN_W {
        INTER_RAM0_PD_EN_W { w: self }
    }
    #[doc = "Bit 26 - enable power down internal SRAM 1 in sleep"]
    #[inline(always)]
    pub fn inter_ram1_pd_en(&mut self) -> INTER_RAM1_PD_EN_W {
        INTER_RAM1_PD_EN_W { w: self }
    }
    #[doc = "Bit 27 - enable power down internal SRAM 2 in sleep"]
    #[inline(always)]
    pub fn inter_ram2_pd_en(&mut self) -> INTER_RAM2_PD_EN_W {
        INTER_RAM2_PD_EN_W { w: self }
    }
    #[doc = "Bit 28 - enable power down internal SRAM 3 in sleep"]
    #[inline(always)]
    pub fn inter_ram3_pd_en(&mut self) -> INTER_RAM3_PD_EN_W {
        INTER_RAM3_PD_EN_W { w: self }
    }
    #[doc = "Bit 29 - enable power down internal SRAM 4 in sleep"]
    #[inline(always)]
    pub fn inter_ram4_pd_en(&mut self) -> INTER_RAM4_PD_EN_W {
        INTER_RAM4_PD_EN_W { w: self }
    }
    #[doc = "Bit 30 - Set this bit to enable PD for the Wi-Fi circuit in sleep."]
    #[inline(always)]
    pub fn wifi_pd_en(&mut self) -> WIFI_PD_EN_W {
        WIFI_PD_EN_W { w: self }
    }
    #[doc = "Bit 31 - Set this bit to enable PD for the digital system in sleep."]
    #[inline(always)]
    pub fn dg_wrap_pd_en(&mut self) -> DG_WRAP_PD_EN_W {
        DG_WRAP_PD_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital system power configuraiton register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dig_pwc]
(index.html) module"]
pub struct DIG_PWC_SPEC;
impl crate::RegisterSpec for DIG_PWC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dig_pwc::R]
(R) reader structure"]
impl crate::Readable for DIG_PWC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dig_pwc::W]
(W) writer structure"]
impl crate::Writable for DIG_PWC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIG_PWC to value 0x0055_5550"]
impl crate::Resettable for DIG_PWC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0055_5550
    }
}
