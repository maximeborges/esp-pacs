#[doc = "Register `FLASH_SUS_CTRL` reader"]
pub struct R(crate::R<FLASH_SUS_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_SUS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_SUS_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_SUS_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_SUS_CTRL` writer"]
pub struct W(crate::W<FLASH_SUS_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_SUS_CTRL_SPEC>;
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
impl From<crate::W<FLASH_SUS_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_SUS_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_PER` reader - program erase resume bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub struct FLASH_PER_R(crate::FieldReader<bool, bool>);
impl FLASH_PER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_PER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_PER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_PER` writer - program erase resume bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub struct FLASH_PER_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_PER_W<'a> {
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
#[doc = "Field `FLASH_PES` reader - program erase suspend bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub struct FLASH_PES_R(crate::FieldReader<bool, bool>);
impl FLASH_PES_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_PES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_PES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_PES` writer - program erase suspend bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub struct FLASH_PES_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_PES_W<'a> {
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
#[doc = "Field `FLASH_PER_WAIT_EN` reader - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\]
 * 4 or *128) SPI_CLK cycles after program erase resume command is sent. 0: SPI1 does not wait after program erase resume command is sent."]
pub struct FLASH_PER_WAIT_EN_R(crate::FieldReader<bool, bool>);
impl FLASH_PER_WAIT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_PER_WAIT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_PER_WAIT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_PER_WAIT_EN` writer - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\]
 * 4 or *128) SPI_CLK cycles after program erase resume command is sent. 0: SPI1 does not wait after program erase resume command is sent."]
pub struct FLASH_PER_WAIT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_PER_WAIT_EN_W<'a> {
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
#[doc = "Field `FLASH_PES_WAIT_EN` reader - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\]
 * 4 or *128) SPI_CLK cycles after program erase suspend command is sent. 0: SPI1 does not wait after program erase suspend command is sent."]
pub struct FLASH_PES_WAIT_EN_R(crate::FieldReader<bool, bool>);
impl FLASH_PES_WAIT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_PES_WAIT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_PES_WAIT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_PES_WAIT_EN` writer - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\]
 * 4 or *128) SPI_CLK cycles after program erase suspend command is sent. 0: SPI1 does not wait after program erase suspend command is sent."]
pub struct FLASH_PES_WAIT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_PES_WAIT_EN_W<'a> {
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
#[doc = "Field `PES_PER_EN` reader - Set this bit to enable PES end triggers PER transfer option. If this bit is 0, application should send PER after PES is done."]
pub struct PES_PER_EN_R(crate::FieldReader<bool, bool>);
impl PES_PER_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PES_PER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PES_PER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PES_PER_EN` writer - Set this bit to enable PES end triggers PER transfer option. If this bit is 0, application should send PER after PES is done."]
pub struct PES_PER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PES_PER_EN_W<'a> {
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
#[doc = "Field `FLASH_PES_EN` reader - Set this bit to enable Auto-suspending function."]
pub struct FLASH_PES_EN_R(crate::FieldReader<bool, bool>);
impl FLASH_PES_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_PES_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_PES_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_PES_EN` writer - Set this bit to enable Auto-suspending function."]
pub struct FLASH_PES_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_PES_EN_W<'a> {
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
#[doc = "Field `PESR_END_MSK` reader - The mask value when check SUS/SUS1/SUS2 status bit. If the read status value is status_in\\[15:0\\]
(only status_in\\[7:0\\]
 is valid when only one byte of data is read out, status_in\\[15:0\\]
 is valid when two bytes of data are read out), SUS/SUS1/SUS2 = status_in\\[15:0\\]
