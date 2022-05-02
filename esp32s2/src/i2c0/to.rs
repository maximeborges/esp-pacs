#[doc = "Register `TO` reader"]
pub struct R(crate::R<TO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TO` writer"]
pub struct W(crate::W<TO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TO_SPEC>;
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
impl From<crate::W<TO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIME_OUT_VALUE` reader - This register is used to configure the timeout for receiving a data bit in APB clock cycles."]
pub struct TIME_OUT_VALUE_R(crate::FieldReader<u32>);
impl TIME_OUT_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TIME_OUT_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIME_OUT_VALUE_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIME_OUT_VALUE` writer - This register is used to configure the timeout for receiving a data bit in APB clock cycles."]
pub struct TIME_OUT_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIME_OUT_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Field `TIME_OUT_EN` reader - This is the enable bit for time out control."]
pub struct TIME_OUT_EN_R(crate::FieldReader<bool>);
impl TIME_OUT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIME_OUT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIME_OUT_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIME_OUT_EN` writer - This is the enable bit for time out control."]
pub struct TIME_OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIME_OUT_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 24)) | ((value as u32 & 1) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - This register is used to configure the timeout for receiving a data bit in APB clock cycles."]
    #[inline(always)]
    pub fn time_out_value(&self) -> TIME_OUT_VALUE_R {
        TIME_OUT_VALUE_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 24 - This is the enable bit for time out control."]
    #[inline(always)]
    pub fn time_out_en(&self) -> TIME_OUT_EN_R {
        TIME_OUT_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - This register is used to configure the timeout for receiving a data bit in APB clock cycles."]
    #[inline(always)]
    pub fn time_out_value(&mut self) -> TIME_OUT_VALUE_W {
        TIME_OUT_VALUE_W { w: self }
    }
    #[doc = "Bit 24 - This is the enable bit for time out control."]
    #[inline(always)]
    pub fn time_out_en(&mut self) -> TIME_OUT_EN_W {
        TIME_OUT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Setting time out control for receiving data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [to](index.html) module"]
pub struct TO_SPEC;
impl crate::RegisterSpec for TO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [to::R](R) reader structure"]
impl crate::Readable for TO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [to::W](W) writer structure"]
impl crate::Writable for TO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TO to value 0"]
impl crate::Resettable for TO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
