#[doc = "Register `INT_ENA` reader"]
pub struct R(crate::R<INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA` writer"]
pub struct W(crate::W<INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_SPEC>;
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
impl From<crate::W<INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READ_DONE_INT_ENA` reader - read done interrupt enable"]
pub type READ_DONE_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `READ_DONE_INT_ENA` writer - read done interrupt enable"]
pub type READ_DONE_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 0>;
#[doc = "Field `PGM_DONE_INT_ENA` reader - program done interrupt enable"]
pub type PGM_DONE_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `PGM_DONE_INT_ENA` writer - program done interrupt enable"]
pub type PGM_DONE_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - read done interrupt enable"]
    #[inline(always)]
    pub fn read_done_int_ena(&self) -> READ_DONE_INT_ENA_R {
        READ_DONE_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - program done interrupt enable"]
    #[inline(always)]
    pub fn pgm_done_int_ena(&self) -> PGM_DONE_INT_ENA_R {
        PGM_DONE_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - read done interrupt enable"]
    #[inline(always)]
    pub fn read_done_int_ena(&mut self) -> READ_DONE_INT_ENA_W {
        READ_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - program done interrupt enable"]
    #[inline(always)]
    pub fn pgm_done_int_ena(&mut self) -> PGM_DONE_INT_ENA_W {
        PGM_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](index.html) module"]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena::R](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena::W](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
