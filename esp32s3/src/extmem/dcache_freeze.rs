#[doc = "Register `DCACHE_FREEZE` reader"]
pub struct R(crate::R<DCACHE_FREEZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_FREEZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_FREEZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_FREEZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCACHE_FREEZE` writer"]
pub struct W(crate::W<DCACHE_FREEZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCACHE_FREEZE_SPEC>;
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
impl From<crate::W<DCACHE_FREEZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCACHE_FREEZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENA` reader - The bit is used to enable dcache freeze mode"]
pub type ENA_R = crate::BitReader<bool>;
#[doc = "Field `ENA` writer - The bit is used to enable dcache freeze mode"]
pub type ENA_W<'a> = crate::BitWriter<'a, u32, DCACHE_FREEZE_SPEC, bool, 0>;
#[doc = "Field `MODE` reader - The bit is used to configure freeze mode, 0: assert busy if CPU miss 1: assert hit if CPU miss"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - The bit is used to configure freeze mode, 0: assert busy if CPU miss 1: assert hit if CPU miss"]
pub type MODE_W<'a> = crate::BitWriter<'a, u32, DCACHE_FREEZE_SPEC, bool, 1>;
#[doc = "Field `DONE` reader - The bit is used to indicate dcache freeze success"]
pub type DONE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable dcache freeze mode"]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to configure freeze mode, 0: assert busy if CPU miss 1: assert hit if CPU miss"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to indicate dcache freeze success"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable dcache freeze mode"]
    #[inline(always)]
    pub fn ena(&mut self) -> ENA_W {
        ENA_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to configure freeze mode, 0: assert busy if CPU miss 1: assert hit if CPU miss"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcache_freeze](index.html) module"]
pub struct DCACHE_FREEZE_SPEC;
impl crate::RegisterSpec for DCACHE_FREEZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcache_freeze::R](R) reader structure"]
impl crate::Readable for DCACHE_FREEZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcache_freeze::W](W) writer structure"]
impl crate::Writable for DCACHE_FREEZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCACHE_FREEZE to value 0x04"]
impl crate::Resettable for DCACHE_FREEZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
