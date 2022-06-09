#[doc = "Register `CACHE_ACS_CNT_CLR` writer"]
pub struct W(crate::W<CACHE_ACS_CNT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_ACS_CNT_CLR_SPEC>;
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
impl From<crate::W<CACHE_ACS_CNT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_ACS_CNT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IBUS_ACS_CNT_CLR` writer - The bit is used to clear ibus counter."]
pub type IBUS_ACS_CNT_CLR_W<'a> = crate::BitWriter<'a, u32, CACHE_ACS_CNT_CLR_SPEC, bool, 0>;
#[doc = "Field `DBUS_ACS_CNT_CLR` writer - The bit is used to clear dbus counter."]
pub type DBUS_ACS_CNT_CLR_W<'a> = crate::BitWriter<'a, u32, CACHE_ACS_CNT_CLR_SPEC, bool, 1>;
impl W {
    #[doc = "Bit 0 - The bit is used to clear ibus counter."]
    #[inline(always)]
    pub fn ibus_acs_cnt_clr(&mut self) -> IBUS_ACS_CNT_CLR_W {
        IBUS_ACS_CNT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to clear dbus counter."]
    #[inline(always)]
    pub fn dbus_acs_cnt_clr(&mut self) -> DBUS_ACS_CNT_CLR_W {
        DBUS_ACS_CNT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This description will be updated in the near future.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_acs_cnt_clr](index.html) module"]
pub struct CACHE_ACS_CNT_CLR_SPEC;
impl crate::RegisterSpec for CACHE_ACS_CNT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cache_acs_cnt_clr::W](W) writer structure"]
impl crate::Writable for CACHE_ACS_CNT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE_ACS_CNT_CLR to value 0"]
impl crate::Resettable for CACHE_ACS_CNT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
