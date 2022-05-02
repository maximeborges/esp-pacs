#[doc = "Register `TOUCH_CTRL1` reader"]
pub struct R(crate::R<TOUCH_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOUCH_CTRL1` writer"]
pub struct W(crate::W<TOUCH_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_CTRL1_SPEC>;
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
impl From<crate::W<TOUCH_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_SLEEP_CYCLES` reader - Set sleep cycles for touch timer."]
pub struct TOUCH_SLEEP_CYCLES_R(crate::FieldReader<u16>);
impl TOUCH_SLEEP_CYCLES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TOUCH_SLEEP_CYCLES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_SLEEP_CYCLES_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_SLEEP_CYCLES` writer - Set sleep cycles for touch timer."]
pub struct TOUCH_SLEEP_CYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_SLEEP_CYCLES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `TOUCH_MEAS_NUM` reader - Configure measurement length (in 8 MHz), i.e., charge/discharge times."]
pub struct TOUCH_MEAS_NUM_R(crate::FieldReader<u16>);
impl TOUCH_MEAS_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TOUCH_MEAS_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_MEAS_NUM_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_MEAS_NUM` writer - Configure measurement length (in 8 MHz), i.e., charge/discharge times."]
pub struct TOUCH_MEAS_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_MEAS_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Set sleep cycles for touch timer."]
    #[inline(always)]
    pub fn touch_sleep_cycles(&self) -> TOUCH_SLEEP_CYCLES_R {
        TOUCH_SLEEP_CYCLES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Configure measurement length (in 8 MHz), i.e., charge/discharge times."]
    #[inline(always)]
    pub fn touch_meas_num(&self) -> TOUCH_MEAS_NUM_R {
        TOUCH_MEAS_NUM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Set sleep cycles for touch timer."]
    #[inline(always)]
    pub fn touch_sleep_cycles(&mut self) -> TOUCH_SLEEP_CYCLES_W {
        TOUCH_SLEEP_CYCLES_W { w: self }
    }
    #[doc = "Bits 16:31 - Configure measurement length (in 8 MHz), i.e., charge/discharge times."]
    #[inline(always)]
    pub fn touch_meas_num(&mut self) -> TOUCH_MEAS_NUM_W {
        TOUCH_MEAS_NUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Touch control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_ctrl1](index.html) module"]
pub struct TOUCH_CTRL1_SPEC;
impl crate::RegisterSpec for TOUCH_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch_ctrl1::R](R) reader structure"]
impl crate::Readable for TOUCH_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch_ctrl1::W](W) writer structure"]
impl crate::Writable for TOUCH_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TOUCH_CTRL1 to value 0x1000_0100"]
impl crate::Resettable for TOUCH_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000_0100
    }
}
