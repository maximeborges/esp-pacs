#[doc = "Register `DOUT_NUM` reader"]
pub struct R(crate::R<DOUT_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUT_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUT_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUT_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOUT_NUM` writer"]
pub struct W(crate::W<DOUT_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOUT_NUM_SPEC>;
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
impl From<crate::W<DOUT_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOUT_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUT0_NUM` reader - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub struct DOUT0_NUM_R(crate::FieldReader<u8, u8>);
impl DOUT0_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DOUT0_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT0_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT0_NUM` writer - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub struct DOUT0_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT0_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `DOUT1_NUM` reader - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub struct DOUT1_NUM_R(crate::FieldReader<u8, u8>);
impl DOUT1_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DOUT1_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT1_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT1_NUM` writer - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub struct DOUT1_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT1_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `DOUT2_NUM` reader - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub struct DOUT2_NUM_R(crate::FieldReader<u8, u8>);
impl DOUT2_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DOUT2_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT2_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT2_NUM` writer - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub struct DOUT2_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT2_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `DOUT3_NUM` reader - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub struct DOUT3_NUM_R(crate::FieldReader<u8, u8>);
impl DOUT3_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DOUT3_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT3_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT3_NUM` writer - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub struct DOUT3_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT3_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `DOUT4_NUM` reader - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub struct DOUT4_NUM_R(crate::FieldReader<u8, u8>);
impl DOUT4_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DOUT4_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT4_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT4_NUM` writer - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub struct DOUT4_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT4_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `DOUT5_NUM` reader - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub struct DOUT5_NUM_R(crate::FieldReader<u8, u8>);
impl DOUT5_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DOUT5_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT5_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT5_NUM` writer - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub struct DOUT5_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT5_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `DOUT6_NUM` reader - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub struct DOUT6_NUM_R(crate::FieldReader<u8, u8>);
impl DOUT6_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DOUT6_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT6_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT6_NUM` writer - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub struct DOUT6_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT6_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `DOUT7_NUM` reader - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub struct DOUT7_NUM_R(crate::FieldReader<u8, u8>);
impl DOUT7_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DOUT7_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT7_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT7_NUM` writer - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub struct DOUT7_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT7_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout0_num(&self) -> DOUT0_NUM_R {
        DOUT0_NUM_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout1_num(&self) -> DOUT1_NUM_R {
        DOUT1_NUM_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout2_num(&self) -> DOUT2_NUM_R {
        DOUT2_NUM_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout3_num(&self) -> DOUT3_NUM_R {
        DOUT3_NUM_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout4_num(&self) -> DOUT4_NUM_R {
        DOUT4_NUM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout5_num(&self) -> DOUT5_NUM_R {
        DOUT5_NUM_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout6_num(&self) -> DOUT6_NUM_R {
        DOUT6_NUM_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout7_num(&self) -> DOUT7_NUM_R {
        DOUT7_NUM_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout0_num(&mut self) -> DOUT0_NUM_W {
        DOUT0_NUM_W { w: self }
    }
    #[doc = "Bits 2:3 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout1_num(&mut self) -> DOUT1_NUM_W {
        DOUT1_NUM_W { w: self }
    }
    #[doc = "Bits 4:5 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout2_num(&mut self) -> DOUT2_NUM_W {
        DOUT2_NUM_W { w: self }
    }
    #[doc = "Bits 6:7 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout3_num(&mut self) -> DOUT3_NUM_W {
        DOUT3_NUM_W { w: self }
    }
    #[doc = "Bits 8:9 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout4_num(&mut self) -> DOUT4_NUM_W {
        DOUT4_NUM_W { w: self }
    }
    #[doc = "Bits 10:11 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout5_num(&mut self) -> DOUT5_NUM_W {
        DOUT5_NUM_W { w: self }
    }
    #[doc = "Bits 12:13 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout6_num(&mut self) -> DOUT6_NUM_W {
        DOUT6_NUM_W { w: self }
    }
    #[doc = "Bits 14:15 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout7_num(&mut self) -> DOUT7_NUM_W {
        DOUT7_NUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI output delay number configuration\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout_num]
(index.html) module"]
pub struct DOUT_NUM_SPEC;
impl crate::RegisterSpec for DOUT_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dout_num::R]
(R) reader structure"]
impl crate::Readable for DOUT_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dout_num::W]
(W) writer structure"]
impl crate::Writable for DOUT_NUM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOUT_NUM to value 0"]
impl crate::Resettable for DOUT_NUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
