#[doc = "Register `XTAL_32P_PAD` reader"]
pub struct R(crate::R<XTAL_32P_PAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTAL_32P_PAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTAL_32P_PAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTAL_32P_PAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTAL_32P_PAD` writer"]
pub struct W(crate::W<XTAL_32P_PAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTAL_32P_PAD_SPEC>;
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
impl From<crate::W<XTAL_32P_PAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTAL_32P_PAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `X32P_FUN_IE` reader - input enable in work mode"]
pub struct X32P_FUN_IE_R(crate::FieldReader<bool>);
impl X32P_FUN_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X32P_FUN_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32P_FUN_IE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32P_FUN_IE` writer - input enable in work mode"]
pub struct X32P_FUN_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> X32P_FUN_IE_W<'a> {
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
#[doc = "Field `X32P_SLP_OE` reader - output enable in sleep mode"]
pub struct X32P_SLP_OE_R(crate::FieldReader<bool>);
impl X32P_SLP_OE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X32P_SLP_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32P_SLP_OE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32P_SLP_OE` writer - output enable in sleep mode"]
pub struct X32P_SLP_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> X32P_SLP_OE_W<'a> {
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
#[doc = "Field `X32P_SLP_IE` reader - input enable in sleep mode"]
pub struct X32P_SLP_IE_R(crate::FieldReader<bool>);
impl X32P_SLP_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X32P_SLP_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32P_SLP_IE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32P_SLP_IE` writer - input enable in sleep mode"]
pub struct X32P_SLP_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> X32P_SLP_IE_W<'a> {
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
#[doc = "Field `X32P_SLP_SEL` reader - 1: enable sleep mode during sleep,0: no sleep mode"]
pub struct X32P_SLP_SEL_R(crate::FieldReader<bool>);
impl X32P_SLP_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X32P_SLP_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32P_SLP_SEL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32P_SLP_SEL` writer - 1: enable sleep mode during sleep,0: no sleep mode"]
pub struct X32P_SLP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> X32P_SLP_SEL_W<'a> {
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
#[doc = "Field `X32P_FUN_SEL` reader - function sel"]
pub struct X32P_FUN_SEL_R(crate::FieldReader<u8>);
impl X32P_FUN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        X32P_FUN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32P_FUN_SEL_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32P_FUN_SEL` writer - function sel"]
pub struct X32P_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> X32P_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 17)) | ((value as u32 & 3) << 17);
        self.w
    }
}
#[doc = "Field `X32P_MUX_SEL` reader - 1: use RTC GPIO,0: use digital GPIO"]
pub struct X32P_MUX_SEL_R(crate::FieldReader<bool>);
impl X32P_MUX_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X32P_MUX_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32P_MUX_SEL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32P_MUX_SEL` writer - 1: use RTC GPIO,0: use digital GPIO"]
pub struct X32P_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> X32P_MUX_SEL_W<'a> {
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
#[doc = "Field `X32P_RUE` reader - RUE"]
pub struct X32P_RUE_R(crate::FieldReader<bool>);
impl X32P_RUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X32P_RUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32P_RUE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32P_RUE` writer - RUE"]
pub struct X32P_RUE_W<'a> {
    w: &'a mut W,
}
impl<'a> X32P_RUE_W<'a> {
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
#[doc = "Field `X32P_RDE` reader - RDE"]
pub struct X32P_RDE_R(crate::FieldReader<bool>);
impl X32P_RDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X32P_RDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32P_RDE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32P_RDE` writer - RDE"]
pub struct X32P_RDE_W<'a> {
    w: &'a mut W,
}
impl<'a> X32P_RDE_W<'a> {
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
#[doc = "Field `X32P_DRV` reader - DRV"]
pub struct X32P_DRV_R(crate::FieldReader<u8>);
impl X32P_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        X32P_DRV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32P_DRV_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32P_DRV` writer - DRV"]
pub struct X32P_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> X32P_DRV_W<'a> {
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
    pub fn x32p_fun_ie(&self) -> X32P_FUN_IE_R {
        X32P_FUN_IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - output enable in sleep mode"]
    #[inline(always)]
    pub fn x32p_slp_oe(&self) -> X32P_SLP_OE_R {
        X32P_SLP_OE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - input enable in sleep mode"]
    #[inline(always)]
    pub fn x32p_slp_ie(&self) -> X32P_SLP_IE_R {
        X32P_SLP_IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn x32p_slp_sel(&self) -> X32P_SLP_SEL_R {
        X32P_SLP_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - function sel"]
    #[inline(always)]
    pub fn x32p_fun_sel(&self) -> X32P_FUN_SEL_R {
        X32P_FUN_SEL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - 1: use RTC GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn x32p_mux_sel(&self) -> X32P_MUX_SEL_R {
        X32P_MUX_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 27 - RUE"]
    #[inline(always)]
    pub fn x32p_rue(&self) -> X32P_RUE_R {
        X32P_RUE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - RDE"]
    #[inline(always)]
    pub fn x32p_rde(&self) -> X32P_RDE_R {
        X32P_RDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - DRV"]
    #[inline(always)]
    pub fn x32p_drv(&self) -> X32P_DRV_R {
        X32P_DRV_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 13 - input enable in work mode"]
    #[inline(always)]
    pub fn x32p_fun_ie(&mut self) -> X32P_FUN_IE_W {
        X32P_FUN_IE_W { w: self }
    }
    #[doc = "Bit 14 - output enable in sleep mode"]
    #[inline(always)]
    pub fn x32p_slp_oe(&mut self) -> X32P_SLP_OE_W {
        X32P_SLP_OE_W { w: self }
    }
    #[doc = "Bit 15 - input enable in sleep mode"]
    #[inline(always)]
    pub fn x32p_slp_ie(&mut self) -> X32P_SLP_IE_W {
        X32P_SLP_IE_W { w: self }
    }
    #[doc = "Bit 16 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn x32p_slp_sel(&mut self) -> X32P_SLP_SEL_W {
        X32P_SLP_SEL_W { w: self }
    }
    #[doc = "Bits 17:18 - function sel"]
    #[inline(always)]
    pub fn x32p_fun_sel(&mut self) -> X32P_FUN_SEL_W {
        X32P_FUN_SEL_W { w: self }
    }
    #[doc = "Bit 19 - 1: use RTC GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn x32p_mux_sel(&mut self) -> X32P_MUX_SEL_W {
        X32P_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 27 - RUE"]
    #[inline(always)]
    pub fn x32p_rue(&mut self) -> X32P_RUE_W {
        X32P_RUE_W { w: self }
    }
    #[doc = "Bit 28 - RDE"]
    #[inline(always)]
    pub fn x32p_rde(&mut self) -> X32P_RDE_W {
        X32P_RDE_W { w: self }
    }
    #[doc = "Bits 29:30 - DRV"]
    #[inline(always)]
    pub fn x32p_drv(&mut self) -> X32P_DRV_W {
        X32P_DRV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure RTC PAD15\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal_32p_pad](index.html) module"]
pub struct XTAL_32P_PAD_SPEC;
impl crate::RegisterSpec for XTAL_32P_PAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtal_32p_pad::R](R) reader structure"]
impl crate::Readable for XTAL_32P_PAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtal_32p_pad::W](W) writer structure"]
impl crate::Writable for XTAL_32P_PAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XTAL_32P_PAD to value 0x4000_0000"]
impl crate::Resettable for XTAL_32P_PAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000_0000
    }
}
