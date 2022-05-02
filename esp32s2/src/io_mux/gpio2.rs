#[doc = "Register `GPIO2` reader"]
pub struct R(crate::R<GPIO2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO2` writer"]
pub struct W(crate::W<GPIO2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO2_SPEC>;
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
impl From<crate::W<GPIO2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCU_OE` reader - Output enable of the pin in sleep mode. 1: Output enabled. 0: Output disabled."]
pub struct MCU_OE_R(crate::FieldReader<bool>);
impl MCU_OE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCU_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCU_OE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCU_OE` writer - Output enable of the pin in sleep mode. 1: Output enabled. 0: Output disabled."]
pub struct MCU_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_OE_W<'a> {
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
#[doc = "Field `SLP_SEL` reader - Sleep mode selection of this pin. Set to 1 to put the pin in sleep mode."]
pub struct SLP_SEL_R(crate::FieldReader<bool>);
impl SLP_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLP_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLP_SEL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLP_SEL` writer - Sleep mode selection of this pin. Set to 1 to put the pin in sleep mode."]
pub struct SLP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `MCU_WPD` reader - Pull-down enable of the pin during sleep mode. 1: Internal pull-down enabled. 0: internal pull-down disabled."]
pub struct MCU_WPD_R(crate::FieldReader<bool>);
impl MCU_WPD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCU_WPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCU_WPD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCU_WPD` writer - Pull-down enable of the pin during sleep mode. 1: Internal pull-down enabled. 0: internal pull-down disabled."]
pub struct MCU_WPD_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_WPD_W<'a> {
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
#[doc = "Field `MCU_WPU` reader - Pull-up enable of the pin during sleep mode. 1: Internal pull-up enabled. 0: Internal pull-up disabled."]
pub struct MCU_WPU_R(crate::FieldReader<bool>);
impl MCU_WPU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCU_WPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCU_WPU_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCU_WPU` writer - Pull-up enable of the pin during sleep mode. 1: Internal pull-up enabled. 0: Internal pull-up disabled."]
pub struct MCU_WPU_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_WPU_W<'a> {
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
#[doc = "Field `MCU_IE` reader - Input enable of the pin during sleep mode. 1: Input enabled. 0: Input disabled."]
pub struct MCU_IE_R(crate::FieldReader<bool>);
impl MCU_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCU_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCU_IE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCU_IE` writer - Input enable of the pin during sleep mode. 1: Input enabled. 0: Input disabled."]
pub struct MCU_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_IE_W<'a> {
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
#[doc = "Field `FUN_WPD` reader - Pull-down enable of the pin. 1: Internal pull-down enabled. 0: internal pull-down disabled."]
pub struct FUN_WPD_R(crate::FieldReader<bool>);
impl FUN_WPD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FUN_WPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUN_WPD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUN_WPD` writer - Pull-down enable of the pin. 1: Internal pull-down enabled. 0: internal pull-down disabled."]
pub struct FUN_WPD_W<'a> {
    w: &'a mut W,
}
impl<'a> FUN_WPD_W<'a> {
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
#[doc = "Field `FUN_WPU` reader - Pull-up enable of the pin. 1: Internal pull-up enabled. 0: Internal pull-up disabled."]
pub struct FUN_WPU_R(crate::FieldReader<bool>);
impl FUN_WPU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FUN_WPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUN_WPU_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUN_WPU` writer - Pull-up enable of the pin. 1: Internal pull-up enabled. 0: Internal pull-up disabled."]
pub struct FUN_WPU_W<'a> {
    w: &'a mut W,
}
impl<'a> FUN_WPU_W<'a> {
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
#[doc = "Field `FUN_IE` reader - Input enable of the pin. 1: Input enabled. 0: Input disabled."]
pub struct FUN_IE_R(crate::FieldReader<bool>);
impl FUN_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FUN_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUN_IE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUN_IE` writer - Input enable of the pin. 1: Input enabled. 0: Input disabled."]
pub struct FUN_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> FUN_IE_W<'a> {
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
#[doc = "Field `FUN_DRV` reader - Select the drive strength of the pin. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
pub struct FUN_DRV_R(crate::FieldReader<u8>);
impl FUN_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FUN_DRV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUN_DRV_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUN_DRV` writer - Select the drive strength of the pin. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
pub struct FUN_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> FUN_DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 10)) | ((value as u32 & 3) << 10);
        self.w
    }
}
#[doc = "Field `MCU_SEL` reader - Select IO MUX function for this signal. 0: Select Function 1. 1: Select Function 2, etc."]
pub struct MCU_SEL_R(crate::FieldReader<u8>);
impl MCU_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MCU_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCU_SEL_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCU_SEL` writer - Select IO MUX function for this signal. 0: Select Function 1. 1: Select Function 2, etc."]
pub struct MCU_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 12)) | ((value as u32 & 7) << 12);
        self.w
    }
}
#[doc = "Field `FILTER_EN` reader - Enable filter for pin input signals. 1: Filter enabled. 2: Filter disabled."]
pub struct FILTER_EN_R(crate::FieldReader<bool>);
impl FILTER_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FILTER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTER_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTER_EN` writer - Enable filter for pin input signals. 1: Filter enabled. 2: Filter disabled."]
pub struct FILTER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_EN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Output enable of the pin in sleep mode. 1: Output enabled. 0: Output disabled."]
    #[inline(always)]
    pub fn mcu_oe(&self) -> MCU_OE_R {
        MCU_OE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sleep mode selection of this pin. Set to 1 to put the pin in sleep mode."]
    #[inline(always)]
    pub fn slp_sel(&self) -> SLP_SEL_R {
        SLP_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pull-down enable of the pin during sleep mode. 1: Internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    pub fn mcu_wpd(&self) -> MCU_WPD_R {
        MCU_WPD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pull-up enable of the pin during sleep mode. 1: Internal pull-up enabled. 0: Internal pull-up disabled."]
    #[inline(always)]
    pub fn mcu_wpu(&self) -> MCU_WPU_R {
        MCU_WPU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Input enable of the pin during sleep mode. 1: Input enabled. 0: Input disabled."]
    #[inline(always)]
    pub fn mcu_ie(&self) -> MCU_IE_R {
        MCU_IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Pull-down enable of the pin. 1: Internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    pub fn fun_wpd(&self) -> FUN_WPD_R {
        FUN_WPD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pull-up enable of the pin. 1: Internal pull-up enabled. 0: Internal pull-up disabled."]
    #[inline(always)]
    pub fn fun_wpu(&self) -> FUN_WPU_R {
        FUN_WPU_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Input enable of the pin. 1: Input enabled. 0: Input disabled."]
    #[inline(always)]
    pub fn fun_ie(&self) -> FUN_IE_R {
        FUN_IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Select the drive strength of the pin. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
    #[inline(always)]
    pub fn fun_drv(&self) -> FUN_DRV_R {
        FUN_DRV_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - Select IO MUX function for this signal. 0: Select Function 1. 1: Select Function 2, etc."]
    #[inline(always)]
    pub fn mcu_sel(&self) -> MCU_SEL_R {
        MCU_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Enable filter for pin input signals. 1: Filter enabled. 2: Filter disabled."]
    #[inline(always)]
    pub fn filter_en(&self) -> FILTER_EN_R {
        FILTER_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output enable of the pin in sleep mode. 1: Output enabled. 0: Output disabled."]
    #[inline(always)]
    pub fn mcu_oe(&mut self) -> MCU_OE_W {
        MCU_OE_W { w: self }
    }
    #[doc = "Bit 1 - Sleep mode selection of this pin. Set to 1 to put the pin in sleep mode."]
    #[inline(always)]
    pub fn slp_sel(&mut self) -> SLP_SEL_W {
        SLP_SEL_W { w: self }
    }
    #[doc = "Bit 2 - Pull-down enable of the pin during sleep mode. 1: Internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    pub fn mcu_wpd(&mut self) -> MCU_WPD_W {
        MCU_WPD_W { w: self }
    }
    #[doc = "Bit 3 - Pull-up enable of the pin during sleep mode. 1: Internal pull-up enabled. 0: Internal pull-up disabled."]
    #[inline(always)]
    pub fn mcu_wpu(&mut self) -> MCU_WPU_W {
        MCU_WPU_W { w: self }
    }
    #[doc = "Bit 4 - Input enable of the pin during sleep mode. 1: Input enabled. 0: Input disabled."]
    #[inline(always)]
    pub fn mcu_ie(&mut self) -> MCU_IE_W {
        MCU_IE_W { w: self }
    }
    #[doc = "Bit 7 - Pull-down enable of the pin. 1: Internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    pub fn fun_wpd(&mut self) -> FUN_WPD_W {
        FUN_WPD_W { w: self }
    }
    #[doc = "Bit 8 - Pull-up enable of the pin. 1: Internal pull-up enabled. 0: Internal pull-up disabled."]
    #[inline(always)]
    pub fn fun_wpu(&mut self) -> FUN_WPU_W {
        FUN_WPU_W { w: self }
    }
    #[doc = "Bit 9 - Input enable of the pin. 1: Input enabled. 0: Input disabled."]
    #[inline(always)]
    pub fn fun_ie(&mut self) -> FUN_IE_W {
        FUN_IE_W { w: self }
    }
    #[doc = "Bits 10:11 - Select the drive strength of the pin. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
    #[inline(always)]
    pub fn fun_drv(&mut self) -> FUN_DRV_W {
        FUN_DRV_W { w: self }
    }
    #[doc = "Bits 12:14 - Select IO MUX function for this signal. 0: Select Function 1. 1: Select Function 2, etc."]
    #[inline(always)]
    pub fn mcu_sel(&mut self) -> MCU_SEL_W {
        MCU_SEL_W { w: self }
    }
    #[doc = "Bit 15 - Enable filter for pin input signals. 1: Filter enabled. 2: Filter disabled."]
    #[inline(always)]
    pub fn filter_en(&mut self) -> FILTER_EN_W {
        FILTER_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register for pin GPIO2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio2](index.html) module"]
pub struct GPIO2_SPEC;
impl crate::RegisterSpec for GPIO2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio2::R](R) reader structure"]
impl crate::Readable for GPIO2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio2::W](W) writer structure"]
impl crate::Writable for GPIO2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO2 to value 0x0b00"]
impl crate::Resettable for GPIO2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0b00
    }
}
