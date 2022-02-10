#[doc = "Register `DCACHE_AUTOLOAD_SCT0_ADDR` reader"]
pub struct R(crate::R<DCACHE_AUTOLOAD_SCT0_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_AUTOLOAD_SCT0_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_AUTOLOAD_SCT0_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_AUTOLOAD_SCT0_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCACHE_AUTOLOAD_SCT0_ADDR` writer"]
pub struct W(crate::W<DCACHE_AUTOLOAD_SCT0_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCACHE_AUTOLOAD_SCT0_ADDR_SPEC>;
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
impl From<crate::W<DCACHE_AUTOLOAD_SCT0_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCACHE_AUTOLOAD_SCT0_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCACHE_AUTOLOAD_SCT0_ADDR` reader - The bits are used to configure the start virtual address of the first section for autoload operation. It should be combined with dcache_autoload_sct0_ena."]
pub struct DCACHE_AUTOLOAD_SCT0_ADDR_R(crate::FieldReader<u32, u32>);
impl DCACHE_AUTOLOAD_SCT0_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DCACHE_AUTOLOAD_SCT0_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_AUTOLOAD_SCT0_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_AUTOLOAD_SCT0_ADDR` writer - The bits are used to configure the start virtual address of the first section for autoload operation. It should be combined with dcache_autoload_sct0_ena."]
pub struct DCACHE_AUTOLOAD_SCT0_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_AUTOLOAD_SCT0_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address of the first section for autoload operation. It should be combined with dcache_autoload_sct0_ena."]
    #[inline(always)]
    pub fn dcache_autoload_sct0_addr(&self) -> DCACHE_AUTOLOAD_SCT0_ADDR_R {
        DCACHE_AUTOLOAD_SCT0_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address of the first section for autoload operation. It should be combined with dcache_autoload_sct0_ena."]
    #[inline(always)]
    pub fn dcache_autoload_sct0_addr(&mut self) -> DCACHE_AUTOLOAD_SCT0_ADDR_W {
        DCACHE_AUTOLOAD_SCT0_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcache_autoload_sct0_addr]
(index.html) module"]
pub struct DCACHE_AUTOLOAD_SCT0_ADDR_SPEC;
impl crate::RegisterSpec for DCACHE_AUTOLOAD_SCT0_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcache_autoload_sct0_addr::R]
(R) reader structure"]
impl crate::Readable for DCACHE_AUTOLOAD_SCT0_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcache_autoload_sct0_addr::W]
(W) writer structure"]
impl crate::Writable for DCACHE_AUTOLOAD_SCT0_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCACHE_AUTOLOAD_SCT0_ADDR to value 0"]
impl crate::Resettable for DCACHE_AUTOLOAD_SCT0_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
