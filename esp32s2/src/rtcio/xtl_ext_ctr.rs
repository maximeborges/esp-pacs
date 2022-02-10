#[doc = "Register `XTL_EXT_CTR` reader"]
pub struct R(crate::R<XTL_EXT_CTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTL_EXT_CTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTL_EXT_CTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTL_EXT_CTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTL_EXT_CTR` writer"]
pub struct W(crate::W<XTL_EXT_CTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTL_EXT_CTR_SPEC>;
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
impl From<crate::W<XTL_EXT_CTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTL_EXT_CTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - Select the external crystal power down enable source to get into sleep mode. 0: select GPIO0. 1: select GPIO1, etc. The input value on this pin XOR RTC_CNTL_EXT_XTL_CONF_REG\\[30\\]
 is the crystal power down enable signal."]
pub struct SEL_R(crate::FieldReader<u8, u8>);
impl SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL` writer - Select the external crystal power down enable source to get into sleep mode. 0: select GPIO0. 1: select GPIO1, etc. The input value on this pin XOR RTC_CNTL_EXT_XTL_CONF_REG\\[30\\]
 is the crystal power down enable signal."]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | ((value as u32 & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:31 - Select the external crystal power down enable source to get into sleep mode. 0: select GPIO0. 1: select GPIO1, etc. The input value on this pin XOR RTC_CNTL_EXT_XTL_CONF_REG\\[30\\]
 is the crystal power down enable signal."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 27:31 - Select the external crystal power down enable source to get into sleep mode. 0: select GPIO0. 1: select GPIO1, etc. The input value on this pin XOR RTC_CNTL_EXT_XTL_CONF_REG\\[30\\]
 is the crystal power down enable signal."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crystal power down enable GPIO source\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtl_ext_ctr]
(index.html) module"]
pub struct XTL_EXT_CTR_SPEC;
impl crate::RegisterSpec for XTL_EXT_CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtl_ext_ctr::R]
(R) reader structure"]
impl crate::Readable for XTL_EXT_CTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtl_ext_ctr::W]
(W) writer structure"]
impl crate::Writable for XTL_EXT_CTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XTL_EXT_CTR to value 0"]
impl crate::Resettable for XTL_EXT_CTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
