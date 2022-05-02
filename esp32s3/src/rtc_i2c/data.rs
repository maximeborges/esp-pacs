#[doc = "Register `DATA` reader"]
pub struct R(crate::R<DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA` writer"]
pub struct W(crate::W<DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_SPEC>;
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
impl From<crate::W<DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_RDATA` reader - data received"]
pub struct I2C_RDATA_R(crate::FieldReader<u8>);
impl I2C_RDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        I2C_RDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_RDATA_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_TX_DATA` reader - data sent by slave"]
pub struct SLAVE_TX_DATA_R(crate::FieldReader<u8>);
impl SLAVE_TX_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLAVE_TX_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_TX_DATA_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_TX_DATA` writer - data sent by slave"]
pub struct SLAVE_TX_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_TX_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `I2C_DONE` reader - i2c done"]
pub struct I2C_DONE_R(crate::FieldReader<bool>);
impl I2C_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_DONE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - data received"]
    #[inline(always)]
    pub fn i2c_rdata(&self) -> I2C_RDATA_R {
        I2C_RDATA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - data sent by slave"]
    #[inline(always)]
    pub fn slave_tx_data(&self) -> SLAVE_TX_DATA_R {
        SLAVE_TX_DATA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - i2c done"]
    #[inline(always)]
    pub fn i2c_done(&self) -> I2C_DONE_R {
        I2C_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:15 - data sent by slave"]
    #[inline(always)]
    pub fn slave_tx_data(&mut self) -> SLAVE_TX_DATA_W {
        SLAVE_TX_DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "get i2c data status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](index.html) module"]
pub struct DATA_SPEC;
impl crate::RegisterSpec for DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data::R](R) reader structure"]
impl crate::Readable for DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data::W](W) writer structure"]
impl crate::Writable for DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA to value 0"]
impl crate::Resettable for DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
