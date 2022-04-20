#[doc = "Register `CTRL2` reader"]
pub struct R(crate::R<CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL2` writer"]
pub struct W(crate::W<CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL2_SPEC>;
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
impl From<crate::W<CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS_SETUP_TIME` reader - (cycles+1) of prepare phase by spi clock this bits are combined with SPI_CS_SETUP bit. Can be configured in CONF state."]
pub struct CS_SETUP_TIME_R(crate::FieldReader<u16, u16>);
impl CS_SETUP_TIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CS_SETUP_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_SETUP_TIME_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS_SETUP_TIME` writer - (cycles+1) of prepare phase by spi clock this bits are combined with SPI_CS_SETUP bit. Can be configured in CONF state."]
pub struct CS_SETUP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_SETUP_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | (value as u32 & 0x1fff);
        self.w
    }
}
#[doc = "Field `CS_HOLD_TIME` reader - delay cycles of cs pin by spi clock this bits are combined with SPI_CS_HOLD bit. Can be configured in CONF state."]
pub struct CS_HOLD_TIME_R(crate::FieldReader<u16, u16>);
impl CS_HOLD_TIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CS_HOLD_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_HOLD_TIME_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS_HOLD_TIME` writer - delay cycles of cs pin by spi clock this bits are combined with SPI_CS_HOLD bit. Can be configured in CONF state."]
pub struct CS_HOLD_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_HOLD_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 13)) | ((value as u32 & 0x1fff) << 13);
        self.w
    }
}
#[doc = "Field `CS_DELAY_MODE` reader - spi_cs signal is delayed by spi_clk . 0: zero 1: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by half cycle else delayed by one cycle 2: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by one cycle, else delayed by half cycle 3: delayed one cycle. Can be configured in CONF state."]
pub struct CS_DELAY_MODE_R(crate::FieldReader<u8, u8>);
impl CS_DELAY_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CS_DELAY_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_DELAY_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS_DELAY_MODE` writer - spi_cs signal is delayed by spi_clk . 0: zero 1: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by half cycle else delayed by one cycle 2: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by one cycle, else delayed by half cycle 3: delayed one cycle. Can be configured in CONF state."]
pub struct CS_DELAY_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_DELAY_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 26)) | ((value as u32 & 7) << 26);
        self.w
    }
}
#[doc = "Field `CS_DELAY_NUM` reader - spi_cs signal is delayed by system clock cycles. Can be configured in CONF state."]
pub struct CS_DELAY_NUM_R(crate::FieldReader<u8, u8>);
impl CS_DELAY_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CS_DELAY_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_DELAY_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS_DELAY_NUM` writer - spi_cs signal is delayed by system clock cycles. Can be configured in CONF state."]
pub struct CS_DELAY_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_DELAY_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 29)) | ((value as u32 & 3) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12 - (cycles+1) of prepare phase by spi clock this bits are combined with SPI_CS_SETUP bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_setup_time(&self) -> CS_SETUP_TIME_R {
        CS_SETUP_TIME_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:25 - delay cycles of cs pin by spi clock this bits are combined with SPI_CS_HOLD bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_hold_time(&self) -> CS_HOLD_TIME_R {
        CS_HOLD_TIME_R::new(((self.bits >> 13) & 0x1fff) as u16)
    }
    #[doc = "Bits 26:28 - spi_cs signal is delayed by spi_clk . 0: zero 1: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by half cycle else delayed by one cycle 2: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by one cycle, else delayed by half cycle 3: delayed one cycle. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_delay_mode(&self) -> CS_DELAY_MODE_R {
        CS_DELAY_MODE_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:30 - spi_cs signal is delayed by system clock cycles. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_delay_num(&self) -> CS_DELAY_NUM_R {
        CS_DELAY_NUM_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:12 - (cycles+1) of prepare phase by spi clock this bits are combined with SPI_CS_SETUP bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_setup_time(&mut self) -> CS_SETUP_TIME_W {
        CS_SETUP_TIME_W { w: self }
    }
    #[doc = "Bits 13:25 - delay cycles of cs pin by spi clock this bits are combined with SPI_CS_HOLD bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_hold_time(&mut self) -> CS_HOLD_TIME_W {
        CS_HOLD_TIME_W { w: self }
    }
    #[doc = "Bits 26:28 - spi_cs signal is delayed by spi_clk . 0: zero 1: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by half cycle else delayed by one cycle 2: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by one cycle, else delayed by half cycle 3: delayed one cycle. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_delay_mode(&mut self) -> CS_DELAY_MODE_W {
        CS_DELAY_MODE_W { w: self }
    }
    #[doc = "Bits 29:30 - spi_cs signal is delayed by system clock cycles. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_delay_num(&mut self) -> CS_DELAY_NUM_W {
        CS_DELAY_NUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI control register 2\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2]
(index.html) module"]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl2::R]
(R) reader structure"]
impl crate::Readable for CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl2::W]
(W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL2 to value 0x2000"]
impl crate::Resettable for CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000
    }
}
