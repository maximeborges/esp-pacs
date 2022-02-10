#[doc = "Register `DCACHE_PRELOCK_CTRL` reader"]
pub struct R(crate::R<DCACHE_PRELOCK_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_PRELOCK_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_PRELOCK_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_PRELOCK_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCACHE_PRELOCK_CTRL` writer"]
pub struct W(crate::W<DCACHE_PRELOCK_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCACHE_PRELOCK_CTRL_SPEC>;
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
impl From<crate::W<DCACHE_PRELOCK_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCACHE_PRELOCK_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCACHE_PRELOCK_SCT0_EN` reader - The bit is used to enable the first section of prelock function."]
pub struct DCACHE_PRELOCK_SCT0_EN_R(crate::FieldReader<bool, bool>);
impl DCACHE_PRELOCK_SCT0_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCACHE_PRELOCK_SCT0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_PRELOCK_SCT0_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_PRELOCK_SCT0_EN` writer - The bit is used to enable the first section of prelock function."]
pub struct DCACHE_PRELOCK_SCT0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_PRELOCK_SCT0_EN_W<'a> {
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
#[doc = "Field `DCACHE_PRELOCK_SCT1_EN` reader - The bit is used to enable the second section of prelock function."]
pub struct DCACHE_PRELOCK_SCT1_EN_R(crate::FieldReader<bool, bool>);
impl DCACHE_PRELOCK_SCT1_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCACHE_PRELOCK_SCT1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_PRELOCK_SCT1_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_PRELOCK_SCT1_EN` writer - The bit is used to enable the second section of prelock function."]
pub struct DCACHE_PRELOCK_SCT1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_PRELOCK_SCT1_EN_W<'a> {
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
    #[doc = "Bit 0 - The bit is used to enable the first section of prelock function."]
    #[inline(always)]
    pub fn dcache_prelock_sct0_en(&self) -> DCACHE_PRELOCK_SCT0_EN_R {
        DCACHE_PRELOCK_SCT0_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable the second section of prelock function."]
    #[inline(always)]
    pub fn dcache_prelock_sct1_en(&self) -> DCACHE_PRELOCK_SCT1_EN_R {
        DCACHE_PRELOCK_SCT1_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable the first section of prelock function."]
    #[inline(always)]
    pub fn dcache_prelock_sct0_en(&mut self) -> DCACHE_PRELOCK_SCT0_EN_W {
        DCACHE_PRELOCK_SCT0_EN_W { w: self }
    }
    #[doc = "Bit 1 - The bit is used to enable the second section of prelock function."]
    #[inline(always)]
    pub fn dcache_prelock_sct1_en(&mut self) -> DCACHE_PRELOCK_SCT1_EN_W {
        DCACHE_PRELOCK_SCT1_EN_W { w: self }
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
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcache_prelock_ctrl]
(index.html) module"]
pub struct DCACHE_PRELOCK_CTRL_SPEC;
impl crate::RegisterSpec for DCACHE_PRELOCK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcache_prelock_ctrl::R]
(R) reader structure"]
impl crate::Readable for DCACHE_PRELOCK_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcache_prelock_ctrl::W]
(W) writer structure"]
impl crate::Writable for DCACHE_PRELOCK_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCACHE_PRELOCK_CTRL to value 0"]
impl crate::Resettable for DCACHE_PRELOCK_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
