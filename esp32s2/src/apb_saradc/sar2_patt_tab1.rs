#[doc = "Register `SAR2_PATT_TAB1` reader"]
pub struct R(crate::R<SAR2_PATT_TAB1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR2_PATT_TAB1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR2_PATT_TAB1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR2_PATT_TAB1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR2_PATT_TAB1` writer"]
pub struct W(crate::W<SAR2_PATT_TAB1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR2_PATT_TAB1_SPEC>;
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
impl From<crate::W<SAR2_PATT_TAB1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR2_PATT_TAB1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR2_PATT_TAB1` reader - item 0 ~ 3 for pattern table 2 (each item one byte)"]
pub struct SAR2_PATT_TAB1_R(crate::FieldReader<u32, u32>);
impl SAR2_PATT_TAB1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SAR2_PATT_TAB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_PATT_TAB1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_PATT_TAB1` writer - item 0 ~ 3 for pattern table 2 (each item one byte)"]
pub struct SAR2_PATT_TAB1_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_PATT_TAB1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - item 0 ~ 3 for pattern table 2 (each item one byte)"]
    #[inline(always)]
    pub fn sar2_patt_tab1(&self) -> SAR2_PATT_TAB1_R {
        SAR2_PATT_TAB1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - item 0 ~ 3 for pattern table 2 (each item one byte)"]
    #[inline(always)]
    pub fn sar2_patt_tab1(&mut self) -> SAR2_PATT_TAB1_W {
        SAR2_PATT_TAB1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "item 0 ~ 3 for pattern table 2 (each item one byte)\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar2_patt_tab1]
(index.html) module"]
pub struct SAR2_PATT_TAB1_SPEC;
impl crate::RegisterSpec for SAR2_PATT_TAB1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar2_patt_tab1::R]
(R) reader structure"]
impl crate::Readable for SAR2_PATT_TAB1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar2_patt_tab1::W]
(W) writer structure"]
impl crate::Writable for SAR2_PATT_TAB1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR2_PATT_TAB1 to value 0x0f0f_0f0f"]
impl crate::Resettable for SAR2_PATT_TAB1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f0f_0f0f
    }
}
