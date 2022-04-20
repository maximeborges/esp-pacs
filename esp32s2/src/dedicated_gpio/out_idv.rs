#[doc = "Register `OUT_IDV` writer"]
pub struct W(crate::W<OUT_IDV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_IDV_SPEC>;
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
impl From<crate::W<OUT_IDV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_IDV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0` writer - Configure channel 0 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
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
#[doc = "Field `CH1` writer - Configure channel 1 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
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
#[doc = "Field `CH2` writer - Configure channel 2 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
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
#[doc = "Field `CH3` writer - Configure channel 3 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
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
#[doc = "Field `CH4` writer - Configure channel 4 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
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
#[doc = "Field `CH5` writer - Configure channel 5 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
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
#[doc = "Field `CH6` writer - Configure channel 6 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
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
#[doc = "Field `CH7` writer - Configure channel 7 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
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
impl W {
    #[doc = "Bits 0:1 - Configure channel 0 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W {
        CH0_W { w: self }
    }
    #[doc = "Bits 2:3 - Configure channel 1 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH1_W {
        CH1_W { w: self }
    }
    #[doc = "Bits 4:5 - Configure channel 2 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
    #[inline(always)]
    pub fn ch2(&mut self) -> CH2_W {
        CH2_W { w: self }
    }
    #[doc = "Bits 6:7 - Configure channel 3 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
    #[inline(always)]
    pub fn ch3(&mut self) -> CH3_W {
        CH3_W { w: self }
    }
    #[doc = "Bits 8:9 - Configure channel 4 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
    #[inline(always)]
    pub fn ch4(&mut self) -> CH4_W {
        CH4_W { w: self }
    }
    #[doc = "Bits 10:11 - Configure channel 5 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
    #[inline(always)]
    pub fn ch5(&mut self) -> CH5_W {
        CH5_W { w: self }
    }
    #[doc = "Bits 12:13 - Configure channel 6 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
    #[inline(always)]
    pub fn ch6(&mut self) -> CH6_W {
        CH6_W { w: self }
    }
    #[doc = "Bits 14:15 - Configure channel 7 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
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
#[doc = "Dedicated GPIO individual output register\n\nThis register you can [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_idv]
(index.html) module"]
pub struct OUT_IDV_SPEC;
impl crate::RegisterSpec for OUT_IDV_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [out_idv::W]
(W) writer structure"]
impl crate::Writable for OUT_IDV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT_IDV to value 0"]
impl crate::Resettable for OUT_IDV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
