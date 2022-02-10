#[doc = "Register `BUS_TIMING_0` reader"]
pub struct R(crate::R<BUS_TIMING_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUS_TIMING_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUS_TIMING_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUS_TIMING_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUS_TIMING_0` writer"]
pub struct W(crate::W<BUS_TIMING_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUS_TIMING_0_SPEC>;
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
impl From<crate::W<BUS_TIMING_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUS_TIMING_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BAUD_PRESC` reader - Baud Rate Prescaler, determines the frequency dividing ratio."]
pub struct BAUD_PRESC_R(crate::FieldReader<u16, u16>);
impl BAUD_PRESC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BAUD_PRESC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BAUD_PRESC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BAUD_PRESC` writer - Baud Rate Prescaler, determines the frequency dividing ratio."]
pub struct BAUD_PRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> BAUD_PRESC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
#[doc = "Field `SYNC_JUMP_WIDTH` reader - Synchronization Jump Width (SJW), 1 \\verb+~+ 14 Tq wide."]
pub struct SYNC_JUMP_WIDTH_R(crate::FieldReader<u8, u8>);
impl SYNC_JUMP_WIDTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SYNC_JUMP_WIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNC_JUMP_WIDTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNC_JUMP_WIDTH` writer - Synchronization Jump Width (SJW), 1 \\verb+~+ 14 Tq wide."]
pub struct SYNC_JUMP_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_JUMP_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - Baud Rate Prescaler, determines the frequency dividing ratio."]
    #[inline(always)]
    pub fn baud_presc(&self) -> BAUD_PRESC_R {
        BAUD_PRESC_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:15 - Synchronization Jump Width (SJW), 1 \\verb+~+ 14 Tq wide."]
    #[inline(always)]
    pub fn sync_jump_width(&self) -> SYNC_JUMP_WIDTH_R {
        SYNC_JUMP_WIDTH_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - Baud Rate Prescaler, determines the frequency dividing ratio."]
    #[inline(always)]
    pub fn baud_presc(&mut self) -> BAUD_PRESC_W {
        BAUD_PRESC_W { w: self }
    }
    #[doc = "Bits 14:15 - Synchronization Jump Width (SJW), 1 \\verb+~+ 14 Tq wide."]
    #[inline(always)]
    pub fn sync_jump_width(&mut self) -> SYNC_JUMP_WIDTH_W {
        SYNC_JUMP_WIDTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus Timing Register 0\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_timing_0]
(index.html) module"]
pub struct BUS_TIMING_0_SPEC;
impl crate::RegisterSpec for BUS_TIMING_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bus_timing_0::R]
(R) reader structure"]
impl crate::Readable for BUS_TIMING_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bus_timing_0::W]
(W) writer structure"]
impl crate::Writable for BUS_TIMING_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUS_TIMING_0 to value 0"]
impl crate::Resettable for BUS_TIMING_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