^ SPI_MEM_PESR_END_MSK\\[15:0\\]
."]
pub struct PESR_END_MSK_R(crate::FieldReader<u16, u16>);
impl PESR_END_MSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PESR_END_MSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PESR_END_MSK_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PESR_END_MSK` writer - The mask value when check SUS/SUS1/SUS2 status bit. If the read status value is status_in\\[15:0\\]
(only status_in\\[7:0\\]
 is valid when only one byte of data is read out, status_in\\[15:0\\]
 is valid when two bytes of data are read out), SUS/SUS1/SUS2 = status_in\\[15:0\\]
^ SPI_MEM_PESR_END_MSK\\[15:0\\]
."]
pub struct PESR_END_MSK_W<'a> {
    w: &'a mut W,
}
impl<'a> PESR_END_MSK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 6)) | ((value as u32 & 0xffff) << 6);
        self.w
    }
}
#[doc = "Field `RD_SUS_2B` reader - 1: Read two bytes when check flash SUS/SUS1/SUS2 status bit. 0: Read one byte when check flash SUS/SUS1/SUS2 status bit"]
pub struct RD_SUS_2B_R(crate::FieldReader<bool, bool>);
impl RD_SUS_2B_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RD_SUS_2B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_SUS_2B_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_SUS_2B` writer - 1: Read two bytes when check flash SUS/SUS1/SUS2 status bit. 0: Read one byte when check flash SUS/SUS1/SUS2 status bit"]
pub struct RD_SUS_2B_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_SUS_2B_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `PER_END_EN` reader - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the resume status of flash. 0: Only need to check WIP is 0."]
pub struct PER_END_EN_R(crate::FieldReader<bool, bool>);
impl PER_END_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PER_END_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PER_END_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PER_END_EN` writer - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the resume status of flash. 0: Only need to check WIP is 0."]
pub struct PER_END_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_END_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `PES_END_EN` reader - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the suspend status of flash. 0: Only need to check WIP is 0."]
pub struct PES_END_EN_R(crate::FieldReader<bool, bool>);
impl PES_END_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PES_END_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PES_END_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PES_END_EN` writer - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the suspend status of flash. 0: Only need to check WIP is 0."]
pub struct PES_END_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PES_END_EN_W<'a> {
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
#[doc = "Field `SUS_TIMEOUT_CNT` reader - When SPI1 checks SUS/SUS1/SUS2 bits fail for SPI_MEM_SUS_TIMEOUT_CNT\\[6:0\\]
 times, it will be treated as check pass."]
