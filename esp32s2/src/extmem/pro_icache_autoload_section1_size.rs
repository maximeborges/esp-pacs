#[doc = "Register `PRO_ICACHE_AUTOLOAD_SECTION1_SIZE` reader"]
pub struct R(crate::R<PRO_ICACHE_AUTOLOAD_SECTION1_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_ICACHE_AUTOLOAD_SECTION1_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_ICACHE_AUTOLOAD_SECTION1_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_ICACHE_AUTOLOAD_SECTION1_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_ICACHE_AUTOLOAD_SECTION1_SIZE` writer"]
pub struct W(crate::W<PRO_ICACHE_AUTOLOAD_SECTION1_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_ICACHE_AUTOLOAD_SECTION1_SIZE_SPEC>;
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
impl From<crate::W<PRO_ICACHE_AUTOLOAD_SECTION1_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_ICACHE_AUTOLOAD_SECTION1_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_ICACHE_AUTOLOAD_SCT1_SIZE` reader - The bits are used to configure the length of the second section for conditional pre-load operation. It should be combined with pro_icache_autoload_sct1_ena."]
pub struct PRO_ICACHE_AUTOLOAD_SCT1_SIZE_R(crate::FieldReader<u32>);
impl PRO_ICACHE_AUTOLOAD_SCT1_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PRO_ICACHE_AUTOLOAD_SCT1_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_ICACHE_AUTOLOAD_SCT1_SIZE_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_ICACHE_AUTOLOAD_SCT1_SIZE` writer - The bits are used to configure the length of the second section for conditional pre-load operation. It should be combined with pro_icache_autoload_sct1_ena."]
pub struct PRO_ICACHE_AUTOLOAD_SCT1_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_ICACHE_AUTOLOAD_SCT1_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - The bits are used to configure the length of the second section for conditional pre-load operation. It should be combined with pro_icache_autoload_sct1_ena."]
    #[inline(always)]
    pub fn pro_icache_autoload_sct1_size(&self) -> PRO_ICACHE_AUTOLOAD_SCT1_SIZE_R {
        PRO_ICACHE_AUTOLOAD_SCT1_SIZE_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - The bits are used to configure the length of the second section for conditional pre-load operation. It should be combined with pro_icache_autoload_sct1_ena."]
    #[inline(always)]
    pub fn pro_icache_autoload_sct1_size(&mut self) -> PRO_ICACHE_AUTOLOAD_SCT1_SIZE_W {
        PRO_ICACHE_AUTOLOAD_SCT1_SIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_icache_autoload_section1_size](index.html) module"]
pub struct PRO_ICACHE_AUTOLOAD_SECTION1_SIZE_SPEC;
impl crate::RegisterSpec for PRO_ICACHE_AUTOLOAD_SECTION1_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_icache_autoload_section1_size::R](R) reader structure"]
impl crate::Readable for PRO_ICACHE_AUTOLOAD_SECTION1_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_icache_autoload_section1_size::W](W) writer structure"]
impl crate::Writable for PRO_ICACHE_AUTOLOAD_SECTION1_SIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_ICACHE_AUTOLOAD_SECTION1_SIZE to value 0x8000"]
impl crate::Resettable for PRO_ICACHE_AUTOLOAD_SECTION1_SIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
