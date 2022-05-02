#[doc = "Register `CPU_INTR_FROM_CPU_0_MAP` reader"]
pub struct R(crate::R<CPU_INTR_FROM_CPU_0_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_INTR_FROM_CPU_0_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_INTR_FROM_CPU_0_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_INTR_FROM_CPU_0_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_INTR_FROM_CPU_0_MAP` writer"]
pub struct W(crate::W<CPU_INTR_FROM_CPU_0_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_INTR_FROM_CPU_0_MAP_SPEC>;
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
impl From<crate::W<CPU_INTR_FROM_CPU_0_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_INTR_FROM_CPU_0_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPU_INTR_FROM_CPU_0_MAP` reader - this register used to map cpu_intr_from_cpu_0 interrupt to one of core0's external interrupt"]
pub struct CPU_INTR_FROM_CPU_0_MAP_R(crate::FieldReader<u8>);
impl CPU_INTR_FROM_CPU_0_MAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CPU_INTR_FROM_CPU_0_MAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_INTR_FROM_CPU_0_MAP_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_INTR_FROM_CPU_0_MAP` writer - this register used to map cpu_intr_from_cpu_0 interrupt to one of core0's external interrupt"]
pub struct CPU_INTR_FROM_CPU_0_MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_INTR_FROM_CPU_0_MAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - this register used to map cpu_intr_from_cpu_0 interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn cpu_intr_from_cpu_0_map(&self) -> CPU_INTR_FROM_CPU_0_MAP_R {
        CPU_INTR_FROM_CPU_0_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map cpu_intr_from_cpu_0 interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn cpu_intr_from_cpu_0_map(&mut self) -> CPU_INTR_FROM_CPU_0_MAP_W {
        CPU_INTR_FROM_CPU_0_MAP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cpu_intr_from_cpu_0 interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_intr_from_cpu_0_map](index.html) module"]
pub struct CPU_INTR_FROM_CPU_0_MAP_SPEC;
impl crate::RegisterSpec for CPU_INTR_FROM_CPU_0_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_intr_from_cpu_0_map::R](R) reader structure"]
impl crate::Readable for CPU_INTR_FROM_CPU_0_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_intr_from_cpu_0_map::W](W) writer structure"]
impl crate::Writable for CPU_INTR_FROM_CPU_0_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPU_INTR_FROM_CPU_0_MAP to value 0x10"]
impl crate::Resettable for CPU_INTR_FROM_CPU_0_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