pub struct SUS_TIMEOUT_CNT_R(crate::FieldReader<u8, u8>);
impl SUS_TIMEOUT_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SUS_TIMEOUT_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUS_TIMEOUT_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUS_TIMEOUT_CNT` writer - When SPI1 checks SUS/SUS1/SUS2 bits fail for SPI_MEM_SUS_TIMEOUT_CNT\\[6:0\\]
 times, it will be treated as check pass."]
pub struct SUS_TIMEOUT_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SUS_TIMEOUT_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | ((value as u32 & 0x7f) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - program erase resume bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_per(&self) -> FLASH_PER_R {
        FLASH_PER_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - program erase suspend bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_pes(&self) -> FLASH_PES_R {
        FLASH_PES_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\]
 * 4 or *128) SPI_CLK cycles after program erase resume command is sent. 0: SPI1 does not wait after program erase resume command is sent."]
    #[inline(always)]
    pub fn flash_per_wait_en(&self) -> FLASH_PER_WAIT_EN_R {
        FLASH_PER_WAIT_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\]
 * 4 or *128) SPI_CLK cycles after program erase suspend command is sent. 0: SPI1 does not wait after program erase suspend command is sent."]
    #[inline(always)]
    pub fn flash_pes_wait_en(&self) -> FLASH_PES_WAIT_EN_R {
        FLASH_PES_WAIT_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable PES end triggers PER transfer option. If this bit is 0, application should send PER after PES is done."]
    #[inline(always)]
    pub fn pes_per_en(&self) -> PES_PER_EN_R {
        PES_PER_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Set this bit to enable Auto-suspending function."]
    #[inline(always)]
    pub fn flash_pes_en(&self) -> FLASH_PES_EN_R {
        FLASH_PES_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:21 - The mask value when check SUS/SUS1/SUS2 status bit. If the read status value is status_in\\[15:0\\]
(only status_in\\[7:0\\]
 is valid when only one byte of data is read out, status_in\\[15:0\\]
 is valid when two bytes of data are read out), SUS/SUS1/SUS2 = status_in\\[15:0\\]
^ SPI_MEM_PESR_END_MSK\\[15:0\\]
."]
    #[inline(always)]
    pub fn pesr_end_msk(&self) -> PESR_END_MSK_R {
        PESR_END_MSK_R::new(((self.bits >> 6) & 0xffff) as u16)
    }
    #[doc = "Bit 22 - 1: Read two bytes when check flash SUS/SUS1/SUS2 status bit. 0: Read one byte when check flash SUS/SUS1/SUS2 status bit"]
    #[inline(always)]
    pub fn rd_sus_2b(&self) -> RD_SUS_2B_R {
        RD_SUS_2B_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the resume status of flash. 0: Only need to check WIP is 0."]
    #[inline(always)]
    pub fn per_end_en(&self) -> PER_END_EN_R {
        PER_END_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the suspend status of flash. 0: Only need to check WIP is 0."]
    #[inline(always)]
    pub fn pes_end_en(&self) -> PES_END_EN_R {
        PES_END_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:31 - When SPI1 checks SUS/SUS1/SUS2 bits fail for SPI_MEM_SUS_TIMEOUT_CNT\\[6:0\\]
 times, it will be treated as check pass."]
    #[inline(always)]
    pub fn sus_timeout_cnt(&self) -> SUS_TIMEOUT_CNT_R {
        SUS_TIMEOUT_CNT_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - program erase resume bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_per(&mut self) -> FLASH_PER_W {
        FLASH_PER_W { w: self }
    }
    #[doc = "Bit 1 - program erase suspend bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_pes(&mut self) -> FLASH_PES_W {
        FLASH_PES_W { w: self }
    }
    #[doc = "Bit 2 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\]
 * 4 or *128) SPI_CLK cycles after program erase resume command is sent. 0: SPI1 does not wait after program erase resume command is sent."]
    #[inline(always)]
    pub fn flash_per_wait_en(&mut self) -> FLASH_PER_WAIT_EN_W {
        FLASH_PER_WAIT_EN_W { w: self }
    }
    #[doc = "Bit 3 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\]
 * 4 or *128) SPI_CLK cycles after program erase suspend command is sent. 0: SPI1 does not wait after program erase suspend command is sent."]
    #[inline(always)]
    pub fn flash_pes_wait_en(&mut self) -> FLASH_PES_WAIT_EN_W {
        FLASH_PES_WAIT_EN_W { w: self }
    }
    #[doc = "Bit 4 - Set this bit to enable PES end triggers PER transfer option. If this bit is 0, application should send PER after PES is done."]
    #[inline(always)]
    pub fn pes_per_en(&mut self) -> PES_PER_EN_W {
        PES_PER_EN_W { w: self }
    }
    #[doc = "Bit 5 - Set this bit to enable Auto-suspending function."]
    #[inline(always)]
    pub fn flash_pes_en(&mut self) -> FLASH_PES_EN_W {
        FLASH_PES_EN_W { w: self }
    }
    #[doc = "Bits 6:21 - The mask value when check SUS/SUS1/SUS2 status bit. If the read status value is status_in\\[15:0\\]
(only status_in\\[7:0\\]
 is valid when only one byte of data is read out, status_in\\[15:0\\]
 is valid when two bytes of data are read out), SUS/SUS1/SUS2 = status_in\\[15:0\\]
^ SPI_MEM_PESR_END_MSK\\[15:0\\]
."]
    #[inline(always)]
    pub fn pesr_end_msk(&mut self) -> PESR_END_MSK_W {
        PESR_END_MSK_W { w: self }
    }
    #[doc = "Bit 22 - 1: Read two bytes when check flash SUS/SUS1/SUS2 status bit. 0: Read one byte when check flash SUS/SUS1/SUS2 status bit"]
    #[inline(always)]
    pub fn rd_sus_2b(&mut self) -> RD_SUS_2B_W {
        RD_SUS_2B_W { w: self }
    }
    #[doc = "Bit 23 - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the resume status of flash. 0: Only need to check WIP is 0."]
    #[inline(always)]
    pub fn per_end_en(&mut self) -> PER_END_EN_W {
        PER_END_EN_W { w: self }
    }
    #[doc = "Bit 24 - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the suspend status of flash. 0: Only need to check WIP is 0."]
    #[inline(always)]
    pub fn pes_end_en(&mut self) -> PES_END_EN_W {
        PES_END_EN_W { w: self }
    }
    #[doc = "Bits 25:31 - When SPI1 checks SUS/SUS1/SUS2 bits fail for SPI_MEM_SUS_TIMEOUT_CNT\\[6:0\\]
 times, it will be treated as check pass."]
    #[inline(always)]
    pub fn sus_timeout_cnt(&mut self) -> SUS_TIMEOUT_CNT_W {
        SUS_TIMEOUT_CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 flash suspend control register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_sus_ctrl]
(index.html) module"]
pub struct FLASH_SUS_CTRL_SPEC;
impl crate::RegisterSpec for FLASH_SUS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_sus_ctrl::R]
(R) reader structure"]
impl crate::Readable for FLASH_SUS_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_sus_ctrl::W]
(W) writer structure"]
impl crate::Writable for FLASH_SUS_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_SUS_CTRL to value 0x0800_2000"]
impl crate::Resettable for FLASH_SUS_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0800_2000
    }
}
