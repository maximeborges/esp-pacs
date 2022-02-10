#[doc = "Register `ROM_CTRL_0` reader"]
pub struct R(crate::R<ROM_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROM_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROM_CTRL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROM_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROM_CTRL_0` writer"]
pub struct W(crate::W<ROM_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROM_CTRL_0_SPEC>;
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
impl From<crate::W<ROM_CTRL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROM_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROM_FO` reader - This field is used to force on clock gate of internal ROM."]
pub struct ROM_FO_R(crate::FieldReader<u8, u8>);
impl ROM_FO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ROM_FO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_FO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM_FO` writer - This field is used to force on clock gate of internal ROM."]
pub struct ROM_FO_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_FO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - This field is used to force on clock gate of internal ROM."]
    #[inline(always)]
    pub fn rom_fo(&self) -> ROM_FO_R {
        ROM_FO_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - This field is used to force on clock gate of internal ROM."]
    #[inline(always)]
    pub fn rom_fo(&mut self) -> ROM_FO_W {
        ROM_FO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System ROM configuration register 0\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_ctrl_0]
(index.html) module"]
pub struct ROM_CTRL_0_SPEC;
impl crate::RegisterSpec for ROM_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rom_ctrl_0::R]
(R) reader structure"]
impl crate::Readable for ROM_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rom_ctrl_0::W]
(W) writer structure"]
impl crate::Writable for ROM_CTRL_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROM_CTRL_0 to value 0x03"]
impl crate::Resettable for ROM_CTRL_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
