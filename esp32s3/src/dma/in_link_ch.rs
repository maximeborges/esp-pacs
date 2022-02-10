#[doc = "Register `IN_LINK_CH%s` reader"]
pub struct R(crate::R<IN_LINK_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_LINK_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_LINK_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_LINK_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN_LINK_CH%s` writer"]
pub struct W(crate::W<IN_LINK_CH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN_LINK_CH_SPEC>;
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
impl From<crate::W<IN_LINK_CH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN_LINK_CH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INLINK_ADDR_CH` reader - This register stores the 20 least significant bits of the first inlink descriptor's address."]
pub struct INLINK_ADDR_CH_R(crate::FieldReader<u32, u32>);
impl INLINK_ADDR_CH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        INLINK_ADDR_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INLINK_ADDR_CH_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INLINK_ADDR_CH` writer - This register stores the 20 least significant bits of the first inlink descriptor's address."]
pub struct INLINK_ADDR_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> INLINK_ADDR_CH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
#[doc = "Field `INLINK_AUTO_RET_CH` reader - Set this bit to return to current inlink descriptor's address, when there are some errors in current receiving data."]
pub struct INLINK_AUTO_RET_CH_R(crate::FieldReader<bool, bool>);
impl INLINK_AUTO_RET_CH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INLINK_AUTO_RET_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INLINK_AUTO_RET_CH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INLINK_AUTO_RET_CH` writer - Set this bit to return to current inlink descriptor's address, when there are some errors in current receiving data."]
pub struct INLINK_AUTO_RET_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> INLINK_AUTO_RET_CH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `INLINK_STOP_CH` reader - Set this bit to stop dealing with the inlink descriptors."]
pub struct INLINK_STOP_CH_R(crate::FieldReader<bool, bool>);
impl INLINK_STOP_CH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INLINK_STOP_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INLINK_STOP_CH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INLINK_STOP_CH` writer - Set this bit to stop dealing with the inlink descriptors."]
pub struct INLINK_STOP_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> INLINK_STOP_CH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `INLINK_START_CH` reader - Set this bit to start dealing with the inlink descriptors."]
pub struct INLINK_START_CH_R(crate::FieldReader<bool, bool>);
impl INLINK_START_CH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INLINK_START_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INLINK_START_CH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INLINK_START_CH` writer - Set this bit to start dealing with the inlink descriptors."]
pub struct INLINK_START_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> INLINK_START_CH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `INLINK_RESTART_CH` reader - Set this bit to mount a new inlink descriptor."]
pub struct INLINK_RESTART_CH_R(crate::FieldReader<bool, bool>);
impl INLINK_RESTART_CH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INLINK_RESTART_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INLINK_RESTART_CH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INLINK_RESTART_CH` writer - Set this bit to mount a new inlink descriptor."]
pub struct INLINK_RESTART_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> INLINK_RESTART_CH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `INLINK_PARK_CH` reader - 1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working."]
pub struct INLINK_PARK_CH_R(crate::FieldReader<bool, bool>);
impl INLINK_PARK_CH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INLINK_PARK_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INLINK_PARK_CH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:19 - This register stores the 20 least significant bits of the first inlink descriptor's address."]
    #[inline(always)]
    pub fn inlink_addr_ch(&self) -> INLINK_ADDR_CH_R {
        INLINK_ADDR_CH_R::new((self.bits & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 20 - Set this bit to return to current inlink descriptor's address, when there are some errors in current receiving data."]
    #[inline(always)]
    pub fn inlink_auto_ret_ch(&self) -> INLINK_AUTO_RET_CH_R {
        INLINK_AUTO_RET_CH_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Set this bit to stop dealing with the inlink descriptors."]
    #[inline(always)]
    pub fn inlink_stop_ch(&self) -> INLINK_STOP_CH_R {
        INLINK_STOP_CH_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Set this bit to start dealing with the inlink descriptors."]
    #[inline(always)]
    pub fn inlink_start_ch(&self) -> INLINK_START_CH_R {
        INLINK_START_CH_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Set this bit to mount a new inlink descriptor."]
    #[inline(always)]
    pub fn inlink_restart_ch(&self) -> INLINK_RESTART_CH_R {
        INLINK_RESTART_CH_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working."]
    #[inline(always)]
    pub fn inlink_park_ch(&self) -> INLINK_PARK_CH_R {
        INLINK_PARK_CH_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - This register stores the 20 least significant bits of the first inlink descriptor's address."]
    #[inline(always)]
    pub fn inlink_addr_ch(&mut self) -> INLINK_ADDR_CH_W {
        INLINK_ADDR_CH_W { w: self }
    }
    #[doc = "Bit 20 - Set this bit to return to current inlink descriptor's address, when there are some errors in current receiving data."]
    #[inline(always)]
    pub fn inlink_auto_ret_ch(&mut self) -> INLINK_AUTO_RET_CH_W {
        INLINK_AUTO_RET_CH_W { w: self }
    }
    #[doc = "Bit 21 - Set this bit to stop dealing with the inlink descriptors."]
    #[inline(always)]
    pub fn inlink_stop_ch(&mut self) -> INLINK_STOP_CH_W {
        INLINK_STOP_CH_W { w: self }
    }
    #[doc = "Bit 22 - Set this bit to start dealing with the inlink descriptors."]
    #[inline(always)]
    pub fn inlink_start_ch(&mut self) -> INLINK_START_CH_W {
        INLINK_START_CH_W { w: self }
    }
    #[doc = "Bit 23 - Set this bit to mount a new inlink descriptor."]
    #[inline(always)]
    pub fn inlink_restart_ch(&mut self) -> INLINK_RESTART_CH_W {
        INLINK_RESTART_CH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Link descriptor configure and control register of Rx channel 0\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_link_ch]
(index.html) module"]
pub struct IN_LINK_CH_SPEC;
impl crate::RegisterSpec for IN_LINK_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_link_ch::R]
(R) reader structure"]
impl crate::Readable for IN_LINK_CH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in_link_ch::W]
(W) writer structure"]
impl crate::Writable for IN_LINK_CH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IN_LINK_CH%s to value 0x0110_0000"]
impl crate::Resettable for IN_LINK_CH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0110_0000
    }
}
