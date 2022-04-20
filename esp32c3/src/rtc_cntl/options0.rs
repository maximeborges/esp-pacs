#[doc = "Register `OPTIONS0` reader"]
pub struct R(crate::R<OPTIONS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTIONS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTIONS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTIONS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPTIONS0` writer"]
pub struct W(crate::W<OPTIONS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTIONS0_SPEC>;
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
impl From<crate::W<OPTIONS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTIONS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_STALL_APPCPU_C0` reader - {reg_sw_stall_appcpu_c1\\[5:0\\]
, reg_sw_stall_appcpu_c0\\[1:0\\]
} == 0x86 will stall APP CPU"]
pub struct SW_STALL_APPCPU_C0_R(crate::FieldReader<u8, u8>);
impl SW_STALL_APPCPU_C0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SW_STALL_APPCPU_C0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_STALL_APPCPU_C0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_STALL_APPCPU_C0` writer - {reg_sw_stall_appcpu_c1\\[5:0\\]
, reg_sw_stall_appcpu_c0\\[1:0\\]
} == 0x86 will stall APP CPU"]
pub struct SW_STALL_APPCPU_C0_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_STALL_APPCPU_C0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `SW_STALL_PROCPU_C0` reader - {reg_sw_stall_procpu_c1\\[5:0\\]
, reg_sw_stall_procpu_c0\\[1:0\\]
} == 0x86 will stall PRO CPU"]
pub struct SW_STALL_PROCPU_C0_R(crate::FieldReader<u8, u8>);
impl SW_STALL_PROCPU_C0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SW_STALL_PROCPU_C0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_STALL_PROCPU_C0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_STALL_PROCPU_C0` writer - {reg_sw_stall_procpu_c1\\[5:0\\]
, reg_sw_stall_procpu_c0\\[1:0\\]
} == 0x86 will stall PRO CPU"]
pub struct SW_STALL_PROCPU_C0_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_STALL_PROCPU_C0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 2)) | ((value as u32 & 3) << 2);
        self.w
    }
}
#[doc = "Field `SW_APPCPU_RST` writer - APP CPU SW reset"]
pub struct SW_APPCPU_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_APPCPU_RST_W<'a> {
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
#[doc = "Field `SW_PROCPU_RST` writer - PRO CPU SW reset"]
pub struct SW_PROCPU_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_PROCPU_RST_W<'a> {
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
#[doc = "Field `BB_I2C_FORCE_PD` reader - BB_I2C force power down"]
pub struct BB_I2C_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl BB_I2C_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BB_I2C_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BB_I2C_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BB_I2C_FORCE_PD` writer - BB_I2C force power down"]
pub struct BB_I2C_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_I2C_FORCE_PD_W<'a> {
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
#[doc = "Field `BB_I2C_FORCE_PU` reader - BB_I2C force power up"]
pub struct BB_I2C_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl BB_I2C_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BB_I2C_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BB_I2C_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BB_I2C_FORCE_PU` writer - BB_I2C force power up"]
pub struct BB_I2C_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_I2C_FORCE_PU_W<'a> {
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
#[doc = "Field `BBPLL_I2C_FORCE_PD` reader - BB_PLL _I2C force power down"]
pub struct BBPLL_I2C_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl BBPLL_I2C_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BBPLL_I2C_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BBPLL_I2C_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BBPLL_I2C_FORCE_PD` writer - BB_PLL _I2C force power down"]
pub struct BBPLL_I2C_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> BBPLL_I2C_FORCE_PD_W<'a> {
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
#[doc = "Field `BBPLL_I2C_FORCE_PU` reader - BB_PLL_I2C force power up"]
pub struct BBPLL_I2C_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl BBPLL_I2C_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BBPLL_I2C_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BBPLL_I2C_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BBPLL_I2C_FORCE_PU` writer - BB_PLL_I2C force power up"]
pub struct BBPLL_I2C_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> BBPLL_I2C_FORCE_PU_W<'a> {
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
#[doc = "Field `BBPLL_FORCE_PD` reader - BB_PLL force power down"]
pub struct BBPLL_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl BBPLL_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BBPLL_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BBPLL_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BBPLL_FORCE_PD` writer - BB_PLL force power down"]
pub struct BBPLL_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> BBPLL_FORCE_PD_W<'a> {
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
#[doc = "Field `BBPLL_FORCE_PU` reader - BB_PLL force power up"]
pub struct BBPLL_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl BBPLL_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BBPLL_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BBPLL_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BBPLL_FORCE_PU` writer - BB_PLL force power up"]
pub struct BBPLL_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> BBPLL_FORCE_PU_W<'a> {
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
#[doc = "Field `XTL_FORCE_PD` reader - crystall force power down"]
pub struct XTL_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl XTL_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTL_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTL_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTL_FORCE_PD` writer - crystall force power down"]
pub struct XTL_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL_FORCE_PD_W<'a> {
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
#[doc = "Field `XTL_FORCE_PU` reader - crystall force power up"]
pub struct XTL_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl XTL_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTL_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTL_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTL_FORCE_PU` writer - crystall force power up"]
pub struct XTL_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL_FORCE_PU_W<'a> {
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
#[doc = "Field `XTL_EN_WAIT` reader - wait bias_sleep and current source wakeup"]
pub struct XTL_EN_WAIT_R(crate::FieldReader<u8, u8>);
impl XTL_EN_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XTL_EN_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTL_EN_WAIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTL_EN_WAIT` writer - wait bias_sleep and current source wakeup"]
pub struct XTL_EN_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL_EN_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 14)) | ((value as u32 & 0x0f) << 14);
        self.w
    }
}
#[doc = "Field `XTL_EXT_CTR_SEL` reader - analog configure"]
pub struct XTL_EXT_CTR_SEL_R(crate::FieldReader<u8, u8>);
impl XTL_EXT_CTR_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XTL_EXT_CTR_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTL_EXT_CTR_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTL_EXT_CTR_SEL` writer - analog configure"]
pub struct XTL_EXT_CTR_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL_EXT_CTR_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 20)) | ((value as u32 & 7) << 20);
        self.w
    }
}
#[doc = "Field `XTL_FORCE_ISO` reader - analog configure"]
pub struct XTL_FORCE_ISO_R(crate::FieldReader<bool, bool>);
impl XTL_FORCE_ISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTL_FORCE_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTL_FORCE_ISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTL_FORCE_ISO` writer - analog configure"]
pub struct XTL_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL_FORCE_ISO_W<'a> {
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
#[doc = "Field `PLL_FORCE_ISO` reader - analog configure"]
pub struct PLL_FORCE_ISO_R(crate::FieldReader<bool, bool>);
impl PLL_FORCE_ISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLL_FORCE_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_FORCE_ISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_FORCE_ISO` writer - analog configure"]
pub struct PLL_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_FORCE_ISO_W<'a> {
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
#[doc = "Field `ANALOG_FORCE_ISO` reader - analog configure"]
pub struct ANALOG_FORCE_ISO_R(crate::FieldReader<bool, bool>);
impl ANALOG_FORCE_ISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ANALOG_FORCE_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANALOG_FORCE_ISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANALOG_FORCE_ISO` writer - analog configure"]
pub struct ANALOG_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_FORCE_ISO_W<'a> {
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
#[doc = "Field `XTL_FORCE_NOISO` reader - analog configure"]
pub struct XTL_FORCE_NOISO_R(crate::FieldReader<bool, bool>);
impl XTL_FORCE_NOISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTL_FORCE_NOISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTL_FORCE_NOISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTL_FORCE_NOISO` writer - analog configure"]
pub struct XTL_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL_FORCE_NOISO_W<'a> {
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
#[doc = "Field `PLL_FORCE_NOISO` reader - analog configure"]
pub struct PLL_FORCE_NOISO_R(crate::FieldReader<bool, bool>);
impl PLL_FORCE_NOISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLL_FORCE_NOISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_FORCE_NOISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_FORCE_NOISO` writer - analog configure"]
pub struct PLL_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_FORCE_NOISO_W<'a> {
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
#[doc = "Field `ANALOG_FORCE_NOISO` reader - analog configure"]
pub struct ANALOG_FORCE_NOISO_R(crate::FieldReader<bool, bool>);
impl ANALOG_FORCE_NOISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ANALOG_FORCE_NOISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANALOG_FORCE_NOISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANALOG_FORCE_NOISO` writer - analog configure"]
pub struct ANALOG_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_FORCE_NOISO_W<'a> {
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
#[doc = "Field `DG_WRAP_FORCE_RST` reader - digital wrap force reset in deep sleep"]
pub struct DG_WRAP_FORCE_RST_R(crate::FieldReader<bool, bool>);
impl DG_WRAP_FORCE_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DG_WRAP_FORCE_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_WRAP_FORCE_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_WRAP_FORCE_RST` writer - digital wrap force reset in deep sleep"]
pub struct DG_WRAP_FORCE_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_WRAP_FORCE_RST_W<'a> {
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
#[doc = "Field `DG_WRAP_FORCE_NORST` reader - digital core force no reset in deep sleep"]
pub struct DG_WRAP_FORCE_NORST_R(crate::FieldReader<bool, bool>);
impl DG_WRAP_FORCE_NORST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DG_WRAP_FORCE_NORST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_WRAP_FORCE_NORST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_WRAP_FORCE_NORST` writer - digital core force no reset in deep sleep"]
pub struct DG_WRAP_FORCE_NORST_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_WRAP_FORCE_NORST_W<'a> {
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
#[doc = "Field `SW_SYS_RST` writer - SW system reset"]
pub struct SW_SYS_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SYS_RST_W<'a> {
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
    #[doc = "Bits 0:1 - {reg_sw_stall_appcpu_c1\\[5:0\\]
