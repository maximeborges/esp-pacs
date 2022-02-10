#[doc = "Register `CLK_OUT_EN` reader"]
pub struct R(crate::R<CLK_OUT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_OUT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_OUT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_OUT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_OUT_EN` writer"]
pub struct W(crate::W<CLK_OUT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_OUT_EN_SPEC>;
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
impl From<crate::W<CLK_OUT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_OUT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK20_OEN` reader - reg_clk20_oen"]
pub struct CLK20_OEN_R(crate::FieldReader<bool, bool>);
impl CLK20_OEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK20_OEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK20_OEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK20_OEN` writer - reg_clk20_oen"]
pub struct CLK20_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK20_OEN_W<'a> {
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
#[doc = "Field `CLK22_OEN` reader - reg_clk22_oen"]
pub struct CLK22_OEN_R(crate::FieldReader<bool, bool>);
impl CLK22_OEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK22_OEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK22_OEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK22_OEN` writer - reg_clk22_oen"]
pub struct CLK22_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK22_OEN_W<'a> {
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
#[doc = "Field `CLK44_OEN` reader - reg_clk44_oen"]
pub struct CLK44_OEN_R(crate::FieldReader<bool, bool>);
impl CLK44_OEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK44_OEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK44_OEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK44_OEN` writer - reg_clk44_oen"]
pub struct CLK44_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK44_OEN_W<'a> {
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
#[doc = "Field `CLK_BB_OEN` reader - reg_clk_bb_oen"]
pub struct CLK_BB_OEN_R(crate::FieldReader<bool, bool>);
impl CLK_BB_OEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_BB_OEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_BB_OEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_BB_OEN` writer - reg_clk_bb_oen"]
pub struct CLK_BB_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_BB_OEN_W<'a> {
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
#[doc = "Field `CLK80_OEN` reader - reg_clk80_oen"]
pub struct CLK80_OEN_R(crate::FieldReader<bool, bool>);
impl CLK80_OEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK80_OEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK80_OEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK80_OEN` writer - reg_clk80_oen"]
pub struct CLK80_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK80_OEN_W<'a> {
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
#[doc = "Field `CLK160_OEN` reader - reg_clk160_oen"]
pub struct CLK160_OEN_R(crate::FieldReader<bool, bool>);
impl CLK160_OEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK160_OEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK160_OEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK160_OEN` writer - reg_clk160_oen"]
pub struct CLK160_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK160_OEN_W<'a> {
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
#[doc = "Field `CLK_320M_OEN` reader - reg_clk_320m_oen"]
pub struct CLK_320M_OEN_R(crate::FieldReader<bool, bool>);
impl CLK_320M_OEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_320M_OEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_320M_OEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_320M_OEN` writer - reg_clk_320m_oen"]
pub struct CLK_320M_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_320M_OEN_W<'a> {
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
#[doc = "Field `CLK_ADC_INF_OEN` reader - reg_clk_adc_inf_oen"]
pub struct CLK_ADC_INF_OEN_R(crate::FieldReader<bool, bool>);
impl CLK_ADC_INF_OEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_ADC_INF_OEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_ADC_INF_OEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_ADC_INF_OEN` writer - reg_clk_adc_inf_oen"]
pub struct CLK_ADC_INF_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_ADC_INF_OEN_W<'a> {
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
#[doc = "Field `CLK_DAC_CPU_OEN` reader - reg_clk_dac_cpu_oen"]
pub struct CLK_DAC_CPU_OEN_R(crate::FieldReader<bool, bool>);
impl CLK_DAC_CPU_OEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_DAC_CPU_OEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_DAC_CPU_OEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_DAC_CPU_OEN` writer - reg_clk_dac_cpu_oen"]
pub struct CLK_DAC_CPU_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DAC_CPU_OEN_W<'a> {
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
#[doc = "Field `CLK40X_BB_OEN` reader - reg_clk40x_bb_oen"]
pub struct CLK40X_BB_OEN_R(crate::FieldReader<bool, bool>);
impl CLK40X_BB_OEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK40X_BB_OEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK40X_BB_OEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK40X_BB_OEN` writer - reg_clk40x_bb_oen"]
pub struct CLK40X_BB_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK40X_BB_OEN_W<'a> {
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
#[doc = "Field `CLK_XTAL_OEN` reader - reg_clk_xtal_oen"]
pub struct CLK_XTAL_OEN_R(crate::FieldReader<bool, bool>);
impl CLK_XTAL_OEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_XTAL_OEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_XTAL_OEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_XTAL_OEN` writer - reg_clk_xtal_oen"]
pub struct CLK_XTAL_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_XTAL_OEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - reg_clk20_oen"]
    #[inline(always)]
    pub fn clk20_oen(&self) -> CLK20_OEN_R {
        CLK20_OEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - reg_clk22_oen"]
    #[inline(always)]
    pub fn clk22_oen(&self) -> CLK22_OEN_R {
        CLK22_OEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - reg_clk44_oen"]
    #[inline(always)]
    pub fn clk44_oen(&self) -> CLK44_OEN_R {
        CLK44_OEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - reg_clk_bb_oen"]
    #[inline(always)]
    pub fn clk_bb_oen(&self) -> CLK_BB_OEN_R {
        CLK_BB_OEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - reg_clk80_oen"]
    #[inline(always)]
    pub fn clk80_oen(&self) -> CLK80_OEN_R {
        CLK80_OEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - reg_clk160_oen"]
    #[inline(always)]
    pub fn clk160_oen(&self) -> CLK160_OEN_R {
        CLK160_OEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - reg_clk_320m_oen"]
    #[inline(always)]
    pub fn clk_320m_oen(&self) -> CLK_320M_OEN_R {
        CLK_320M_OEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - reg_clk_adc_inf_oen"]
    #[inline(always)]
    pub fn clk_adc_inf_oen(&self) -> CLK_ADC_INF_OEN_R {
        CLK_ADC_INF_OEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - reg_clk_dac_cpu_oen"]
    #[inline(always)]
    pub fn clk_dac_cpu_oen(&self) -> CLK_DAC_CPU_OEN_R {
        CLK_DAC_CPU_OEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - reg_clk40x_bb_oen"]
    #[inline(always)]
    pub fn clk40x_bb_oen(&self) -> CLK40X_BB_OEN_R {
        CLK40X_BB_OEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - reg_clk_xtal_oen"]
    #[inline(always)]
    pub fn clk_xtal_oen(&self) -> CLK_XTAL_OEN_R {
        CLK_XTAL_OEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - reg_clk20_oen"]
    #[inline(always)]
    pub fn clk20_oen(&mut self) -> CLK20_OEN_W {
        CLK20_OEN_W { w: self }
    }
    #[doc = "Bit 1 - reg_clk22_oen"]
    #[inline(always)]
    pub fn clk22_oen(&mut self) -> CLK22_OEN_W {
        CLK22_OEN_W { w: self }
    }
    #[doc = "Bit 2 - reg_clk44_oen"]
    #[inline(always)]
    pub fn clk44_oen(&mut self) -> CLK44_OEN_W {
        CLK44_OEN_W { w: self }
    }
    #[doc = "Bit 3 - reg_clk_bb_oen"]
    #[inline(always)]
    pub fn clk_bb_oen(&mut self) -> CLK_BB_OEN_W {
        CLK_BB_OEN_W { w: self }
    }
    #[doc = "Bit 4 - reg_clk80_oen"]
    #[inline(always)]
    pub fn clk80_oen(&mut self) -> CLK80_OEN_W {
        CLK80_OEN_W { w: self }
    }
    #[doc = "Bit 5 - reg_clk160_oen"]
    #[inline(always)]
    pub fn clk160_oen(&mut self) -> CLK160_OEN_W {
        CLK160_OEN_W { w: self }
    }
    #[doc = "Bit 6 - reg_clk_320m_oen"]
    #[inline(always)]
    pub fn clk_320m_oen(&mut self) -> CLK_320M_OEN_W {
        CLK_320M_OEN_W { w: self }
    }
    #[doc = "Bit 7 - reg_clk_adc_inf_oen"]
    #[inline(always)]
    pub fn clk_adc_inf_oen(&mut self) -> CLK_ADC_INF_OEN_W {
        CLK_ADC_INF_OEN_W { w: self }
    }
    #[doc = "Bit 8 - reg_clk_dac_cpu_oen"]
    #[inline(always)]
    pub fn clk_dac_cpu_oen(&mut self) -> CLK_DAC_CPU_OEN_W {
        CLK_DAC_CPU_OEN_W { w: self }
    }
    #[doc = "Bit 9 - reg_clk40x_bb_oen"]
    #[inline(always)]
    pub fn clk40x_bb_oen(&mut self) -> CLK40X_BB_OEN_W {
        CLK40X_BB_OEN_W { w: self }
    }
    #[doc = "Bit 10 - reg_clk_xtal_oen"]
    #[inline(always)]
    pub fn clk_xtal_oen(&mut self) -> CLK_XTAL_OEN_W {
        CLK_XTAL_OEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB_CTRL_CLK_OUT_EN_REG\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_out_en]
(index.html) module"]
pub struct CLK_OUT_EN_SPEC;
impl crate::RegisterSpec for CLK_OUT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_out_en::R]
(R) reader structure"]
impl crate::Readable for CLK_OUT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_out_en::W]
(W) writer structure"]
impl crate::Writable for CLK_OUT_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_OUT_EN to value 0x07ff"]
impl crate::Resettable for CLK_OUT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07ff
    }
}
