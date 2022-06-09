#[doc = "Register `GEN1_CFG0` reader"]
pub struct R(crate::R<GEN1_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GEN1_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GEN1_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GEN1_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GEN1_CFG0` writer"]
pub struct W(crate::W<GEN1_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GEN1_CFG0_SPEC>;
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
impl From<crate::W<GEN1_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GEN1_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GEN1_CFG_UPMETHOD` reader - "]
pub type GEN1_CFG_UPMETHOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GEN1_CFG_UPMETHOD` writer - "]
pub type GEN1_CFG_UPMETHOD_W<'a> = crate::FieldWriter<'a, u32, GEN1_CFG0_SPEC, u8, u8, 4, 0>;
#[doc = "Field `GEN1_T0_SEL` reader - "]
pub type GEN1_T0_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GEN1_T0_SEL` writer - "]
pub type GEN1_T0_SEL_W<'a> = crate::FieldWriter<'a, u32, GEN1_CFG0_SPEC, u8, u8, 3, 4>;
#[doc = "Field `GEN1_T1_SEL` reader - "]
pub type GEN1_T1_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GEN1_T1_SEL` writer - "]
pub type GEN1_T1_SEL_W<'a> = crate::FieldWriter<'a, u32, GEN1_CFG0_SPEC, u8, u8, 3, 7>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn gen1_cfg_upmethod(&self) -> GEN1_CFG_UPMETHOD_R {
        GEN1_CFG_UPMETHOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn gen1_t0_sel(&self) -> GEN1_T0_SEL_R {
        GEN1_T0_SEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn gen1_t1_sel(&self) -> GEN1_T1_SEL_R {
        GEN1_T1_SEL_R::new(((self.bits >> 7) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn gen1_cfg_upmethod(&mut self) -> GEN1_CFG_UPMETHOD_W {
        GEN1_CFG_UPMETHOD_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn gen1_t0_sel(&mut self) -> GEN1_T0_SEL_W {
        GEN1_T0_SEL_W::new(self)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn gen1_t1_sel(&mut self) -> GEN1_T1_SEL_W {
        GEN1_T1_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gen1_cfg0](index.html) module"]
pub struct GEN1_CFG0_SPEC;
impl crate::RegisterSpec for GEN1_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gen1_cfg0::R](R) reader structure"]
impl crate::Readable for GEN1_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gen1_cfg0::W](W) writer structure"]
impl crate::Writable for GEN1_CFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GEN1_CFG0 to value 0"]
impl crate::Resettable for GEN1_CFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
