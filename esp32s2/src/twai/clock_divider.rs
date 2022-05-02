#[doc = "Register `CLOCK_DIVIDER` reader"]
pub struct R(crate::R<CLOCK_DIVIDER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_DIVIDER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_DIVIDER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCK_DIVIDER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLOCK_DIVIDER` writer"]
pub struct W(crate::W<CLOCK_DIVIDER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCK_DIVIDER_SPEC>;
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
impl From<crate::W<CLOCK_DIVIDER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLOCK_DIVIDER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CD` reader - These bits are used to configure frequency dividing coefficients of the external CLKOUT pin."]
pub struct CD_R(crate::FieldReader<u8>);
impl CD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CD_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CD` writer - These bits are used to configure frequency dividing coefficients of the external CLKOUT pin."]
pub struct CD_W<'a> {
    w: &'a mut W,
}
impl<'a> CD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `CLOCK_OFF` reader - This bit can be configured under reset mode. 1: Disable the external CLKOUT pin; 0: Enable the external CLKOUT pin"]
pub struct CLOCK_OFF_R(crate::FieldReader<bool>);
impl CLOCK_OFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLOCK_OFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLOCK_OFF_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLOCK_OFF` writer - This bit can be configured under reset mode. 1: Disable the external CLKOUT pin; 0: Enable the external CLKOUT pin"]
pub struct CLOCK_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCK_OFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - These bits are used to configure frequency dividing coefficients of the external CLKOUT pin."]
    #[inline(always)]
    pub fn cd(&self) -> CD_R {
        CD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - This bit can be configured under reset mode. 1: Disable the external CLKOUT pin; 0: Enable the external CLKOUT pin"]
    #[inline(always)]
    pub fn clock_off(&self) -> CLOCK_OFF_R {
        CLOCK_OFF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - These bits are used to configure frequency dividing coefficients of the external CLKOUT pin."]
    #[inline(always)]
    pub fn cd(&mut self) -> CD_W {
        CD_W { w: self }
    }
    #[doc = "Bit 8 - This bit can be configured under reset mode. 1: Disable the external CLKOUT pin; 0: Enable the external CLKOUT pin"]
    #[inline(always)]
    pub fn clock_off(&mut self) -> CLOCK_OFF_W {
        CLOCK_OFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Divider register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_divider](index.html) module"]
pub struct CLOCK_DIVIDER_SPEC;
impl crate::RegisterSpec for CLOCK_DIVIDER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock_divider::R](R) reader structure"]
impl crate::Readable for CLOCK_DIVIDER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clock_divider::W](W) writer structure"]
impl crate::Writable for CLOCK_DIVIDER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLOCK_DIVIDER to value 0"]
impl crate::Resettable for CLOCK_DIVIDER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
