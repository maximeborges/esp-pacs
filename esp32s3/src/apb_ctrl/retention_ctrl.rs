#[doc = "Register `RETENTION_CTRL` reader"]
pub struct R(crate::R<RETENTION_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RETENTION_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RETENTION_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RETENTION_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RETENTION_CTRL` writer"]
pub struct W(crate::W<RETENTION_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RETENTION_CTRL_SPEC>;
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
impl From<crate::W<RETENTION_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RETENTION_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RETENTION_CPU_LINK_ADDR` reader - ******* Description ***********"]
pub struct RETENTION_CPU_LINK_ADDR_R(crate::FieldReader<u32, u32>);
impl RETENTION_CPU_LINK_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RETENTION_CPU_LINK_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RETENTION_CPU_LINK_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RETENTION_CPU_LINK_ADDR` writer - ******* Description ***********"]
pub struct RETENTION_CPU_LINK_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> RETENTION_CPU_LINK_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff_ffff) | (value as u32 & 0x07ff_ffff);
        self.w
    }
}
#[doc = "Field `NOBYPASS_CPU_ISO_RST` reader - ******* Description ***********"]
pub struct NOBYPASS_CPU_ISO_RST_R(crate::FieldReader<bool, bool>);
impl NOBYPASS_CPU_ISO_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NOBYPASS_CPU_ISO_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOBYPASS_CPU_ISO_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOBYPASS_CPU_ISO_RST` writer - ******* Description ***********"]
pub struct NOBYPASS_CPU_ISO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> NOBYPASS_CPU_ISO_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 27)) | ((value as u32 & 1) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:26 - ******* Description ***********"]
    #[inline(always)]
    pub fn retention_cpu_link_addr(&self) -> RETENTION_CPU_LINK_ADDR_R {
        RETENTION_CPU_LINK_ADDR_R::new((self.bits & 0x07ff_ffff) as u32)
    }
    #[doc = "Bit 27 - ******* Description ***********"]
    #[inline(always)]
    pub fn nobypass_cpu_iso_rst(&self) -> NOBYPASS_CPU_ISO_RST_R {
        NOBYPASS_CPU_ISO_RST_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:26 - ******* Description ***********"]
    #[inline(always)]
    pub fn retention_cpu_link_addr(&mut self) -> RETENTION_CPU_LINK_ADDR_W {
        RETENTION_CPU_LINK_ADDR_W { w: self }
    }
    #[doc = "Bit 27 - ******* Description ***********"]
    #[inline(always)]
    pub fn nobypass_cpu_iso_rst(&mut self) -> NOBYPASS_CPU_ISO_RST_W {
        NOBYPASS_CPU_ISO_RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [retention_ctrl]
(index.html) module"]
pub struct RETENTION_CTRL_SPEC;
impl crate::RegisterSpec for RETENTION_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [retention_ctrl::R]
(R) reader structure"]
impl crate::Readable for RETENTION_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [retention_ctrl::W]
(W) writer structure"]
impl crate::Writable for RETENTION_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RETENTION_CTRL to value 0"]
impl crate::Resettable for RETENTION_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
