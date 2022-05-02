#[doc = "Register `LCD_DATA_DOUT_MODE` reader"]
pub struct R(crate::R<LCD_DATA_DOUT_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_DATA_DOUT_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_DATA_DOUT_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_DATA_DOUT_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCD_DATA_DOUT_MODE` writer"]
pub struct W(crate::W<LCD_DATA_DOUT_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_DATA_DOUT_MODE_SPEC>;
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
impl From<crate::W<LCD_DATA_DOUT_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_DATA_DOUT_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUT0_MODE` reader - The output data bit 0 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT0_MODE_R(crate::FieldReader<u8>);
impl DOUT0_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DOUT0_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT0_MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT0_MODE` writer - The output data bit 0 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT0_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT0_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `DOUT1_MODE` reader - The output data bit 2 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT1_MODE_R(crate::FieldReader<u8>);
impl DOUT1_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DOUT1_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT1_MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT1_MODE` writer - The output data bit 2 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT1_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT1_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 2)) | ((value as u32 & 3) << 2);
        self.w
    }
}
#[doc = "Field `DOUT2_MODE` reader - The output data bit 4 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT2_MODE_R(crate::FieldReader<u8>);
impl DOUT2_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DOUT2_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT2_MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT2_MODE` writer - The output data bit 4 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT2_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT2_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "Field `DOUT3_MODE` reader - The output data bit 6 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT3_MODE_R(crate::FieldReader<u8>);
impl DOUT3_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DOUT3_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT3_MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT3_MODE` writer - The output data bit 6 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT3_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT3_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u32 & 3) << 6);
        self.w
    }
}
#[doc = "Field `DOUT4_MODE` reader - The output data bit 8 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT4_MODE_R(crate::FieldReader<u8>);
impl DOUT4_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DOUT4_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT4_MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT4_MODE` writer - The output data bit 8 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT4_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT4_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 8)) | ((value as u32 & 3) << 8);
        self.w
    }
}
#[doc = "Field `DOUT5_MODE` reader - The output data bit 10 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT5_MODE_R(crate::FieldReader<u8>);
impl DOUT5_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DOUT5_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT5_MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT5_MODE` writer - The output data bit 10 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT5_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT5_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 10)) | ((value as u32 & 3) << 10);
        self.w
    }
}
#[doc = "Field `DOUT6_MODE` reader - The output data bit 12 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT6_MODE_R(crate::FieldReader<u8>);
impl DOUT6_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DOUT6_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT6_MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT6_MODE` writer - The output data bit 12 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT6_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT6_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 12)) | ((value as u32 & 3) << 12);
        self.w
    }
}
#[doc = "Field `DOUT7_MODE` reader - The output data bit 14 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT7_MODE_R(crate::FieldReader<u8>);
impl DOUT7_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DOUT7_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT7_MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT7_MODE` writer - The output data bit 14 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT7_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT7_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 14)) | ((value as u32 & 3) << 14);
        self.w
    }
}
#[doc = "Field `DOUT8_MODE` reader - The output data bit 16 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT8_MODE_R(crate::FieldReader<u8>);
impl DOUT8_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DOUT8_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT8_MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT8_MODE` writer - The output data bit 16 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT8_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT8_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 16)) | ((value as u32 & 3) << 16);
        self.w
    }
}
#[doc = "Field `DOUT9_MODE` reader - The output data bit 18 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT9_MODE_R(crate::FieldReader<u8>);
impl DOUT9_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DOUT9_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT9_MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT9_MODE` writer - The output data bit 18 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT9_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT9_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 18)) | ((value as u32 & 3) << 18);
        self.w
    }
}
#[doc = "Field `DOUT10_MODE` reader - The output data bit 20 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT10_MODE_R(crate::FieldReader<u8>);
impl DOUT10_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DOUT10_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT10_MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT10_MODE` writer - The output data bit 20 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT10_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT10_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 20)) | ((value as u32 & 3) << 20);
        self.w
    }
}
#[doc = "Field `DOUT11_MODE` reader - The output data bit 22 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT11_MODE_R(crate::FieldReader<u8>);
impl DOUT11_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DOUT11_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT11_MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT11_MODE` writer - The output data bit 22 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT11_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT11_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 22)) | ((value as u32 & 3) << 22);
        self.w
    }
}
#[doc = "Field `DOUT12_MODE` reader - The output data bit 24 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT12_MODE_R(crate::FieldReader<u8>);
impl DOUT12_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DOUT12_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT12_MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT12_MODE` writer - The output data bit 24 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT12_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT12_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 24)) | ((value as u32 & 3) << 24);
        self.w
    }
}
#[doc = "Field `DOUT13_MODE` reader - The output data bit 26 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT13_MODE_R(crate::FieldReader<u8>);
impl DOUT13_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DOUT13_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT13_MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT13_MODE` writer - The output data bit 26 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT13_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT13_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 26)) | ((value as u32 & 3) << 26);
        self.w
    }
}
#[doc = "Field `DOUT14_MODE` reader - The output data bit 28 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT14_MODE_R(crate::FieldReader<u8>);
impl DOUT14_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DOUT14_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT14_MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT14_MODE` writer - The output data bit 28 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT14_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT14_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 28)) | ((value as u32 & 3) << 28);
        self.w
    }
}
#[doc = "Field `DOUT15_MODE` reader - The output data bit 30 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT15_MODE_R(crate::FieldReader<u8>);
impl DOUT15_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DOUT15_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT15_MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT15_MODE` writer - The output data bit 30 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct DOUT15_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT15_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 30)) | ((value as u32 & 3) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - The output data bit 0 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout0_mode(&self) -> DOUT0_MODE_R {
        DOUT0_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - The output data bit 2 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout1_mode(&self) -> DOUT1_MODE_R {
        DOUT1_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - The output data bit 4 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout2_mode(&self) -> DOUT2_MODE_R {
        DOUT2_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - The output data bit 6 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout3_mode(&self) -> DOUT3_MODE_R {
        DOUT3_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - The output data bit 8 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout4_mode(&self) -> DOUT4_MODE_R {
        DOUT4_MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - The output data bit 10 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout5_mode(&self) -> DOUT5_MODE_R {
        DOUT5_MODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - The output data bit 12 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout6_mode(&self) -> DOUT6_MODE_R {
        DOUT6_MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - The output data bit 14 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout7_mode(&self) -> DOUT7_MODE_R {
        DOUT7_MODE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - The output data bit 16 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout8_mode(&self) -> DOUT8_MODE_R {
        DOUT8_MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - The output data bit 18 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout9_mode(&self) -> DOUT9_MODE_R {
        DOUT9_MODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - The output data bit 20 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout10_mode(&self) -> DOUT10_MODE_R {
        DOUT10_MODE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - The output data bit 22 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout11_mode(&self) -> DOUT11_MODE_R {
        DOUT11_MODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - The output data bit 24 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout12_mode(&self) -> DOUT12_MODE_R {
        DOUT12_MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - The output data bit 26 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout13_mode(&self) -> DOUT13_MODE_R {
        DOUT13_MODE_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - The output data bit 28 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout14_mode(&self) -> DOUT14_MODE_R {
        DOUT14_MODE_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - The output data bit 30 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout15_mode(&self) -> DOUT15_MODE_R {
        DOUT15_MODE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - The output data bit 0 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout0_mode(&mut self) -> DOUT0_MODE_W {
        DOUT0_MODE_W { w: self }
    }
    #[doc = "Bits 2:3 - The output data bit 2 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout1_mode(&mut self) -> DOUT1_MODE_W {
        DOUT1_MODE_W { w: self }
    }
    #[doc = "Bits 4:5 - The output data bit 4 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout2_mode(&mut self) -> DOUT2_MODE_W {
        DOUT2_MODE_W { w: self }
    }
    #[doc = "Bits 6:7 - The output data bit 6 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout3_mode(&mut self) -> DOUT3_MODE_W {
        DOUT3_MODE_W { w: self }
    }
    #[doc = "Bits 8:9 - The output data bit 8 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout4_mode(&mut self) -> DOUT4_MODE_W {
        DOUT4_MODE_W { w: self }
    }
    #[doc = "Bits 10:11 - The output data bit 10 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout5_mode(&mut self) -> DOUT5_MODE_W {
        DOUT5_MODE_W { w: self }
    }
    #[doc = "Bits 12:13 - The output data bit 12 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout6_mode(&mut self) -> DOUT6_MODE_W {
        DOUT6_MODE_W { w: self }
    }
    #[doc = "Bits 14:15 - The output data bit 14 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout7_mode(&mut self) -> DOUT7_MODE_W {
        DOUT7_MODE_W { w: self }
    }
    #[doc = "Bits 16:17 - The output data bit 16 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout8_mode(&mut self) -> DOUT8_MODE_W {
        DOUT8_MODE_W { w: self }
    }
    #[doc = "Bits 18:19 - The output data bit 18 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout9_mode(&mut self) -> DOUT9_MODE_W {
        DOUT9_MODE_W { w: self }
    }
    #[doc = "Bits 20:21 - The output data bit 20 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout10_mode(&mut self) -> DOUT10_MODE_W {
        DOUT10_MODE_W { w: self }
    }
    #[doc = "Bits 22:23 - The output data bit 22 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout11_mode(&mut self) -> DOUT11_MODE_W {
        DOUT11_MODE_W { w: self }
    }
    #[doc = "Bits 24:25 - The output data bit 24 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout12_mode(&mut self) -> DOUT12_MODE_W {
        DOUT12_MODE_W { w: self }
    }
    #[doc = "Bits 26:27 - The output data bit 26 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout13_mode(&mut self) -> DOUT13_MODE_W {
        DOUT13_MODE_W { w: self }
    }
    #[doc = "Bits 28:29 - The output data bit 28 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout14_mode(&mut self) -> DOUT14_MODE_W {
        DOUT14_MODE_W { w: self }
    }
    #[doc = "Bits 30:31 - The output data bit 30 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout15_mode(&mut self) -> DOUT15_MODE_W {
        DOUT15_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_data_dout_mode](index.html) module"]
pub struct LCD_DATA_DOUT_MODE_SPEC;
impl crate::RegisterSpec for LCD_DATA_DOUT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_data_dout_mode::R](R) reader structure"]
impl crate::Readable for LCD_DATA_DOUT_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_data_dout_mode::W](W) writer structure"]
impl crate::Writable for LCD_DATA_DOUT_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCD_DATA_DOUT_MODE to value 0"]
impl crate::Resettable for LCD_DATA_DOUT_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