, reg_sw_stall_appcpu_c0\\[1:0\\]
} == 0x86 will stall APP CPU"]
    #[inline(always)]
    pub fn sw_stall_appcpu_c0(&self) -> SW_STALL_APPCPU_C0_R {
        SW_STALL_APPCPU_C0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - {reg_sw_stall_procpu_c1\\[5:0\\]
, reg_sw_stall_procpu_c0\\[1:0\\]
} == 0x86 will stall PRO CPU"]
    #[inline(always)]
    pub fn sw_stall_procpu_c0(&self) -> SW_STALL_PROCPU_C0_R {
        SW_STALL_PROCPU_C0_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 6 - BB_I2C force power down"]
    #[inline(always)]
    pub fn bb_i2c_force_pd(&self) -> BB_I2C_FORCE_PD_R {
        BB_I2C_FORCE_PD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BB_I2C force power up"]
    #[inline(always)]
    pub fn bb_i2c_force_pu(&self) -> BB_I2C_FORCE_PU_R {
        BB_I2C_FORCE_PU_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - BB_PLL _I2C force power down"]
    #[inline(always)]
    pub fn bbpll_i2c_force_pd(&self) -> BBPLL_I2C_FORCE_PD_R {
        BBPLL_I2C_FORCE_PD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BB_PLL_I2C force power up"]
    #[inline(always)]
    pub fn bbpll_i2c_force_pu(&self) -> BBPLL_I2C_FORCE_PU_R {
        BBPLL_I2C_FORCE_PU_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BB_PLL force power down"]
    #[inline(always)]
    pub fn bbpll_force_pd(&self) -> BBPLL_FORCE_PD_R {
        BBPLL_FORCE_PD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BB_PLL force power up"]
    #[inline(always)]
    pub fn bbpll_force_pu(&self) -> BBPLL_FORCE_PU_R {
        BBPLL_FORCE_PU_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - crystall force power down"]
    #[inline(always)]
    pub fn xtl_force_pd(&self) -> XTL_FORCE_PD_R {
        XTL_FORCE_PD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - crystall force power up"]
    #[inline(always)]
    pub fn xtl_force_pu(&self) -> XTL_FORCE_PU_R {
        XTL_FORCE_PU_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:17 - wait bias_sleep and current source wakeup"]
    #[inline(always)]
    pub fn xtl_en_wait(&self) -> XTL_EN_WAIT_R {
        XTL_EN_WAIT_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - analog configure"]
    #[inline(always)]
    pub fn xtl_ext_ctr_sel(&self) -> XTL_EXT_CTR_SEL_R {
        XTL_EXT_CTR_SEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - analog configure"]
    #[inline(always)]
    pub fn xtl_force_iso(&self) -> XTL_FORCE_ISO_R {
        XTL_FORCE_ISO_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - analog configure"]
    #[inline(always)]
    pub fn pll_force_iso(&self) -> PLL_FORCE_ISO_R {
        PLL_FORCE_ISO_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - analog configure"]
    #[inline(always)]
    pub fn analog_force_iso(&self) -> ANALOG_FORCE_ISO_R {
        ANALOG_FORCE_ISO_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - analog configure"]
    #[inline(always)]
    pub fn xtl_force_noiso(&self) -> XTL_FORCE_NOISO_R {
        XTL_FORCE_NOISO_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - analog configure"]
    #[inline(always)]
    pub fn pll_force_noiso(&self) -> PLL_FORCE_NOISO_R {
        PLL_FORCE_NOISO_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - analog configure"]
    #[inline(always)]
    pub fn analog_force_noiso(&self) -> ANALOG_FORCE_NOISO_R {
        ANALOG_FORCE_NOISO_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - digital wrap force reset in deep sleep"]
    #[inline(always)]
    pub fn dg_wrap_force_rst(&self) -> DG_WRAP_FORCE_RST_R {
        DG_WRAP_FORCE_RST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - digital core force no reset in deep sleep"]
    #[inline(always)]
    pub fn dg_wrap_force_norst(&self) -> DG_WRAP_FORCE_NORST_R {
        DG_WRAP_FORCE_NORST_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - {reg_sw_stall_appcpu_c1\\[5:0\\]
, reg_sw_stall_appcpu_c0\\[1:0\\]
} == 0x86 will stall APP CPU"]
    #[inline(always)]
    pub fn sw_stall_appcpu_c0(&mut self) -> SW_STALL_APPCPU_C0_W {
        SW_STALL_APPCPU_C0_W { w: self }
    }
    #[doc = "Bits 2:3 - {reg_sw_stall_procpu_c1\\[5:0\\]
, reg_sw_stall_procpu_c0\\[1:0\\]
} == 0x86 will stall PRO CPU"]
    #[inline(always)]
    pub fn sw_stall_procpu_c0(&mut self) -> SW_STALL_PROCPU_C0_W {
        SW_STALL_PROCPU_C0_W { w: self }
    }
    #[doc = "Bit 4 - APP CPU SW reset"]
    #[inline(always)]
    pub fn sw_appcpu_rst(&mut self) -> SW_APPCPU_RST_W {
        SW_APPCPU_RST_W { w: self }
    }
    #[doc = "Bit 5 - PRO CPU SW reset"]
    #[inline(always)]
    pub fn sw_procpu_rst(&mut self) -> SW_PROCPU_RST_W {
        SW_PROCPU_RST_W { w: self }
    }
    #[doc = "Bit 6 - BB_I2C force power down"]
    #[inline(always)]
    pub fn bb_i2c_force_pd(&mut self) -> BB_I2C_FORCE_PD_W {
        BB_I2C_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 7 - BB_I2C force power up"]
    #[inline(always)]
    pub fn bb_i2c_force_pu(&mut self) -> BB_I2C_FORCE_PU_W {
        BB_I2C_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 8 - BB_PLL _I2C force power down"]
    #[inline(always)]
    pub fn bbpll_i2c_force_pd(&mut self) -> BBPLL_I2C_FORCE_PD_W {
        BBPLL_I2C_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 9 - BB_PLL_I2C force power up"]
    #[inline(always)]
    pub fn bbpll_i2c_force_pu(&mut self) -> BBPLL_I2C_FORCE_PU_W {
        BBPLL_I2C_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 10 - BB_PLL force power down"]
    #[inline(always)]
    pub fn bbpll_force_pd(&mut self) -> BBPLL_FORCE_PD_W {
        BBPLL_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 11 - BB_PLL force power up"]
    #[inline(always)]
    pub fn bbpll_force_pu(&mut self) -> BBPLL_FORCE_PU_W {
        BBPLL_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 12 - crystall force power down"]
    #[inline(always)]
    pub fn xtl_force_pd(&mut self) -> XTL_FORCE_PD_W {
        XTL_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 13 - crystall force power up"]
    #[inline(always)]
    pub fn xtl_force_pu(&mut self) -> XTL_FORCE_PU_W {
        XTL_FORCE_PU_W { w: self }
    }
    #[doc = "Bits 14:17 - wait bias_sleep and current source wakeup"]
    #[inline(always)]
    pub fn xtl_en_wait(&mut self) -> XTL_EN_WAIT_W {
        XTL_EN_WAIT_W { w: self }
    }
    #[doc = "Bits 20:22 - analog configure"]
    #[inline(always)]
    pub fn xtl_ext_ctr_sel(&mut self) -> XTL_EXT_CTR_SEL_W {
        XTL_EXT_CTR_SEL_W { w: self }
    }
    #[doc = "Bit 23 - analog configure"]
    #[inline(always)]
    pub fn xtl_force_iso(&mut self) -> XTL_FORCE_ISO_W {
        XTL_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 24 - analog configure"]
    #[inline(always)]
    pub fn pll_force_iso(&mut self) -> PLL_FORCE_ISO_W {
        PLL_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 25 - analog configure"]
    #[inline(always)]
    pub fn analog_force_iso(&mut self) -> ANALOG_FORCE_ISO_W {
        ANALOG_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 26 - analog configure"]
    #[inline(always)]
    pub fn xtl_force_noiso(&mut self) -> XTL_FORCE_NOISO_W {
        XTL_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 27 - analog configure"]
    #[inline(always)]
    pub fn pll_force_noiso(&mut self) -> PLL_FORCE_NOISO_W {
        PLL_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 28 - analog configure"]
    #[inline(always)]
    pub fn analog_force_noiso(&mut self) -> ANALOG_FORCE_NOISO_W {
        ANALOG_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 29 - digital wrap force reset in deep sleep"]
    #[inline(always)]
    pub fn dg_wrap_force_rst(&mut self) -> DG_WRAP_FORCE_RST_W {
        DG_WRAP_FORCE_RST_W { w: self }
    }
    #[doc = "Bit 30 - digital core force no reset in deep sleep"]
    #[inline(always)]
    pub fn dg_wrap_force_norst(&mut self) -> DG_WRAP_FORCE_NORST_W {
        DG_WRAP_FORCE_NORST_W { w: self }
    }
    #[doc = "Bit 31 - SW system reset"]
    #[inline(always)]
    pub fn sw_sys_rst(&mut self) -> SW_SYS_RST_W {
        SW_SYS_RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [options0]
(index.html) module"]
pub struct OPTIONS0_SPEC;
impl crate::RegisterSpec for OPTIONS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [options0::R]
(R) reader structure"]
impl crate::Readable for OPTIONS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [options0::W]
(W) writer structure"]
impl crate::Writable for OPTIONS0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPTIONS0 to value 0x1c00_a000"]
impl crate::Resettable for OPTIONS0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1c00_a000
    }
}
