#[doc = "Register `SCL_STRETCH_CONF` reader"]
pub struct R(crate::R<SCL_STRETCH_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCL_STRETCH_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCL_STRETCH_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCL_STRETCH_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCL_STRETCH_CONF` writer"]
pub struct W(crate::W<SCL_STRETCH_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCL_STRETCH_CONF_SPEC>;
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
impl From<crate::W<SCL_STRETCH_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCL_STRETCH_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STRETCH_PROTECT_NUM` reader - Configure the period of I2C slave stretching SCL line."]
pub struct STRETCH_PROTECT_NUM_R(crate::FieldReader<u16, u16>);
impl STRETCH_PROTECT_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        STRETCH_PROTECT_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STRETCH_PROTECT_NUM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STRETCH_PROTECT_NUM` writer - Configure the period of I2C slave stretching SCL line."]
pub struct STRETCH_PROTECT_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> STRETCH_PROTECT_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `SLAVE_SCL_STRETCH_EN` reader - The enable bit for slave SCL stretch function. 1: Enable. 0: Disable. The SCL output line will be stretched low when I2C_SLAVE_SCL_STRETCH_EN is 1 and stretch event happens. The stretch cause can be seen in I2C_STRETCH_CAUSE."]
pub struct SLAVE_SCL_STRETCH_EN_R(crate::FieldReader<bool, bool>);
impl SLAVE_SCL_STRETCH_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_SCL_STRETCH_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_SCL_STRETCH_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_SCL_STRETCH_EN` writer - The enable bit for slave SCL stretch function. 1: Enable. 0: Disable. The SCL output line will be stretched low when I2C_SLAVE_SCL_STRETCH_EN is 1 and stretch event happens. The stretch cause can be seen in I2C_STRETCH_CAUSE."]
pub struct SLAVE_SCL_STRETCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_SCL_STRETCH_EN_W<'a> {
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
#[doc = "Field `SLAVE_SCL_STRETCH_CLR` writer - Set this bit to clear the I2C slave SCL stretch function."]
pub struct SLAVE_SCL_STRETCH_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_SCL_STRETCH_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Configure the period of I2C slave stretching SCL line."]
    #[inline(always)]
    pub fn stretch_protect_num(&self) -> STRETCH_PROTECT_NUM_R {
        STRETCH_PROTECT_NUM_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - The enable bit for slave SCL stretch function. 1: Enable. 0: Disable. The SCL output line will be stretched low when I2C_SLAVE_SCL_STRETCH_EN is 1 and stretch event happens. The stretch cause can be seen in I2C_STRETCH_CAUSE."]
    #[inline(always)]
    pub fn slave_scl_stretch_en(&self) -> SLAVE_SCL_STRETCH_EN_R {
        SLAVE_SCL_STRETCH_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Configure the period of I2C slave stretching SCL line."]
    #[inline(always)]
    pub fn stretch_protect_num(&mut self) -> STRETCH_PROTECT_NUM_W {
        STRETCH_PROTECT_NUM_W { w: self }
    }
    #[doc = "Bit 10 - The enable bit for slave SCL stretch function. 1: Enable. 0: Disable. The SCL output line will be stretched low when I2C_SLAVE_SCL_STRETCH_EN is 1 and stretch event happens. The stretch cause can be seen in I2C_STRETCH_CAUSE."]
    #[inline(always)]
    pub fn slave_scl_stretch_en(&mut self) -> SLAVE_SCL_STRETCH_EN_W {
        SLAVE_SCL_STRETCH_EN_W { w: self }
    }
    #[doc = "Bit 11 - Set this bit to clear the I2C slave SCL stretch function."]
    #[inline(always)]
    pub fn slave_scl_stretch_clr(&mut self) -> SLAVE_SCL_STRETCH_CLR_W {
        SLAVE_SCL_STRETCH_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set SCL stretch of I2C slave\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_stretch_conf]
(index.html) module"]
pub struct SCL_STRETCH_CONF_SPEC;
impl crate::RegisterSpec for SCL_STRETCH_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scl_stretch_conf::R]
(R) reader structure"]
impl crate::Readable for SCL_STRETCH_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scl_stretch_conf::W]
(W) writer structure"]
impl crate::Writable for SCL_STRETCH_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCL_STRETCH_CONF to value 0"]
impl crate::Resettable for SCL_STRETCH_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
