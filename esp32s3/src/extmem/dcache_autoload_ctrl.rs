#[doc = "Register `DCACHE_AUTOLOAD_CTRL` reader"]
pub struct R(crate::R<DCACHE_AUTOLOAD_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_AUTOLOAD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_AUTOLOAD_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_AUTOLOAD_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCACHE_AUTOLOAD_CTRL` writer"]
pub struct W(crate::W<DCACHE_AUTOLOAD_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCACHE_AUTOLOAD_CTRL_SPEC>;
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
impl From<crate::W<DCACHE_AUTOLOAD_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCACHE_AUTOLOAD_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCACHE_AUTOLOAD_SCT0_ENA` reader - The bits are used to enable the first section for autoload operation."]
pub struct DCACHE_AUTOLOAD_SCT0_ENA_R(crate::FieldReader<bool, bool>);
impl DCACHE_AUTOLOAD_SCT0_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCACHE_AUTOLOAD_SCT0_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_AUTOLOAD_SCT0_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_AUTOLOAD_SCT0_ENA` writer - The bits are used to enable the first section for autoload operation."]
pub struct DCACHE_AUTOLOAD_SCT0_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_AUTOLOAD_SCT0_ENA_W<'a> {
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
#[doc = "Field `DCACHE_AUTOLOAD_SCT1_ENA` reader - The bits are used to enable the second section for autoload operation."]
pub struct DCACHE_AUTOLOAD_SCT1_ENA_R(crate::FieldReader<bool, bool>);
impl DCACHE_AUTOLOAD_SCT1_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCACHE_AUTOLOAD_SCT1_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_AUTOLOAD_SCT1_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_AUTOLOAD_SCT1_ENA` writer - The bits are used to enable the second section for autoload operation."]
pub struct DCACHE_AUTOLOAD_SCT1_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_AUTOLOAD_SCT1_ENA_W<'a> {
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
#[doc = "Field `DCACHE_AUTOLOAD_ENA` reader - The bit is used to enable and disable autoload operation. It is combined with dcache_autoload_done. 1: enable, 0: disable."]
pub struct DCACHE_AUTOLOAD_ENA_R(crate::FieldReader<bool, bool>);
impl DCACHE_AUTOLOAD_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCACHE_AUTOLOAD_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_AUTOLOAD_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_AUTOLOAD_ENA` writer - The bit is used to enable and disable autoload operation. It is combined with dcache_autoload_done. 1: enable, 0: disable."]
pub struct DCACHE_AUTOLOAD_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_AUTOLOAD_ENA_W<'a> {
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
#[doc = "Field `DCACHE_AUTOLOAD_DONE` reader - The bit is used to indicate autoload operation is finished."]
pub struct DCACHE_AUTOLOAD_DONE_R(crate::FieldReader<bool, bool>);
impl DCACHE_AUTOLOAD_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCACHE_AUTOLOAD_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_AUTOLOAD_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_AUTOLOAD_ORDER` reader - The bits are used to configure the direction of autoload. 1: descending, 0: ascending."]
pub struct DCACHE_AUTOLOAD_ORDER_R(crate::FieldReader<bool, bool>);
impl DCACHE_AUTOLOAD_ORDER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCACHE_AUTOLOAD_ORDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_AUTOLOAD_ORDER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_AUTOLOAD_ORDER` writer - The bits are used to configure the direction of autoload. 1: descending, 0: ascending."]
pub struct DCACHE_AUTOLOAD_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_AUTOLOAD_ORDER_W<'a> {
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
#[doc = "Field `DCACHE_AUTOLOAD_RQST` reader - The bits are used to configure trigger conditions for autoload. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
pub struct DCACHE_AUTOLOAD_RQST_R(crate::FieldReader<u8, u8>);
impl DCACHE_AUTOLOAD_RQST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DCACHE_AUTOLOAD_RQST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_AUTOLOAD_RQST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_AUTOLOAD_RQST` writer - The bits are used to configure trigger conditions for autoload. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
pub struct DCACHE_AUTOLOAD_RQST_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_AUTOLOAD_RQST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `DCACHE_AUTOLOAD_SIZE` reader - The bits are used to configure the numbers of the cache block for the issuing autoload operation."]
pub struct DCACHE_AUTOLOAD_SIZE_R(crate::FieldReader<u8, u8>);
impl DCACHE_AUTOLOAD_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DCACHE_AUTOLOAD_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_AUTOLOAD_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_AUTOLOAD_SIZE` writer - The bits are used to configure the numbers of the cache block for the issuing autoload operation."]
pub struct DCACHE_AUTOLOAD_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_AUTOLOAD_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | ((value as u32 & 0x03) << 7);
        self.w
    }
}
#[doc = "Field `DCACHE_AUTOLOAD_BUFFER_CLEAR` reader - The bit is used to clear autoload buffer in dcache."]
pub struct DCACHE_AUTOLOAD_BUFFER_CLEAR_R(crate::FieldReader<bool, bool>);
impl DCACHE_AUTOLOAD_BUFFER_CLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCACHE_AUTOLOAD_BUFFER_CLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_AUTOLOAD_BUFFER_CLEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_AUTOLOAD_BUFFER_CLEAR` writer - The bit is used to clear autoload buffer in dcache."]
pub struct DCACHE_AUTOLOAD_BUFFER_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_AUTOLOAD_BUFFER_CLEAR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - The bits are used to enable the first section for autoload operation."]
    #[inline(always)]
    pub fn dcache_autoload_sct0_ena(&self) -> DCACHE_AUTOLOAD_SCT0_ENA_R {
        DCACHE_AUTOLOAD_SCT0_ENA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The bits are used to enable the second section for autoload operation."]
    #[inline(always)]
    pub fn dcache_autoload_sct1_ena(&self) -> DCACHE_AUTOLOAD_SCT1_ENA_R {
        DCACHE_AUTOLOAD_SCT1_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The bit is used to enable and disable autoload operation. It is combined with dcache_autoload_done. 1: enable, 0: disable."]
    #[inline(always)]
    pub fn dcache_autoload_ena(&self) -> DCACHE_AUTOLOAD_ENA_R {
        DCACHE_AUTOLOAD_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The bit is used to indicate autoload operation is finished."]
    #[inline(always)]
    pub fn dcache_autoload_done(&self) -> DCACHE_AUTOLOAD_DONE_R {
        DCACHE_AUTOLOAD_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The bits are used to configure the direction of autoload. 1: descending, 0: ascending."]
    #[inline(always)]
    pub fn dcache_autoload_order(&self) -> DCACHE_AUTOLOAD_ORDER_R {
        DCACHE_AUTOLOAD_ORDER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - The bits are used to configure trigger conditions for autoload. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
    #[inline(always)]
    pub fn dcache_autoload_rqst(&self) -> DCACHE_AUTOLOAD_RQST_R {
        DCACHE_AUTOLOAD_RQST_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 7:8 - The bits are used to configure the numbers of the cache block for the issuing autoload operation."]
    #[inline(always)]
    pub fn dcache_autoload_size(&self) -> DCACHE_AUTOLOAD_SIZE_R {
        DCACHE_AUTOLOAD_SIZE_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bit 9 - The bit is used to clear autoload buffer in dcache."]
    #[inline(always)]
    pub fn dcache_autoload_buffer_clear(&self) -> DCACHE_AUTOLOAD_BUFFER_CLEAR_R {
        DCACHE_AUTOLOAD_BUFFER_CLEAR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The bits are used to enable the first section for autoload operation."]
    #[inline(always)]
    pub fn dcache_autoload_sct0_ena(&mut self) -> DCACHE_AUTOLOAD_SCT0_ENA_W {
        DCACHE_AUTOLOAD_SCT0_ENA_W { w: self }
    }
    #[doc = "Bit 1 - The bits are used to enable the second section for autoload operation."]
    #[inline(always)]
    pub fn dcache_autoload_sct1_ena(&mut self) -> DCACHE_AUTOLOAD_SCT1_ENA_W {
        DCACHE_AUTOLOAD_SCT1_ENA_W { w: self }
    }
    #[doc = "Bit 2 - The bit is used to enable and disable autoload operation. It is combined with dcache_autoload_done. 1: enable, 0: disable."]
    #[inline(always)]
    pub fn dcache_autoload_ena(&mut self) -> DCACHE_AUTOLOAD_ENA_W {
        DCACHE_AUTOLOAD_ENA_W { w: self }
    }
    #[doc = "Bit 4 - The bits are used to configure the direction of autoload. 1: descending, 0: ascending."]
    #[inline(always)]
    pub fn dcache_autoload_order(&mut self) -> DCACHE_AUTOLOAD_ORDER_W {
        DCACHE_AUTOLOAD_ORDER_W { w: self }
    }
    #[doc = "Bits 5:6 - The bits are used to configure trigger conditions for autoload. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
    #[inline(always)]
    pub fn dcache_autoload_rqst(&mut self) -> DCACHE_AUTOLOAD_RQST_W {
        DCACHE_AUTOLOAD_RQST_W { w: self }
    }
    #[doc = "Bits 7:8 - The bits are used to configure the numbers of the cache block for the issuing autoload operation."]
    #[inline(always)]
    pub fn dcache_autoload_size(&mut self) -> DCACHE_AUTOLOAD_SIZE_W {
        DCACHE_AUTOLOAD_SIZE_W { w: self }
    }
    #[doc = "Bit 9 - The bit is used to clear autoload buffer in dcache."]
    #[inline(always)]
    pub fn dcache_autoload_buffer_clear(&mut self) -> DCACHE_AUTOLOAD_BUFFER_CLEAR_W {
        DCACHE_AUTOLOAD_BUFFER_CLEAR_W { w: self }
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
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcache_autoload_ctrl]
(index.html) module"]
pub struct DCACHE_AUTOLOAD_CTRL_SPEC;
impl crate::RegisterSpec for DCACHE_AUTOLOAD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcache_autoload_ctrl::R]
(R) reader structure"]
impl crate::Readable for DCACHE_AUTOLOAD_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcache_autoload_ctrl::W]
(W) writer structure"]
impl crate::Writable for DCACHE_AUTOLOAD_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCACHE_AUTOLOAD_CTRL to value 0x08"]
impl crate::Resettable for DCACHE_AUTOLOAD_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
