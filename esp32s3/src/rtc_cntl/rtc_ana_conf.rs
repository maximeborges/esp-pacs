#[doc = "Register `RTC_ANA_CONF` reader"]
pub struct R(crate::R<RTC_ANA_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_ANA_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_ANA_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_ANA_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_ANA_CONF` writer"]
pub struct W(crate::W<RTC_ANA_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_ANA_CONF_SPEC>;
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
impl From<crate::W<RTC_ANA_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_ANA_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_RESET_POR_FORCE_PD` reader - force down I2C_RESET_POR"]
pub struct I2C_RESET_POR_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl I2C_RESET_POR_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_RESET_POR_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_RESET_POR_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_RESET_POR_FORCE_PD` writer - force down I2C_RESET_POR"]
pub struct I2C_RESET_POR_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_RESET_POR_FORCE_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `I2C_RESET_POR_FORCE_PU` reader - force on I2C_RESET_POR"]
pub struct I2C_RESET_POR_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl I2C_RESET_POR_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_RESET_POR_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_RESET_POR_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_RESET_POR_FORCE_PU` writer - force on I2C_RESET_POR"]
pub struct I2C_RESET_POR_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_RESET_POR_FORCE_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `GLITCH_RST_EN` reader - enable clk glitch"]
pub struct GLITCH_RST_EN_R(crate::FieldReader<bool, bool>);
impl GLITCH_RST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GLITCH_RST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GLITCH_RST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GLITCH_RST_EN` writer - enable clk glitch"]
pub struct GLITCH_RST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GLITCH_RST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `SAR_I2C_PU` reader - PLLA force power up"]
pub struct SAR_I2C_PU_R(crate::FieldReader<bool, bool>);
impl SAR_I2C_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR_I2C_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_I2C_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR_I2C_PU` writer - PLLA force power up"]
pub struct SAR_I2C_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_I2C_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `ANALOG_TOP_ISO_SLEEP` reader - PLLA force power down"]
pub struct ANALOG_TOP_ISO_SLEEP_R(crate::FieldReader<bool, bool>);
impl ANALOG_TOP_ISO_SLEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ANALOG_TOP_ISO_SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANALOG_TOP_ISO_SLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANALOG_TOP_ISO_SLEEP` writer - PLLA force power down"]
pub struct ANALOG_TOP_ISO_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_TOP_ISO_SLEEP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `ANALOG_TOP_ISO_MONITOR` reader - PLLA force power up"]
pub struct ANALOG_TOP_ISO_MONITOR_R(crate::FieldReader<bool, bool>);
impl ANALOG_TOP_ISO_MONITOR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ANALOG_TOP_ISO_MONITOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANALOG_TOP_ISO_MONITOR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANALOG_TOP_ISO_MONITOR` writer - PLLA force power up"]
pub struct ANALOG_TOP_ISO_MONITOR_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_TOP_ISO_MONITOR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `BBPLL_CAL_SLP_START` reader - start BBPLL calibration during sleep"]
pub struct BBPLL_CAL_SLP_START_R(crate::FieldReader<bool, bool>);
impl BBPLL_CAL_SLP_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BBPLL_CAL_SLP_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BBPLL_CAL_SLP_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BBPLL_CAL_SLP_START` writer - start BBPLL calibration during sleep"]
pub struct BBPLL_CAL_SLP_START_W<'a> {
    w: &'a mut W,
}
impl<'a> BBPLL_CAL_SLP_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `PVTMON_PU` reader - 1: PVTMON power up, otherwise power down"]
pub struct PVTMON_PU_R(crate::FieldReader<bool, bool>);
impl PVTMON_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PVTMON_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVTMON_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PVTMON_PU` writer - 1: PVTMON power up, otherwise power down"]
pub struct PVTMON_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> PVTMON_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `TXRF_I2C_PU` reader - 1: TXRF_I2C power up, otherwise power down"]
pub struct TXRF_I2C_PU_R(crate::FieldReader<bool, bool>);
impl TXRF_I2C_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXRF_I2C_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRF_I2C_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRF_I2C_PU` writer - 1: TXRF_I2C power up, otherwise power down"]
pub struct TXRF_I2C_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRF_I2C_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `RFRX_PBUS_PU` reader - 1: RFRX_PBUS power up, otherwise power down"]
pub struct RFRX_PBUS_PU_R(crate::FieldReader<bool, bool>);
impl RFRX_PBUS_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFRX_PBUS_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFRX_PBUS_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFRX_PBUS_PU` writer - 1: RFRX_PBUS power up, otherwise power down"]
pub struct RFRX_PBUS_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RFRX_PBUS_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `CKGEN_I2C_PU` reader - 1: CKGEN_I2C power up, otherwise power down"]
pub struct CKGEN_I2C_PU_R(crate::FieldReader<bool, bool>);
impl CKGEN_I2C_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CKGEN_I2C_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKGEN_I2C_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKGEN_I2C_PU` writer - 1: CKGEN_I2C power up, otherwise power down"]
pub struct CKGEN_I2C_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> CKGEN_I2C_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `PLL_I2C_PU` reader - power on pll i2c"]
pub struct PLL_I2C_PU_R(crate::FieldReader<bool, bool>);
impl PLL_I2C_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLL_I2C_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_I2C_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_I2C_PU` writer - power on pll i2c"]
pub struct PLL_I2C_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_I2C_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 18 - force down I2C_RESET_POR"]
    #[inline(always)]
    pub fn i2c_reset_por_force_pd(&self) -> I2C_RESET_POR_FORCE_PD_R {
        I2C_RESET_POR_FORCE_PD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - force on I2C_RESET_POR"]
    #[inline(always)]
    pub fn i2c_reset_por_force_pu(&self) -> I2C_RESET_POR_FORCE_PU_R {
        I2C_RESET_POR_FORCE_PU_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - enable clk glitch"]
    #[inline(always)]
    pub fn glitch_rst_en(&self) -> GLITCH_RST_EN_R {
        GLITCH_RST_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 22 - PLLA force power up"]
    #[inline(always)]
    pub fn sar_i2c_pu(&self) -> SAR_I2C_PU_R {
        SAR_I2C_PU_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - PLLA force power down"]
    #[inline(always)]
    pub fn analog_top_iso_sleep(&self) -> ANALOG_TOP_ISO_SLEEP_R {
        ANALOG_TOP_ISO_SLEEP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - PLLA force power up"]
    #[inline(always)]
    pub fn analog_top_iso_monitor(&self) -> ANALOG_TOP_ISO_MONITOR_R {
        ANALOG_TOP_ISO_MONITOR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - start BBPLL calibration during sleep"]
    #[inline(always)]
    pub fn bbpll_cal_slp_start(&self) -> BBPLL_CAL_SLP_START_R {
        BBPLL_CAL_SLP_START_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 1: PVTMON power up, otherwise power down"]
    #[inline(always)]
    pub fn pvtmon_pu(&self) -> PVTMON_PU_R {
        PVTMON_PU_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 1: TXRF_I2C power up, otherwise power down"]
    #[inline(always)]
    pub fn txrf_i2c_pu(&self) -> TXRF_I2C_PU_R {
        TXRF_I2C_PU_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 1: RFRX_PBUS power up, otherwise power down"]
    #[inline(always)]
    pub fn rfrx_pbus_pu(&self) -> RFRX_PBUS_PU_R {
        RFRX_PBUS_PU_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 1: CKGEN_I2C power up, otherwise power down"]
    #[inline(always)]
    pub fn ckgen_i2c_pu(&self) -> CKGEN_I2C_PU_R {
        CKGEN_I2C_PU_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - power on pll i2c"]
    #[inline(always)]
    pub fn pll_i2c_pu(&self) -> PLL_I2C_PU_R {
        PLL_I2C_PU_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - force down I2C_RESET_POR"]
    #[inline(always)]
    pub fn i2c_reset_por_force_pd(&mut self) -> I2C_RESET_POR_FORCE_PD_W {
        I2C_RESET_POR_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 19 - force on I2C_RESET_POR"]
    #[inline(always)]
    pub fn i2c_reset_por_force_pu(&mut self) -> I2C_RESET_POR_FORCE_PU_W {
        I2C_RESET_POR_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 20 - enable clk glitch"]
    #[inline(always)]
    pub fn glitch_rst_en(&mut self) -> GLITCH_RST_EN_W {
        GLITCH_RST_EN_W { w: self }
    }
    #[doc = "Bit 22 - PLLA force power up"]
    #[inline(always)]
    pub fn sar_i2c_pu(&mut self) -> SAR_I2C_PU_W {
        SAR_I2C_PU_W { w: self }
    }
    #[doc = "Bit 23 - PLLA force power down"]
    #[inline(always)]
    pub fn analog_top_iso_sleep(&mut self) -> ANALOG_TOP_ISO_SLEEP_W {
        ANALOG_TOP_ISO_SLEEP_W { w: self }
    }
    #[doc = "Bit 24 - PLLA force power up"]
    #[inline(always)]
    pub fn analog_top_iso_monitor(&mut self) -> ANALOG_TOP_ISO_MONITOR_W {
        ANALOG_TOP_ISO_MONITOR_W { w: self }
    }
    #[doc = "Bit 25 - start BBPLL calibration during sleep"]
    #[inline(always)]
    pub fn bbpll_cal_slp_start(&mut self) -> BBPLL_CAL_SLP_START_W {
        BBPLL_CAL_SLP_START_W { w: self }
    }
    #[doc = "Bit 26 - 1: PVTMON power up, otherwise power down"]
    #[inline(always)]
    pub fn pvtmon_pu(&mut self) -> PVTMON_PU_W {
        PVTMON_PU_W { w: self }
    }
    #[doc = "Bit 27 - 1: TXRF_I2C power up, otherwise power down"]
    #[inline(always)]
    pub fn txrf_i2c_pu(&mut self) -> TXRF_I2C_PU_W {
        TXRF_I2C_PU_W { w: self }
    }
    #[doc = "Bit 28 - 1: RFRX_PBUS power up, otherwise power down"]
    #[inline(always)]
    pub fn rfrx_pbus_pu(&mut self) -> RFRX_PBUS_PU_W {
        RFRX_PBUS_PU_W { w: self }
    }
    #[doc = "Bit 30 - 1: CKGEN_I2C power up, otherwise power down"]
    #[inline(always)]
    pub fn ckgen_i2c_pu(&mut self) -> CKGEN_I2C_PU_W {
        CKGEN_I2C_PU_W { w: self }
    }
    #[doc = "Bit 31 - power on pll i2c"]
    #[inline(always)]
    pub fn pll_i2c_pu(&mut self) -> PLL_I2C_PU_W {
        PLL_I2C_PU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "analog configure register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_ana_conf]
(index.html) module"]
pub struct RTC_ANA_CONF_SPEC;
impl crate::RegisterSpec for RTC_ANA_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_ana_conf::R]
(R) reader structure"]
impl crate::Readable for RTC_ANA_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_ana_conf::W]
(W) writer structure"]
impl crate::Writable for RTC_ANA_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_ANA_CONF to value 0x0044_0000"]
impl crate::Resettable for RTC_ANA_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0044_0000
    }
}
