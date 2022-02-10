#[doc = "Register `TIMING_CALI` reader"]
pub struct R(crate::R<TIMING_CALI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMING_CALI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMING_CALI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMING_CALI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMING_CALI` writer"]
pub struct W(crate::W<TIMING_CALI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMING_CALI_SPEC>;
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
impl From<crate::W<TIMING_CALI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMING_CALI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMING_CALI` reader - Set this bit to add extra SPI_CLK cycles in DUMMY phase for all reading operations."]
pub struct TIMING_CALI_R(crate::FieldReader<bool, bool>);
impl TIMING_CALI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMING_CALI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMING_CALI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMING_CALI` writer - Set this bit to add extra SPI_CLK cycles in DUMMY phase for all reading operations."]
pub struct TIMING_CALI_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMING_CALI_W<'a> {
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
#[doc = "Field `EXTRA_DUMMY_CYCLELEN` reader - Extra SPI_CLK cycles added in DUMMY phase for timing compensation. Active when SPI_MEM_TIMING_CALI bit is set."]
pub struct EXTRA_DUMMY_CYCLELEN_R(crate::FieldReader<u8, u8>);
impl EXTRA_DUMMY_CYCLELEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EXTRA_DUMMY_CYCLELEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTRA_DUMMY_CYCLELEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTRA_DUMMY_CYCLELEN` writer - Extra SPI_CLK cycles added in DUMMY phase for timing compensation. Active when SPI_MEM_TIMING_CALI bit is set."]
pub struct EXTRA_DUMMY_CYCLELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTRA_DUMMY_CYCLELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u32 & 0x07) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Set this bit to add extra SPI_CLK cycles in DUMMY phase for all reading operations."]
    #[inline(always)]
    pub fn timing_cali(&self) -> TIMING_CALI_R {
        TIMING_CALI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - Extra SPI_CLK cycles added in DUMMY phase for timing compensation. Active when SPI_MEM_TIMING_CALI bit is set."]
    #[inline(always)]
    pub fn extra_dummy_cyclelen(&self) -> EXTRA_DUMMY_CYCLELEN_R {
        EXTRA_DUMMY_CYCLELEN_R::new(((self.bits >> 2) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Set this bit to add extra SPI_CLK cycles in DUMMY phase for all reading operations."]
    #[inline(always)]
    pub fn timing_cali(&mut self) -> TIMING_CALI_W {
        TIMING_CALI_W { w: self }
    }
    #[doc = "Bits 2:4 - Extra SPI_CLK cycles added in DUMMY phase for timing compensation. Active when SPI_MEM_TIMING_CALI bit is set."]
    #[inline(always)]
    pub fn extra_dummy_cyclelen(&mut self) -> EXTRA_DUMMY_CYCLELEN_W {
        EXTRA_DUMMY_CYCLELEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 timing compensation register when accesses to flash or Ext_RAM.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timing_cali]
(index.html) module"]
pub struct TIMING_CALI_SPEC;
impl crate::RegisterSpec for TIMING_CALI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timing_cali::R]
(R) reader structure"]
impl crate::Readable for TIMING_CALI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timing_cali::W]
(W) writer structure"]
impl crate::Writable for TIMING_CALI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMING_CALI to value 0"]
impl crate::Resettable for TIMING_CALI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
