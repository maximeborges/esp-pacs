#[doc = "Register `PRO_ICACHE_MEM_SYNC1` reader"]
pub struct R(crate::R<PRO_ICACHE_MEM_SYNC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_ICACHE_MEM_SYNC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_ICACHE_MEM_SYNC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_ICACHE_MEM_SYNC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_ICACHE_MEM_SYNC1` writer"]
pub struct W(crate::W<PRO_ICACHE_MEM_SYNC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_ICACHE_MEM_SYNC1_SPEC>;
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
impl From<crate::W<PRO_ICACHE_MEM_SYNC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_ICACHE_MEM_SYNC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_ICACHE_MEMSYNC_SIZE` reader - The bits are used to configure the length for invalidate, flush, clean, lock and unlock operations. The manual operations will be issued if it is validate. The auto operations will be issued if it is invalidate. It should be combined with PRO_ICACHE_MEM_SYNC0."]
pub struct PRO_ICACHE_MEMSYNC_SIZE_R(crate::FieldReader<u32, u32>);
impl PRO_ICACHE_MEMSYNC_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PRO_ICACHE_MEMSYNC_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_ICACHE_MEMSYNC_SIZE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_ICACHE_MEMSYNC_SIZE` writer - The bits are used to configure the length for invalidate, flush, clean, lock and unlock operations. The manual operations will be issued if it is validate. The auto operations will be issued if it is invalidate. It should be combined with PRO_ICACHE_MEM_SYNC0."]
pub struct PRO_ICACHE_MEMSYNC_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_ICACHE_MEMSYNC_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0007_ffff) | (value as u32 & 0x0007_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:18 - The bits are used to configure the length for invalidate, flush, clean, lock and unlock operations. The manual operations will be issued if it is validate. The auto operations will be issued if it is invalidate. It should be combined with PRO_ICACHE_MEM_SYNC0."]
    #[inline(always)]
    pub fn pro_icache_memsync_size(&self) -> PRO_ICACHE_MEMSYNC_SIZE_R {
        PRO_ICACHE_MEMSYNC_SIZE_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:18 - The bits are used to configure the length for invalidate, flush, clean, lock and unlock operations. The manual operations will be issued if it is validate. The auto operations will be issued if it is invalidate. It should be combined with PRO_ICACHE_MEM_SYNC0."]
    #[inline(always)]
    pub fn pro_icache_memsync_size(&mut self) -> PRO_ICACHE_MEMSYNC_SIZE_W {
        PRO_ICACHE_MEMSYNC_SIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_icache_mem_sync1]
(index.html) module"]
pub struct PRO_ICACHE_MEM_SYNC1_SPEC;
impl crate::RegisterSpec for PRO_ICACHE_MEM_SYNC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_icache_mem_sync1::R]
(R) reader structure"]
impl crate::Readable for PRO_ICACHE_MEM_SYNC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_icache_mem_sync1::W]
(W) writer structure"]
impl crate::Writable for PRO_ICACHE_MEM_SYNC1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_ICACHE_MEM_SYNC1 to value 0"]
impl crate::Resettable for PRO_ICACHE_MEM_SYNC1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
