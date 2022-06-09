#[doc = "Register `ICACHE_ATOMIC_OPERATE_ENA` reader"]
pub struct R(crate::R<ICACHE_ATOMIC_OPERATE_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICACHE_ATOMIC_OPERATE_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICACHE_ATOMIC_OPERATE_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICACHE_ATOMIC_OPERATE_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICACHE_ATOMIC_OPERATE_ENA` writer"]
pub struct W(crate::W<ICACHE_ATOMIC_OPERATE_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICACHE_ATOMIC_OPERATE_ENA_SPEC>;
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
impl From<crate::W<ICACHE_ATOMIC_OPERATE_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICACHE_ATOMIC_OPERATE_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICACHE_ATOMIC_OPERATE_ENA` reader - The bit is used to activate icache atomic operation protection. In this case, sync/lock operation can not interrupt miss-work. This feature does not work during invalidateAll operation."]
pub type ICACHE_ATOMIC_OPERATE_ENA_R = crate::BitReader<bool>;
#[doc = "Field `ICACHE_ATOMIC_OPERATE_ENA` writer - The bit is used to activate icache atomic operation protection. In this case, sync/lock operation can not interrupt miss-work. This feature does not work during invalidateAll operation."]
pub type ICACHE_ATOMIC_OPERATE_ENA_W<'a> =
    crate::BitWriter<'a, u32, ICACHE_ATOMIC_OPERATE_ENA_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - The bit is used to activate icache atomic operation protection. In this case, sync/lock operation can not interrupt miss-work. This feature does not work during invalidateAll operation."]
    #[inline(always)]
    pub fn icache_atomic_operate_ena(&self) -> ICACHE_ATOMIC_OPERATE_ENA_R {
        ICACHE_ATOMIC_OPERATE_ENA_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to activate icache atomic operation protection. In this case, sync/lock operation can not interrupt miss-work. This feature does not work during invalidateAll operation."]
    #[inline(always)]
    pub fn icache_atomic_operate_ena(&mut self) -> ICACHE_ATOMIC_OPERATE_ENA_W {
        ICACHE_ATOMIC_OPERATE_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This description will be updated in the near future.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_atomic_operate_ena](index.html) module"]
pub struct ICACHE_ATOMIC_OPERATE_ENA_SPEC;
impl crate::RegisterSpec for ICACHE_ATOMIC_OPERATE_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icache_atomic_operate_ena::R](R) reader structure"]
impl crate::Readable for ICACHE_ATOMIC_OPERATE_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icache_atomic_operate_ena::W](W) writer structure"]
impl crate::Writable for ICACHE_ATOMIC_OPERATE_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICACHE_ATOMIC_OPERATE_ENA to value 0x01"]
impl crate::Resettable for ICACHE_ATOMIC_OPERATE_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
