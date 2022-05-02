#[doc = "Register `SDA_FILTER_CFG` reader"]
pub struct R(crate::R<SDA_FILTER_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDA_FILTER_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDA_FILTER_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDA_FILTER_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDA_FILTER_CFG` writer"]
pub struct W(crate::W<SDA_FILTER_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDA_FILTER_CFG_SPEC>;
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
impl From<crate::W<SDA_FILTER_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDA_FILTER_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDA_FILTER_THRES` reader - When a pulse on the SDA input has smaller width than this register value in I2C module clock cycles, the I2C controller will ignore that pulse."]
pub struct SDA_FILTER_THRES_R(crate::FieldReader<u8>);
impl SDA_FILTER_THRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SDA_FILTER_THRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDA_FILTER_THRES_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDA_FILTER_THRES` writer - When a pulse on the SDA input has smaller width than this register value in I2C module clock cycles, the I2C controller will ignore that pulse."]
pub struct SDA_FILTER_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_FILTER_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `SDA_FILTER_EN` reader - This is the filter enable bit for SDA."]
pub struct SDA_FILTER_EN_R(crate::FieldReader<bool>);
impl SDA_FILTER_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDA_FILTER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDA_FILTER_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDA_FILTER_EN` writer - This is the filter enable bit for SDA."]
pub struct SDA_FILTER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_FILTER_EN_W<'a> {
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
impl R {
    #[doc = "Bits 0:3 - When a pulse on the SDA input has smaller width than this register value in I2C module clock cycles, the I2C controller will ignore that pulse."]
    #[inline(always)]
    pub fn sda_filter_thres(&self) -> SDA_FILTER_THRES_R {
        SDA_FILTER_THRES_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - This is the filter enable bit for SDA."]
    #[inline(always)]
    pub fn sda_filter_en(&self) -> SDA_FILTER_EN_R {
        SDA_FILTER_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - When a pulse on the SDA input has smaller width than this register value in I2C module clock cycles, the I2C controller will ignore that pulse."]
    #[inline(always)]
    pub fn sda_filter_thres(&mut self) -> SDA_FILTER_THRES_W {
        SDA_FILTER_THRES_W { w: self }
    }
    #[doc = "Bit 4 - This is the filter enable bit for SDA."]
    #[inline(always)]
    pub fn sda_filter_en(&mut self) -> SDA_FILTER_EN_W {
        SDA_FILTER_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDA filter configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sda_filter_cfg](index.html) module"]
pub struct SDA_FILTER_CFG_SPEC;
impl crate::RegisterSpec for SDA_FILTER_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sda_filter_cfg::R](R) reader structure"]
impl crate::Readable for SDA_FILTER_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sda_filter_cfg::W](W) writer structure"]
impl crate::Writable for SDA_FILTER_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDA_FILTER_CFG to value 0x10"]
impl crate::Resettable for SDA_FILTER_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
