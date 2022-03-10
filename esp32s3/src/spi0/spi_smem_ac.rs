#[doc = "Register `SPI_SMEM_AC` reader"]
pub struct R(crate::R<SPI_SMEM_AC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_SMEM_AC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_SMEM_AC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_SMEM_AC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_SMEM_AC` writer"]
pub struct W(crate::W<SPI_SMEM_AC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_SMEM_AC_SPEC>;
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
impl From<crate::W<SPI_SMEM_AC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_SMEM_AC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_SMEM_CS_SETUP` reader - Set this bit to keep SPI_CS low when MSPI is in PREP state."]
pub struct SPI_SMEM_CS_SETUP_R(crate::FieldReader<bool, bool>);
impl SPI_SMEM_CS_SETUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_SMEM_CS_SETUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_CS_SETUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_CS_SETUP` writer - Set this bit to keep SPI_CS low when MSPI is in PREP state."]
pub struct SPI_SMEM_CS_SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_CS_SETUP_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `SPI_SMEM_CS_HOLD` reader - Set this bit to keep SPI_CS low when MSPI is in DONE state."]
pub struct SPI_SMEM_CS_HOLD_R(crate::FieldReader<bool, bool>);
impl SPI_SMEM_CS_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_SMEM_CS_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_CS_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_CS_HOLD` writer - Set this bit to keep SPI_CS low when MSPI is in DONE state."]
pub struct SPI_SMEM_CS_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_CS_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `SPI_SMEM_CS_SETUP_TIME` reader - (cycles-1) of PREP phase by SPI_CLK, which is the SPI_CS setup time. These bits are combined with SPI_MEM_CS_SETUP bit."]
pub struct SPI_SMEM_CS_SETUP_TIME_R(crate::FieldReader<u8, u8>);
impl SPI_SMEM_CS_SETUP_TIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_SMEM_CS_SETUP_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_CS_SETUP_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_CS_SETUP_TIME` writer - (cycles-1) of PREP phase by SPI_CLK, which is the SPI_CS setup time. These bits are combined with SPI_MEM_CS_SETUP bit."]
pub struct SPI_SMEM_CS_SETUP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_CS_SETUP_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 2)) | ((value as u32 & 0x1f) << 2);
        self.w
    }
}
#[doc = "Field `SPI_SMEM_CS_HOLD_TIME` reader - SPI Bus CS (SPI_CS) signal is delayed to inactive by SPI Bus clock (SPI_CLK), which is the SPI_CS hold time in non-ECC mode. These bits are combined with SPI_MEM_CS_HOLD bit."]
pub struct SPI_SMEM_CS_HOLD_TIME_R(crate::FieldReader<u8, u8>);
impl SPI_SMEM_CS_HOLD_TIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_SMEM_CS_HOLD_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_CS_HOLD_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_CS_HOLD_TIME` writer - SPI Bus CS (SPI_CS) signal is delayed to inactive by SPI Bus clock (SPI_CLK), which is the SPI_CS hold time in non-ECC mode. These bits are combined with SPI_MEM_CS_HOLD bit."]
pub struct SPI_SMEM_CS_HOLD_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_CS_HOLD_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 7)) | ((value as u32 & 0x1f) << 7);
        self.w
    }
}
#[doc = "Field `SPI_SMEM_ECC_CS_HOLD_TIME` reader - SPI_SMEM_CS_HOLD_TIME + SPI_SMEM_ECC_CS_HOLD_TIME is the MSPI CS hold cycles in ECC mode when accesses to external RAM."]
pub struct SPI_SMEM_ECC_CS_HOLD_TIME_R(crate::FieldReader<u8, u8>);
impl SPI_SMEM_ECC_CS_HOLD_TIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_SMEM_ECC_CS_HOLD_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_ECC_CS_HOLD_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_ECC_CS_HOLD_TIME` writer - SPI_SMEM_CS_HOLD_TIME + SPI_SMEM_ECC_CS_HOLD_TIME is the MSPI CS hold cycles in ECC mode when accesses to external RAM."]
pub struct SPI_SMEM_ECC_CS_HOLD_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_ECC_CS_HOLD_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `SPI_SMEM_ECC_SKIP_PAGE_CORNER` reader - 1: MSPI skips page corner when accesses to external RAM. 0: Not skip page corner when accesses to external RAM."]
pub struct SPI_SMEM_ECC_SKIP_PAGE_CORNER_R(crate::FieldReader<bool, bool>);
impl SPI_SMEM_ECC_SKIP_PAGE_CORNER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_SMEM_ECC_SKIP_PAGE_CORNER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_ECC_SKIP_PAGE_CORNER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_ECC_SKIP_PAGE_CORNER` writer - 1: MSPI skips page corner when accesses to external RAM. 0: Not skip page corner when accesses to external RAM."]
pub struct SPI_SMEM_ECC_SKIP_PAGE_CORNER_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_ECC_SKIP_PAGE_CORNER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `SPI_SMEM_ECC_16TO18_BYTE_EN` reader - Set this bit to enable MSPI ECC 16 bytes data with 2 ECC bytes mode when accesses to external RAM."]
pub struct SPI_SMEM_ECC_16TO18_BYTE_EN_R(crate::FieldReader<bool, bool>);
impl SPI_SMEM_ECC_16TO18_BYTE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_SMEM_ECC_16TO18_BYTE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_ECC_16TO18_BYTE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_ECC_16TO18_BYTE_EN` writer - Set this bit to enable MSPI ECC 16 bytes data with 2 ECC bytes mode when accesses to external RAM."]
pub struct SPI_SMEM_ECC_16TO18_BYTE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_ECC_16TO18_BYTE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `SPI_SMEM_ECC_ERR_INT_EN` reader - Set this bit to calculate the error times of MSPI ECC read when accesses to external RAM."]
pub struct SPI_SMEM_ECC_ERR_INT_EN_R(crate::FieldReader<bool, bool>);
impl SPI_SMEM_ECC_ERR_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_SMEM_ECC_ERR_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_ECC_ERR_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_ECC_ERR_INT_EN` writer - Set this bit to calculate the error times of MSPI ECC read when accesses to external RAM."]
pub struct SPI_SMEM_ECC_ERR_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_ECC_ERR_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `SPI_SMEM_CS_HOLD_DELAY` reader - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to external RAM. tSHSL is (SPI_SMEM_CS_HOLD_DELAY\\[5:0\\]
 + 1) MSPI core clock cycles."]
