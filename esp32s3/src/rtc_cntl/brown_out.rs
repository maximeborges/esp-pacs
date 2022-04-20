#[doc = "Register `BROWN_OUT` reader"]
pub struct R(crate::R<BROWN_OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BROWN_OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BROWN_OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BROWN_OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BROWN_OUT` writer"]
pub struct W(crate::W<BROWN_OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BROWN_OUT_SPEC>;
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
impl From<crate::W<BROWN_OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BROWN_OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_WAIT` reader - brown out interrupt wait cycles"]
pub struct INT_WAIT_R(crate::FieldReader<u16, u16>);
impl INT_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        INT_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_WAIT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_WAIT` writer - brown out interrupt wait cycles"]
pub struct INT_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 4)) | ((value as u32 & 0x03ff) << 4);
        self.w
    }
}
#[doc = "Field `CLOSE_FLASH_ENA` reader - enable close flash when brown out happens"]
pub struct CLOSE_FLASH_ENA_R(crate::FieldReader<bool, bool>);
impl CLOSE_FLASH_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLOSE_FLASH_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLOSE_FLASH_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLOSE_FLASH_ENA` writer - enable close flash when brown out happens"]
pub struct CLOSE_FLASH_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOSE_FLASH_ENA_W<'a> {
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
#[doc = "Field `PD_RF_ENA` reader - enable power down RF when brown out happens"]
pub struct PD_RF_ENA_R(crate::FieldReader<bool, bool>);
impl PD_RF_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD_RF_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_RF_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD_RF_ENA` writer - enable power down RF when brown out happens"]
pub struct PD_RF_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_RF_ENA_W<'a> {
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
#[doc = "Field `RST_WAIT` reader - brown out reset wait cycles"]
pub struct RST_WAIT_R(crate::FieldReader<u16, u16>);
impl RST_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RST_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RST_WAIT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RST_WAIT` writer - brown out reset wait cycles"]
pub struct RST_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | ((value as u32 & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Field `RST_ENA` reader - enable brown out reset"]
pub struct RST_ENA_R(crate::FieldReader<bool, bool>);
impl RST_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RST_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RST_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RST_ENA` writer - enable brown out reset"]
pub struct RST_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_ENA_W<'a> {
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
#[doc = "Field `RST_SEL` reader - 1: 4-pos reset, 0: sys_reset"]
pub struct RST_SEL_R(crate::FieldReader<bool, bool>);
impl RST_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RST_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RST_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RST_SEL` writer - 1: 4-pos reset, 0: sys_reset"]
pub struct RST_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_SEL_W<'a> {
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
#[doc = "Field `ANA_RST_EN` reader - enable brown out reset en"]
pub struct ANA_RST_EN_R(crate::FieldReader<bool, bool>);
impl ANA_RST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ANA_RST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANA_RST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANA_RST_EN` writer - enable brown out reset en"]
pub struct ANA_RST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ANA_RST_EN_W<'a> {
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
#[doc = "Field `CNT_CLR` writer - clear brown out counter"]
pub struct CNT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_CLR_W<'a> {
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
#[doc = "Field `ENA` reader - enable brown out"]
pub struct ENA_R(crate::FieldReader<bool, bool>);
impl ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA` writer - enable brown out"]
pub struct ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_W<'a> {
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
#[doc = "Field `RTC_BROWN_OUT_DET` reader - get brown out detect"]
pub struct RTC_BROWN_OUT_DET_R(crate::FieldReader<bool, bool>);
impl RTC_BROWN_OUT_DET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_BROWN_OUT_DET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_BROWN_OUT_DET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 4:13 - brown out interrupt wait cycles"]
    #[inline(always)]
    pub fn int_wait(&self) -> INT_WAIT_R {
        INT_WAIT_R::new(((self.bits >> 4) & 0x03ff) as u16)
    }
    #[doc = "Bit 14 - enable close flash when brown out happens"]
    #[inline(always)]
    pub fn close_flash_ena(&self) -> CLOSE_FLASH_ENA_R {
        CLOSE_FLASH_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - enable power down RF when brown out happens"]
    #[inline(always)]
    pub fn pd_rf_ena(&self) -> PD_RF_ENA_R {
        PD_RF_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:25 - brown out reset wait cycles"]
    #[inline(always)]
    pub fn rst_wait(&self) -> RST_WAIT_R {
        RST_WAIT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - enable brown out reset"]
    #[inline(always)]
    pub fn rst_ena(&self) -> RST_ENA_R {
        RST_ENA_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 1: 4-pos reset, 0: sys_reset"]
    #[inline(always)]
    pub fn rst_sel(&self) -> RST_SEL_R {
        RST_SEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - enable brown out reset en"]
    #[inline(always)]
    pub fn ana_rst_en(&self) -> ANA_RST_EN_R {
        ANA_RST_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - enable brown out"]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - get brown out detect"]
    #[inline(always)]
    pub fn rtc_brown_out_det(&self) -> RTC_BROWN_OUT_DET_R {
        RTC_BROWN_OUT_DET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:13 - brown out interrupt wait cycles"]
    #[inline(always)]
    pub fn int_wait(&mut self) -> INT_WAIT_W {
        INT_WAIT_W { w: self }
    }
    #[doc = "Bit 14 - enable close flash when brown out happens"]
    #[inline(always)]
    pub fn close_flash_ena(&mut self) -> CLOSE_FLASH_ENA_W {
        CLOSE_FLASH_ENA_W { w: self }
    }
    #[doc = "Bit 15 - enable power down RF when brown out happens"]
    #[inline(always)]
    pub fn pd_rf_ena(&mut self) -> PD_RF_ENA_W {
        PD_RF_ENA_W { w: self }
    }
    #[doc = "Bits 16:25 - brown out reset wait cycles"]
    #[inline(always)]
    pub fn rst_wait(&mut self) -> RST_WAIT_W {
        RST_WAIT_W { w: self }
    }
    #[doc = "Bit 26 - enable brown out reset"]
    #[inline(always)]
    pub fn rst_ena(&mut self) -> RST_ENA_W {
        RST_ENA_W { w: self }
    }
    #[doc = "Bit 27 - 1: 4-pos reset, 0: sys_reset"]
    #[inline(always)]
    pub fn rst_sel(&mut self) -> RST_SEL_W {
        RST_SEL_W { w: self }
    }
    #[doc = "Bit 28 - enable brown out reset en"]
    #[inline(always)]
    pub fn ana_rst_en(&mut self) -> ANA_RST_EN_W {
        ANA_RST_EN_W { w: self }
    }
    #[doc = "Bit 29 - clear brown out counter"]
    #[inline(always)]
    pub fn cnt_clr(&mut self) -> CNT_CLR_W {
        CNT_CLR_W { w: self }
    }
    #[doc = "Bit 30 - enable brown out"]
    #[inline(always)]
    pub fn ena(&mut self) -> ENA_W {
        ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "congfigure brownout\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brown_out]
(index.html) module"]
pub struct BROWN_OUT_SPEC;
impl crate::RegisterSpec for BROWN_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brown_out::R]
(R) reader structure"]
impl crate::Readable for BROWN_OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brown_out::W]
(W) writer structure"]
impl crate::Writable for BROWN_OUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BROWN_OUT to value 0x43ff_0010"]
impl crate::Resettable for BROWN_OUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x43ff_0010
    }
}
