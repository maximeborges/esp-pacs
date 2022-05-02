#[doc = "Register `IN_DLY` reader"]
pub struct R(crate::R<IN_DLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_DLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_DLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_DLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN_DLY` writer"]
pub struct W(crate::W<IN_DLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN_DLY_SPEC>;
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
impl From<crate::W<IN_DLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN_DLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0` reader - Configure GPIO0 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub struct CH0_R(crate::FieldReader<u8>);
impl CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0` writer - Configure GPIO0 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub struct CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `CH1` reader - Configure GPIO1 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub struct CH1_R(crate::FieldReader<u8>);
impl CH1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1` writer - Configure GPIO1 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub struct CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 2)) | ((value as u32 & 3) << 2);
        self.w
    }
}
#[doc = "Field `CH2` reader - Configure GPIO2 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub struct CH2_R(crate::FieldReader<u8>);
impl CH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2` writer - Configure GPIO2 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub struct CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "Field `CH3` reader - Configure GPIO3 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub struct CH3_R(crate::FieldReader<u8>);
impl CH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3` writer - Configure GPIO3 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub struct CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u32 & 3) << 6);
        self.w
    }
}
#[doc = "Field `CH4` reader - Configure GPIO4 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub struct CH4_R(crate::FieldReader<u8>);
impl CH4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH4_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH4` writer - Configure GPIO4 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub struct CH4_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 8)) | ((value as u32 & 3) << 8);
        self.w
    }
}
#[doc = "Field `CH5` reader - Configure GPIO5 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub struct CH5_R(crate::FieldReader<u8>);
impl CH5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH5_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH5` writer - Configure GPIO5 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub struct CH5_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 10)) | ((value as u32 & 3) << 10);
        self.w
    }
}
#[doc = "Field `CH6` reader - Configure GPIO6 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub struct CH6_R(crate::FieldReader<u8>);
impl CH6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH6_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH6` writer - Configure GPIO6 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub struct CH6_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 12)) | ((value as u32 & 3) << 12);
        self.w
    }
}
#[doc = "Field `CH7` reader - Configure GPIO7 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub struct CH7_R(crate::FieldReader<u8>);
impl CH7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH7_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH7` writer - Configure GPIO7 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub struct CH7_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 14)) | ((value as u32 & 3) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Configure GPIO0 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Configure GPIO1 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Configure GPIO2 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Configure GPIO3 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Configure GPIO4 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Configure GPIO5 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Configure GPIO6 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Configure GPIO7 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configure GPIO0 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W {
        CH0_W { w: self }
    }
    #[doc = "Bits 2:3 - Configure GPIO1 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH1_W {
        CH1_W { w: self }
    }
    #[doc = "Bits 4:5 - Configure GPIO2 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch2(&mut self) -> CH2_W {
        CH2_W { w: self }
    }
    #[doc = "Bits 6:7 - Configure GPIO3 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch3(&mut self) -> CH3_W {
        CH3_W { w: self }
    }
    #[doc = "Bits 8:9 - Configure GPIO4 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch4(&mut self) -> CH4_W {
        CH4_W { w: self }
    }
    #[doc = "Bits 10:11 - Configure GPIO5 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch5(&mut self) -> CH5_W {
        CH5_W { w: self }
    }
    #[doc = "Bits 12:13 - Configure GPIO6 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch6(&mut self) -> CH6_W {
        CH6_W { w: self }
    }
    #[doc = "Bits 14:15 - Configure GPIO7 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch7(&mut self) -> CH7_W {
        CH7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dedicated GPIO input delay configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_dly](index.html) module"]
pub struct IN_DLY_SPEC;
impl crate::RegisterSpec for IN_DLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_dly::R](R) reader structure"]
impl crate::Readable for IN_DLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in_dly::W](W) writer structure"]
impl crate::Writable for IN_DLY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IN_DLY to value 0"]
impl crate::Resettable for IN_DLY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
