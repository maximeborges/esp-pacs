#[doc = "Register `TOUCH_SLP_THRES` reader"]
pub struct R(crate::R<TOUCH_SLP_THRES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_SLP_THRES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_SLP_THRES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_SLP_THRES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOUCH_SLP_THRES` writer"]
pub struct W(crate::W<TOUCH_SLP_THRES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_SLP_THRES_SPEC>;
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
impl From<crate::W<TOUCH_SLP_THRES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_SLP_THRES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_SLP_TH` reader - Set the threshold for touch sleep pad."]
pub struct TOUCH_SLP_TH_R(crate::FieldReader<u32, u32>);
impl TOUCH_SLP_TH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TOUCH_SLP_TH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_SLP_TH_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_SLP_TH` writer - Set the threshold for touch sleep pad."]
pub struct TOUCH_SLP_TH_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_SLP_TH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | (value as u32 & 0x003f_ffff);
        self.w
    }
}
#[doc = "Field `TOUCH_SLP_APPROACH_EN` reader - Enable the proximity mode of touch sleep pad."]
pub struct TOUCH_SLP_APPROACH_EN_R(crate::FieldReader<bool, bool>);
impl TOUCH_SLP_APPROACH_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_SLP_APPROACH_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_SLP_APPROACH_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_SLP_APPROACH_EN` writer - Enable the proximity mode of touch sleep pad."]
pub struct TOUCH_SLP_APPROACH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_SLP_APPROACH_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `TOUCH_SLP_PAD` reader - Select sleep pad."]
pub struct TOUCH_SLP_PAD_R(crate::FieldReader<u8, u8>);
impl TOUCH_SLP_PAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_SLP_PAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_SLP_PAD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_SLP_PAD` writer - Select sleep pad."]
pub struct TOUCH_SLP_PAD_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_SLP_PAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | ((value as u32 & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21 - Set the threshold for touch sleep pad."]
    #[inline(always)]
    pub fn touch_slp_th(&self) -> TOUCH_SLP_TH_R {
        TOUCH_SLP_TH_R::new((self.bits & 0x003f_ffff) as u32)
    }
    #[doc = "Bit 26 - Enable the proximity mode of touch sleep pad."]
    #[inline(always)]
    pub fn touch_slp_approach_en(&self) -> TOUCH_SLP_APPROACH_EN_R {
        TOUCH_SLP_APPROACH_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 27:31 - Select sleep pad."]
    #[inline(always)]
    pub fn touch_slp_pad(&self) -> TOUCH_SLP_PAD_R {
        TOUCH_SLP_PAD_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:21 - Set the threshold for touch sleep pad."]
    #[inline(always)]
    pub fn touch_slp_th(&mut self) -> TOUCH_SLP_TH_W {
        TOUCH_SLP_TH_W { w: self }
    }
    #[doc = "Bit 26 - Enable the proximity mode of touch sleep pad."]
    #[inline(always)]
    pub fn touch_slp_approach_en(&mut self) -> TOUCH_SLP_APPROACH_EN_W {
        TOUCH_SLP_APPROACH_EN_W { w: self }
    }
    #[doc = "Bits 27:31 - Select sleep pad."]
    #[inline(always)]
    pub fn touch_slp_pad(&mut self) -> TOUCH_SLP_PAD_W {
        TOUCH_SLP_PAD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure the settings of touch sleep pad\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_slp_thres]
(index.html) module"]
pub struct TOUCH_SLP_THRES_SPEC;
impl crate::RegisterSpec for TOUCH_SLP_THRES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch_slp_thres::R]
(R) reader structure"]
impl crate::Readable for TOUCH_SLP_THRES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch_slp_thres::W]
(W) writer structure"]
impl crate::Writable for TOUCH_SLP_THRES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TOUCH_SLP_THRES to value 0x7800_0000"]
impl crate::Resettable for TOUCH_SLP_THRES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7800_0000
    }
}
