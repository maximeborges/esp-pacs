#[doc = "Register `DCACHE_CTRL` reader"]
pub struct R(crate::R<DCACHE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCACHE_CTRL` writer"]
pub struct W(crate::W<DCACHE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCACHE_CTRL_SPEC>;
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
impl From<crate::W<DCACHE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCACHE_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCACHE_ENABLE` reader - The bit is used to activate the data cache. 0: disable, 1: enable"]
pub struct DCACHE_ENABLE_R(crate::FieldReader<bool, bool>);
impl DCACHE_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCACHE_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_ENABLE` writer - The bit is used to activate the data cache. 0: disable, 1: enable"]
pub struct DCACHE_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_ENABLE_W<'a> {
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
#[doc = "Field `DCACHE_SIZE_MODE` reader - The bit is used to configure cache memory size.0: 32KB, 1: 64KB"]
pub struct DCACHE_SIZE_MODE_R(crate::FieldReader<bool, bool>);
impl DCACHE_SIZE_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCACHE_SIZE_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_SIZE_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_SIZE_MODE` writer - The bit is used to configure cache memory size.0: 32KB, 1: 64KB"]
pub struct DCACHE_SIZE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_SIZE_MODE_W<'a> {
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
#[doc = "Field `DCACHE_BLOCKSIZE_MODE` reader - The bit is used to configure cache block size.0: 16 bytes, 1: 32 bytes,2: 64 bytes"]
pub struct DCACHE_BLOCKSIZE_MODE_R(crate::FieldReader<u8, u8>);
impl DCACHE_BLOCKSIZE_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DCACHE_BLOCKSIZE_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_BLOCKSIZE_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_BLOCKSIZE_MODE` writer - The bit is used to configure cache block size.0: 16 bytes, 1: 32 bytes,2: 64 bytes"]
pub struct DCACHE_BLOCKSIZE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_BLOCKSIZE_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - The bit is used to activate the data cache. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn dcache_enable(&self) -> DCACHE_ENABLE_R {
        DCACHE_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - The bit is used to configure cache memory size.0: 32KB, 1: 64KB"]
    #[inline(always)]
    pub fn dcache_size_mode(&self) -> DCACHE_SIZE_MODE_R {
        DCACHE_SIZE_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - The bit is used to configure cache block size.0: 16 bytes, 1: 32 bytes,2: 64 bytes"]
    #[inline(always)]
    pub fn dcache_blocksize_mode(&self) -> DCACHE_BLOCKSIZE_MODE_R {
        DCACHE_BLOCKSIZE_MODE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to activate the data cache. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn dcache_enable(&mut self) -> DCACHE_ENABLE_W {
        DCACHE_ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - The bit is used to configure cache memory size.0: 32KB, 1: 64KB"]
    #[inline(always)]
    pub fn dcache_size_mode(&mut self) -> DCACHE_SIZE_MODE_W {
        DCACHE_SIZE_MODE_W { w: self }
    }
    #[doc = "Bits 3:4 - The bit is used to configure cache block size.0: 16 bytes, 1: 32 bytes,2: 64 bytes"]
    #[inline(always)]
    pub fn dcache_blocksize_mode(&mut self) -> DCACHE_BLOCKSIZE_MODE_W {
        DCACHE_BLOCKSIZE_MODE_W { w: self }
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
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcache_ctrl]
(index.html) module"]
pub struct DCACHE_CTRL_SPEC;
impl crate::RegisterSpec for DCACHE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcache_ctrl::R]
(R) reader structure"]
impl crate::Readable for DCACHE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcache_ctrl::W]
(W) writer structure"]
impl crate::Writable for DCACHE_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCACHE_CTRL to value 0"]
impl crate::Resettable for DCACHE_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
