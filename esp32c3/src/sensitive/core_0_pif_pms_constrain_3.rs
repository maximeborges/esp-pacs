#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_3` reader"]
pub struct R(crate::R<CORE_0_PIF_PMS_CONSTRAIN_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_PIF_PMS_CONSTRAIN_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_PIF_PMS_CONSTRAIN_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_PIF_PMS_CONSTRAIN_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_3` writer"]
pub struct W(crate::W<CORE_0_PIF_PMS_CONSTRAIN_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_PIF_PMS_CONSTRAIN_3_SPEC>;
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
impl From<crate::W<CORE_0_PIF_PMS_CONSTRAIN_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_PIF_PMS_CONSTRAIN_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2` reader - core_0_pif_pms_constrain_world_0_spi_2"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2` writer - core_0_pif_pms_constrain_world_0_spi_2"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL` reader - core_0_pif_pms_constrain_world_0_apb_ctrl"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL` writer - core_0_pif_pms_constrain_world_0_apb_ctrl"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CAN` reader - core_0_pif_pms_constrain_world_0_can"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CAN_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CAN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CAN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CAN` writer - core_0_pif_pms_constrain_world_0_can"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CAN_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CAN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 10)) | ((value as u32 & 3) << 10);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2S1` reader - core_0_pif_pms_constrain_world_0_i2s1"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2S1` writer - core_0_pif_pms_constrain_world_0_i2s1"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 14)) | ((value as u32 & 3) << 14);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RWBT` reader - core_0_pif_pms_constrain_world_0_rwbt"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RWBT` writer - core_0_pif_pms_constrain_world_0_rwbt"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 22)) | ((value as u32 & 3) << 22);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC` reader - core_0_pif_pms_constrain_world_0_wifimac"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC` writer - core_0_pif_pms_constrain_world_0_wifimac"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 26)) | ((value as u32 & 3) << 26);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWR` reader - core_0_pif_pms_constrain_world_0_pwr"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWR_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWR` writer - core_0_pif_pms_constrain_world_0_pwr"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWR_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 28)) | ((value as u32 & 3) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - core_0_pif_pms_constrain_world_0_spi_2"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_spi_2(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - core_0_pif_pms_constrain_world_0_apb_ctrl"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_apb_ctrl(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 10:11 - core_0_pif_pms_constrain_world_0_can"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_can(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CAN_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CAN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 14:15 - core_0_pif_pms_constrain_world_0_i2s1"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_i2s1(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 22:23 - core_0_pif_pms_constrain_world_0_rwbt"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_rwbt(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 26:27 - core_0_pif_pms_constrain_world_0_wifimac"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_wifimac(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - core_0_pif_pms_constrain_world_0_pwr"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_pwr(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWR_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWR_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - core_0_pif_pms_constrain_world_0_spi_2"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_spi_2(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_W { w: self }
    }
    #[doc = "Bits 4:5 - core_0_pif_pms_constrain_world_0_apb_ctrl"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_apb_ctrl(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_W { w: self }
    }
    #[doc = "Bits 10:11 - core_0_pif_pms_constrain_world_0_can"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_can(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CAN_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CAN_W { w: self }
    }
    #[doc = "Bits 14:15 - core_0_pif_pms_constrain_world_0_i2s1"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_i2s1(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_W { w: self }
    }
    #[doc = "Bits 22:23 - core_0_pif_pms_constrain_world_0_rwbt"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_rwbt(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_W { w: self }
    }
    #[doc = "Bits 26:27 - core_0_pif_pms_constrain_world_0_wifimac"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_wifimac(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_W { w: self }
    }
    #[doc = "Bits 28:29 - core_0_pif_pms_constrain_world_0_pwr"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_pwr(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWR_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_3_REG\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_pif_pms_constrain_3]
(index.html) module"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_3_SPEC;
impl crate::RegisterSpec for CORE_0_PIF_PMS_CONSTRAIN_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_pif_pms_constrain_3::R]
(R) reader structure"]
impl crate::Readable for CORE_0_PIF_PMS_CONSTRAIN_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_pif_pms_constrain_3::W]
(W) writer structure"]
impl crate::Writable for CORE_0_PIF_PMS_CONSTRAIN_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_0_PIF_PMS_CONSTRAIN_3 to value 0x3cc0_cc33"]
impl crate::Resettable for CORE_0_PIF_PMS_CONSTRAIN_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3cc0_cc33
    }
}
