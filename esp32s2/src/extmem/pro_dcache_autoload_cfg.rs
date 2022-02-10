#[doc = "Register `PRO_DCACHE_AUTOLOAD_CFG` reader"]
pub struct R(crate::R<PRO_DCACHE_AUTOLOAD_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_DCACHE_AUTOLOAD_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_DCACHE_AUTOLOAD_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_DCACHE_AUTOLOAD_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_DCACHE_AUTOLOAD_CFG` writer"]
pub struct W(crate::W<PRO_DCACHE_AUTOLOAD_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_DCACHE_AUTOLOAD_CFG_SPEC>;
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
impl From<crate::W<PRO_DCACHE_AUTOLOAD_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_DCACHE_AUTOLOAD_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_DCACHE_AUTOLOAD_MODE` reader - Reserved."]
pub struct PRO_DCACHE_AUTOLOAD_MODE_R(crate::FieldReader<bool, bool>);
impl PRO_DCACHE_AUTOLOAD_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_DCACHE_AUTOLOAD_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DCACHE_AUTOLOAD_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DCACHE_AUTOLOAD_MODE` writer - Reserved."]
pub struct PRO_DCACHE_AUTOLOAD_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_DCACHE_AUTOLOAD_MODE_W<'a> {
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
#[doc = "Field `PRO_DCACHE_AUTOLOAD_STEP` reader - Reserved."]
pub struct PRO_DCACHE_AUTOLOAD_STEP_R(crate::FieldReader<u8, u8>);
impl PRO_DCACHE_AUTOLOAD_STEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRO_DCACHE_AUTOLOAD_STEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DCACHE_AUTOLOAD_STEP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DCACHE_AUTOLOAD_STEP` writer - Reserved."]
pub struct PRO_DCACHE_AUTOLOAD_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_DCACHE_AUTOLOAD_STEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `PRO_DCACHE_AUTOLOAD_ORDER` reader - The bits are used to configure the direction of conditional pre-load operation. 1: descending, 0: ascending."]
pub struct PRO_DCACHE_AUTOLOAD_ORDER_R(crate::FieldReader<bool, bool>);
impl PRO_DCACHE_AUTOLOAD_ORDER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_DCACHE_AUTOLOAD_ORDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DCACHE_AUTOLOAD_ORDER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DCACHE_AUTOLOAD_ORDER` writer - The bits are used to configure the direction of conditional pre-load operation. 1: descending, 0: ascending."]
pub struct PRO_DCACHE_AUTOLOAD_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_DCACHE_AUTOLOAD_ORDER_W<'a> {
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
#[doc = "Field `PRO_DCACHE_AUTOLOAD_RQST` reader - The bits are used to configure trigger conditions for conditional pre-load. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
pub struct PRO_DCACHE_AUTOLOAD_RQST_R(crate::FieldReader<u8, u8>);
impl PRO_DCACHE_AUTOLOAD_RQST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRO_DCACHE_AUTOLOAD_RQST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DCACHE_AUTOLOAD_RQST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DCACHE_AUTOLOAD_RQST` writer - The bits are used to configure trigger conditions for conditional pre-load. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
pub struct PRO_DCACHE_AUTOLOAD_RQST_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_DCACHE_AUTOLOAD_RQST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `PRO_DCACHE_AUTOLOAD_SIZE` reader - The bits are used to configure the numbers of the cache block for the issuing conditional pre-load operation."]
pub struct PRO_DCACHE_AUTOLOAD_SIZE_R(crate::FieldReader<u8, u8>);
impl PRO_DCACHE_AUTOLOAD_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRO_DCACHE_AUTOLOAD_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DCACHE_AUTOLOAD_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DCACHE_AUTOLOAD_SIZE` writer - The bits are used to configure the numbers of the cache block for the issuing conditional pre-load operation."]
pub struct PRO_DCACHE_AUTOLOAD_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_DCACHE_AUTOLOAD_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `PRO_DCACHE_AUTOLOAD_SCT0_ENA` reader - The bits are used to enable the second section for conditional pre-load operation."]
pub struct PRO_DCACHE_AUTOLOAD_SCT0_ENA_R(crate::FieldReader<bool, bool>);
impl PRO_DCACHE_AUTOLOAD_SCT0_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_DCACHE_AUTOLOAD_SCT0_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DCACHE_AUTOLOAD_SCT0_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DCACHE_AUTOLOAD_SCT0_ENA` writer - The bits are used to enable the second section for conditional pre-load operation."]
pub struct PRO_DCACHE_AUTOLOAD_SCT0_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_DCACHE_AUTOLOAD_SCT0_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `PRO_DCACHE_AUTOLOAD_SCT1_ENA` reader - The bits are used to enable the first section for conditional pre-load operation."]
pub struct PRO_DCACHE_AUTOLOAD_SCT1_ENA_R(crate::FieldReader<bool, bool>);
impl PRO_DCACHE_AUTOLOAD_SCT1_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_DCACHE_AUTOLOAD_SCT1_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DCACHE_AUTOLOAD_SCT1_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DCACHE_AUTOLOAD_SCT1_ENA` writer - The bits are used to enable the first section for conditional pre-load operation."]
pub struct PRO_DCACHE_AUTOLOAD_SCT1_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_DCACHE_AUTOLOAD_SCT1_ENA_W<'a> {
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
    #[doc = "Bit 0 - Reserved."]
    #[inline(always)]
    pub fn pro_dcache_autoload_mode(&self) -> PRO_DCACHE_AUTOLOAD_MODE_R {
        PRO_DCACHE_AUTOLOAD_MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Reserved."]
    #[inline(always)]
    pub fn pro_dcache_autoload_step(&self) -> PRO_DCACHE_AUTOLOAD_STEP_R {
        PRO_DCACHE_AUTOLOAD_STEP_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - The bits are used to configure the direction of conditional pre-load operation. 1: descending, 0: ascending."]
    #[inline(always)]
    pub fn pro_dcache_autoload_order(&self) -> PRO_DCACHE_AUTOLOAD_ORDER_R {
        PRO_DCACHE_AUTOLOAD_ORDER_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - The bits are used to configure trigger conditions for conditional pre-load. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
    #[inline(always)]
    pub fn pro_dcache_autoload_rqst(&self) -> PRO_DCACHE_AUTOLOAD_RQST_R {
        PRO_DCACHE_AUTOLOAD_RQST_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - The bits are used to configure the numbers of the cache block for the issuing conditional pre-load operation."]
    #[inline(always)]
    pub fn pro_dcache_autoload_size(&self) -> PRO_DCACHE_AUTOLOAD_SIZE_R {
        PRO_DCACHE_AUTOLOAD_SIZE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - The bits are used to enable the second section for conditional pre-load operation."]
    #[inline(always)]
    pub fn pro_dcache_autoload_sct0_ena(&self) -> PRO_DCACHE_AUTOLOAD_SCT0_ENA_R {
        PRO_DCACHE_AUTOLOAD_SCT0_ENA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - The bits are used to enable the first section for conditional pre-load operation."]
    #[inline(always)]
    pub fn pro_dcache_autoload_sct1_ena(&self) -> PRO_DCACHE_AUTOLOAD_SCT1_ENA_R {
        PRO_DCACHE_AUTOLOAD_SCT1_ENA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved."]
    #[inline(always)]
    pub fn pro_dcache_autoload_mode(&mut self) -> PRO_DCACHE_AUTOLOAD_MODE_W {
        PRO_DCACHE_AUTOLOAD_MODE_W { w: self }
    }
    #[doc = "Bits 1:2 - Reserved."]
    #[inline(always)]
    pub fn pro_dcache_autoload_step(&mut self) -> PRO_DCACHE_AUTOLOAD_STEP_W {
        PRO_DCACHE_AUTOLOAD_STEP_W { w: self }
    }
    #[doc = "Bit 3 - The bits are used to configure the direction of conditional pre-load operation. 1: descending, 0: ascending."]
    #[inline(always)]
    pub fn pro_dcache_autoload_order(&mut self) -> PRO_DCACHE_AUTOLOAD_ORDER_W {
        PRO_DCACHE_AUTOLOAD_ORDER_W { w: self }
    }
    #[doc = "Bits 4:5 - The bits are used to configure trigger conditions for conditional pre-load. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
    #[inline(always)]
    pub fn pro_dcache_autoload_rqst(&mut self) -> PRO_DCACHE_AUTOLOAD_RQST_W {
        PRO_DCACHE_AUTOLOAD_RQST_W { w: self }
    }
    #[doc = "Bits 6:7 - The bits are used to configure the numbers of the cache block for the issuing conditional pre-load operation."]
    #[inline(always)]
    pub fn pro_dcache_autoload_size(&mut self) -> PRO_DCACHE_AUTOLOAD_SIZE_W {
        PRO_DCACHE_AUTOLOAD_SIZE_W { w: self }
    }
    #[doc = "Bit 8 - The bits are used to enable the second section for conditional pre-load operation."]
    #[inline(always)]
    pub fn pro_dcache_autoload_sct0_ena(&mut self) -> PRO_DCACHE_AUTOLOAD_SCT0_ENA_W {
        PRO_DCACHE_AUTOLOAD_SCT0_ENA_W { w: self }
    }
    #[doc = "Bit 9 - The bits are used to enable the first section for conditional pre-load operation."]
    #[inline(always)]
    pub fn pro_dcache_autoload_sct1_ena(&mut self) -> PRO_DCACHE_AUTOLOAD_SCT1_ENA_W {
        PRO_DCACHE_AUTOLOAD_SCT1_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_dcache_autoload_cfg]
(index.html) module"]
pub struct PRO_DCACHE_AUTOLOAD_CFG_SPEC;
impl crate::RegisterSpec for PRO_DCACHE_AUTOLOAD_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_dcache_autoload_cfg::R]
(R) reader structure"]
impl crate::Readable for PRO_DCACHE_AUTOLOAD_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_dcache_autoload_cfg::W]
(W) writer structure"]
impl crate::Writable for PRO_DCACHE_AUTOLOAD_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_DCACHE_AUTOLOAD_CFG to value 0"]
impl crate::Resettable for PRO_DCACHE_AUTOLOAD_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
