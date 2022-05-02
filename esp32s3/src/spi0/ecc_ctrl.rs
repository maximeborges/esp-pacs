#[doc = "Register `ECC_CTRL` reader"]
pub struct R(crate::R<ECC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECC_CTRL` writer"]
pub struct W(crate::W<ECC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECC_CTRL_SPEC>;
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
impl From<crate::W<ECC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECC_ERR_INT_NUM` reader - Set the error times of MSPI ECC read to generate MSPI SPI_MEM_ECC_ERR_INT interrupt."]
pub struct ECC_ERR_INT_NUM_R(crate::FieldReader<u8>);
impl ECC_ERR_INT_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ECC_ERR_INT_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECC_ERR_INT_NUM_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECC_ERR_INT_NUM` writer - Set the error times of MSPI ECC read to generate MSPI SPI_MEM_ECC_ERR_INT interrupt."]
pub struct ECC_ERR_INT_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> ECC_ERR_INT_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `SPI_FMEM_ECC_ERR_INT_EN` reader - Set this bit to calculate the error times of MSPI ECC read when accesses to flash."]
pub struct SPI_FMEM_ECC_ERR_INT_EN_R(crate::FieldReader<bool>);
impl SPI_FMEM_ECC_ERR_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_FMEM_ECC_ERR_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_FMEM_ECC_ERR_INT_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_FMEM_ECC_ERR_INT_EN` writer - Set this bit to calculate the error times of MSPI ECC read when accesses to flash."]
pub struct SPI_FMEM_ECC_ERR_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_FMEM_ECC_ERR_INT_EN_W<'a> {
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
impl R {
    #[doc = "Bits 0:7 - Set the error times of MSPI ECC read to generate MSPI SPI_MEM_ECC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn ecc_err_int_num(&self) -> ECC_ERR_INT_NUM_R {
        ECC_ERR_INT_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Set this bit to calculate the error times of MSPI ECC read when accesses to flash."]
    #[inline(always)]
    pub fn spi_fmem_ecc_err_int_en(&self) -> SPI_FMEM_ECC_ERR_INT_EN_R {
        SPI_FMEM_ECC_ERR_INT_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Set the error times of MSPI ECC read to generate MSPI SPI_MEM_ECC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn ecc_err_int_num(&mut self) -> ECC_ERR_INT_NUM_W {
        ECC_ERR_INT_NUM_W { w: self }
    }
    #[doc = "Bit 8 - Set this bit to calculate the error times of MSPI ECC read when accesses to flash."]
    #[inline(always)]
    pub fn spi_fmem_ecc_err_int_en(&mut self) -> SPI_FMEM_ECC_ERR_INT_EN_W {
        SPI_FMEM_ECC_ERR_INT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MSPI ECC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_ctrl](index.html) module"]
pub struct ECC_CTRL_SPEC;
impl crate::RegisterSpec for ECC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecc_ctrl::R](R) reader structure"]
impl crate::Readable for ECC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecc_ctrl::W](W) writer structure"]
impl crate::Writable for ECC_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ECC_CTRL to value 0x0a"]
impl crate::Resettable for ECC_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a
    }
}
