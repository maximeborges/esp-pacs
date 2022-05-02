#[doc = "Register `LOG_MEM_END` reader"]
pub struct R(crate::R<LOG_MEM_END_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOG_MEM_END_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOG_MEM_END_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOG_MEM_END_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOG_MEM_END` writer"]
pub struct W(crate::W<LOG_MEM_END_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOG_MEM_END_SPEC>;
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
impl From<crate::W<LOG_MEM_END_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOG_MEM_END_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOG_MEM_END` reader - mem end addr"]
pub struct LOG_MEM_END_R(crate::FieldReader<u32>);
impl LOG_MEM_END_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        LOG_MEM_END_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOG_MEM_END_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOG_MEM_END` writer - mem end addr"]
pub struct LOG_MEM_END_W<'a> {
    w: &'a mut W,
}
impl<'a> LOG_MEM_END_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - mem end addr"]
    #[inline(always)]
    pub fn log_mem_end(&self) -> LOG_MEM_END_R {
        LOG_MEM_END_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - mem end addr"]
    #[inline(always)]
    pub fn log_mem_end(&mut self) -> LOG_MEM_END_W {
        LOG_MEM_END_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "log mem region configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [log_mem_end](index.html) module"]
pub struct LOG_MEM_END_SPEC;
impl crate::RegisterSpec for LOG_MEM_END_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [log_mem_end::R](R) reader structure"]
impl crate::Readable for LOG_MEM_END_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [log_mem_end::W](W) writer structure"]
impl crate::Writable for LOG_MEM_END_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOG_MEM_END to value 0"]
impl crate::Resettable for LOG_MEM_END_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
