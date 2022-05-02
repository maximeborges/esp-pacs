#[doc = "Register `ANA_CONF` reader"]
pub struct R(crate::R<ANA_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANA_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANA_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANA_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANA_CONF` writer"]
pub struct W(crate::W<ANA_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANA_CONF_SPEC>;
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
impl From<crate::W<ANA_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANA_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_RESET_POR_FORCE_PD` reader - SLEEP_I2CPOR force pd"]
pub struct I2C_RESET_POR_FORCE_PD_R(crate::FieldReader<bool>);
impl I2C_RESET_POR_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_RESET_POR_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_RESET_POR_FORCE_PD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_RESET_POR_FORCE_PD` writer - SLEEP_I2CPOR force pd"]
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
        self.w.bits = (self.w.bits & !(1 << 18)) | ((value as u32 & 1) << 18);
        self.w
    }
}
#[doc = "Field `I2C_RESET_POR_FORCE_PU` reader - SLEEP_I2CPOR force pu"]
pub struct I2C_RESET_POR_FORCE_PU_R(crate::FieldReader<bool>);
impl I2C_RESET_POR_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_RESET_POR_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_RESET_POR_FORCE_PU_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_RESET_POR_FORCE_PU` writer - SLEEP_I2CPOR force pu"]
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
        self.w.bits = (self.w.bits & !(1 << 19)) | ((value as u32 & 1) << 19);
        self.w
    }
}
#[doc = "Field `GLITCH_RST_EN` reader - Set this bit to enable a reset when the system detects a glitch."]
pub struct GLITCH_RST_EN_R(crate::FieldReader<bool>);
impl GLITCH_RST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GLITCH_RST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GLITCH_RST_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GLITCH_RST_EN` writer - Set this bit to enable a reset when the system detects a glitch."]
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
        self.w.bits = (self.w.bits & !(1 << 20)) | ((value as u32 & 1) << 20);
        self.w
    }
}
#[doc = "Field `SAR_I2C_FORCE_PD` reader - Sets this bit to FPD the SAR_I2C."]
pub struct SAR_I2C_FORCE_PD_R(crate::FieldReader<bool>);
impl SAR_I2C_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR_I2C_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_I2C_FORCE_PD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR_I2C_FORCE_PD` writer - Sets this bit to FPD the SAR_I2C."]
pub struct SAR_I2C_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_I2C_FORCE_PD_W<'a> {
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
#[doc = "Field `SAR_I2C_FORCE_PU` reader - Sets this bit to FPU the SAR_I2C."]
pub struct SAR_I2C_FORCE_PU_R(crate::FieldReader<bool>);
impl SAR_I2C_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR_I2C_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_I2C_FORCE_PU_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR_I2C_FORCE_PU` writer - Sets this bit to FPU the SAR_I2C."]
pub struct SAR_I2C_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_I2C_FORCE_PU_W<'a> {
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
#[doc = "Field `PLLA_FORCE_PD` reader - Sets this bit to FPD the PLLA."]
pub struct PLLA_FORCE_PD_R(crate::FieldReader<bool>);
impl PLLA_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLLA_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLA_FORCE_PD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLA_FORCE_PD` writer - Sets this bit to FPD the PLLA."]
pub struct PLLA_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLA_FORCE_PD_W<'a> {
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
#[doc = "Field `PLLA_FORCE_PU` reader - Sets this bit to FPU the PLLA."]
pub struct PLLA_FORCE_PU_R(crate::FieldReader<bool>);
impl PLLA_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLLA_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLA_FORCE_PU_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLA_FORCE_PU` writer - Sets this bit to FPU the PLLA."]
pub struct PLLA_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLA_FORCE_PU_W<'a> {
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
#[doc = "Field `BBPLL_CAL_SLP_START` reader - start BBPLL calibration during sleep"]
pub struct BBPLL_CAL_SLP_START_R(crate::FieldReader<bool>);
impl BBPLL_CAL_SLP_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BBPLL_CAL_SLP_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BBPLL_CAL_SLP_START_R {
    type Target = crate::FieldReader<bool>;
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
        self.w.bits = (self.w.bits & !(1 << 25)) | ((value as u32 & 1) << 25);
        self.w
    }
}
#[doc = "Field `PVTMON_PU` reader - 1: PVTMON power up , otherwise power down"]
pub struct PVTMON_PU_R(crate::FieldReader<bool>);
impl PVTMON_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PVTMON_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVTMON_PU_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PVTMON_PU` writer - 1: PVTMON power up , otherwise power down"]
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
        self.w.bits = (self.w.bits & !(1 << 26)) | ((value as u32 & 1) << 26);
        self.w
    }
}
#[doc = "Field `TXRF_I2C_PU` reader - 1: TXRF_I2C power up , otherwise power down"]
pub struct TXRF_I2C_PU_R(crate::FieldReader<bool>);
impl TXRF_I2C_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXRF_I2C_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRF_I2C_PU_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRF_I2C_PU` writer - 1: TXRF_I2C power up , otherwise power down"]
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
        self.w.bits = (self.w.bits & !(1 << 27)) | ((value as u32 & 1) << 27);
        self.w
    }
}
#[doc = "Field `RFRX_PBUS_PU` reader - 1: RFRX_PBUS power up , otherwise power down"]
pub struct RFRX_PBUS_PU_R(crate::FieldReader<bool>);
impl RFRX_PBUS_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFRX_PBUS_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFRX_PBUS_PU_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFRX_PBUS_PU` writer - 1: RFRX_PBUS power up , otherwise power down"]
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
        self.w.bits = (self.w.bits & !(1 << 28)) | ((value as u32 & 1) << 28);
        self.w
    }
}
#[doc = "Field `CKGEN_I2C_PU` reader - 1: CKGEN_I2C power up , otherwise power down"]
pub struct CKGEN_I2C_PU_R(crate::FieldReader<bool>);
impl CKGEN_I2C_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CKGEN_I2C_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKGEN_I2C_PU_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKGEN_I2C_PU` writer - 1: CKGEN_I2C power up , otherwise power down"]
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "Field `PLL_I2C_PU` reader - 1. PLL_I2C power up ,otherwise power down"]
pub struct PLL_I2C_PU_R(crate::FieldReader<bool>);
impl PLL_I2C_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLL_I2C_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_I2C_PU_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_I2C_PU` writer - 1. PLL_I2C power up ,otherwise power down"]
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 18 - SLEEP_I2CPOR force pd"]
    #[inline(always)]
    pub fn i2c_reset_por_force_pd(&self) -> I2C_RESET_POR_FORCE_PD_R {
        I2C_RESET_POR_FORCE_PD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SLEEP_I2CPOR force pu"]
    #[inline(always)]
    pub fn i2c_reset_por_force_pu(&self) -> I2C_RESET_POR_FORCE_PU_R {
        I2C_RESET_POR_FORCE_PU_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Set this bit to enable a reset when the system detects a glitch."]
    #[inline(always)]
    pub fn glitch_rst_en(&self) -> GLITCH_RST_EN_R {
        GLITCH_RST_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Sets this bit to FPD the SAR_I2C."]
    #[inline(always)]
    pub fn sar_i2c_force_pd(&self) -> SAR_I2C_FORCE_PD_R {
        SAR_I2C_FORCE_PD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Sets this bit to FPU the SAR_I2C."]
    #[inline(always)]
    pub fn sar_i2c_force_pu(&self) -> SAR_I2C_FORCE_PU_R {
        SAR_I2C_FORCE_PU_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Sets this bit to FPD the PLLA."]
    #[inline(always)]
    pub fn plla_force_pd(&self) -> PLLA_FORCE_PD_R {
        PLLA_FORCE_PD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Sets this bit to FPU the PLLA."]
    #[inline(always)]
    pub fn plla_force_pu(&self) -> PLLA_FORCE_PU_R {
        PLLA_FORCE_PU_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - start BBPLL calibration during sleep"]
    #[inline(always)]
    pub fn bbpll_cal_slp_start(&self) -> BBPLL_CAL_SLP_START_R {
        BBPLL_CAL_SLP_START_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: PVTMON power up , otherwise power down"]
    #[inline(always)]
    pub fn pvtmon_pu(&self) -> PVTMON_PU_R {
        PVTMON_PU_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 1: TXRF_I2C power up , otherwise power down"]
    #[inline(always)]
    pub fn txrf_i2c_pu(&self) -> TXRF_I2C_PU_R {
        TXRF_I2C_PU_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: RFRX_PBUS power up , otherwise power down"]
    #[inline(always)]
    pub fn rfrx_pbus_pu(&self) -> RFRX_PBUS_PU_R {
        RFRX_PBUS_PU_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: CKGEN_I2C power up , otherwise power down"]
    #[inline(always)]
    pub fn ckgen_i2c_pu(&self) -> CKGEN_I2C_PU_R {
        CKGEN_I2C_PU_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1. PLL_I2C power up ,otherwise power down"]
    #[inline(always)]
    pub fn pll_i2c_pu(&self) -> PLL_I2C_PU_R {
        PLL_I2C_PU_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - SLEEP_I2CPOR force pd"]
    #[inline(always)]
    pub fn i2c_reset_por_force_pd(&mut self) -> I2C_RESET_POR_FORCE_PD_W {
        I2C_RESET_POR_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 19 - SLEEP_I2CPOR force pu"]
    #[inline(always)]
    pub fn i2c_reset_por_force_pu(&mut self) -> I2C_RESET_POR_FORCE_PU_W {
        I2C_RESET_POR_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 20 - Set this bit to enable a reset when the system detects a glitch."]
    #[inline(always)]
    pub fn glitch_rst_en(&mut self) -> GLITCH_RST_EN_W {
        GLITCH_RST_EN_W { w: self }
    }
    #[doc = "Bit 21 - Sets this bit to FPD the SAR_I2C."]
    #[inline(always)]
    pub fn sar_i2c_force_pd(&mut self) -> SAR_I2C_FORCE_PD_W {
        SAR_I2C_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 22 - Sets this bit to FPU the SAR_I2C."]
    #[inline(always)]
    pub fn sar_i2c_force_pu(&mut self) -> SAR_I2C_FORCE_PU_W {
        SAR_I2C_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 23 - Sets this bit to FPD the PLLA."]
    #[inline(always)]
    pub fn plla_force_pd(&mut self) -> PLLA_FORCE_PD_W {
        PLLA_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 24 - Sets this bit to FPU the PLLA."]
    #[inline(always)]
    pub fn plla_force_pu(&mut self) -> PLLA_FORCE_PU_W {
        PLLA_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 25 - start BBPLL calibration during sleep"]
    #[inline(always)]
    pub fn bbpll_cal_slp_start(&mut self) -> BBPLL_CAL_SLP_START_W {
        BBPLL_CAL_SLP_START_W { w: self }
    }
    #[doc = "Bit 26 - 1: PVTMON power up , otherwise power down"]
    #[inline(always)]
    pub fn pvtmon_pu(&mut self) -> PVTMON_PU_W {
        PVTMON_PU_W { w: self }
    }
    #[doc = "Bit 27 - 1: TXRF_I2C power up , otherwise power down"]
    #[inline(always)]
    pub fn txrf_i2c_pu(&mut self) -> TXRF_I2C_PU_W {
        TXRF_I2C_PU_W { w: self }
    }
    #[doc = "Bit 28 - 1: RFRX_PBUS power up , otherwise power down"]
    #[inline(always)]
    pub fn rfrx_pbus_pu(&mut self) -> RFRX_PBUS_PU_W {
        RFRX_PBUS_PU_W { w: self }
    }
    #[doc = "Bit 30 - 1: CKGEN_I2C power up , otherwise power down"]
    #[inline(always)]
    pub fn ckgen_i2c_pu(&mut self) -> CKGEN_I2C_PU_W {
        CKGEN_I2C_PU_W { w: self }
    }
    #[doc = "Bit 31 - 1. PLL_I2C power up ,otherwise power down"]
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
#[doc = "Configures the power options for I2C and PLLA\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_conf](index.html) module"]
pub struct ANA_CONF_SPEC;
impl crate::RegisterSpec for ANA_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ana_conf::R](R) reader structure"]
impl crate::Readable for ANA_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ana_conf::W](W) writer structure"]
impl crate::Writable for ANA_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ANA_CONF to value 0x00a4_0000"]
impl crate::Resettable for ANA_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00a4_0000
    }
}
