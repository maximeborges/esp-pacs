#[doc = "Register `OUT_LINK_CH2` reader"]
pub struct R(crate::R<OUT_LINK_CH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_LINK_CH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_LINK_CH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_LINK_CH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_LINK_CH2` writer"]
pub struct W(crate::W<OUT_LINK_CH2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_LINK_CH2_SPEC>;
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
impl From<crate::W<OUT_LINK_CH2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_LINK_CH2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTLINK_ADDR_CH2` reader - This register stores the 20 least significant bits of the first outlink descriptor's address."]
pub struct OUTLINK_ADDR_CH2_R(crate::FieldReader<u32, u32>);
impl OUTLINK_ADDR_CH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        OUTLINK_ADDR_CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTLINK_ADDR_CH2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTLINK_ADDR_CH2` writer - This register stores the 20 least significant bits of the first outlink descriptor's address."]
pub struct OUTLINK_ADDR_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTLINK_ADDR_CH2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
#[doc = "Field `OUTLINK_STOP_CH2` reader - Set this bit to stop dealing with the outlink descriptors."]
pub struct OUTLINK_STOP_CH2_R(crate::FieldReader<bool, bool>);
impl OUTLINK_STOP_CH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTLINK_STOP_CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTLINK_STOP_CH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTLINK_STOP_CH2` writer - Set this bit to stop dealing with the outlink descriptors."]
pub struct OUTLINK_STOP_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTLINK_STOP_CH2_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 20)) | ((value as u32 & 1) << 20);
        self.w
    }
}
#[doc = "Field `OUTLINK_START_CH2` reader - Set this bit to start dealing with the outlink descriptors."]
pub struct OUTLINK_START_CH2_R(crate::FieldReader<bool, bool>);
impl OUTLINK_START_CH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTLINK_START_CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTLINK_START_CH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTLINK_START_CH2` writer - Set this bit to start dealing with the outlink descriptors."]
pub struct OUTLINK_START_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTLINK_START_CH2_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 21)) | ((value as u32 & 1) << 21);
        self.w
    }
}
#[doc = "Field `OUTLINK_RESTART_CH2` reader - Set this bit to restart a new outlink from the last address."]
pub struct OUTLINK_RESTART_CH2_R(crate::FieldReader<bool, bool>);
impl OUTLINK_RESTART_CH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTLINK_RESTART_CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTLINK_RESTART_CH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTLINK_RESTART_CH2` writer - Set this bit to restart a new outlink from the last address."]
pub struct OUTLINK_RESTART_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTLINK_RESTART_CH2_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 22)) | ((value as u32 & 1) << 22);
        self.w
    }
}
#[doc = "Field `OUTLINK_PARK_CH2` reader - 1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working."]
pub struct OUTLINK_PARK_CH2_R(crate::FieldReader<bool, bool>);
impl OUTLINK_PARK_CH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTLINK_PARK_CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTLINK_PARK_CH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:19 - This register stores the 20 least significant bits of the first outlink descriptor's address."]
    #[inline(always)]
    pub fn outlink_addr_ch2(&self) -> OUTLINK_ADDR_CH2_R {
        OUTLINK_ADDR_CH2_R::new((self.bits & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 20 - Set this bit to stop dealing with the outlink descriptors."]
    #[inline(always)]
    pub fn outlink_stop_ch2(&self) -> OUTLINK_STOP_CH2_R {
        OUTLINK_STOP_CH2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to start dealing with the outlink descriptors."]
    #[inline(always)]
    pub fn outlink_start_ch2(&self) -> OUTLINK_START_CH2_R {
        OUTLINK_START_CH2_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set this bit to restart a new outlink from the last address."]
    #[inline(always)]
    pub fn outlink_restart_ch2(&self) -> OUTLINK_RESTART_CH2_R {
        OUTLINK_RESTART_CH2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working."]
    #[inline(always)]
    pub fn outlink_park_ch2(&self) -> OUTLINK_PARK_CH2_R {
        OUTLINK_PARK_CH2_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - This register stores the 20 least significant bits of the first outlink descriptor's address."]
    #[inline(always)]
    pub fn outlink_addr_ch2(&mut self) -> OUTLINK_ADDR_CH2_W {
        OUTLINK_ADDR_CH2_W { w: self }
    }
    #[doc = "Bit 20 - Set this bit to stop dealing with the outlink descriptors."]
    #[inline(always)]
    pub fn outlink_stop_ch2(&mut self) -> OUTLINK_STOP_CH2_W {
        OUTLINK_STOP_CH2_W { w: self }
    }
    #[doc = "Bit 21 - Set this bit to start dealing with the outlink descriptors."]
    #[inline(always)]
    pub fn outlink_start_ch2(&mut self) -> OUTLINK_START_CH2_W {
        OUTLINK_START_CH2_W { w: self }
    }
    #[doc = "Bit 22 - Set this bit to restart a new outlink from the last address."]
    #[inline(always)]
    pub fn outlink_restart_ch2(&mut self) -> OUTLINK_RESTART_CH2_W {
        OUTLINK_RESTART_CH2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_OUT_LINK_CH2_REG.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_link_ch2]
(index.html) module"]
pub struct OUT_LINK_CH2_SPEC;
impl crate::RegisterSpec for OUT_LINK_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_link_ch2::R]
(R) reader structure"]
impl crate::Readable for OUT_LINK_CH2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_link_ch2::W]
(W) writer structure"]
impl crate::Writable for OUT_LINK_CH2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT_LINK_CH2 to value 0x0080_0000"]
impl crate::Resettable for OUT_LINK_CH2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0080_0000
    }
}