pub struct SPI_SMEM_CS_HOLD_DELAY_R(crate::FieldReader<u8, u8>);
impl SPI_SMEM_CS_HOLD_DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_SMEM_CS_HOLD_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_CS_HOLD_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_CS_HOLD_DELAY` writer - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to external RAM. tSHSL is (SPI_SMEM_CS_HOLD_DELAY\\[5:0\\]
 + 1) MSPI core clock cycles."]
pub struct SPI_SMEM_CS_HOLD_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_CS_HOLD_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 25)) | ((value as u32 & 0x3f) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set this bit to keep SPI_CS low when MSPI is in PREP state."]
    #[inline(always)]
    pub fn spi_smem_cs_setup(&self) -> SPI_SMEM_CS_SETUP_R {
        SPI_SMEM_CS_SETUP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set this bit to keep SPI_CS low when MSPI is in DONE state."]
    #[inline(always)]
    pub fn spi_smem_cs_hold(&self) -> SPI_SMEM_CS_HOLD_R {
        SPI_SMEM_CS_HOLD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:6 - (cycles-1) of PREP phase by SPI_CLK, which is the SPI_CS setup time. These bits are combined with SPI_MEM_CS_SETUP bit."]
    #[inline(always)]
    pub fn spi_smem_cs_setup_time(&self) -> SPI_SMEM_CS_SETUP_TIME_R {
        SPI_SMEM_CS_SETUP_TIME_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 7:11 - SPI Bus CS (SPI_CS) signal is delayed to inactive by SPI Bus clock (SPI_CLK), which is the SPI_CS hold time in non-ECC mode. These bits are combined with SPI_MEM_CS_HOLD bit."]
    #[inline(always)]
    pub fn spi_smem_cs_hold_time(&self) -> SPI_SMEM_CS_HOLD_TIME_R {
        SPI_SMEM_CS_HOLD_TIME_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:14 - SPI_SMEM_CS_HOLD_TIME + SPI_SMEM_ECC_CS_HOLD_TIME is the MSPI CS hold cycles in ECC mode when accesses to external RAM."]
    #[inline(always)]
    pub fn spi_smem_ecc_cs_hold_time(&self) -> SPI_SMEM_ECC_CS_HOLD_TIME_R {
        SPI_SMEM_ECC_CS_HOLD_TIME_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - 1: MSPI skips page corner when accesses to external RAM. 0: Not skip page corner when accesses to external RAM."]
    #[inline(always)]
    pub fn spi_smem_ecc_skip_page_corner(&self) -> SPI_SMEM_ECC_SKIP_PAGE_CORNER_R {
        SPI_SMEM_ECC_SKIP_PAGE_CORNER_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Set this bit to enable MSPI ECC 16 bytes data with 2 ECC bytes mode when accesses to external RAM."]
    #[inline(always)]
    pub fn spi_smem_ecc_16to18_byte_en(&self) -> SPI_SMEM_ECC_16TO18_BYTE_EN_R {
        SPI_SMEM_ECC_16TO18_BYTE_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Set this bit to calculate the error times of MSPI ECC read when accesses to external RAM."]
    #[inline(always)]
    pub fn spi_smem_ecc_err_int_en(&self) -> SPI_SMEM_ECC_ERR_INT_EN_R {
        SPI_SMEM_ECC_ERR_INT_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:30 - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to external RAM. tSHSL is (SPI_SMEM_CS_HOLD_DELAY\\[5:0\\]
 + 1) MSPI core clock cycles."]
    #[inline(always)]
    pub fn spi_smem_cs_hold_delay(&self) -> SPI_SMEM_CS_HOLD_DELAY_R {
        SPI_SMEM_CS_HOLD_DELAY_R::new(((self.bits >> 25) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to keep SPI_CS low when MSPI is in PREP state."]
    #[inline(always)]
    pub fn spi_smem_cs_setup(&mut self) -> SPI_SMEM_CS_SETUP_W {
        SPI_SMEM_CS_SETUP_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to keep SPI_CS low when MSPI is in DONE state."]
    #[inline(always)]
    pub fn spi_smem_cs_hold(&mut self) -> SPI_SMEM_CS_HOLD_W {
        SPI_SMEM_CS_HOLD_W { w: self }
    }
    #[doc = "Bits 2:6 - (cycles-1) of PREP phase by SPI_CLK, which is the SPI_CS setup time. These bits are combined with SPI_MEM_CS_SETUP bit."]
    #[inline(always)]
    pub fn spi_smem_cs_setup_time(&mut self) -> SPI_SMEM_CS_SETUP_TIME_W {
        SPI_SMEM_CS_SETUP_TIME_W { w: self }
    }
    #[doc = "Bits 7:11 - SPI Bus CS (SPI_CS) signal is delayed to inactive by SPI Bus clock (SPI_CLK), which is the SPI_CS hold time in non-ECC mode. These bits are combined with SPI_MEM_CS_HOLD bit."]
    #[inline(always)]
    pub fn spi_smem_cs_hold_time(&mut self) -> SPI_SMEM_CS_HOLD_TIME_W {
        SPI_SMEM_CS_HOLD_TIME_W { w: self }
    }
    #[doc = "Bits 12:14 - SPI_SMEM_CS_HOLD_TIME + SPI_SMEM_ECC_CS_HOLD_TIME is the MSPI CS hold cycles in ECC mode when accesses to external RAM."]
    #[inline(always)]
    pub fn spi_smem_ecc_cs_hold_time(&mut self) -> SPI_SMEM_ECC_CS_HOLD_TIME_W {
        SPI_SMEM_ECC_CS_HOLD_TIME_W { w: self }
    }
    #[doc = "Bit 15 - 1: MSPI skips page corner when accesses to external RAM. 0: Not skip page corner when accesses to external RAM."]
    #[inline(always)]
    pub fn spi_smem_ecc_skip_page_corner(&mut self) -> SPI_SMEM_ECC_SKIP_PAGE_CORNER_W {
        SPI_SMEM_ECC_SKIP_PAGE_CORNER_W { w: self }
    }
    #[doc = "Bit 16 - Set this bit to enable MSPI ECC 16 bytes data with 2 ECC bytes mode when accesses to external RAM."]
    #[inline(always)]
    pub fn spi_smem_ecc_16to18_byte_en(&mut self) -> SPI_SMEM_ECC_16TO18_BYTE_EN_W {
        SPI_SMEM_ECC_16TO18_BYTE_EN_W { w: self }
    }
    #[doc = "Bit 24 - Set this bit to calculate the error times of MSPI ECC read when accesses to external RAM."]
    #[inline(always)]
    pub fn spi_smem_ecc_err_int_en(&mut self) -> SPI_SMEM_ECC_ERR_INT_EN_W {
        SPI_SMEM_ECC_ERR_INT_EN_W { w: self }
    }
    #[doc = "Bits 25:30 - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to external RAM. tSHSL is (SPI_SMEM_CS_HOLD_DELAY\\[5:0\\]
 + 1) MSPI core clock cycles."]
    #[inline(always)]
    pub fn spi_smem_cs_hold_delay(&mut self) -> SPI_SMEM_CS_HOLD_DELAY_W {
        SPI_SMEM_CS_HOLD_DELAY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MSPI external RAM ECC and SPI CS timing control register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_smem_ac]
(index.html) module"]
pub struct SPI_SMEM_AC_SPEC;
impl crate::RegisterSpec for SPI_SMEM_AC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_smem_ac::R]
(R) reader structure"]
impl crate::Readable for SPI_SMEM_AC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_smem_ac::W]
(W) writer structure"]
impl crate::Writable for SPI_SMEM_AC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_SMEM_AC to value 0xb084"]
impl crate::Resettable for SPI_SMEM_AC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb084
    }
}
