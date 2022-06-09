#[doc = "Register `SAR1_PATT_TAB4` reader"]
pub struct R(crate::R<SAR1_PATT_TAB4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR1_PATT_TAB4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR1_PATT_TAB4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR1_PATT_TAB4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR1_PATT_TAB4` writer"]
pub struct W(crate::W<SAR1_PATT_TAB4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR1_PATT_TAB4_SPEC>;
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
impl From<crate::W<SAR1_PATT_TAB4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR1_PATT_TAB4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SARADC_SAR1_PATT_TAB4` reader - Item 12 ~ 15 for pattern table 1 (each item 6bit)"]
pub type SARADC_SAR1_PATT_TAB4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SARADC_SAR1_PATT_TAB4` writer - Item 12 ~ 15 for pattern table 1 (each item 6bit)"]
pub type SARADC_SAR1_PATT_TAB4_W<'a> =
    crate::FieldWriter<'a, u32, SAR1_PATT_TAB4_SPEC, u32, u32, 24, 0>;
impl R {
    #[doc = "Bits 0:23 - Item 12 ~ 15 for pattern table 1 (each item 6bit)"]
    #[inline(always)]
    pub fn saradc_sar1_patt_tab4(&self) -> SARADC_SAR1_PATT_TAB4_R {
        SARADC_SAR1_PATT_TAB4_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Item 12 ~ 15 for pattern table 1 (each item 6bit)"]
    #[inline(always)]
    pub fn saradc_sar1_patt_tab4(&mut self) -> SARADC_SAR1_PATT_TAB4_W {
        SARADC_SAR1_PATT_TAB4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure apb saradc pattern table\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar1_patt_tab4](index.html) module"]
pub struct SAR1_PATT_TAB4_SPEC;
impl crate::RegisterSpec for SAR1_PATT_TAB4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar1_patt_tab4::R](R) reader structure"]
impl crate::Readable for SAR1_PATT_TAB4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar1_patt_tab4::W](W) writer structure"]
impl crate::Writable for SAR1_PATT_TAB4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR1_PATT_TAB4 to value 0"]
impl crate::Resettable for SAR1_PATT_TAB4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
