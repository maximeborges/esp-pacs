#[doc = "Register `STATUS1` reader"]
pub struct R(crate::R<STATUS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS1` writer"]
pub struct W(crate::W<STATUS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS1_SPEC>;
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
impl From<crate::W<STATUS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERRUPT` reader - GPIO32 ~ 53 interrupt status register."]
pub struct INTERRUPT_R(crate::FieldReader<u32>);
impl INTERRUPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        INTERRUPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERRUPT_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERRUPT` writer - GPIO32 ~ 53 interrupt status register."]
pub struct INTERRUPT_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERRUPT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | (value as u32 & 0x003f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21 - GPIO32 ~ 53 interrupt status register."]
    #[inline(always)]
    pub fn interrupt(&self) -> INTERRUPT_R {
        INTERRUPT_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21 - GPIO32 ~ 53 interrupt status register."]
    #[inline(always)]
    pub fn interrupt(&mut self) -> INTERRUPT_W {
        INTERRUPT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO32 ~ 53 interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status1](index.html) module"]
pub struct STATUS1_SPEC;
impl crate::RegisterSpec for STATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status1::R](R) reader structure"]
impl crate::Readable for STATUS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status1::W](W) writer structure"]
impl crate::Writable for STATUS1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS1 to value 0"]
impl crate::Resettable for STATUS1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
