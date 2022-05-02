#[doc = "Register `AHBLITE_MPU_TABLE_I2C_EXT1` reader"]
pub struct R(crate::R<AHBLITE_MPU_TABLE_I2C_EXT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBLITE_MPU_TABLE_I2C_EXT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBLITE_MPU_TABLE_I2C_EXT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBLITE_MPU_TABLE_I2C_EXT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBLITE_MPU_TABLE_I2C_EXT1` writer"]
pub struct W(crate::W<AHBLITE_MPU_TABLE_I2C_EXT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBLITE_MPU_TABLE_I2C_EXT1_SPEC>;
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
impl From<crate::W<AHBLITE_MPU_TABLE_I2C_EXT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBLITE_MPU_TABLE_I2C_EXT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2CEXT1_ACCESS_GRANT_CONFIG` reader - "]
pub struct I2CEXT1_ACCESS_GRANT_CONFIG_R(crate::FieldReader<u8>);
impl I2CEXT1_ACCESS_GRANT_CONFIG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        I2CEXT1_ACCESS_GRANT_CONFIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2CEXT1_ACCESS_GRANT_CONFIG_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2CEXT1_ACCESS_GRANT_CONFIG` writer - "]
pub struct I2CEXT1_ACCESS_GRANT_CONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CEXT1_ACCESS_GRANT_CONFIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn i2cext1_access_grant_config(&self) -> I2CEXT1_ACCESS_GRANT_CONFIG_R {
        I2CEXT1_ACCESS_GRANT_CONFIG_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn i2cext1_access_grant_config(&mut self) -> I2CEXT1_ACCESS_GRANT_CONFIG_W {
        I2CEXT1_ACCESS_GRANT_CONFIG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahblite_mpu_table_i2c_ext1](index.html) module"]
pub struct AHBLITE_MPU_TABLE_I2C_EXT1_SPEC;
impl crate::RegisterSpec for AHBLITE_MPU_TABLE_I2C_EXT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahblite_mpu_table_i2c_ext1::R](R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_I2C_EXT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_i2c_ext1::W](W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_I2C_EXT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBLITE_MPU_TABLE_I2C_EXT1 to value 0"]
impl crate::Resettable for AHBLITE_MPU_TABLE_I2C_EXT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
