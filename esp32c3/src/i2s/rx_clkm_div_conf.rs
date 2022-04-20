#[doc = "Register `RX_CLKM_DIV_CONF` reader"]
pub struct R(crate::R<RX_CLKM_DIV_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_CLKM_DIV_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_CLKM_DIV_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_CLKM_DIV_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_CLKM_DIV_CONF` writer"]
pub struct W(crate::W<RX_CLKM_DIV_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_CLKM_DIV_CONF_SPEC>;
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
impl From<crate::W<RX_CLKM_DIV_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_CLKM_DIV_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_CLKM_DIV_Z` reader - For b <= a/2, the value of I2S_RX_CLKM_DIV_Z is b. For b > a/2, the value of I2S_RX_CLKM_DIV_Z is (a-b)."]
pub struct RX_CLKM_DIV_Z_R(crate::FieldReader<u16, u16>);
impl RX_CLKM_DIV_Z_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RX_CLKM_DIV_Z_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_CLKM_DIV_Z_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_CLKM_DIV_Z` writer - For b <= a/2, the value of I2S_RX_CLKM_DIV_Z is b. For b > a/2, the value of I2S_RX_CLKM_DIV_Z is (a-b)."]
pub struct RX_CLKM_DIV_Z_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CLKM_DIV_Z_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
#[doc = "Field `RX_CLKM_DIV_Y` reader - For b <= a/2, the value of I2S_RX_CLKM_DIV_Y is (a%b) . For b > a/2, the value of I2S_RX_CLKM_DIV_Y is (a%(a-b))."]
pub struct RX_CLKM_DIV_Y_R(crate::FieldReader<u16, u16>);
impl RX_CLKM_DIV_Y_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RX_CLKM_DIV_Y_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_CLKM_DIV_Y_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_CLKM_DIV_Y` writer - For b <= a/2, the value of I2S_RX_CLKM_DIV_Y is (a%b) . For b > a/2, the value of I2S_RX_CLKM_DIV_Y is (a%(a-b))."]
pub struct RX_CLKM_DIV_Y_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CLKM_DIV_Y_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 9)) | ((value as u32 & 0x01ff) << 9);
        self.w
    }
}
#[doc = "Field `RX_CLKM_DIV_X` reader - For b <= a/2, the value of I2S_RX_CLKM_DIV_X is (a/b) - 1. For b > a/2, the value of I2S_RX_CLKM_DIV_X is (a/(a-b)) - 1."]
pub struct RX_CLKM_DIV_X_R(crate::FieldReader<u16, u16>);
impl RX_CLKM_DIV_X_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RX_CLKM_DIV_X_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_CLKM_DIV_X_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_CLKM_DIV_X` writer - For b <= a/2, the value of I2S_RX_CLKM_DIV_X is (a/b) - 1. For b > a/2, the value of I2S_RX_CLKM_DIV_X is (a/(a-b)) - 1."]
pub struct RX_CLKM_DIV_X_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CLKM_DIV_X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 18)) | ((value as u32 & 0x01ff) << 18);
        self.w
    }
}
#[doc = "Field `RX_CLKM_DIV_YN1` reader - For b <= a/2, the value of I2S_RX_CLKM_DIV_YN1 is 0 . For b > a/2, the value of I2S_RX_CLKM_DIV_YN1 is 1."]
pub struct RX_CLKM_DIV_YN1_R(crate::FieldReader<bool, bool>);
impl RX_CLKM_DIV_YN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_CLKM_DIV_YN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_CLKM_DIV_YN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_CLKM_DIV_YN1` writer - For b <= a/2, the value of I2S_RX_CLKM_DIV_YN1 is 0 . For b > a/2, the value of I2S_RX_CLKM_DIV_YN1 is 1."]
pub struct RX_CLKM_DIV_YN1_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CLKM_DIV_YN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 27)) | ((value as u32 & 1) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - For b <= a/2, the value of I2S_RX_CLKM_DIV_Z is b. For b > a/2, the value of I2S_RX_CLKM_DIV_Z is (a-b)."]
    #[inline(always)]
    pub fn rx_clkm_div_z(&self) -> RX_CLKM_DIV_Z_R {
        RX_CLKM_DIV_Z_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17 - For b <= a/2, the value of I2S_RX_CLKM_DIV_Y is (a%b) . For b > a/2, the value of I2S_RX_CLKM_DIV_Y is (a%(a-b))."]
    #[inline(always)]
    pub fn rx_clkm_div_y(&self) -> RX_CLKM_DIV_Y_R {
        RX_CLKM_DIV_Y_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bits 18:26 - For b <= a/2, the value of I2S_RX_CLKM_DIV_X is (a/b) - 1. For b > a/2, the value of I2S_RX_CLKM_DIV_X is (a/(a-b)) - 1."]
    #[inline(always)]
    pub fn rx_clkm_div_x(&self) -> RX_CLKM_DIV_X_R {
        RX_CLKM_DIV_X_R::new(((self.bits >> 18) & 0x01ff) as u16)
    }
    #[doc = "Bit 27 - For b <= a/2, the value of I2S_RX_CLKM_DIV_YN1 is 0 . For b > a/2, the value of I2S_RX_CLKM_DIV_YN1 is 1."]
    #[inline(always)]
    pub fn rx_clkm_div_yn1(&self) -> RX_CLKM_DIV_YN1_R {
        RX_CLKM_DIV_YN1_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - For b <= a/2, the value of I2S_RX_CLKM_DIV_Z is b. For b > a/2, the value of I2S_RX_CLKM_DIV_Z is (a-b)."]
    #[inline(always)]
    pub fn rx_clkm_div_z(&mut self) -> RX_CLKM_DIV_Z_W {
        RX_CLKM_DIV_Z_W { w: self }
    }
    #[doc = "Bits 9:17 - For b <= a/2, the value of I2S_RX_CLKM_DIV_Y is (a%b) . For b > a/2, the value of I2S_RX_CLKM_DIV_Y is (a%(a-b))."]
    #[inline(always)]
    pub fn rx_clkm_div_y(&mut self) -> RX_CLKM_DIV_Y_W {
        RX_CLKM_DIV_Y_W { w: self }
    }
    #[doc = "Bits 18:26 - For b <= a/2, the value of I2S_RX_CLKM_DIV_X is (a/b) - 1. For b > a/2, the value of I2S_RX_CLKM_DIV_X is (a/(a-b)) - 1."]
    #[inline(always)]
    pub fn rx_clkm_div_x(&mut self) -> RX_CLKM_DIV_X_W {
        RX_CLKM_DIV_X_W { w: self }
    }
    #[doc = "Bit 27 - For b <= a/2, the value of I2S_RX_CLKM_DIV_YN1 is 0 . For b > a/2, the value of I2S_RX_CLKM_DIV_YN1 is 1."]
    #[inline(always)]
    pub fn rx_clkm_div_yn1(&mut self) -> RX_CLKM_DIV_YN1_W {
        RX_CLKM_DIV_YN1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S RX module clock divider configure register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_clkm_div_conf]
(index.html) module"]
pub struct RX_CLKM_DIV_CONF_SPEC;
impl crate::RegisterSpec for RX_CLKM_DIV_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_clkm_div_conf::R]
(R) reader structure"]
impl crate::Readable for RX_CLKM_DIV_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_clkm_div_conf::W]
(W) writer structure"]
impl crate::Writable for RX_CLKM_DIV_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_CLKM_DIV_CONF to value 0x0200"]
impl crate::Resettable for RX_CLKM_DIV_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
