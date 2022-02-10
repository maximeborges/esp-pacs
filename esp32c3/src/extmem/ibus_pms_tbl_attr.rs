#[doc = "Register `IBUS_PMS_TBL_ATTR` reader"]
pub struct R(crate::R<IBUS_PMS_TBL_ATTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IBUS_PMS_TBL_ATTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IBUS_PMS_TBL_ATTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IBUS_PMS_TBL_ATTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IBUS_PMS_TBL_ATTR` writer"]
pub struct W(crate::W<IBUS_PMS_TBL_ATTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IBUS_PMS_TBL_ATTR_SPEC>;
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
impl From<crate::W<IBUS_PMS_TBL_ATTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IBUS_PMS_TBL_ATTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IBUS_PMS_SCT1_ATTR` reader - The bit is used to configure attribute of the ibus permission control section1, bit0: fetch in world0, bit1: load in world0, bit2: fetch in world1, bit3: load in world1"]
pub struct IBUS_PMS_SCT1_ATTR_R(crate::FieldReader<u8, u8>);
impl IBUS_PMS_SCT1_ATTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IBUS_PMS_SCT1_ATTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBUS_PMS_SCT1_ATTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBUS_PMS_SCT1_ATTR` writer - The bit is used to configure attribute of the ibus permission control section1, bit0: fetch in world0, bit1: load in world0, bit2: fetch in world1, bit3: load in world1"]
pub struct IBUS_PMS_SCT1_ATTR_W<'a> {
    w: &'a mut W,
}
impl<'a> IBUS_PMS_SCT1_ATTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `IBUS_PMS_SCT2_ATTR` reader - The bit is used to configure attribute of the ibus permission control section2, bit0: fetch in world0, bit1: load in world0, bit2: fetch in world1, bit3: load in world1"]
pub struct IBUS_PMS_SCT2_ATTR_R(crate::FieldReader<u8, u8>);
impl IBUS_PMS_SCT2_ATTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IBUS_PMS_SCT2_ATTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBUS_PMS_SCT2_ATTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBUS_PMS_SCT2_ATTR` writer - The bit is used to configure attribute of the ibus permission control section2, bit0: fetch in world0, bit1: load in world0, bit2: fetch in world1, bit3: load in world1"]
pub struct IBUS_PMS_SCT2_ATTR_W<'a> {
    w: &'a mut W,
}
impl<'a> IBUS_PMS_SCT2_ATTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - The bit is used to configure attribute of the ibus permission control section1, bit0: fetch in world0, bit1: load in world0, bit2: fetch in world1, bit3: load in world1"]
    #[inline(always)]
    pub fn ibus_pms_sct1_attr(&self) -> IBUS_PMS_SCT1_ATTR_R {
        IBUS_PMS_SCT1_ATTR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - The bit is used to configure attribute of the ibus permission control section2, bit0: fetch in world0, bit1: load in world0, bit2: fetch in world1, bit3: load in world1"]
    #[inline(always)]
    pub fn ibus_pms_sct2_attr(&self) -> IBUS_PMS_SCT2_ATTR_R {
        IBUS_PMS_SCT2_ATTR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - The bit is used to configure attribute of the ibus permission control section1, bit0: fetch in world0, bit1: load in world0, bit2: fetch in world1, bit3: load in world1"]
    #[inline(always)]
    pub fn ibus_pms_sct1_attr(&mut self) -> IBUS_PMS_SCT1_ATTR_W {
        IBUS_PMS_SCT1_ATTR_W { w: self }
    }
    #[doc = "Bits 4:7 - The bit is used to configure attribute of the ibus permission control section2, bit0: fetch in world0, bit1: load in world0, bit2: fetch in world1, bit3: load in world1"]
    #[inline(always)]
    pub fn ibus_pms_sct2_attr(&mut self) -> IBUS_PMS_SCT2_ATTR_W {
        IBUS_PMS_SCT2_ATTR_W { w: self }
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
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ibus_pms_tbl_attr]
(index.html) module"]
pub struct IBUS_PMS_TBL_ATTR_SPEC;
impl crate::RegisterSpec for IBUS_PMS_TBL_ATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ibus_pms_tbl_attr::R]
(R) reader structure"]
impl crate::Readable for IBUS_PMS_TBL_ATTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ibus_pms_tbl_attr::W]
(W) writer structure"]
impl crate::Writable for IBUS_PMS_TBL_ATTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IBUS_PMS_TBL_ATTR to value 0xff"]
impl crate::Resettable for IBUS_PMS_TBL_ATTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
