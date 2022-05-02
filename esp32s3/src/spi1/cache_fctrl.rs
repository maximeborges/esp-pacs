#[doc = "Register `CACHE_FCTRL` reader"]
pub struct R(crate::R<CACHE_FCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_FCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_FCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_FCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_FCTRL` writer"]
pub struct W(crate::W<CACHE_FCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_FCTRL_SPEC>;
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
impl From<crate::W<CACHE_FCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_FCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHE_USR_CMD_4BYTE` reader - Set this bit to enable SPI1 transfer with 32 bits address. The value of SPI_MEM_USR_ADDR_BITLEN should be 31."]
pub struct CACHE_USR_CMD_4BYTE_R(crate::FieldReader<bool>);
impl CACHE_USR_CMD_4BYTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_USR_CMD_4BYTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_USR_CMD_4BYTE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_USR_CMD_4BYTE` writer - Set this bit to enable SPI1 transfer with 32 bits address. The value of SPI_MEM_USR_ADDR_BITLEN should be 31."]
pub struct CACHE_USR_CMD_4BYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_USR_CMD_4BYTE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `FDIN_DUAL` reader - When SPI1 accesses to flash or Ext_RAM, set this bit to enable 2-bm in DIN phase."]
pub struct FDIN_DUAL_R(crate::FieldReader<bool>);
impl FDIN_DUAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FDIN_DUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDIN_DUAL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDIN_DUAL` writer - When SPI1 accesses to flash or Ext_RAM, set this bit to enable 2-bm in DIN phase."]
pub struct FDIN_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> FDIN_DUAL_W<'a> {
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
#[doc = "Field `FDOUT_DUAL` reader - When SPI1 accesses to flash or Ext_RAM, set this bit to enable 2-bm in DOUT phase."]
pub struct FDOUT_DUAL_R(crate::FieldReader<bool>);
impl FDOUT_DUAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FDOUT_DUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDOUT_DUAL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDOUT_DUAL` writer - When SPI1 accesses to flash or Ext_RAM, set this bit to enable 2-bm in DOUT phase."]
pub struct FDOUT_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> FDOUT_DUAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `FADDR_DUAL` reader - When SPI1 accesses to flash or Ext_RAM, set this bit to enable 2-bm in ADDR phase."]
pub struct FADDR_DUAL_R(crate::FieldReader<bool>);
impl FADDR_DUAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FADDR_DUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FADDR_DUAL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FADDR_DUAL` writer - When SPI1 accesses to flash or Ext_RAM, set this bit to enable 2-bm in ADDR phase."]
pub struct FADDR_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> FADDR_DUAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `FDIN_QUAD` reader - When SPI1 accesses to flash or Ext_RAM, set this bit to enable 4-bm in DIN phase."]
pub struct FDIN_QUAD_R(crate::FieldReader<bool>);
impl FDIN_QUAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FDIN_QUAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDIN_QUAD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDIN_QUAD` writer - When SPI1 accesses to flash or Ext_RAM, set this bit to enable 4-bm in DIN phase."]
pub struct FDIN_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> FDIN_QUAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `FDOUT_QUAD` reader - When SPI1 accesses to flash or Ext_RAM, set this bit to enable 4-bm in DOUT phase."]
pub struct FDOUT_QUAD_R(crate::FieldReader<bool>);
impl FDOUT_QUAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FDOUT_QUAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDOUT_QUAD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDOUT_QUAD` writer - When SPI1 accesses to flash or Ext_RAM, set this bit to enable 4-bm in DOUT phase."]
pub struct FDOUT_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> FDOUT_QUAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `FADDR_QUAD` reader - When SPI1 accesses to flash or Ext_RAM, set this bit to enable 4-bm in ADDR phase."]
pub struct FADDR_QUAD_R(crate::FieldReader<bool>);
impl FADDR_QUAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FADDR_QUAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FADDR_QUAD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FADDR_QUAD` writer - When SPI1 accesses to flash or Ext_RAM, set this bit to enable 4-bm in ADDR phase."]
pub struct FADDR_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> FADDR_QUAD_W<'a> {
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
    #[doc = "Bit 1 - Set this bit to enable SPI1 transfer with 32 bits address. The value of SPI_MEM_USR_ADDR_BITLEN should be 31."]
    #[inline(always)]
    pub fn cache_usr_cmd_4byte(&self) -> CACHE_USR_CMD_4BYTE_R {
        CACHE_USR_CMD_4BYTE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - When SPI1 accesses to flash or Ext_RAM, set this bit to enable 2-bm in DIN phase."]
    #[inline(always)]
    pub fn fdin_dual(&self) -> FDIN_DUAL_R {
        FDIN_DUAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When SPI1 accesses to flash or Ext_RAM, set this bit to enable 2-bm in DOUT phase."]
    #[inline(always)]
    pub fn fdout_dual(&self) -> FDOUT_DUAL_R {
        FDOUT_DUAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When SPI1 accesses to flash or Ext_RAM, set this bit to enable 2-bm in ADDR phase."]
    #[inline(always)]
    pub fn faddr_dual(&self) -> FADDR_DUAL_R {
        FADDR_DUAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When SPI1 accesses to flash or Ext_RAM, set this bit to enable 4-bm in DIN phase."]
    #[inline(always)]
    pub fn fdin_quad(&self) -> FDIN_QUAD_R {
        FDIN_QUAD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When SPI1 accesses to flash or Ext_RAM, set this bit to enable 4-bm in DOUT phase."]
    #[inline(always)]
    pub fn fdout_quad(&self) -> FDOUT_QUAD_R {
        FDOUT_QUAD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - When SPI1 accesses to flash or Ext_RAM, set this bit to enable 4-bm in ADDR phase."]
    #[inline(always)]
    pub fn faddr_quad(&self) -> FADDR_QUAD_R {
        FADDR_QUAD_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Set this bit to enable SPI1 transfer with 32 bits address. The value of SPI_MEM_USR_ADDR_BITLEN should be 31."]
    #[inline(always)]
    pub fn cache_usr_cmd_4byte(&mut self) -> CACHE_USR_CMD_4BYTE_W {
        CACHE_USR_CMD_4BYTE_W { w: self }
    }
    #[doc = "Bit 3 - When SPI1 accesses to flash or Ext_RAM, set this bit to enable 2-bm in DIN phase."]
    #[inline(always)]
    pub fn fdin_dual(&mut self) -> FDIN_DUAL_W {
        FDIN_DUAL_W { w: self }
    }
    #[doc = "Bit 4 - When SPI1 accesses to flash or Ext_RAM, set this bit to enable 2-bm in DOUT phase."]
    #[inline(always)]
    pub fn fdout_dual(&mut self) -> FDOUT_DUAL_W {
        FDOUT_DUAL_W { w: self }
    }
    #[doc = "Bit 5 - When SPI1 accesses to flash or Ext_RAM, set this bit to enable 2-bm in ADDR phase."]
    #[inline(always)]
    pub fn faddr_dual(&mut self) -> FADDR_DUAL_W {
        FADDR_DUAL_W { w: self }
    }
    #[doc = "Bit 6 - When SPI1 accesses to flash or Ext_RAM, set this bit to enable 4-bm in DIN phase."]
    #[inline(always)]
    pub fn fdin_quad(&mut self) -> FDIN_QUAD_W {
        FDIN_QUAD_W { w: self }
    }
    #[doc = "Bit 7 - When SPI1 accesses to flash or Ext_RAM, set this bit to enable 4-bm in DOUT phase."]
    #[inline(always)]
    pub fn fdout_quad(&mut self) -> FDOUT_QUAD_W {
        FDOUT_QUAD_W { w: self }
    }
    #[doc = "Bit 8 - When SPI1 accesses to flash or Ext_RAM, set this bit to enable 4-bm in ADDR phase."]
    #[inline(always)]
    pub fn faddr_quad(&mut self) -> FADDR_QUAD_W {
        FADDR_QUAD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 bit mode control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_fctrl](index.html) module"]
pub struct CACHE_FCTRL_SPEC;
impl crate::RegisterSpec for CACHE_FCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_fctrl::R](R) reader structure"]
impl crate::Readable for CACHE_FCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_fctrl::W](W) writer structure"]
impl crate::Writable for CACHE_FCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE_FCTRL to value 0"]
impl crate::Resettable for CACHE_FCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
