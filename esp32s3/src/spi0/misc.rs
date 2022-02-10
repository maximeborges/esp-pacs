#[doc = "Register `MISC` reader"]
pub struct R(crate::R<MISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC` writer"]
pub struct W(crate::W<MISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC_SPEC>;
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
impl From<crate::W<MISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS0_DIS` reader - Set this bit to raise high SPI_CS pin, which means that the SPI device(flash) connected to SPI_CS is in low level when SPI1 transfer starts."]
pub struct CS0_DIS_R(crate::FieldReader<bool, bool>);
impl CS0_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CS0_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS0_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS0_DIS` writer - Set this bit to raise high SPI_CS pin, which means that the SPI device(flash) connected to SPI_CS is in low level when SPI1 transfer starts."]
pub struct CS0_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CS0_DIS_W<'a> {
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
#[doc = "Field `CS1_DIS` reader - Set this bit to raise high SPI_CS1 pin, which means that the SPI device(Ext_RAM) connected to SPI_CS1 is in low level when SPI1 transfer starts."]
pub struct CS1_DIS_R(crate::FieldReader<bool, bool>);
impl CS1_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CS1_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS1_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS1_DIS` writer - Set this bit to raise high SPI_CS1 pin, which means that the SPI device(Ext_RAM) connected to SPI_CS1 is in low level when SPI1 transfer starts."]
pub struct CS1_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CS1_DIS_W<'a> {
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
#[doc = "Field `CK_IDLE_EDGE` reader - 1: SPI_CLK line is high when MSPI is idle. 0: SPI_CLK line is low when MSPI is idle."]
pub struct CK_IDLE_EDGE_R(crate::FieldReader<bool, bool>);
impl CK_IDLE_EDGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CK_IDLE_EDGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CK_IDLE_EDGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK_IDLE_EDGE` writer - 1: SPI_CLK line is high when MSPI is idle. 0: SPI_CLK line is low when MSPI is idle."]
pub struct CK_IDLE_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> CK_IDLE_EDGE_W<'a> {
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
#[doc = "Field `CS_KEEP_ACTIVE` reader - SPI_CS line keep low when the bit is set."]
pub struct CS_KEEP_ACTIVE_R(crate::FieldReader<bool, bool>);
impl CS_KEEP_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CS_KEEP_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_KEEP_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS_KEEP_ACTIVE` writer - SPI_CS line keep low when the bit is set."]
pub struct CS_KEEP_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_KEEP_ACTIVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `AUTO_PER` reader - Set this bit to enable auto PER function. Hardware will sent out PER command if PES command is sent."]
pub struct AUTO_PER_R(crate::FieldReader<bool, bool>);
impl AUTO_PER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTO_PER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTO_PER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTO_PER` writer - Set this bit to enable auto PER function. Hardware will sent out PER command if PES command is sent."]
pub struct AUTO_PER_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_PER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set this bit to raise high SPI_CS pin, which means that the SPI device(flash) connected to SPI_CS is in low level when SPI1 transfer starts."]
    #[inline(always)]
    pub fn cs0_dis(&self) -> CS0_DIS_R {
        CS0_DIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set this bit to raise high SPI_CS1 pin, which means that the SPI device(Ext_RAM) connected to SPI_CS1 is in low level when SPI1 transfer starts."]
    #[inline(always)]
    pub fn cs1_dis(&self) -> CS1_DIS_R {
        CS1_DIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 1: SPI_CLK line is high when MSPI is idle. 0: SPI_CLK line is low when MSPI is idle."]
    #[inline(always)]
    pub fn ck_idle_edge(&self) -> CK_IDLE_EDGE_R {
        CK_IDLE_EDGE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SPI_CS line keep low when the bit is set."]
    #[inline(always)]
    pub fn cs_keep_active(&self) -> CS_KEEP_ACTIVE_R {
        CS_KEEP_ACTIVE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Set this bit to enable auto PER function. Hardware will sent out PER command if PES command is sent."]
    #[inline(always)]
    pub fn auto_per(&self) -> AUTO_PER_R {
        AUTO_PER_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to raise high SPI_CS pin, which means that the SPI device(flash) connected to SPI_CS is in low level when SPI1 transfer starts."]
    #[inline(always)]
    pub fn cs0_dis(&mut self) -> CS0_DIS_W {
        CS0_DIS_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to raise high SPI_CS1 pin, which means that the SPI device(Ext_RAM) connected to SPI_CS1 is in low level when SPI1 transfer starts."]
    #[inline(always)]
    pub fn cs1_dis(&mut self) -> CS1_DIS_W {
        CS1_DIS_W { w: self }
    }
    #[doc = "Bit 9 - 1: SPI_CLK line is high when MSPI is idle. 0: SPI_CLK line is low when MSPI is idle."]
    #[inline(always)]
    pub fn ck_idle_edge(&mut self) -> CK_IDLE_EDGE_W {
        CK_IDLE_EDGE_W { w: self }
    }
    #[doc = "Bit 10 - SPI_CS line keep low when the bit is set."]
    #[inline(always)]
    pub fn cs_keep_active(&mut self) -> CS_KEEP_ACTIVE_W {
        CS_KEEP_ACTIVE_W { w: self }
    }
    #[doc = "Bit 11 - Set this bit to enable auto PER function. Hardware will sent out PER command if PES command is sent."]
    #[inline(always)]
    pub fn auto_per(&mut self) -> AUTO_PER_W {
        AUTO_PER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 misc register.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc]
(index.html) module"]
pub struct MISC_SPEC;
impl crate::RegisterSpec for MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc::R]
(R) reader structure"]
impl crate::Readable for MISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc::W]
(W) writer structure"]
impl crate::Writable for MISC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MISC to value 0x02"]
impl crate::Resettable for MISC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
