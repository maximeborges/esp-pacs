#[doc = "Register `CPU_INT_PRI_23` reader"]
pub struct R(crate::R<CPU_INT_PRI_23_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_INT_PRI_23_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_INT_PRI_23_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_INT_PRI_23_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_INT_PRI_23` writer"]
pub struct W(crate::W<CPU_INT_PRI_23_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_INT_PRI_23_SPEC>;
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
impl From<crate::W<CPU_INT_PRI_23_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_INT_PRI_23_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPU_PRI_23_MAP` reader - reg_core0_cpu_pri_23_map"]
pub struct CPU_PRI_23_MAP_R(crate::FieldReader<u8, u8>);
impl CPU_PRI_23_MAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CPU_PRI_23_MAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_PRI_23_MAP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_PRI_23_MAP` writer - reg_core0_cpu_pri_23_map"]
pub struct CPU_PRI_23_MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_PRI_23_MAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - reg_core0_cpu_pri_23_map"]
    #[inline(always)]
    pub fn cpu_pri_23_map(&self) -> CPU_PRI_23_MAP_R {
        CPU_PRI_23_MAP_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - reg_core0_cpu_pri_23_map"]
    #[inline(always)]
    pub fn cpu_pri_23_map(&mut self) -> CPU_PRI_23_MAP_W {
        CPU_PRI_23_MAP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mac intr map register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_int_pri_23]
(index.html) module"]
pub struct CPU_INT_PRI_23_SPEC;
impl crate::RegisterSpec for CPU_INT_PRI_23_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_int_pri_23::R]
(R) reader structure"]
impl crate::Readable for CPU_INT_PRI_23_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_int_pri_23::W]
(W) writer structure"]
impl crate::Writable for CPU_INT_PRI_23_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPU_INT_PRI_23 to value 0"]
impl crate::Resettable for CPU_INT_PRI_23_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
