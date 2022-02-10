#[doc = "Register `ICACHE_CTRL1` reader"]
pub struct R(crate::R<ICACHE_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICACHE_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICACHE_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICACHE_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICACHE_CTRL1` writer"]
pub struct W(crate::W<ICACHE_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICACHE_CTRL1_SPEC>;
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
impl From<crate::W<ICACHE_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICACHE_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICACHE_SHUT_IBUS` reader - The bit is used to disable core0 ibus, 0: enable, 1: disable"]
pub struct ICACHE_SHUT_IBUS_R(crate::FieldReader<bool, bool>);
impl ICACHE_SHUT_IBUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ICACHE_SHUT_IBUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_SHUT_IBUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_SHUT_IBUS` writer - The bit is used to disable core0 ibus, 0: enable, 1: disable"]
pub struct ICACHE_SHUT_IBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHE_SHUT_IBUS_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `ICACHE_SHUT_DBUS` reader - The bit is used to disable core1 ibus, 0: enable, 1: disable"]
pub struct ICACHE_SHUT_DBUS_R(crate::FieldReader<bool, bool>);
impl ICACHE_SHUT_DBUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ICACHE_SHUT_DBUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_SHUT_DBUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_SHUT_DBUS` writer - The bit is used to disable core1 ibus, 0: enable, 1: disable"]
pub struct ICACHE_SHUT_DBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHE_SHUT_DBUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - The bit is used to disable core0 ibus, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn icache_shut_ibus(&self) -> ICACHE_SHUT_IBUS_R {
        ICACHE_SHUT_IBUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The bit is used to disable core1 ibus, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn icache_shut_dbus(&self) -> ICACHE_SHUT_DBUS_R {
        ICACHE_SHUT_DBUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to disable core0 ibus, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn icache_shut_ibus(&mut self) -> ICACHE_SHUT_IBUS_W {
        ICACHE_SHUT_IBUS_W { w: self }
    }
    #[doc = "Bit 1 - The bit is used to disable core1 ibus, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn icache_shut_dbus(&mut self) -> ICACHE_SHUT_DBUS_W {
        ICACHE_SHUT_DBUS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This description will be updated in the near future.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_ctrl1]
(index.html) module"]
pub struct ICACHE_CTRL1_SPEC;
impl crate::RegisterSpec for ICACHE_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icache_ctrl1::R]
(R) reader structure"]
impl crate::Readable for ICACHE_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icache_ctrl1::W]
(W) writer structure"]
impl crate::Writable for ICACHE_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICACHE_CTRL1 to value 0x03"]
impl crate::Resettable for ICACHE_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
