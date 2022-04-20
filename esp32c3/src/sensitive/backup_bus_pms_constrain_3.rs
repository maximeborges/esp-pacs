#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_3` reader"]
pub struct R(crate::R<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_3` writer"]
pub struct W(crate::W<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC>;
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
impl From<crate::W<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SPI_2` reader - backup_bus_pms_constrain_spi_2"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_SPI_2_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_SPI_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_SPI_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_SPI_2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SPI_2` writer - backup_bus_pms_constrain_spi_2"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_SPI_2_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_SPI_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_APB_CTRL` reader - backup_bus_pms_constrain_apb_ctrl"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_APB_CTRL_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_APB_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_APB_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_APB_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_APB_CTRL` writer - backup_bus_pms_constrain_apb_ctrl"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_APB_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_APB_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_CAN` reader - backup_bus_pms_constrain_can"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_CAN_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_CAN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_CAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_CAN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_CAN` writer - backup_bus_pms_constrain_can"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_CAN_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_CAN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 10)) | ((value as u32 & 3) << 10);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_I2S1` reader - backup_bus_pms_constrain_i2s1"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_I2S1_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_I2S1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_I2S1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_I2S1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_I2S1` writer - backup_bus_pms_constrain_i2s1"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_I2S1_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_I2S1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 14)) | ((value as u32 & 3) << 14);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RWBT` reader - backup_bus_pms_constrain_rwbt"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_RWBT_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_RWBT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_RWBT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_RWBT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RWBT` writer - backup_bus_pms_constrain_rwbt"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_RWBT_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_RWBT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 22)) | ((value as u32 & 3) << 22);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_WIFIMAC` reader - backup_bus_pms_constrain_wifimac"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_WIFIMAC_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_WIFIMAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_WIFIMAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_WIFIMAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_WIFIMAC` writer - backup_bus_pms_constrain_wifimac"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_WIFIMAC_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_WIFIMAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 26)) | ((value as u32 & 3) << 26);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_PWR` reader - backup_bus_pms_constrain_pwr"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_PWR_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_PWR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_PWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_PWR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_PWR` writer - backup_bus_pms_constrain_pwr"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_PWR_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_PWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 28)) | ((value as u32 & 3) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - backup_bus_pms_constrain_spi_2"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_spi_2(&self) -> BACKUP_BUS_PMS_CONSTRAIN_SPI_2_R {
        BACKUP_BUS_PMS_CONSTRAIN_SPI_2_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - backup_bus_pms_constrain_apb_ctrl"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_apb_ctrl(&self) -> BACKUP_BUS_PMS_CONSTRAIN_APB_CTRL_R {
        BACKUP_BUS_PMS_CONSTRAIN_APB_CTRL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 10:11 - backup_bus_pms_constrain_can"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_can(&self) -> BACKUP_BUS_PMS_CONSTRAIN_CAN_R {
        BACKUP_BUS_PMS_CONSTRAIN_CAN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 14:15 - backup_bus_pms_constrain_i2s1"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_i2s1(&self) -> BACKUP_BUS_PMS_CONSTRAIN_I2S1_R {
        BACKUP_BUS_PMS_CONSTRAIN_I2S1_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 22:23 - backup_bus_pms_constrain_rwbt"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_rwbt(&self) -> BACKUP_BUS_PMS_CONSTRAIN_RWBT_R {
        BACKUP_BUS_PMS_CONSTRAIN_RWBT_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 26:27 - backup_bus_pms_constrain_wifimac"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_wifimac(&self) -> BACKUP_BUS_PMS_CONSTRAIN_WIFIMAC_R {
        BACKUP_BUS_PMS_CONSTRAIN_WIFIMAC_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - backup_bus_pms_constrain_pwr"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_pwr(&self) -> BACKUP_BUS_PMS_CONSTRAIN_PWR_R {
        BACKUP_BUS_PMS_CONSTRAIN_PWR_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - backup_bus_pms_constrain_spi_2"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_spi_2(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_SPI_2_W {
        BACKUP_BUS_PMS_CONSTRAIN_SPI_2_W { w: self }
    }
    #[doc = "Bits 4:5 - backup_bus_pms_constrain_apb_ctrl"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_apb_ctrl(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_APB_CTRL_W {
        BACKUP_BUS_PMS_CONSTRAIN_APB_CTRL_W { w: self }
    }
    #[doc = "Bits 10:11 - backup_bus_pms_constrain_can"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_can(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_CAN_W {
        BACKUP_BUS_PMS_CONSTRAIN_CAN_W { w: self }
    }
    #[doc = "Bits 14:15 - backup_bus_pms_constrain_i2s1"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_i2s1(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_I2S1_W {
        BACKUP_BUS_PMS_CONSTRAIN_I2S1_W { w: self }
    }
    #[doc = "Bits 22:23 - backup_bus_pms_constrain_rwbt"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_rwbt(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_RWBT_W {
        BACKUP_BUS_PMS_CONSTRAIN_RWBT_W { w: self }
    }
    #[doc = "Bits 26:27 - backup_bus_pms_constrain_wifimac"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_wifimac(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_WIFIMAC_W {
        BACKUP_BUS_PMS_CONSTRAIN_WIFIMAC_W { w: self }
    }
    #[doc = "Bits 28:29 - backup_bus_pms_constrain_pwr"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_pwr(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_PWR_W {
        BACKUP_BUS_PMS_CONSTRAIN_PWR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_3_REG\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backup_bus_pms_constrain_3]
(index.html) module"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_3_SPEC;
impl crate::RegisterSpec for BACKUP_BUS_PMS_CONSTRAIN_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [backup_bus_pms_constrain_3::R]
(R) reader structure"]
impl crate::Readable for BACKUP_BUS_PMS_CONSTRAIN_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [backup_bus_pms_constrain_3::W]
(W) writer structure"]
impl crate::Writable for BACKUP_BUS_PMS_CONSTRAIN_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BACKUP_BUS_PMS_CONSTRAIN_3 to value 0x3cc0_cc33"]
impl crate::Resettable for BACKUP_BUS_PMS_CONSTRAIN_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3cc0_cc33
    }
}
