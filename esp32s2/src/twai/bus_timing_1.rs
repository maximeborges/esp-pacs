#[doc = "Register `BUS_TIMING_1` reader"]
pub struct R(crate::R<BUS_TIMING_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUS_TIMING_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUS_TIMING_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUS_TIMING_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUS_TIMING_1` writer"]
pub struct W(crate::W<BUS_TIMING_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUS_TIMING_1_SPEC>;
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
impl From<crate::W<BUS_TIMING_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUS_TIMING_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIME_SEG1` reader - The width of PBS1."]
pub struct TIME_SEG1_R(crate::FieldReader<u8, u8>);
impl TIME_SEG1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIME_SEG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIME_SEG1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIME_SEG1` writer - The width of PBS1."]
pub struct TIME_SEG1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIME_SEG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `TIME_SEG2` reader - The width of PBS2."]
pub struct TIME_SEG2_R(crate::FieldReader<u8, u8>);
impl TIME_SEG2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIME_SEG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIME_SEG2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIME_SEG2` writer - The width of PBS2."]
pub struct TIME_SEG2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIME_SEG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `TIME_SAMP` reader - The number of sample points. 0: the bus is sampled once; 1: the bus is sampled three times"]
pub struct TIME_SAMP_R(crate::FieldReader<bool, bool>);
impl TIME_SAMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIME_SAMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIME_SAMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIME_SAMP` writer - The number of sample points. 0: the bus is sampled once; 1: the bus is sampled three times"]
pub struct TIME_SAMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIME_SAMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - The width of PBS1."]
    #[inline(always)]
    pub fn time_seg1(&self) -> TIME_SEG1_R {
        TIME_SEG1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - The width of PBS2."]
    #[inline(always)]
    pub fn time_seg2(&self) -> TIME_SEG2_R {
        TIME_SEG2_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - The number of sample points. 0: the bus is sampled once; 1: the bus is sampled three times"]
    #[inline(always)]
    pub fn time_samp(&self) -> TIME_SAMP_R {
        TIME_SAMP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - The width of PBS1."]
    #[inline(always)]
    pub fn time_seg1(&mut self) -> TIME_SEG1_W {
        TIME_SEG1_W { w: self }
    }
    #[doc = "Bits 4:6 - The width of PBS2."]
    #[inline(always)]
    pub fn time_seg2(&mut self) -> TIME_SEG2_W {
        TIME_SEG2_W { w: self }
    }
    #[doc = "Bit 7 - The number of sample points. 0: the bus is sampled once; 1: the bus is sampled three times"]
    #[inline(always)]
    pub fn time_samp(&mut self) -> TIME_SAMP_W {
        TIME_SAMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus Timing Register 1\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_timing_1]
(index.html) module"]
pub struct BUS_TIMING_1_SPEC;
impl crate::RegisterSpec for BUS_TIMING_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bus_timing_1::R]
(R) reader structure"]
impl crate::Readable for BUS_TIMING_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bus_timing_1::W]
(W) writer structure"]
impl crate::Writable for BUS_TIMING_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUS_TIMING_1 to value 0"]
impl crate::Resettable for BUS_TIMING_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
