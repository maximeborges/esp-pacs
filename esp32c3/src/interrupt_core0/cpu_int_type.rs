#[doc = "Register `CPU_INT_TYPE` reader"]
pub struct R(crate::R<CPU_INT_TYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_INT_TYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_INT_TYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_INT_TYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_INT_TYPE` writer"]
pub struct W(crate::W<CPU_INT_TYPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_INT_TYPE_SPEC>;
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
impl From<crate::W<CPU_INT_TYPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_INT_TYPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPU_INT_TYPE` reader - reg_core0_cpu_int_type"]
pub struct CPU_INT_TYPE_R(crate::FieldReader<u32, u32>);
impl CPU_INT_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CPU_INT_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_INT_TYPE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_INT_TYPE` writer - reg_core0_cpu_int_type"]
pub struct CPU_INT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_INT_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - reg_core0_cpu_int_type"]
    #[inline(always)]
    pub fn cpu_int_type(&self) -> CPU_INT_TYPE_R {
        CPU_INT_TYPE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - reg_core0_cpu_int_type"]
    #[inline(always)]
    pub fn cpu_int_type(&mut self) -> CPU_INT_TYPE_W {
        CPU_INT_TYPE_W { w: self }
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
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_int_type]
(index.html) module"]
pub struct CPU_INT_TYPE_SPEC;
impl crate::RegisterSpec for CPU_INT_TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_int_type::R]
(R) reader structure"]
impl crate::Readable for CPU_INT_TYPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_int_type::W]
(W) writer structure"]
impl crate::Writable for CPU_INT_TYPE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPU_INT_TYPE to value 0"]
impl crate::Resettable for CPU_INT_TYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
