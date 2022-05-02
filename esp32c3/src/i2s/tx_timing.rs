#[doc = "Register `TX_TIMING` reader"]
pub struct R(crate::R<TX_TIMING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_TIMING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_TIMING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_TIMING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_TIMING` writer"]
pub struct W(crate::W<TX_TIMING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_TIMING_SPEC>;
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
impl From<crate::W<TX_TIMING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_TIMING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_SD_OUT_DM` reader - The delay mode of I2S TX SD output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub struct TX_SD_OUT_DM_R(crate::FieldReader<u8>);
impl TX_SD_OUT_DM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_SD_OUT_DM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_SD_OUT_DM_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_SD_OUT_DM` writer - The delay mode of I2S TX SD output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub struct TX_SD_OUT_DM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SD_OUT_DM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `TX_SD1_OUT_DM` reader - The delay mode of I2S TX SD1 output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub struct TX_SD1_OUT_DM_R(crate::FieldReader<u8>);
impl TX_SD1_OUT_DM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_SD1_OUT_DM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_SD1_OUT_DM_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_SD1_OUT_DM` writer - The delay mode of I2S TX SD1 output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub struct TX_SD1_OUT_DM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SD1_OUT_DM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "Field `TX_WS_OUT_DM` reader - The delay mode of I2S TX WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub struct TX_WS_OUT_DM_R(crate::FieldReader<u8>);
impl TX_WS_OUT_DM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_WS_OUT_DM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_WS_OUT_DM_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_WS_OUT_DM` writer - The delay mode of I2S TX WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub struct TX_WS_OUT_DM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_WS_OUT_DM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 16)) | ((value as u32 & 3) << 16);
        self.w
    }
}
#[doc = "Field `TX_BCK_OUT_DM` reader - The delay mode of I2S TX BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub struct TX_BCK_OUT_DM_R(crate::FieldReader<u8>);
impl TX_BCK_OUT_DM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_BCK_OUT_DM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BCK_OUT_DM_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BCK_OUT_DM` writer - The delay mode of I2S TX BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub struct TX_BCK_OUT_DM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BCK_OUT_DM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 20)) | ((value as u32 & 3) << 20);
        self.w
    }
}
#[doc = "Field `TX_WS_IN_DM` reader - The delay mode of I2S TX WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub struct TX_WS_IN_DM_R(crate::FieldReader<u8>);
impl TX_WS_IN_DM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_WS_IN_DM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_WS_IN_DM_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_WS_IN_DM` writer - The delay mode of I2S TX WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub struct TX_WS_IN_DM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_WS_IN_DM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 24)) | ((value as u32 & 3) << 24);
        self.w
    }
}
#[doc = "Field `TX_BCK_IN_DM` reader - The delay mode of I2S TX BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub struct TX_BCK_IN_DM_R(crate::FieldReader<u8>);
impl TX_BCK_IN_DM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_BCK_IN_DM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BCK_IN_DM_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BCK_IN_DM` writer - The delay mode of I2S TX BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub struct TX_BCK_IN_DM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BCK_IN_DM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 28)) | ((value as u32 & 3) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - The delay mode of I2S TX SD output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn tx_sd_out_dm(&self) -> TX_SD_OUT_DM_R {
        TX_SD_OUT_DM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - The delay mode of I2S TX SD1 output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn tx_sd1_out_dm(&self) -> TX_SD1_OUT_DM_R {
        TX_SD1_OUT_DM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:17 - The delay mode of I2S TX WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn tx_ws_out_dm(&self) -> TX_WS_OUT_DM_R {
        TX_WS_OUT_DM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - The delay mode of I2S TX BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn tx_bck_out_dm(&self) -> TX_BCK_OUT_DM_R {
        TX_BCK_OUT_DM_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - The delay mode of I2S TX WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn tx_ws_in_dm(&self) -> TX_WS_IN_DM_R {
        TX_WS_IN_DM_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - The delay mode of I2S TX BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn tx_bck_in_dm(&self) -> TX_BCK_IN_DM_R {
        TX_BCK_IN_DM_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - The delay mode of I2S TX SD output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn tx_sd_out_dm(&mut self) -> TX_SD_OUT_DM_W {
        TX_SD_OUT_DM_W { w: self }
    }
    #[doc = "Bits 4:5 - The delay mode of I2S TX SD1 output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn tx_sd1_out_dm(&mut self) -> TX_SD1_OUT_DM_W {
        TX_SD1_OUT_DM_W { w: self }
    }
    #[doc = "Bits 16:17 - The delay mode of I2S TX WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn tx_ws_out_dm(&mut self) -> TX_WS_OUT_DM_W {
        TX_WS_OUT_DM_W { w: self }
    }
    #[doc = "Bits 20:21 - The delay mode of I2S TX BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn tx_bck_out_dm(&mut self) -> TX_BCK_OUT_DM_W {
        TX_BCK_OUT_DM_W { w: self }
    }
    #[doc = "Bits 24:25 - The delay mode of I2S TX WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn tx_ws_in_dm(&mut self) -> TX_WS_IN_DM_W {
        TX_WS_IN_DM_W { w: self }
    }
    #[doc = "Bits 28:29 - The delay mode of I2S TX BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn tx_bck_in_dm(&mut self) -> TX_BCK_IN_DM_W {
        TX_BCK_IN_DM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S TX timing control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_timing](index.html) module"]
pub struct TX_TIMING_SPEC;
impl crate::RegisterSpec for TX_TIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_timing::R](R) reader structure"]
impl crate::Readable for TX_TIMING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_timing::W](W) writer structure"]
impl crate::Writable for TX_TIMING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_TIMING to value 0"]
impl crate::Resettable for TX_TIMING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
