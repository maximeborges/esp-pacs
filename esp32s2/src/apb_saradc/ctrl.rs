#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START_FORCE` reader - 0: select FSM to start SAR ADC. 1: select software to start SAR ADC."]
pub struct START_FORCE_R(crate::FieldReader<bool, bool>);
impl START_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        START_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START_FORCE` writer - 0: select FSM to start SAR ADC. 1: select software to start SAR ADC."]
pub struct START_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> START_FORCE_W<'a> {
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
#[doc = "Field `START` reader - Start SAR ADC by software."]
pub struct START_R(crate::FieldReader<bool, bool>);
impl START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START` writer - Start SAR ADC by software."]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
#[doc = "Field `WORK_MODE` reader - 0: single-channel scan mode. 1: double-channel scan mode. 2: alternate-channel scan mode."]
pub struct WORK_MODE_R(crate::FieldReader<u8, u8>);
impl WORK_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WORK_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WORK_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WORK_MODE` writer - 0: single-channel scan mode. 1: double-channel scan mode. 2: alternate-channel scan mode."]
pub struct WORK_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WORK_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `SAR_SEL` reader - 0: select SAR ADC1. 1: select SAR ADC2, only work for single-channel scan mode."]
pub struct SAR_SEL_R(crate::FieldReader<bool, bool>);
impl SAR_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR_SEL` writer - 0: select SAR ADC1. 1: select SAR ADC2, only work for single-channel scan mode."]
pub struct SAR_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_SEL_W<'a> {
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
#[doc = "Field `SAR_CLK_GATED` reader - SAR clock gate enable bit."]
pub struct SAR_CLK_GATED_R(crate::FieldReader<bool, bool>);
impl SAR_CLK_GATED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR_CLK_GATED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_CLK_GATED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR_CLK_GATED` writer - SAR clock gate enable bit."]
pub struct SAR_CLK_GATED_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_CLK_GATED_W<'a> {
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
#[doc = "Field `SAR_CLK_DIV` reader - SAR clock divider"]
pub struct SAR_CLK_DIV_R(crate::FieldReader<u8, u8>);
impl SAR_CLK_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAR_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_CLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR_CLK_DIV` writer - SAR clock divider"]
pub struct SAR_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 7)) | ((value as u32 & 0xff) << 7);
        self.w
    }
}
#[doc = "Field `SAR1_PATT_LEN` reader - 0 ~ 15 means length 1 ~ 16"]
pub struct SAR1_PATT_LEN_R(crate::FieldReader<u8, u8>);
impl SAR1_PATT_LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAR1_PATT_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR1_PATT_LEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR1_PATT_LEN` writer - 0 ~ 15 means length 1 ~ 16"]
pub struct SAR1_PATT_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR1_PATT_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 15)) | ((value as u32 & 0x0f) << 15);
        self.w
    }
}
#[doc = "Field `SAR2_PATT_LEN` reader - 0 ~ 15 means length 1 ~ 16"]
pub struct SAR2_PATT_LEN_R(crate::FieldReader<u8, u8>);
impl SAR2_PATT_LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAR2_PATT_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_PATT_LEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_PATT_LEN` writer - 0 ~ 15 means length 1 ~ 16"]
pub struct SAR2_PATT_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_PATT_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 19)) | ((value as u32 & 0x0f) << 19);
        self.w
    }
}
#[doc = "Field `SAR1_PATT_P_CLEAR` reader - Clear the pointer of pattern table for DIG ADC1 CTRL."]
pub struct SAR1_PATT_P_CLEAR_R(crate::FieldReader<bool, bool>);
impl SAR1_PATT_P_CLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR1_PATT_P_CLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR1_PATT_P_CLEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR1_PATT_P_CLEAR` writer - Clear the pointer of pattern table for DIG ADC1 CTRL."]
pub struct SAR1_PATT_P_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR1_PATT_P_CLEAR_W<'a> {
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
#[doc = "Field `SAR2_PATT_P_CLEAR` reader - Clear the pointer of pattern table for DIG ADC2 CTRL."]
pub struct SAR2_PATT_P_CLEAR_R(crate::FieldReader<bool, bool>);
impl SAR2_PATT_P_CLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR2_PATT_P_CLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_PATT_P_CLEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_PATT_P_CLEAR` writer - Clear the pointer of pattern table for DIG ADC2 CTRL."]
pub struct SAR2_PATT_P_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_PATT_P_CLEAR_W<'a> {
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
#[doc = "Field `DATA_SAR_SEL` reader - 1: sar_sel will be coded to the MSB of the 16-bit output data, in this case the resolution should not be larger than 11 bits."]
pub struct DATA_SAR_SEL_R(crate::FieldReader<bool, bool>);
impl DATA_SAR_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATA_SAR_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_SAR_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_SAR_SEL` writer - 1: sar_sel will be coded to the MSB of the 16-bit output data, in this case the resolution should not be larger than 11 bits."]
pub struct DATA_SAR_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_SAR_SEL_W<'a> {
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
#[doc = "Field `DATA_TO_I2S` reader - 1: I2S input data is from SAR ADC (for DMA), 0: I2S input data is from GPIO matrix"]
pub struct DATA_TO_I2S_R(crate::FieldReader<bool, bool>);
impl DATA_TO_I2S_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATA_TO_I2S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_TO_I2S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_TO_I2S` writer - 1: I2S input data is from SAR ADC (for DMA), 0: I2S input data is from GPIO matrix"]
pub struct DATA_TO_I2S_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_TO_I2S_W<'a> {
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
#[doc = "Field `XPD_SAR_FORCE` reader - Force option to xpd sar blocks."]
pub struct XPD_SAR_FORCE_R(crate::FieldReader<u8, u8>);
impl XPD_SAR_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XPD_SAR_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XPD_SAR_FORCE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XPD_SAR_FORCE` writer - Force option to xpd sar blocks."]
pub struct XPD_SAR_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> XPD_SAR_FORCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | ((value as u32 & 0x03) << 27);
        self.w
    }
}
#[doc = "Field `WAIT_ARB_CYCLE` reader - Wait arbit signal stable after sar_done."]
pub struct WAIT_ARB_CYCLE_R(crate::FieldReader<u8, u8>);
impl WAIT_ARB_CYCLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WAIT_ARB_CYCLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAIT_ARB_CYCLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAIT_ARB_CYCLE` writer - Wait arbit signal stable after sar_done."]
pub struct WAIT_ARB_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_ARB_CYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0: select FSM to start SAR ADC. 1: select software to start SAR ADC."]
    #[inline(always)]
    pub fn start_force(&self) -> START_FORCE_R {
        START_FORCE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start SAR ADC by software."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - 0: single-channel scan mode. 1: double-channel scan mode. 2: alternate-channel scan mode."]
    #[inline(always)]
    pub fn work_mode(&self) -> WORK_MODE_R {
        WORK_MODE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - 0: select SAR ADC1. 1: select SAR ADC2, only work for single-channel scan mode."]
    #[inline(always)]
    pub fn sar_sel(&self) -> SAR_SEL_R {
        SAR_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SAR clock gate enable bit."]
    #[inline(always)]
    pub fn sar_clk_gated(&self) -> SAR_CLK_GATED_R {
        SAR_CLK_GATED_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 7:14 - SAR clock divider"]
    #[inline(always)]
    pub fn sar_clk_div(&self) -> SAR_CLK_DIV_R {
        SAR_CLK_DIV_R::new(((self.bits >> 7) & 0xff) as u8)
    }
    #[doc = "Bits 15:18 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn sar1_patt_len(&self) -> SAR1_PATT_LEN_R {
        SAR1_PATT_LEN_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:22 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn sar2_patt_len(&self) -> SAR2_PATT_LEN_R {
        SAR2_PATT_LEN_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - Clear the pointer of pattern table for DIG ADC1 CTRL."]
    #[inline(always)]
    pub fn sar1_patt_p_clear(&self) -> SAR1_PATT_P_CLEAR_R {
        SAR1_PATT_P_CLEAR_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Clear the pointer of pattern table for DIG ADC2 CTRL."]
    #[inline(always)]
    pub fn sar2_patt_p_clear(&self) -> SAR2_PATT_P_CLEAR_R {
        SAR2_PATT_P_CLEAR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 1: sar_sel will be coded to the MSB of the 16-bit output data, in this case the resolution should not be larger than 11 bits."]
    #[inline(always)]
    pub fn data_sar_sel(&self) -> DATA_SAR_SEL_R {
        DATA_SAR_SEL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 1: I2S input data is from SAR ADC (for DMA), 0: I2S input data is from GPIO matrix"]
    #[inline(always)]
    pub fn data_to_i2s(&self) -> DATA_TO_I2S_R {
        DATA_TO_I2S_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 27:28 - Force option to xpd sar blocks."]
    #[inline(always)]
    pub fn xpd_sar_force(&self) -> XPD_SAR_FORCE_R {
        XPD_SAR_FORCE_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Wait arbit signal stable after sar_done."]
    #[inline(always)]
    pub fn wait_arb_cycle(&self) -> WAIT_ARB_CYCLE_R {
        WAIT_ARB_CYCLE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0: select FSM to start SAR ADC. 1: select software to start SAR ADC."]
    #[inline(always)]
    pub fn start_force(&mut self) -> START_FORCE_W {
        START_FORCE_W { w: self }
    }
    #[doc = "Bit 1 - Start SAR ADC by software."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bits 3:4 - 0: single-channel scan mode. 1: double-channel scan mode. 2: alternate-channel scan mode."]
    #[inline(always)]
    pub fn work_mode(&mut self) -> WORK_MODE_W {
        WORK_MODE_W { w: self }
    }
    #[doc = "Bit 5 - 0: select SAR ADC1. 1: select SAR ADC2, only work for single-channel scan mode."]
    #[inline(always)]
    pub fn sar_sel(&mut self) -> SAR_SEL_W {
        SAR_SEL_W { w: self }
    }
    #[doc = "Bit 6 - SAR clock gate enable bit."]
    #[inline(always)]
    pub fn sar_clk_gated(&mut self) -> SAR_CLK_GATED_W {
        SAR_CLK_GATED_W { w: self }
    }
    #[doc = "Bits 7:14 - SAR clock divider"]
    #[inline(always)]
    pub fn sar_clk_div(&mut self) -> SAR_CLK_DIV_W {
        SAR_CLK_DIV_W { w: self }
    }
    #[doc = "Bits 15:18 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn sar1_patt_len(&mut self) -> SAR1_PATT_LEN_W {
        SAR1_PATT_LEN_W { w: self }
    }
    #[doc = "Bits 19:22 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn sar2_patt_len(&mut self) -> SAR2_PATT_LEN_W {
        SAR2_PATT_LEN_W { w: self }
    }
    #[doc = "Bit 23 - Clear the pointer of pattern table for DIG ADC1 CTRL."]
    #[inline(always)]
    pub fn sar1_patt_p_clear(&mut self) -> SAR1_PATT_P_CLEAR_W {
        SAR1_PATT_P_CLEAR_W { w: self }
    }
    #[doc = "Bit 24 - Clear the pointer of pattern table for DIG ADC2 CTRL."]
    #[inline(always)]
    pub fn sar2_patt_p_clear(&mut self) -> SAR2_PATT_P_CLEAR_W {
        SAR2_PATT_P_CLEAR_W { w: self }
    }
    #[doc = "Bit 25 - 1: sar_sel will be coded to the MSB of the 16-bit output data, in this case the resolution should not be larger than 11 bits."]
    #[inline(always)]
    pub fn data_sar_sel(&mut self) -> DATA_SAR_SEL_W {
        DATA_SAR_SEL_W { w: self }
    }
    #[doc = "Bit 26 - 1: I2S input data is from SAR ADC (for DMA), 0: I2S input data is from GPIO matrix"]
    #[inline(always)]
    pub fn data_to_i2s(&mut self) -> DATA_TO_I2S_W {
        DATA_TO_I2S_W { w: self }
    }
    #[doc = "Bits 27:28 - Force option to xpd sar blocks."]
    #[inline(always)]
    pub fn xpd_sar_force(&mut self) -> XPD_SAR_FORCE_W {
        XPD_SAR_FORCE_W { w: self }
    }
    #[doc = "Bits 30:31 - Wait arbit signal stable after sar_done."]
    #[inline(always)]
    pub fn wait_arb_cycle(&mut self) -> WAIT_ARB_CYCLE_W {
        WAIT_ARB_CYCLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DIG ADC common configuration\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl]
(index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R]
(R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W]
(W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0x407f_8240"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x407f_8240
    }
}
