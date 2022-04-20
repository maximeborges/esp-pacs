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
#[doc = "Field `TRANS_END` reader - The bit is used to indicate the spi0_mst_st controlled transmitting is done."]
pub struct TRANS_END_R(crate::FieldReader<bool, bool>);
impl TRANS_END_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRANS_END_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANS_END_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANS_END` writer - The bit is used to indicate the spi0_mst_st controlled transmitting is done."]
pub struct TRANS_END_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_END_W<'a> {
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
#[doc = "Field `TRANS_END_INT_ENA` reader - The bit is used to enable the interrupt of spi0_mst_st controlled transmitting is done."]
pub struct TRANS_END_INT_ENA_R(crate::FieldReader<bool, bool>);
impl TRANS_END_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRANS_END_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANS_END_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANS_END_INT_ENA` writer - The bit is used to enable the interrupt of spi0_mst_st controlled transmitting is done."]
pub struct TRANS_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_END_INT_ENA_W<'a> {
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
#[doc = "Field `CSPI_ST_TRANS_END` reader - The bit is used to indicate the spi0_slv_st controlled transmitting is done."]
pub struct CSPI_ST_TRANS_END_R(crate::FieldReader<bool, bool>);
impl CSPI_ST_TRANS_END_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSPI_ST_TRANS_END_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSPI_ST_TRANS_END_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSPI_ST_TRANS_END` writer - The bit is used to indicate the spi0_slv_st controlled transmitting is done."]
pub struct CSPI_ST_TRANS_END_W<'a> {
    w: &'a mut W,
}
impl<'a> CSPI_ST_TRANS_END_W<'a> {
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
#[doc = "Field `CSPI_ST_TRANS_END_INT_ENA` reader - The bit is used to enable the interrupt of spi0_slv_st controlled transmitting is done."]
pub struct CSPI_ST_TRANS_END_INT_ENA_R(crate::FieldReader<bool, bool>);
impl CSPI_ST_TRANS_END_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSPI_ST_TRANS_END_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSPI_ST_TRANS_END_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSPI_ST_TRANS_END_INT_ENA` writer - The bit is used to enable the interrupt of spi0_slv_st controlled transmitting is done."]
pub struct CSPI_ST_TRANS_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CSPI_ST_TRANS_END_INT_ENA_W<'a> {
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
#[doc = "Field `CK_IDLE_EDGE` reader - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
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
#[doc = "Field `CK_IDLE_EDGE` writer - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "Field `CS_KEEP_ACTIVE` reader - spi cs line keep low when the bit is set."]
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
#[doc = "Field `CS_KEEP_ACTIVE` writer - spi cs line keep low when the bit is set."]
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - The bit is used to indicate the spi0_mst_st controlled transmitting is done."]
    #[inline(always)]
    pub fn trans_end(&self) -> TRANS_END_R {
        TRANS_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to enable the interrupt of spi0_mst_st controlled transmitting is done."]
    #[inline(always)]
    pub fn trans_end_int_ena(&self) -> TRANS_END_INT_ENA_R {
        TRANS_END_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The bit is used to indicate the spi0_slv_st controlled transmitting is done."]
    #[inline(always)]
    pub fn cspi_st_trans_end(&self) -> CSPI_ST_TRANS_END_R {
        CSPI_ST_TRANS_END_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The bit is used to enable the interrupt of spi0_slv_st controlled transmitting is done."]
    #[inline(always)]
    pub fn cspi_st_trans_end_int_ena(&self) -> CSPI_ST_TRANS_END_INT_ENA_R {
        CSPI_ST_TRANS_END_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    pub fn ck_idle_edge(&self) -> CK_IDLE_EDGE_R {
        CK_IDLE_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - spi cs line keep low when the bit is set."]
    #[inline(always)]
    pub fn cs_keep_active(&self) -> CS_KEEP_ACTIVE_R {
        CS_KEEP_ACTIVE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - The bit is used to indicate the spi0_mst_st controlled transmitting is done."]
    #[inline(always)]
    pub fn trans_end(&mut self) -> TRANS_END_W {
        TRANS_END_W { w: self }
    }
    #[doc = "Bit 4 - The bit is used to enable the interrupt of spi0_mst_st controlled transmitting is done."]
    #[inline(always)]
    pub fn trans_end_int_ena(&mut self) -> TRANS_END_INT_ENA_W {
        TRANS_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5 - The bit is used to indicate the spi0_slv_st controlled transmitting is done."]
    #[inline(always)]
    pub fn cspi_st_trans_end(&mut self) -> CSPI_ST_TRANS_END_W {
        CSPI_ST_TRANS_END_W { w: self }
    }
    #[doc = "Bit 6 - The bit is used to enable the interrupt of spi0_slv_st controlled transmitting is done."]
    #[inline(always)]
    pub fn cspi_st_trans_end_int_ena(&mut self) -> CSPI_ST_TRANS_END_INT_ENA_W {
        CSPI_ST_TRANS_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 9 - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    pub fn ck_idle_edge(&mut self) -> CK_IDLE_EDGE_W {
        CK_IDLE_EDGE_W { w: self }
    }
    #[doc = "Bit 10 - spi cs line keep low when the bit is set."]
    #[inline(always)]
    pub fn cs_keep_active(&mut self) -> CS_KEEP_ACTIVE_W {
        CS_KEEP_ACTIVE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 misc register\n\nThis register you can [`read`]
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
#[doc = "`reset()` method sets MISC to value 0"]
impl crate::Resettable for MISC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
