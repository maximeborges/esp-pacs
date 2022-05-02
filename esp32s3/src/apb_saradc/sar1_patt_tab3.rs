#[doc = "Register `SAR1_PATT_TAB3` reader"]
pub struct R(crate::R<SAR1_PATT_TAB3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR1_PATT_TAB3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR1_PATT_TAB3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR1_PATT_TAB3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR1_PATT_TAB3` writer"]
pub struct W(crate::W<SAR1_PATT_TAB3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR1_PATT_TAB3_SPEC>;
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
impl From<crate::W<SAR1_PATT_TAB3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR1_PATT_TAB3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SARADC_SAR1_PATT_TAB3` reader - Item 8 ~ 11 for pattern table 1 (each item 6bit)"]
pub struct SARADC_SAR1_PATT_TAB3_R(crate::FieldReader<u32>);
impl SARADC_SAR1_PATT_TAB3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SARADC_SAR1_PATT_TAB3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_SAR1_PATT_TAB3_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_SAR1_PATT_TAB3` writer - Item 8 ~ 11 for pattern table 1 (each item 6bit)"]
pub struct SARADC_SAR1_PATT_TAB3_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_SAR1_PATT_TAB3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Item 8 ~ 11 for pattern table 1 (each item 6bit)"]
    #[inline(always)]
    pub fn saradc_sar1_patt_tab3(&self) -> SARADC_SAR1_PATT_TAB3_R {
        SARADC_SAR1_PATT_TAB3_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Item 8 ~ 11 for pattern table 1 (each item 6bit)"]
    #[inline(always)]
    pub fn saradc_sar1_patt_tab3(&mut self) -> SARADC_SAR1_PATT_TAB3_W {
        SARADC_SAR1_PATT_TAB3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure apb saradc pattern table\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar1_patt_tab3](index.html) module"]
pub struct SAR1_PATT_TAB3_SPEC;
impl crate::RegisterSpec for SAR1_PATT_TAB3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar1_patt_tab3::R](R) reader structure"]
impl crate::Readable for SAR1_PATT_TAB3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar1_patt_tab3::W](W) writer structure"]
impl crate::Writable for SAR1_PATT_TAB3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR1_PATT_TAB3 to value 0"]
impl crate::Resettable for SAR1_PATT_TAB3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
