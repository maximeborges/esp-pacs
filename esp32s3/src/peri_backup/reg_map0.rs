#[doc = "Register `REG_MAP0` reader"]
pub struct R(crate::R<REG_MAP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_MAP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_MAP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_MAP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_MAP0` writer"]
pub struct W(crate::W<REG_MAP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_MAP0_SPEC>;
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
impl From<crate::W<REG_MAP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_MAP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAP0` reader - x"]
pub struct MAP0_R(crate::FieldReader<u32>);
impl MAP0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        MAP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAP0_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAP0` writer - x"]
pub struct MAP0_W<'a> {
    w: &'a mut W,
}
impl<'a> MAP0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - x"]
    #[inline(always)]
    pub fn map0(&self) -> MAP0_R {
        MAP0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - x"]
    #[inline(always)]
    pub fn map0(&mut self) -> MAP0_W {
        MAP0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "x\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_map0](index.html) module"]
pub struct REG_MAP0_SPEC;
impl crate::RegisterSpec for REG_MAP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_map0::R](R) reader structure"]
impl crate::Readable for REG_MAP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_map0::W](W) writer structure"]
impl crate::Writable for REG_MAP0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG_MAP0 to value 0"]
impl crate::Resettable for REG_MAP0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
