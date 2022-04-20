#[doc = "Register `USER1` reader"]
pub struct R(crate::R<USER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USER1` writer"]
pub struct W(crate::W<USER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USER1_SPEC>;
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
impl From<crate::W<USER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USR_DUMMY_CYCLELEN` reader - The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state."]
pub struct USR_DUMMY_CYCLELEN_R(crate::FieldReader<u8, u8>);
impl USR_DUMMY_CYCLELEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USR_DUMMY_CYCLELEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_DUMMY_CYCLELEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_DUMMY_CYCLELEN` writer - The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state."]
pub struct USR_DUMMY_CYCLELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_DUMMY_CYCLELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `MST_WFULL_ERR_END_EN` reader - 1: SPI transfer is ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode."]
pub struct MST_WFULL_ERR_END_EN_R(crate::FieldReader<bool, bool>);
impl MST_WFULL_ERR_END_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MST_WFULL_ERR_END_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MST_WFULL_ERR_END_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MST_WFULL_ERR_END_EN` writer - 1: SPI transfer is ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode."]
pub struct MST_WFULL_ERR_END_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MST_WFULL_ERR_END_EN_W<'a> {
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
#[doc = "Field `CS_SETUP_TIME` reader - (cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state."]
pub struct CS_SETUP_TIME_R(crate::FieldReader<u8, u8>);
impl CS_SETUP_TIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CS_SETUP_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_SETUP_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS_SETUP_TIME` writer - (cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state."]
pub struct CS_SETUP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_SETUP_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 17)) | ((value as u32 & 0x1f) << 17);
        self.w
    }
}
#[doc = "Field `CS_HOLD_TIME` reader - delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state."]
pub struct CS_HOLD_TIME_R(crate::FieldReader<u8, u8>);
impl CS_HOLD_TIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CS_HOLD_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_HOLD_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS_HOLD_TIME` writer - delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state."]
pub struct CS_HOLD_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_HOLD_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 22)) | ((value as u32 & 0x1f) << 22);
        self.w
    }
}
#[doc = "Field `USR_ADDR_BITLEN` reader - The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state."]
pub struct USR_ADDR_BITLEN_R(crate::FieldReader<u8, u8>);
impl USR_ADDR_BITLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USR_ADDR_BITLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_ADDR_BITLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_ADDR_BITLEN` writer - The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state."]
pub struct USR_ADDR_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_ADDR_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | ((value as u32 & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_dummy_cyclelen(&self) -> USR_DUMMY_CYCLELEN_R {
        USR_DUMMY_CYCLELEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - 1: SPI transfer is ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode."]
    #[inline(always)]
    pub fn mst_wfull_err_end_en(&self) -> MST_WFULL_ERR_END_EN_R {
        MST_WFULL_ERR_END_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21 - (cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_setup_time(&self) -> CS_SETUP_TIME_R {
        CS_SETUP_TIME_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:26 - delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_hold_time(&self) -> CS_HOLD_TIME_R {
        CS_HOLD_TIME_R::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_addr_bitlen(&self) -> USR_ADDR_BITLEN_R {
        USR_ADDR_BITLEN_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_dummy_cyclelen(&mut self) -> USR_DUMMY_CYCLELEN_W {
        USR_DUMMY_CYCLELEN_W { w: self }
    }
    #[doc = "Bit 16 - 1: SPI transfer is ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode."]
    #[inline(always)]
    pub fn mst_wfull_err_end_en(&mut self) -> MST_WFULL_ERR_END_EN_W {
        MST_WFULL_ERR_END_EN_W { w: self }
    }
    #[doc = "Bits 17:21 - (cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_setup_time(&mut self) -> CS_SETUP_TIME_W {
        CS_SETUP_TIME_W { w: self }
    }
    #[doc = "Bits 22:26 - delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_hold_time(&mut self) -> CS_HOLD_TIME_W {
        CS_HOLD_TIME_W { w: self }
    }
    #[doc = "Bits 27:31 - The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_addr_bitlen(&mut self) -> USR_ADDR_BITLEN_W {
        USR_ADDR_BITLEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI USER control register 1\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [user1]
(index.html) module"]
pub struct USER1_SPEC;
impl crate::RegisterSpec for USER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [user1::R]
(R) reader structure"]
impl crate::Readable for USER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [user1::W]
(W) writer structure"]
impl crate::Writable for USER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USER1 to value 0xb841_0007"]
impl crate::Resettable for USER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb841_0007
    }
}
