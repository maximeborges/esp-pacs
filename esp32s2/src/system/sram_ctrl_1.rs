#[doc = "Register `SRAM_CTRL_1` reader"]
pub struct R(crate::R<SRAM_CTRL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM_CTRL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAM_CTRL_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAM_CTRL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAM_CTRL_1` writer"]
pub struct W(crate::W<SRAM_CTRL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM_CTRL_1_SPEC>;
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
impl From<crate::W<SRAM_CTRL_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAM_CTRL_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAM_FORCE_PD` reader - This field is used to power down internal SRAM."]
pub struct SRAM_FORCE_PD_R(crate::FieldReader<u32, u32>);
impl SRAM_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SRAM_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_FORCE_PD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_FORCE_PD` writer - This field is used to power down internal SRAM."]
pub struct SRAM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_FORCE_PD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | (value as u32 & 0x003f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21 - This field is used to power down internal SRAM."]
    #[inline(always)]
    pub fn sram_force_pd(&self) -> SRAM_FORCE_PD_R {
        SRAM_FORCE_PD_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21 - This field is used to power down internal SRAM."]
    #[inline(always)]
    pub fn sram_force_pd(&mut self) -> SRAM_FORCE_PD_W {
        SRAM_FORCE_PD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System SRAM configuration register 1\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_ctrl_1]
(index.html) module"]
pub struct SRAM_CTRL_1_SPEC;
impl crate::RegisterSpec for SRAM_CTRL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sram_ctrl_1::R]
(R) reader structure"]
impl crate::Readable for SRAM_CTRL_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram_ctrl_1::W]
(W) writer structure"]
impl crate::Writable for SRAM_CTRL_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRAM_CTRL_1 to value 0"]
impl crate::Resettable for SRAM_CTRL_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
