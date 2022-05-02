#[doc = "Register `WIFI_RST_EN` reader"]
pub struct R(crate::R<WIFI_RST_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIFI_RST_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIFI_RST_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIFI_RST_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WIFI_RST_EN` writer"]
pub struct W(crate::W<WIFI_RST_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIFI_RST_EN_SPEC>;
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
impl From<crate::W<WIFI_RST_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIFI_RST_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIFI_RST` reader - reg_wifi_rst"]
pub struct WIFI_RST_R(crate::FieldReader<u32>);
impl WIFI_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        WIFI_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFI_RST_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIFI_RST` writer - reg_wifi_rst"]
pub struct WIFI_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_RST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - reg_wifi_rst"]
    #[inline(always)]
    pub fn wifi_rst(&self) -> WIFI_RST_R {
        WIFI_RST_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - reg_wifi_rst"]
    #[inline(always)]
    pub fn wifi_rst(&mut self) -> WIFI_RST_W {
        WIFI_RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB_CTRL_WIFI_RST_EN_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifi_rst_en](index.html) module"]
pub struct WIFI_RST_EN_SPEC;
impl crate::RegisterSpec for WIFI_RST_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wifi_rst_en::R](R) reader structure"]
impl crate::Readable for WIFI_RST_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wifi_rst_en::W](W) writer structure"]
impl crate::Writable for WIFI_RST_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WIFI_RST_EN to value 0"]
impl crate::Resettable for WIFI_RST_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
