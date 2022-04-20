#[doc = "Register `SPI_MEM_ECC_CTRL` reader"]
pub struct R(crate::R<SPI_MEM_ECC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_ECC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_ECC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_ECC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_ECC_CTRL` writer"]
pub struct W(crate::W<SPI_MEM_ECC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_ECC_CTRL_SPEC>;
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
impl From<crate::W<SPI_MEM_ECC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_ECC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_PAGE_SIZE` reader - Set the page size of the used MSPI flash. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
pub struct FLASH_PAGE_SIZE_R(crate::FieldReader<u8, u8>);
impl FLASH_PAGE_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FLASH_PAGE_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_PAGE_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_PAGE_SIZE` writer - Set the page size of the used MSPI flash. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
pub struct FLASH_PAGE_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_PAGE_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 18)) | ((value as u32 & 3) << 18);
        self.w
    }
}
#[doc = "Field `SRAM_PAGE_SIZE` reader - Set the page size of the used MSPI external RAM. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
pub struct SRAM_PAGE_SIZE_R(crate::FieldReader<u8, u8>);
impl SRAM_PAGE_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRAM_PAGE_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_PAGE_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_PAGE_SIZE` writer - Set the page size of the used MSPI external RAM. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
pub struct SRAM_PAGE_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_PAGE_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 20)) | ((value as u32 & 3) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 18:19 - Set the page size of the used MSPI flash. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
    #[inline(always)]
    pub fn flash_page_size(&self) -> FLASH_PAGE_SIZE_R {
        FLASH_PAGE_SIZE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Set the page size of the used MSPI external RAM. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
    #[inline(always)]
    pub fn sram_page_size(&self) -> SRAM_PAGE_SIZE_R {
        SRAM_PAGE_SIZE_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 18:19 - Set the page size of the used MSPI flash. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
    #[inline(always)]
    pub fn flash_page_size(&mut self) -> FLASH_PAGE_SIZE_W {
        FLASH_PAGE_SIZE_W { w: self }
    }
    #[doc = "Bits 20:21 - Set the page size of the used MSPI external RAM. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
    #[inline(always)]
    pub fn sram_page_size(&mut self) -> SRAM_PAGE_SIZE_W {
        SRAM_PAGE_SIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_ecc_ctrl]
(index.html) module"]
pub struct SPI_MEM_ECC_CTRL_SPEC;
impl crate::RegisterSpec for SPI_MEM_ECC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_ecc_ctrl::R]
(R) reader structure"]
impl crate::Readable for SPI_MEM_ECC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_ecc_ctrl::W]
(W) writer structure"]
impl crate::Writable for SPI_MEM_ECC_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_MEM_ECC_CTRL to value 0x0020_0000"]
impl crate::Resettable for SPI_MEM_ECC_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0020_0000
    }
}
