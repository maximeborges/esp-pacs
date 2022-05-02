#[doc = "Register `TOUCH_PAD8` reader"]
pub struct R(crate::R<TOUCH_PAD8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_PAD8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_PAD8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_PAD8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOUCH_PAD8` writer"]
pub struct W(crate::W<TOUCH_PAD8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_PAD8_SPEC>;
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
impl From<crate::W<TOUCH_PAD8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_PAD8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FUN_IE` reader - input enable in work mode"]
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
#[doc = "Field `FUN_IE` writer - input enable in work mode"]
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `SLP_OE` reader - output enable in sleep mode"]
pub struct SLP_OE_R(crate::FieldReader<bool>);
impl SLP_OE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLP_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLP_OE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLP_OE` writer - output enable in sleep mode"]
pub struct SLP_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_OE_W<'a> {
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
#[doc = "Field `SLP_IE` reader - input enable in sleep mode"]
pub struct SLP_IE_R(crate::FieldReader<bool>);
impl SLP_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLP_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLP_IE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLP_IE` writer - input enable in sleep mode"]
pub struct SLP_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_IE_W<'a> {
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
#[doc = "Field `SLP_SEL` reader - 1: enable sleep mode during sleep,0: no sleep mode"]
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
#[doc = "Field `SLP_SEL` writer - 1: enable sleep mode during sleep,0: no sleep mode"]
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "Field `FUN_SEL` reader - function sel"]
pub struct FUN_SEL_R(crate::FieldReader<u8>);
impl FUN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FUN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUN_SEL_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUN_SEL` writer - function sel"]
pub struct FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 17)) | ((value as u32 & 3) << 17);
        self.w
    }
}
#[doc = "Field `MUX_SEL` reader - 1: use RTC GPIO,0: use digital GPIO"]
pub struct MUX_SEL_R(crate::FieldReader<bool>);
impl MUX_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MUX_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_SEL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_SEL` writer - 1: use RTC GPIO,0: use digital GPIO"]
pub struct MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_SEL_W<'a> {
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
#[doc = "Field `XPD` reader - TOUCH_XPD"]
pub struct XPD_R(crate::FieldReader<bool>);
impl XPD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XPD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XPD` writer - TOUCH_XPD"]
pub struct XPD_W<'a> {
    w: &'a mut W,
}
impl<'a> XPD_W<'a> {
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
#[doc = "Field `TIE_OPT` reader - TOUCH_TIE_OPT"]
pub struct TIE_OPT_R(crate::FieldReader<bool>);
impl TIE_OPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE_OPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE_OPT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE_OPT` writer - TOUCH_TIE_OPT"]
pub struct TIE_OPT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE_OPT_W<'a> {
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
#[doc = "Field `START` reader - TOUCH_START"]
pub struct START_R(crate::FieldReader<bool>);
impl START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START` writer - TOUCH_START"]
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
        self.w.bits = (self.w.bits & !(1 << 22)) | ((value as u32 & 1) << 22);
        self.w
    }
}
#[doc = "Field `RUE` reader - RUE"]
pub struct RUE_R(crate::FieldReader<bool>);
impl RUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUE` writer - RUE"]
pub struct RUE_W<'a> {
    w: &'a mut W,
}
impl<'a> RUE_W<'a> {
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
#[doc = "Field `RDE` reader - RDE"]
pub struct RDE_R(crate::FieldReader<bool>);
impl RDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDE` writer - RDE"]
pub struct RDE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDE_W<'a> {
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
#[doc = "Field `DRV` reader - DRV"]
pub struct DRV_R(crate::FieldReader<u8>);
impl DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DRV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRV_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRV` writer - DRV"]
pub struct DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 29)) | ((value as u32 & 3) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 13 - input enable in work mode"]
    #[inline(always)]
    pub fn fun_ie(&self) -> FUN_IE_R {
        FUN_IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - output enable in sleep mode"]
    #[inline(always)]
    pub fn slp_oe(&self) -> SLP_OE_R {
        SLP_OE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - input enable in sleep mode"]
    #[inline(always)]
    pub fn slp_ie(&self) -> SLP_IE_R {
        SLP_IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn slp_sel(&self) -> SLP_SEL_R {
        SLP_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - function sel"]
    #[inline(always)]
    pub fn fun_sel(&self) -> FUN_SEL_R {
        FUN_SEL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - 1: use RTC GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn mux_sel(&self) -> MUX_SEL_R {
        MUX_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TOUCH_XPD"]
    #[inline(always)]
    pub fn xpd(&self) -> XPD_R {
        XPD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TOUCH_TIE_OPT"]
    #[inline(always)]
    pub fn tie_opt(&self) -> TIE_OPT_R {
        TIE_OPT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TOUCH_START"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 27 - RUE"]
    #[inline(always)]
    pub fn rue(&self) -> RUE_R {
        RUE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - RDE"]
    #[inline(always)]
    pub fn rde(&self) -> RDE_R {
        RDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - DRV"]
    #[inline(always)]
    pub fn drv(&self) -> DRV_R {
        DRV_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 13 - input enable in work mode"]
    #[inline(always)]
    pub fn fun_ie(&mut self) -> FUN_IE_W {
        FUN_IE_W { w: self }
    }
    #[doc = "Bit 14 - output enable in sleep mode"]
    #[inline(always)]
    pub fn slp_oe(&mut self) -> SLP_OE_W {
        SLP_OE_W { w: self }
    }
    #[doc = "Bit 15 - input enable in sleep mode"]
    #[inline(always)]
    pub fn slp_ie(&mut self) -> SLP_IE_W {
        SLP_IE_W { w: self }
    }
    #[doc = "Bit 16 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn slp_sel(&mut self) -> SLP_SEL_W {
        SLP_SEL_W { w: self }
    }
    #[doc = "Bits 17:18 - function sel"]
    #[inline(always)]
    pub fn fun_sel(&mut self) -> FUN_SEL_W {
        FUN_SEL_W { w: self }
    }
    #[doc = "Bit 19 - 1: use RTC GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn mux_sel(&mut self) -> MUX_SEL_W {
        MUX_SEL_W { w: self }
    }
    #[doc = "Bit 20 - TOUCH_XPD"]
    #[inline(always)]
    pub fn xpd(&mut self) -> XPD_W {
        XPD_W { w: self }
    }
    #[doc = "Bit 21 - TOUCH_TIE_OPT"]
    #[inline(always)]
    pub fn tie_opt(&mut self) -> TIE_OPT_W {
        TIE_OPT_W { w: self }
    }
    #[doc = "Bit 22 - TOUCH_START"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 27 - RUE"]
    #[inline(always)]
    pub fn rue(&mut self) -> RUE_W {
        RUE_W { w: self }
    }
    #[doc = "Bit 28 - RDE"]
    #[inline(always)]
    pub fn rde(&mut self) -> RDE_W {
        RDE_W { w: self }
    }
    #[doc = "Bits 29:30 - DRV"]
    #[inline(always)]
    pub fn drv(&mut self) -> DRV_W {
        DRV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure RTC PAD8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_pad8](index.html) module"]
pub struct TOUCH_PAD8_SPEC;
impl crate::RegisterSpec for TOUCH_PAD8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch_pad8::R](R) reader structure"]
impl crate::Readable for TOUCH_PAD8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch_pad8::W](W) writer structure"]
impl crate::Writable for TOUCH_PAD8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TOUCH_PAD8 to value 0x4000_0000"]
impl crate::Resettable for TOUCH_PAD8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000_0000
    }
}
