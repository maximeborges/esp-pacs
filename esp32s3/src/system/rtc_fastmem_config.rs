#[doc = "Register `RTC_FASTMEM_CONFIG` reader"]
pub struct R(crate::R<RTC_FASTMEM_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_FASTMEM_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_FASTMEM_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_FASTMEM_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_FASTMEM_CONFIG` writer"]
pub struct W(crate::W<RTC_FASTMEM_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_FASTMEM_CONFIG_SPEC>;
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
impl From<crate::W<RTC_FASTMEM_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_FASTMEM_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_MEM_CRC_START` reader - Set 1 to start the CRC of RTC memory"]
pub struct RTC_MEM_CRC_START_R(crate::FieldReader<bool>);
impl RTC_MEM_CRC_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_MEM_CRC_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_MEM_CRC_START_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_MEM_CRC_START` writer - Set 1 to start the CRC of RTC memory"]
pub struct RTC_MEM_CRC_START_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_MEM_CRC_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `RTC_MEM_CRC_ADDR` reader - This field is used to set address of RTC memory for CRC."]
pub struct RTC_MEM_CRC_ADDR_R(crate::FieldReader<u16>);
impl RTC_MEM_CRC_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RTC_MEM_CRC_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_MEM_CRC_ADDR_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_MEM_CRC_ADDR` writer - This field is used to set address of RTC memory for CRC."]
pub struct RTC_MEM_CRC_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_MEM_CRC_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 9)) | ((value as u32 & 0x07ff) << 9);
        self.w
    }
}
#[doc = "Field `RTC_MEM_CRC_LEN` reader - This field is used to set length of RTC memory for CRC based on start address."]
pub struct RTC_MEM_CRC_LEN_R(crate::FieldReader<u16>);
impl RTC_MEM_CRC_LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RTC_MEM_CRC_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_MEM_CRC_LEN_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_MEM_CRC_LEN` writer - This field is used to set length of RTC memory for CRC based on start address."]
pub struct RTC_MEM_CRC_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_MEM_CRC_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 20)) | ((value as u32 & 0x07ff) << 20);
        self.w
    }
}
#[doc = "Field `RTC_MEM_CRC_FINISH` reader - This bit stores the status of RTC memory CRC.1 means finished."]
pub struct RTC_MEM_CRC_FINISH_R(crate::FieldReader<bool>);
impl RTC_MEM_CRC_FINISH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_MEM_CRC_FINISH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_MEM_CRC_FINISH_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 8 - Set 1 to start the CRC of RTC memory"]
    #[inline(always)]
    pub fn rtc_mem_crc_start(&self) -> RTC_MEM_CRC_START_R {
        RTC_MEM_CRC_START_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:19 - This field is used to set address of RTC memory for CRC."]
    #[inline(always)]
    pub fn rtc_mem_crc_addr(&self) -> RTC_MEM_CRC_ADDR_R {
        RTC_MEM_CRC_ADDR_R::new(((self.bits >> 9) & 0x07ff) as u16)
    }
    #[doc = "Bits 20:30 - This field is used to set length of RTC memory for CRC based on start address."]
    #[inline(always)]
    pub fn rtc_mem_crc_len(&self) -> RTC_MEM_CRC_LEN_R {
        RTC_MEM_CRC_LEN_R::new(((self.bits >> 20) & 0x07ff) as u16)
    }
    #[doc = "Bit 31 - This bit stores the status of RTC memory CRC.1 means finished."]
    #[inline(always)]
    pub fn rtc_mem_crc_finish(&self) -> RTC_MEM_CRC_FINISH_R {
        RTC_MEM_CRC_FINISH_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Set 1 to start the CRC of RTC memory"]
    #[inline(always)]
    pub fn rtc_mem_crc_start(&mut self) -> RTC_MEM_CRC_START_W {
        RTC_MEM_CRC_START_W { w: self }
    }
    #[doc = "Bits 9:19 - This field is used to set address of RTC memory for CRC."]
    #[inline(always)]
    pub fn rtc_mem_crc_addr(&mut self) -> RTC_MEM_CRC_ADDR_W {
        RTC_MEM_CRC_ADDR_W { w: self }
    }
    #[doc = "Bits 20:30 - This field is used to set length of RTC memory for CRC based on start address."]
    #[inline(always)]
    pub fn rtc_mem_crc_len(&mut self) -> RTC_MEM_CRC_LEN_W {
        RTC_MEM_CRC_LEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC fast memory configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_fastmem_config](index.html) module"]
pub struct RTC_FASTMEM_CONFIG_SPEC;
impl crate::RegisterSpec for RTC_FASTMEM_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_fastmem_config::R](R) reader structure"]
impl crate::Readable for RTC_FASTMEM_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_fastmem_config::W](W) writer structure"]
impl crate::Writable for RTC_FASTMEM_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_FASTMEM_CONFIG to value 0x7ff0_0000"]
impl crate::Resettable for RTC_FASTMEM_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7ff0_0000
    }
}
