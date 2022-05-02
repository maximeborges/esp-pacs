#[doc = "Register `OUT_PERI_SEL_CH%s` reader"]
pub struct R(crate::R<OUT_PERI_SEL_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_PERI_SEL_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_PERI_SEL_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_PERI_SEL_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_PERI_SEL_CH%s` writer"]
pub struct W(crate::W<OUT_PERI_SEL_CH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_PERI_SEL_CH_SPEC>;
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
impl From<crate::W<OUT_PERI_SEL_CH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_PERI_SEL_CH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERI_OUT_SEL_CH` reader - This register is used to select peripheral for Tx channel 0. 0:SPI2. 1: SPI3. 2: UHCI0. 3: I2S0. 4: I2S1. 5: LCD_CAM. 6: AES. 7: SHA. 8: ADC_DAC. 9: RMT."]
pub struct PERI_OUT_SEL_CH_R(crate::FieldReader<u8>);
impl PERI_OUT_SEL_CH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PERI_OUT_SEL_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERI_OUT_SEL_CH_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERI_OUT_SEL_CH` writer - This register is used to select peripheral for Tx channel 0. 0:SPI2. 1: SPI3. 2: UHCI0. 3: I2S0. 4: I2S1. 5: LCD_CAM. 6: AES. 7: SHA. 8: ADC_DAC. 9: RMT."]
pub struct PERI_OUT_SEL_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> PERI_OUT_SEL_CH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - This register is used to select peripheral for Tx channel 0. 0:SPI2. 1: SPI3. 2: UHCI0. 3: I2S0. 4: I2S1. 5: LCD_CAM. 6: AES. 7: SHA. 8: ADC_DAC. 9: RMT."]
    #[inline(always)]
    pub fn peri_out_sel_ch(&self) -> PERI_OUT_SEL_CH_R {
        PERI_OUT_SEL_CH_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - This register is used to select peripheral for Tx channel 0. 0:SPI2. 1: SPI3. 2: UHCI0. 3: I2S0. 4: I2S1. 5: LCD_CAM. 6: AES. 7: SHA. 8: ADC_DAC. 9: RMT."]
    #[inline(always)]
    pub fn peri_out_sel_ch(&mut self) -> PERI_OUT_SEL_CH_W {
        PERI_OUT_SEL_CH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral selection of Tx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_peri_sel_ch](index.html) module"]
pub struct OUT_PERI_SEL_CH_SPEC;
impl crate::RegisterSpec for OUT_PERI_SEL_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_peri_sel_ch::R](R) reader structure"]
impl crate::Readable for OUT_PERI_SEL_CH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_peri_sel_ch::W](W) writer structure"]
impl crate::Writable for OUT_PERI_SEL_CH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT_PERI_SEL_CH%s to value 0x3f"]
impl crate::Resettable for OUT_PERI_SEL_CH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3f
    }
}
