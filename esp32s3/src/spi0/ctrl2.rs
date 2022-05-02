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
#[doc = "Field `CS_SETUP_TIME` reader - (cycles-1) of PREP phase by SPI_CLK, which is the SPI_CS setup time. These bits are combined with SPI_MEM_CS_SETUP bit."]
pub struct CS_SETUP_TIME_R(crate::FieldReader<u8>);
impl CS_SETUP_TIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CS_SETUP_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_SETUP_TIME_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS_SETUP_TIME` writer - (cycles-1) of PREP phase by SPI_CLK, which is the SPI_CS setup time. These bits are combined with SPI_MEM_CS_SETUP bit."]
pub struct CS_SETUP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_SETUP_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `CS_HOLD_TIME` reader - SPI Bus CS (SPI_CS) signal is delayed to inactive by SPI Bus clock (SPI_CLK), which is the SPI_CS hold time in non-ECC mode. These bits are combined with SPI_MEM_CS_HOLD bit."]
pub struct CS_HOLD_TIME_R(crate::FieldReader<u8>);
impl CS_HOLD_TIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CS_HOLD_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_HOLD_TIME_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS_HOLD_TIME` writer - SPI Bus CS (SPI_CS) signal is delayed to inactive by SPI Bus clock (SPI_CLK), which is the SPI_CS hold time in non-ECC mode. These bits are combined with SPI_MEM_CS_HOLD bit."]
pub struct CS_HOLD_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_HOLD_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | ((value as u32 & 0x1f) << 5);
        self.w
    }
}
#[doc = "Field `ECC_CS_HOLD_TIME` reader - SPI_MEM_CS_HOLD_TIME + SPI_MEM_ECC_CS_HOLD_TIME is the SPI_CS hold cycle in ECC mode when accessed flash."]
pub struct ECC_CS_HOLD_TIME_R(crate::FieldReader<u8>);
impl ECC_CS_HOLD_TIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ECC_CS_HOLD_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECC_CS_HOLD_TIME_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECC_CS_HOLD_TIME` writer - SPI_MEM_CS_HOLD_TIME + SPI_MEM_ECC_CS_HOLD_TIME is the SPI_CS hold cycle in ECC mode when accessed flash."]
pub struct ECC_CS_HOLD_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> ECC_CS_HOLD_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 10)) | ((value as u32 & 7) << 10);
        self.w
    }
}
#[doc = "Field `ECC_SKIP_PAGE_CORNER` reader - 1: MSPI skips page corner when accesses flash. 0: Not skip page corner when accesses flash."]
pub struct ECC_SKIP_PAGE_CORNER_R(crate::FieldReader<bool>);
impl ECC_SKIP_PAGE_CORNER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECC_SKIP_PAGE_CORNER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECC_SKIP_PAGE_CORNER_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECC_SKIP_PAGE_CORNER` writer - 1: MSPI skips page corner when accesses flash. 0: Not skip page corner when accesses flash."]
pub struct ECC_SKIP_PAGE_CORNER_W<'a> {
    w: &'a mut W,
}
impl<'a> ECC_SKIP_PAGE_CORNER_W<'a> {
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
#[doc = "Field `ECC_16TO18_BYTE_EN` reader - Set this bit to enable MSPI ECC 16 bytes data with 2 ECC bytes mode when accesses flash."]
pub struct ECC_16TO18_BYTE_EN_R(crate::FieldReader<bool>);
impl ECC_16TO18_BYTE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECC_16TO18_BYTE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECC_16TO18_BYTE_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECC_16TO18_BYTE_EN` writer - Set this bit to enable MSPI ECC 16 bytes data with 2 ECC bytes mode when accesses flash."]
pub struct ECC_16TO18_BYTE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ECC_16TO18_BYTE_EN_W<'a> {
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
#[doc = "Field `CS_HOLD_DELAY` reader - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
pub struct CS_HOLD_DELAY_R(crate::FieldReader<u8>);
impl CS_HOLD_DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CS_HOLD_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_HOLD_DELAY_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS_HOLD_DELAY` writer - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
pub struct CS_HOLD_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_HOLD_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 25)) | ((value as u32 & 0x3f) << 25);
        self.w
    }
}
#[doc = "Field `SYNC_RESET` reader - The FSM will be reset."]
pub struct SYNC_RESET_R(crate::FieldReader<bool>);
impl SYNC_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYNC_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNC_RESET_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNC_RESET` writer - The FSM will be reset."]
pub struct SYNC_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - (cycles-1) of PREP phase by SPI_CLK, which is the SPI_CS setup time. These bits are combined with SPI_MEM_CS_SETUP bit."]
    #[inline(always)]
    pub fn cs_setup_time(&self) -> CS_SETUP_TIME_R {
        CS_SETUP_TIME_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - SPI Bus CS (SPI_CS) signal is delayed to inactive by SPI Bus clock (SPI_CLK), which is the SPI_CS hold time in non-ECC mode. These bits are combined with SPI_MEM_CS_HOLD bit."]
    #[inline(always)]
    pub fn cs_hold_time(&self) -> CS_HOLD_TIME_R {
        CS_HOLD_TIME_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:12 - SPI_MEM_CS_HOLD_TIME + SPI_MEM_ECC_CS_HOLD_TIME is the SPI_CS hold cycle in ECC mode when accessed flash."]
    #[inline(always)]
    pub fn ecc_cs_hold_time(&self) -> ECC_CS_HOLD_TIME_R {
        ECC_CS_HOLD_TIME_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 13 - 1: MSPI skips page corner when accesses flash. 0: Not skip page corner when accesses flash."]
    #[inline(always)]
    pub fn ecc_skip_page_corner(&self) -> ECC_SKIP_PAGE_CORNER_R {
        ECC_SKIP_PAGE_CORNER_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set this bit to enable MSPI ECC 16 bytes data with 2 ECC bytes mode when accesses flash."]
    #[inline(always)]
    pub fn ecc_16to18_byte_en(&self) -> ECC_16TO18_BYTE_EN_R {
        ECC_16TO18_BYTE_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 25:30 - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
    #[inline(always)]
    pub fn cs_hold_delay(&self) -> CS_HOLD_DELAY_R {
        CS_HOLD_DELAY_R::new(((self.bits >> 25) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - The FSM will be reset."]
    #[inline(always)]
    pub fn sync_reset(&self) -> SYNC_RESET_R {
        SYNC_RESET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - (cycles-1) of PREP phase by SPI_CLK, which is the SPI_CS setup time. These bits are combined with SPI_MEM_CS_SETUP bit."]
    #[inline(always)]
    pub fn cs_setup_time(&mut self) -> CS_SETUP_TIME_W {
        CS_SETUP_TIME_W { w: self }
    }
    #[doc = "Bits 5:9 - SPI Bus CS (SPI_CS) signal is delayed to inactive by SPI Bus clock (SPI_CLK), which is the SPI_CS hold time in non-ECC mode. These bits are combined with SPI_MEM_CS_HOLD bit."]
    #[inline(always)]
    pub fn cs_hold_time(&mut self) -> CS_HOLD_TIME_W {
        CS_HOLD_TIME_W { w: self }
    }
    #[doc = "Bits 10:12 - SPI_MEM_CS_HOLD_TIME + SPI_MEM_ECC_CS_HOLD_TIME is the SPI_CS hold cycle in ECC mode when accessed flash."]
    #[inline(always)]
    pub fn ecc_cs_hold_time(&mut self) -> ECC_CS_HOLD_TIME_W {
        ECC_CS_HOLD_TIME_W { w: self }
    }
    #[doc = "Bit 13 - 1: MSPI skips page corner when accesses flash. 0: Not skip page corner when accesses flash."]
    #[inline(always)]
    pub fn ecc_skip_page_corner(&mut self) -> ECC_SKIP_PAGE_CORNER_W {
        ECC_SKIP_PAGE_CORNER_W { w: self }
    }
    #[doc = "Bit 14 - Set this bit to enable MSPI ECC 16 bytes data with 2 ECC bytes mode when accesses flash."]
    #[inline(always)]
    pub fn ecc_16to18_byte_en(&mut self) -> ECC_16TO18_BYTE_EN_W {
        ECC_16TO18_BYTE_EN_W { w: self }
    }
    #[doc = "Bits 25:30 - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
    #[inline(always)]
    pub fn cs_hold_delay(&mut self) -> CS_HOLD_DELAY_W {
        CS_HOLD_DELAY_W { w: self }
    }
    #[doc = "Bit 31 - The FSM will be reset."]
    #[inline(always)]
    pub fn sync_reset(&mut self) -> SYNC_RESET_W {
        SYNC_RESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 control 2 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2](index.html) module"]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl2::R](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl2::W](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL2 to value 0x2c21"]
impl crate::Resettable for CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2c21
    }
}
