#[doc = "Register `PAD_DAC2` reader"]
pub struct R(crate::R<PAD_DAC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_DAC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_DAC2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_DAC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_DAC2` writer"]
pub struct W(crate::W<PAD_DAC2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_DAC2_SPEC>;
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
impl From<crate::W<PAD_DAC2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_DAC2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDAC2_DAC` reader - PDAC2_DAC"]
pub struct PDAC2_DAC_R(crate::FieldReader<u8>);
impl PDAC2_DAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PDAC2_DAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC2_DAC_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC2_DAC` writer - PDAC2_DAC"]
pub struct PDAC2_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC2_DAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 3)) | ((value as u32 & 0xff) << 3);
        self.w
    }
}
#[doc = "Field `PDAC2_XPD_DAC` reader - PDAC2_XPD_DAC"]
pub struct PDAC2_XPD_DAC_R(crate::FieldReader<bool>);
impl PDAC2_XPD_DAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDAC2_XPD_DAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC2_XPD_DAC_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC2_XPD_DAC` writer - PDAC2_XPD_DAC"]
pub struct PDAC2_XPD_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC2_XPD_DAC_W<'a> {
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
#[doc = "Field `PDAC2_DAC_XPD_FORCE` reader - 1: use reg_pdac2_xpd_dac to control PDAC2_XPD_DAC,0: use SAR ADC FSM to control PDAC2_XPD_DAC"]
pub struct PDAC2_DAC_XPD_FORCE_R(crate::FieldReader<bool>);
impl PDAC2_DAC_XPD_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDAC2_DAC_XPD_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC2_DAC_XPD_FORCE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC2_DAC_XPD_FORCE` writer - 1: use reg_pdac2_xpd_dac to control PDAC2_XPD_DAC,0: use SAR ADC FSM to control PDAC2_XPD_DAC"]
pub struct PDAC2_DAC_XPD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC2_DAC_XPD_FORCE_W<'a> {
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
#[doc = "Field `PDAC2_FUN_IE` reader - input enable in work mode"]
pub struct PDAC2_FUN_IE_R(crate::FieldReader<bool>);
impl PDAC2_FUN_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDAC2_FUN_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC2_FUN_IE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC2_FUN_IE` writer - input enable in work mode"]
pub struct PDAC2_FUN_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC2_FUN_IE_W<'a> {
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
#[doc = "Field `PDAC2_SLP_OE` reader - output enable in sleep mode"]
pub struct PDAC2_SLP_OE_R(crate::FieldReader<bool>);
impl PDAC2_SLP_OE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDAC2_SLP_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC2_SLP_OE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC2_SLP_OE` writer - output enable in sleep mode"]
pub struct PDAC2_SLP_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC2_SLP_OE_W<'a> {
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
#[doc = "Field `PDAC2_SLP_IE` reader - input enable in sleep mode"]
pub struct PDAC2_SLP_IE_R(crate::FieldReader<bool>);
impl PDAC2_SLP_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDAC2_SLP_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC2_SLP_IE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC2_SLP_IE` writer - input enable in sleep mode"]
pub struct PDAC2_SLP_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC2_SLP_IE_W<'a> {
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
#[doc = "Field `PDAC2_SLP_SEL` reader - 1: enable sleep mode during sleep,0: no sleep mode"]
pub struct PDAC2_SLP_SEL_R(crate::FieldReader<bool>);
impl PDAC2_SLP_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDAC2_SLP_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC2_SLP_SEL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC2_SLP_SEL` writer - 1: enable sleep mode during sleep,0: no sleep mode"]
pub struct PDAC2_SLP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC2_SLP_SEL_W<'a> {
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
#[doc = "Field `PDAC2_FUN_SEL` reader - PDAC1 function sel"]
pub struct PDAC2_FUN_SEL_R(crate::FieldReader<u8>);
impl PDAC2_FUN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PDAC2_FUN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC2_FUN_SEL_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC2_FUN_SEL` writer - PDAC1 function sel"]
pub struct PDAC2_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC2_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 17)) | ((value as u32 & 3) << 17);
        self.w
    }
}
#[doc = "Field `PDAC2_MUX_SEL` reader - 1: use RTC GPIO,0: use digital GPIO"]
pub struct PDAC2_MUX_SEL_R(crate::FieldReader<bool>);
impl PDAC2_MUX_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDAC2_MUX_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC2_MUX_SEL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC2_MUX_SEL` writer - 1: use RTC GPIO,0: use digital GPIO"]
pub struct PDAC2_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC2_MUX_SEL_W<'a> {
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
#[doc = "Field `PDAC2_RUE` reader - PDAC2_RUE"]
pub struct PDAC2_RUE_R(crate::FieldReader<bool>);
impl PDAC2_RUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDAC2_RUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC2_RUE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC2_RUE` writer - PDAC2_RUE"]
pub struct PDAC2_RUE_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC2_RUE_W<'a> {
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
#[doc = "Field `PDAC2_RDE` reader - PDAC2_RDE"]
pub struct PDAC2_RDE_R(crate::FieldReader<bool>);
impl PDAC2_RDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDAC2_RDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC2_RDE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC2_RDE` writer - PDAC2_RDE"]
pub struct PDAC2_RDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC2_RDE_W<'a> {
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
#[doc = "Field `PDAC2_DRV` reader - PDAC2_DRV"]
pub struct PDAC2_DRV_R(crate::FieldReader<u8>);
impl PDAC2_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PDAC2_DRV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC2_DRV_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC2_DRV` writer - PDAC2_DRV"]
pub struct PDAC2_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC2_DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 29)) | ((value as u32 & 3) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:10 - PDAC2_DAC"]
    #[inline(always)]
    pub fn pdac2_dac(&self) -> PDAC2_DAC_R {
        PDAC2_DAC_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 11 - PDAC2_XPD_DAC"]
    #[inline(always)]
    pub fn pdac2_xpd_dac(&self) -> PDAC2_XPD_DAC_R {
        PDAC2_XPD_DAC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: use reg_pdac2_xpd_dac to control PDAC2_XPD_DAC,0: use SAR ADC FSM to control PDAC2_XPD_DAC"]
    #[inline(always)]
    pub fn pdac2_dac_xpd_force(&self) -> PDAC2_DAC_XPD_FORCE_R {
        PDAC2_DAC_XPD_FORCE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - input enable in work mode"]
    #[inline(always)]
    pub fn pdac2_fun_ie(&self) -> PDAC2_FUN_IE_R {
        PDAC2_FUN_IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - output enable in sleep mode"]
    #[inline(always)]
    pub fn pdac2_slp_oe(&self) -> PDAC2_SLP_OE_R {
        PDAC2_SLP_OE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - input enable in sleep mode"]
    #[inline(always)]
    pub fn pdac2_slp_ie(&self) -> PDAC2_SLP_IE_R {
        PDAC2_SLP_IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn pdac2_slp_sel(&self) -> PDAC2_SLP_SEL_R {
        PDAC2_SLP_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - PDAC1 function sel"]
    #[inline(always)]
    pub fn pdac2_fun_sel(&self) -> PDAC2_FUN_SEL_R {
        PDAC2_FUN_SEL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - 1: use RTC GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn pdac2_mux_sel(&self) -> PDAC2_MUX_SEL_R {
        PDAC2_MUX_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 27 - PDAC2_RUE"]
    #[inline(always)]
    pub fn pdac2_rue(&self) -> PDAC2_RUE_R {
        PDAC2_RUE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PDAC2_RDE"]
    #[inline(always)]
    pub fn pdac2_rde(&self) -> PDAC2_RDE_R {
        PDAC2_RDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - PDAC2_DRV"]
    #[inline(always)]
    pub fn pdac2_drv(&self) -> PDAC2_DRV_R {
        PDAC2_DRV_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 3:10 - PDAC2_DAC"]
    #[inline(always)]
    pub fn pdac2_dac(&mut self) -> PDAC2_DAC_W {
        PDAC2_DAC_W { w: self }
    }
    #[doc = "Bit 11 - PDAC2_XPD_DAC"]
    #[inline(always)]
    pub fn pdac2_xpd_dac(&mut self) -> PDAC2_XPD_DAC_W {
        PDAC2_XPD_DAC_W { w: self }
    }
    #[doc = "Bit 12 - 1: use reg_pdac2_xpd_dac to control PDAC2_XPD_DAC,0: use SAR ADC FSM to control PDAC2_XPD_DAC"]
    #[inline(always)]
    pub fn pdac2_dac_xpd_force(&mut self) -> PDAC2_DAC_XPD_FORCE_W {
        PDAC2_DAC_XPD_FORCE_W { w: self }
    }
    #[doc = "Bit 13 - input enable in work mode"]
    #[inline(always)]
    pub fn pdac2_fun_ie(&mut self) -> PDAC2_FUN_IE_W {
        PDAC2_FUN_IE_W { w: self }
    }
    #[doc = "Bit 14 - output enable in sleep mode"]
    #[inline(always)]
    pub fn pdac2_slp_oe(&mut self) -> PDAC2_SLP_OE_W {
        PDAC2_SLP_OE_W { w: self }
    }
    #[doc = "Bit 15 - input enable in sleep mode"]
    #[inline(always)]
    pub fn pdac2_slp_ie(&mut self) -> PDAC2_SLP_IE_W {
        PDAC2_SLP_IE_W { w: self }
    }
    #[doc = "Bit 16 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn pdac2_slp_sel(&mut self) -> PDAC2_SLP_SEL_W {
        PDAC2_SLP_SEL_W { w: self }
    }
    #[doc = "Bits 17:18 - PDAC1 function sel"]
    #[inline(always)]
    pub fn pdac2_fun_sel(&mut self) -> PDAC2_FUN_SEL_W {
        PDAC2_FUN_SEL_W { w: self }
    }
    #[doc = "Bit 19 - 1: use RTC GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn pdac2_mux_sel(&mut self) -> PDAC2_MUX_SEL_W {
        PDAC2_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 27 - PDAC2_RUE"]
    #[inline(always)]
    pub fn pdac2_rue(&mut self) -> PDAC2_RUE_W {
        PDAC2_RUE_W { w: self }
    }
    #[doc = "Bit 28 - PDAC2_RDE"]
    #[inline(always)]
    pub fn pdac2_rde(&mut self) -> PDAC2_RDE_W {
        PDAC2_RDE_W { w: self }
    }
    #[doc = "Bits 29:30 - PDAC2_DRV"]
    #[inline(always)]
    pub fn pdac2_drv(&mut self) -> PDAC2_DRV_W {
        PDAC2_DRV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure RTC PAD18\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_dac2](index.html) module"]
pub struct PAD_DAC2_SPEC;
impl crate::RegisterSpec for PAD_DAC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_dac2::R](R) reader structure"]
impl crate::Readable for PAD_DAC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_dac2::W](W) writer structure"]
impl crate::Writable for PAD_DAC2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAD_DAC2 to value 0x4000_0000"]
impl crate::Resettable for PAD_DAC2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000_0000
    }
}
