#[doc = "Register `APB_TSENS_CTRL` reader"]
pub struct R(crate::R<APB_TSENS_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_TSENS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_TSENS_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_TSENS_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_TSENS_CTRL` writer"]
pub struct W(crate::W<APB_TSENS_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_TSENS_CTRL_SPEC>;
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
impl From<crate::W<APB_TSENS_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_TSENS_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSENS_OUT` reader - temperature sensor data out"]
pub struct TSENS_OUT_R(crate::FieldReader<u8, u8>);
impl TSENS_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TSENS_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENS_OUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENS_IN_INV` reader - invert temperature sensor data"]
pub struct TSENS_IN_INV_R(crate::FieldReader<bool, bool>);
impl TSENS_IN_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSENS_IN_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENS_IN_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENS_IN_INV` writer - invert temperature sensor data"]
pub struct TSENS_IN_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_IN_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `TSENS_CLK_DIV` reader - temperature sensor clock divider"]
pub struct TSENS_CLK_DIV_R(crate::FieldReader<u8, u8>);
impl TSENS_CLK_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TSENS_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENS_CLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENS_CLK_DIV` writer - temperature sensor clock divider"]
pub struct TSENS_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 14)) | ((value as u32 & 0xff) << 14);
        self.w
    }
}
#[doc = "Field `TSENS_PU` reader - temperature sensor power up"]
pub struct TSENS_PU_R(crate::FieldReader<bool, bool>);
impl TSENS_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSENS_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENS_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENS_PU` writer - temperature sensor power up"]
pub struct TSENS_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 22)) | ((value as u32 & 1) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - temperature sensor data out"]
    #[inline(always)]
    pub fn tsens_out(&self) -> TSENS_OUT_R {
        TSENS_OUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 13 - invert temperature sensor data"]
    #[inline(always)]
    pub fn tsens_in_inv(&self) -> TSENS_IN_INV_R {
        TSENS_IN_INV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:21 - temperature sensor clock divider"]
    #[inline(always)]
    pub fn tsens_clk_div(&self) -> TSENS_CLK_DIV_R {
        TSENS_CLK_DIV_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bit 22 - temperature sensor power up"]
    #[inline(always)]
    pub fn tsens_pu(&self) -> TSENS_PU_R {
        TSENS_PU_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - invert temperature sensor data"]
    #[inline(always)]
    pub fn tsens_in_inv(&mut self) -> TSENS_IN_INV_W {
        TSENS_IN_INV_W { w: self }
    }
    #[doc = "Bits 14:21 - temperature sensor clock divider"]
    #[inline(always)]
    pub fn tsens_clk_div(&mut self) -> TSENS_CLK_DIV_W {
        TSENS_CLK_DIV_W { w: self }
    }
    #[doc = "Bit 22 - temperature sensor power up"]
    #[inline(always)]
    pub fn tsens_pu(&mut self) -> TSENS_PU_W {
        TSENS_PU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "digital tsens configure register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_tsens_ctrl]
(index.html) module"]
pub struct APB_TSENS_CTRL_SPEC;
impl crate::RegisterSpec for APB_TSENS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_tsens_ctrl::R]
(R) reader structure"]
impl crate::Readable for APB_TSENS_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_tsens_ctrl::W]
(W) writer structure"]
impl crate::Writable for APB_TSENS_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB_TSENS_CTRL to value 0x0001_8000"]
impl crate::Resettable for APB_TSENS_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_8000
    }
}
