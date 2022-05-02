#[doc = "Register `SAR_SLAVE_ADDR3` reader"]
pub struct R(crate::R<SAR_SLAVE_ADDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_SLAVE_ADDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_SLAVE_ADDR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_SLAVE_ADDR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_SLAVE_ADDR3` writer"]
pub struct W(crate::W<SAR_SLAVE_ADDR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_SLAVE_ADDR3_SPEC>;
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
impl From<crate::W<SAR_SLAVE_ADDR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_SLAVE_ADDR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_SLAVE_ADDR5` reader - RTC I2C slave address 5"]
pub struct I2C_SLAVE_ADDR5_R(crate::FieldReader<u16>);
impl I2C_SLAVE_ADDR5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        I2C_SLAVE_ADDR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_SLAVE_ADDR5_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_SLAVE_ADDR5` writer - RTC I2C slave address 5"]
pub struct I2C_SLAVE_ADDR5_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SLAVE_ADDR5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
#[doc = "Field `I2C_SLAVE_ADDR4` reader - RTC I2C slave address 4"]
pub struct I2C_SLAVE_ADDR4_R(crate::FieldReader<u16>);
impl I2C_SLAVE_ADDR4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        I2C_SLAVE_ADDR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_SLAVE_ADDR4_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_SLAVE_ADDR4` writer - RTC I2C slave address 4"]
pub struct I2C_SLAVE_ADDR4_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SLAVE_ADDR4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 11)) | ((value as u32 & 0x07ff) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - RTC I2C slave address 5"]
    #[inline(always)]
    pub fn i2c_slave_addr5(&self) -> I2C_SLAVE_ADDR5_R {
        I2C_SLAVE_ADDR5_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - RTC I2C slave address 4"]
    #[inline(always)]
    pub fn i2c_slave_addr4(&self) -> I2C_SLAVE_ADDR4_R {
        I2C_SLAVE_ADDR4_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - RTC I2C slave address 5"]
    #[inline(always)]
    pub fn i2c_slave_addr5(&mut self) -> I2C_SLAVE_ADDR5_W {
        I2C_SLAVE_ADDR5_W { w: self }
    }
    #[doc = "Bits 11:21 - RTC I2C slave address 4"]
    #[inline(always)]
    pub fn i2c_slave_addr4(&mut self) -> I2C_SLAVE_ADDR4_W {
        I2C_SLAVE_ADDR4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure slave addresses 4-5 of RTC I2C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_slave_addr3](index.html) module"]
pub struct SAR_SLAVE_ADDR3_SPEC;
impl crate::RegisterSpec for SAR_SLAVE_ADDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_slave_addr3::R](R) reader structure"]
impl crate::Readable for SAR_SLAVE_ADDR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_slave_addr3::W](W) writer structure"]
impl crate::Writable for SAR_SLAVE_ADDR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_SLAVE_ADDR3 to value 0"]
impl crate::Resettable for SAR_SLAVE_ADDR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
