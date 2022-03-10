#[doc = "Register `CACHE_SCTRL` reader"]
pub struct R(crate::R<CACHE_SCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_SCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_SCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_SCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_SCTRL` writer"]
pub struct W(crate::W<CACHE_SCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_SCTRL_SPEC>;
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
impl From<crate::W<CACHE_SCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_SCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHE_USR_SCMD_4BYTE` reader - Set this bit to enable SPI0 read Ext_RAM with 32 bits address. The value of SPI_MEM_SRAM_ADDR_BITLEN should be 31."]
pub struct CACHE_USR_SCMD_4BYTE_R(crate::FieldReader<bool, bool>);
impl CACHE_USR_SCMD_4BYTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_USR_SCMD_4BYTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_USR_SCMD_4BYTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_USR_SCMD_4BYTE` writer - Set this bit to enable SPI0 read Ext_RAM with 32 bits address. The value of SPI_MEM_SRAM_ADDR_BITLEN should be 31."]
pub struct CACHE_USR_SCMD_4BYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_USR_SCMD_4BYTE_W<'a> {
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
#[doc = "Field `USR_SRAM_DIO` reader - Set the bit to enable 2-bm in all the phases of SPI0 Ext_RAM transfer."]
pub struct USR_SRAM_DIO_R(crate::FieldReader<bool, bool>);
impl USR_SRAM_DIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USR_SRAM_DIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_SRAM_DIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_SRAM_DIO` writer - Set the bit to enable 2-bm in all the phases of SPI0 Ext_RAM transfer."]
pub struct USR_SRAM_DIO_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_SRAM_DIO_W<'a> {
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
#[doc = "Field `USR_SRAM_QIO` reader - Set the bit to enable QPI mode in all SPI0 Ext_RAM transfer."]
pub struct USR_SRAM_QIO_R(crate::FieldReader<bool, bool>);
impl USR_SRAM_QIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USR_SRAM_QIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_SRAM_QIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_SRAM_QIO` writer - Set the bit to enable QPI mode in all SPI0 Ext_RAM transfer."]
pub struct USR_SRAM_QIO_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_SRAM_QIO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `USR_WR_SRAM_DUMMY` reader - When SPI0 accesses to Ext_RAM, set this bit to enable DUMMY phase in write operations."]
pub struct USR_WR_SRAM_DUMMY_R(crate::FieldReader<bool, bool>);
impl USR_WR_SRAM_DUMMY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USR_WR_SRAM_DUMMY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_WR_SRAM_DUMMY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_WR_SRAM_DUMMY` writer - When SPI0 accesses to Ext_RAM, set this bit to enable DUMMY phase in write operations."]
pub struct USR_WR_SRAM_DUMMY_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_WR_SRAM_DUMMY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `USR_RD_SRAM_DUMMY` reader - When SPI0 accesses to Ext_RAM, set this bit to enable DUMMY phase in read operations."]
pub struct USR_RD_SRAM_DUMMY_R(crate::FieldReader<bool, bool>);
impl USR_RD_SRAM_DUMMY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USR_RD_SRAM_DUMMY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_RD_SRAM_DUMMY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_RD_SRAM_DUMMY` writer - When SPI0 accesses to Ext_RAM, set this bit to enable DUMMY phase in read operations."]
pub struct USR_RD_SRAM_DUMMY_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_RD_SRAM_DUMMY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `CACHE_SRAM_USR_RCMD` reader - 1: The command value of SPI0 read Ext_RAM is SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE. 0: The value is 0x2."]
pub struct CACHE_SRAM_USR_RCMD_R(crate::FieldReader<bool, bool>);
impl CACHE_SRAM_USR_RCMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_SRAM_USR_RCMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_SRAM_USR_RCMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_SRAM_USR_RCMD` writer - 1: The command value of SPI0 read Ext_RAM is SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE. 0: The value is 0x2."]
pub struct CACHE_SRAM_USR_RCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_SRAM_USR_RCMD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `SRAM_RDUMMY_CYCLELEN` reader - When SPI0 accesses to Ext_RAM, it is the SPI_CLK cycles minus 1 of DUMMY phase in read data transfer."]
pub struct SRAM_RDUMMY_CYCLELEN_R(crate::FieldReader<u8, u8>);
impl SRAM_RDUMMY_CYCLELEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRAM_RDUMMY_CYCLELEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_RDUMMY_CYCLELEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_RDUMMY_CYCLELEN` writer - When SPI0 accesses to Ext_RAM, it is the SPI_CLK cycles minus 1 of DUMMY phase in read data transfer."]
pub struct SRAM_RDUMMY_CYCLELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_RDUMMY_CYCLELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | ((value as u32 & 0x3f) << 6);
        self.w
    }
}
#[doc = "Field `SRAM_ADDR_BITLEN` reader - When SPI0 accesses to Ext_RAM, it is the length in bits of ADDR phase. The register value shall be (bit_num-1)."]
pub struct SRAM_ADDR_BITLEN_R(crate::FieldReader<u8, u8>);
impl SRAM_ADDR_BITLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRAM_ADDR_BITLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_ADDR_BITLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_ADDR_BITLEN` writer - When SPI0 accesses to Ext_RAM, it is the length in bits of ADDR phase. The register value shall be (bit_num-1)."]
pub struct SRAM_ADDR_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_ADDR_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 14)) | ((value as u32 & 0x3f) << 14);
        self.w
    }
}
#[doc = "Field `CACHE_SRAM_USR_WCMD` reader - 1: The command value of SPI0 write Ext_RAM is SPI_MEM_CACHE_SRAM_USR_RD_CMD_VALUE. 0: The value is 0x3."]
pub struct CACHE_SRAM_USR_WCMD_R(crate::FieldReader<bool, bool>);
impl CACHE_SRAM_USR_WCMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_SRAM_USR_WCMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_SRAM_USR_WCMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_SRAM_USR_WCMD` writer - 1: The command value of SPI0 write Ext_RAM is SPI_MEM_CACHE_SRAM_USR_RD_CMD_VALUE. 0: The value is 0x3."]
pub struct CACHE_SRAM_USR_WCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_SRAM_USR_WCMD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `SRAM_OCT` reader - Set the bit to enable OPI mode in all SPI0 Ext_RAM transfer."]
pub struct SRAM_OCT_R(crate::FieldReader<bool, bool>);
impl SRAM_OCT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_OCT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_OCT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_OCT` writer - Set the bit to enable OPI mode in all SPI0 Ext_RAM transfer."]
pub struct SRAM_OCT_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_OCT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `SRAM_WDUMMY_CYCLELEN` reader - When SPI0 accesses to Ext_RAM, it is the SPI_CLK cycles minus 1 of DUMMY phase in write data transfer."]
pub struct SRAM_WDUMMY_CYCLELEN_R(crate::FieldReader<u8, u8>);
impl SRAM_WDUMMY_CYCLELEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRAM_WDUMMY_CYCLELEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_WDUMMY_CYCLELEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_WDUMMY_CYCLELEN` writer - When SPI0 accesses to Ext_RAM, it is the SPI_CLK cycles minus 1 of DUMMY phase in write data transfer."]
pub struct SRAM_WDUMMY_CYCLELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_WDUMMY_CYCLELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 22)) | ((value as u32 & 0x3f) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set this bit to enable SPI0 read Ext_RAM with 32 bits address. The value of SPI_MEM_SRAM_ADDR_BITLEN should be 31."]
    #[inline(always)]
    pub fn cache_usr_scmd_4byte(&self) -> CACHE_USR_SCMD_4BYTE_R {
        CACHE_USR_SCMD_4BYTE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set the bit to enable 2-bm in all the phases of SPI0 Ext_RAM transfer."]
    #[inline(always)]
    pub fn usr_sram_dio(&self) -> USR_SRAM_DIO_R {
        USR_SRAM_DIO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set the bit to enable QPI mode in all SPI0 Ext_RAM transfer."]
    #[inline(always)]
    pub fn usr_sram_qio(&self) -> USR_SRAM_QIO_R {
        USR_SRAM_QIO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When SPI0 accesses to Ext_RAM, set this bit to enable DUMMY phase in write operations."]
    #[inline(always)]
    pub fn usr_wr_sram_dummy(&self) -> USR_WR_SRAM_DUMMY_R {
        USR_WR_SRAM_DUMMY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When SPI0 accesses to Ext_RAM, set this bit to enable DUMMY phase in read operations."]
    #[inline(always)]
    pub fn usr_rd_sram_dummy(&self) -> USR_RD_SRAM_DUMMY_R {
        USR_RD_SRAM_DUMMY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 1: The command value of SPI0 read Ext_RAM is SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE. 0: The value is 0x2."]
    #[inline(always)]
    pub fn cache_sram_usr_rcmd(&self) -> CACHE_SRAM_USR_RCMD_R {
        CACHE_SRAM_USR_RCMD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:11 - When SPI0 accesses to Ext_RAM, it is the SPI_CLK cycles minus 1 of DUMMY phase in read data transfer."]
    #[inline(always)]
    pub fn sram_rdummy_cyclelen(&self) -> SRAM_RDUMMY_CYCLELEN_R {
        SRAM_RDUMMY_CYCLELEN_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 14:19 - When SPI0 accesses to Ext_RAM, it is the length in bits of ADDR phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn sram_addr_bitlen(&self) -> SRAM_ADDR_BITLEN_R {
        SRAM_ADDR_BITLEN_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bit 20 - 1: The command value of SPI0 write Ext_RAM is SPI_MEM_CACHE_SRAM_USR_RD_CMD_VALUE. 0: The value is 0x3."]
    #[inline(always)]
    pub fn cache_sram_usr_wcmd(&self) -> CACHE_SRAM_USR_WCMD_R {
        CACHE_SRAM_USR_WCMD_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Set the bit to enable OPI mode in all SPI0 Ext_RAM transfer."]
    #[inline(always)]
    pub fn sram_oct(&self) -> SRAM_OCT_R {
        SRAM_OCT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 22:27 - When SPI0 accesses to Ext_RAM, it is the SPI_CLK cycles minus 1 of DUMMY phase in write data transfer."]
    #[inline(always)]
    pub fn sram_wdummy_cyclelen(&self) -> SRAM_WDUMMY_CYCLELEN_R {
        SRAM_WDUMMY_CYCLELEN_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable SPI0 read Ext_RAM with 32 bits address. The value of SPI_MEM_SRAM_ADDR_BITLEN should be 31."]
    #[inline(always)]
    pub fn cache_usr_scmd_4byte(&mut self) -> CACHE_USR_SCMD_4BYTE_W {
        CACHE_USR_SCMD_4BYTE_W { w: self }
    }
    #[doc = "Bit 1 - Set the bit to enable 2-bm in all the phases of SPI0 Ext_RAM transfer."]
    #[inline(always)]
    pub fn usr_sram_dio(&mut self) -> USR_SRAM_DIO_W {
        USR_SRAM_DIO_W { w: self }
    }
    #[doc = "Bit 2 - Set the bit to enable QPI mode in all SPI0 Ext_RAM transfer."]
    #[inline(always)]
    pub fn usr_sram_qio(&mut self) -> USR_SRAM_QIO_W {
        USR_SRAM_QIO_W { w: self }
    }
    #[doc = "Bit 3 - When SPI0 accesses to Ext_RAM, set this bit to enable DUMMY phase in write operations."]
    #[inline(always)]
    pub fn usr_wr_sram_dummy(&mut self) -> USR_WR_SRAM_DUMMY_W {
        USR_WR_SRAM_DUMMY_W { w: self }
    }
    #[doc = "Bit 4 - When SPI0 accesses to Ext_RAM, set this bit to enable DUMMY phase in read operations."]
    #[inline(always)]
    pub fn usr_rd_sram_dummy(&mut self) -> USR_RD_SRAM_DUMMY_W {
        USR_RD_SRAM_DUMMY_W { w: self }
    }
    #[doc = "Bit 5 - 1: The command value of SPI0 read Ext_RAM is SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE. 0: The value is 0x2."]
    #[inline(always)]
    pub fn cache_sram_usr_rcmd(&mut self) -> CACHE_SRAM_USR_RCMD_W {
        CACHE_SRAM_USR_RCMD_W { w: self }
    }
    #[doc = "Bits 6:11 - When SPI0 accesses to Ext_RAM, it is the SPI_CLK cycles minus 1 of DUMMY phase in read data transfer."]
    #[inline(always)]
    pub fn sram_rdummy_cyclelen(&mut self) -> SRAM_RDUMMY_CYCLELEN_W {
        SRAM_RDUMMY_CYCLELEN_W { w: self }
    }
    #[doc = "Bits 14:19 - When SPI0 accesses to Ext_RAM, it is the length in bits of ADDR phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn sram_addr_bitlen(&mut self) -> SRAM_ADDR_BITLEN_W {
        SRAM_ADDR_BITLEN_W { w: self }
    }
    #[doc = "Bit 20 - 1: The command value of SPI0 write Ext_RAM is SPI_MEM_CACHE_SRAM_USR_RD_CMD_VALUE. 0: The value is 0x3."]
    #[inline(always)]
    pub fn cache_sram_usr_wcmd(&mut self) -> CACHE_SRAM_USR_WCMD_W {
        CACHE_SRAM_USR_WCMD_W { w: self }
    }
    #[doc = "Bit 21 - Set the bit to enable OPI mode in all SPI0 Ext_RAM transfer."]
    #[inline(always)]
    pub fn sram_oct(&mut self) -> SRAM_OCT_W {
        SRAM_OCT_W { w: self }
    }
    #[doc = "Bits 22:27 - When SPI0 accesses to Ext_RAM, it is the SPI_CLK cycles minus 1 of DUMMY phase in write data transfer."]
    #[inline(always)]
    pub fn sram_wdummy_cyclelen(&mut self) -> SRAM_WDUMMY_CYCLELEN_W {
        SRAM_WDUMMY_CYCLELEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 external RAM control register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_sctrl]
(index.html) module"]
pub struct CACHE_SCTRL_SPEC;
impl crate::RegisterSpec for CACHE_SCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_sctrl::R]
(R) reader structure"]
impl crate::Readable for CACHE_SCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_sctrl::W]
(W) writer structure"]
impl crate::Writable for CACHE_SCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE_SCTRL to value 0x0055_c070"]
impl crate::Resettable for CACHE_SCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0055_c070
    }
}
