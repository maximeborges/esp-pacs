#[doc = "Register `CONFIG` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG` writer"]
pub struct W(crate::W<CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_SPEC>;
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
impl From<crate::W<CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLOW_ERR` reader - x"]
pub struct FLOW_ERR_R(crate::FieldReader<u8, u8>);
impl FLOW_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FLOW_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLOW_ERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR_MAP_MODE` reader - x"]
pub struct ADDR_MAP_MODE_R(crate::FieldReader<bool, bool>);
impl ADDR_MAP_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADDR_MAP_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_MAP_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR_MAP_MODE` writer - x"]
pub struct ADDR_MAP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_MAP_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `BURST_LIMIT` reader - x"]
pub struct BURST_LIMIT_R(crate::FieldReader<u8, u8>);
impl BURST_LIMIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BURST_LIMIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BURST_LIMIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BURST_LIMIT` writer - x"]
pub struct BURST_LIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> BURST_LIMIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 4)) | ((value as u32 & 0x1f) << 4);
        self.w
    }
}
#[doc = "Field `TOUT_THRES` reader - x"]
pub struct TOUT_THRES_R(crate::FieldReader<u16, u16>);
impl TOUT_THRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TOUT_THRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUT_THRES_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUT_THRES` writer - x"]
pub struct TOUT_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUT_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 9)) | ((value as u32 & 0x03ff) << 9);
        self.w
    }
}
#[doc = "Field `SIZE` reader - x"]
pub struct SIZE_R(crate::FieldReader<u16, u16>);
impl SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIZE` writer - x"]
pub struct SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 19)) | ((value as u32 & 0x03ff) << 19);
        self.w
    }
}
#[doc = "Field `START` writer - x"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 29)) | ((value as u32 & 1) << 29);
        self.w
    }
}
#[doc = "Field `TO_MEM` reader - x"]
pub struct TO_MEM_R(crate::FieldReader<bool, bool>);
impl TO_MEM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TO_MEM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TO_MEM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TO_MEM` writer - x"]
pub struct TO_MEM_W<'a> {
    w: &'a mut W,
}
impl<'a> TO_MEM_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "Field `ENA` reader - x"]
pub struct ENA_R(crate::FieldReader<bool, bool>);
impl ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA` writer - x"]
pub struct ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - x"]
    #[inline(always)]
    pub fn flow_err(&self) -> FLOW_ERR_R {
        FLOW_ERR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - x"]
    #[inline(always)]
    pub fn addr_map_mode(&self) -> ADDR_MAP_MODE_R {
        ADDR_MAP_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:8 - x"]
    #[inline(always)]
    pub fn burst_limit(&self) -> BURST_LIMIT_R {
        BURST_LIMIT_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:18 - x"]
    #[inline(always)]
    pub fn tout_thres(&self) -> TOUT_THRES_R {
        TOUT_THRES_R::new(((self.bits >> 9) & 0x03ff) as u16)
    }
    #[doc = "Bits 19:28 - x"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - x"]
    #[inline(always)]
    pub fn to_mem(&self) -> TO_MEM_R {
        TO_MEM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - x"]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - x"]
    #[inline(always)]
    pub fn addr_map_mode(&mut self) -> ADDR_MAP_MODE_W {
        ADDR_MAP_MODE_W { w: self }
    }
    #[doc = "Bits 4:8 - x"]
    #[inline(always)]
    pub fn burst_limit(&mut self) -> BURST_LIMIT_W {
        BURST_LIMIT_W { w: self }
    }
    #[doc = "Bits 9:18 - x"]
    #[inline(always)]
    pub fn tout_thres(&mut self) -> TOUT_THRES_W {
        TOUT_THRES_W { w: self }
    }
    #[doc = "Bits 19:28 - x"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W {
        SIZE_W { w: self }
    }
    #[doc = "Bit 29 - x"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 30 - x"]
    #[inline(always)]
    pub fn to_mem(&mut self) -> TO_MEM_W {
        TO_MEM_W { w: self }
    }
    #[doc = "Bit 31 - x"]
    #[inline(always)]
    pub fn ena(&mut self) -> ENA_W {
        ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "x\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config]
(index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config::R]
(R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config::W]
(W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFIG to value 0x6480"]
impl crate::Resettable for CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x6480
    }
}
