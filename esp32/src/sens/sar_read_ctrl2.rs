#[doc = "Register `SAR_READ_CTRL2` reader"]
pub struct R(crate::R<SAR_READ_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_READ_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_READ_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_READ_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_READ_CTRL2` writer"]
pub struct W(crate::W<SAR_READ_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_READ_CTRL2_SPEC>;
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
impl From<crate::W<SAR_READ_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_READ_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR2_CLK_DIV` reader - clock divider"]
pub type SAR2_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAR2_CLK_DIV` writer - clock divider"]
pub type SAR2_CLK_DIV_W<'a> = crate::FieldWriter<'a, u32, SAR_READ_CTRL2_SPEC, u8, u8, 8, 0>;
#[doc = "Field `SAR2_SAMPLE_CYCLE` reader - sample cycles for SAR ADC2"]
pub type SAR2_SAMPLE_CYCLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAR2_SAMPLE_CYCLE` writer - sample cycles for SAR ADC2"]
pub type SAR2_SAMPLE_CYCLE_W<'a> = crate::FieldWriter<'a, u32, SAR_READ_CTRL2_SPEC, u8, u8, 8, 8>;
#[doc = "Field `SAR2_SAMPLE_BIT` reader - 00: for 9-bit width 01: for 10-bit width 10: for 11-bit width 11: for 12-bit width"]
pub type SAR2_SAMPLE_BIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAR2_SAMPLE_BIT` writer - 00: for 9-bit width 01: for 10-bit width 10: for 11-bit width 11: for 12-bit width"]
pub type SAR2_SAMPLE_BIT_W<'a> = crate::FieldWriter<'a, u32, SAR_READ_CTRL2_SPEC, u8, u8, 2, 16>;
#[doc = "Field `SAR2_CLK_GATED` reader - "]
pub type SAR2_CLK_GATED_R = crate::BitReader<bool>;
#[doc = "Field `SAR2_CLK_GATED` writer - "]
pub type SAR2_CLK_GATED_W<'a> = crate::BitWriter<'a, u32, SAR_READ_CTRL2_SPEC, bool, 18>;
#[doc = "Field `SAR2_SAMPLE_NUM` reader - "]
pub type SAR2_SAMPLE_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAR2_SAMPLE_NUM` writer - "]
pub type SAR2_SAMPLE_NUM_W<'a> = crate::FieldWriter<'a, u32, SAR_READ_CTRL2_SPEC, u8, u8, 8, 19>;
#[doc = "Field `SAR2_PWDET_FORCE` reader - "]
pub type SAR2_PWDET_FORCE_R = crate::BitReader<bool>;
#[doc = "Field `SAR2_PWDET_FORCE` writer - "]
pub type SAR2_PWDET_FORCE_W<'a> = crate::BitWriter<'a, u32, SAR_READ_CTRL2_SPEC, bool, 27>;
#[doc = "Field `SAR2_DIG_FORCE` reader - 1: SAR ADC2 controlled by DIG ADC2 CTRL or PWDET CTRL 0: SAR ADC2 controlled by RTC ADC2 CTRL"]
pub type SAR2_DIG_FORCE_R = crate::BitReader<bool>;
#[doc = "Field `SAR2_DIG_FORCE` writer - 1: SAR ADC2 controlled by DIG ADC2 CTRL or PWDET CTRL 0: SAR ADC2 controlled by RTC ADC2 CTRL"]
pub type SAR2_DIG_FORCE_W<'a> = crate::BitWriter<'a, u32, SAR_READ_CTRL2_SPEC, bool, 28>;
#[doc = "Field `SAR2_DATA_INV` reader - Invert SAR ADC2 data"]
pub type SAR2_DATA_INV_R = crate::BitReader<bool>;
#[doc = "Field `SAR2_DATA_INV` writer - Invert SAR ADC2 data"]
pub type SAR2_DATA_INV_W<'a> = crate::BitWriter<'a, u32, SAR_READ_CTRL2_SPEC, bool, 29>;
impl R {
    #[doc = "Bits 0:7 - clock divider"]
    #[inline(always)]
    pub fn sar2_clk_div(&self) -> SAR2_CLK_DIV_R {
        SAR2_CLK_DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - sample cycles for SAR ADC2"]
    #[inline(always)]
    pub fn sar2_sample_cycle(&self) -> SAR2_SAMPLE_CYCLE_R {
        SAR2_SAMPLE_CYCLE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - 00: for 9-bit width 01: for 10-bit width 10: for 11-bit width 11: for 12-bit width"]
    #[inline(always)]
    pub fn sar2_sample_bit(&self) -> SAR2_SAMPLE_BIT_R {
        SAR2_SAMPLE_BIT_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sar2_clk_gated(&self) -> SAR2_CLK_GATED_R {
        SAR2_CLK_GATED_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:26"]
    #[inline(always)]
    pub fn sar2_sample_num(&self) -> SAR2_SAMPLE_NUM_R {
        SAR2_SAMPLE_NUM_R::new(((self.bits >> 19) & 0xff) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sar2_pwdet_force(&self) -> SAR2_PWDET_FORCE_R {
        SAR2_PWDET_FORCE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: SAR ADC2 controlled by DIG ADC2 CTRL or PWDET CTRL 0: SAR ADC2 controlled by RTC ADC2 CTRL"]
    #[inline(always)]
    pub fn sar2_dig_force(&self) -> SAR2_DIG_FORCE_R {
        SAR2_DIG_FORCE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Invert SAR ADC2 data"]
    #[inline(always)]
    pub fn sar2_data_inv(&self) -> SAR2_DATA_INV_R {
        SAR2_DATA_INV_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - clock divider"]
    #[inline(always)]
    pub fn sar2_clk_div(&mut self) -> SAR2_CLK_DIV_W {
        SAR2_CLK_DIV_W::new(self)
    }
    #[doc = "Bits 8:15 - sample cycles for SAR ADC2"]
    #[inline(always)]
    pub fn sar2_sample_cycle(&mut self) -> SAR2_SAMPLE_CYCLE_W {
        SAR2_SAMPLE_CYCLE_W::new(self)
    }
    #[doc = "Bits 16:17 - 00: for 9-bit width 01: for 10-bit width 10: for 11-bit width 11: for 12-bit width"]
    #[inline(always)]
    pub fn sar2_sample_bit(&mut self) -> SAR2_SAMPLE_BIT_W {
        SAR2_SAMPLE_BIT_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sar2_clk_gated(&mut self) -> SAR2_CLK_GATED_W {
        SAR2_CLK_GATED_W::new(self)
    }
    #[doc = "Bits 19:26"]
    #[inline(always)]
    pub fn sar2_sample_num(&mut self) -> SAR2_SAMPLE_NUM_W {
        SAR2_SAMPLE_NUM_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sar2_pwdet_force(&mut self) -> SAR2_PWDET_FORCE_W {
        SAR2_PWDET_FORCE_W::new(self)
    }
    #[doc = "Bit 28 - 1: SAR ADC2 controlled by DIG ADC2 CTRL or PWDET CTRL 0: SAR ADC2 controlled by RTC ADC2 CTRL"]
    #[inline(always)]
    pub fn sar2_dig_force(&mut self) -> SAR2_DIG_FORCE_W {
        SAR2_DIG_FORCE_W::new(self)
    }
    #[doc = "Bit 29 - Invert SAR ADC2 data"]
    #[inline(always)]
    pub fn sar2_data_inv(&mut self) -> SAR2_DATA_INV_W {
        SAR2_DATA_INV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_read_ctrl2](index.html) module"]
pub struct SAR_READ_CTRL2_SPEC;
impl crate::RegisterSpec for SAR_READ_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_read_ctrl2::R](R) reader structure"]
impl crate::Readable for SAR_READ_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_read_ctrl2::W](W) writer structure"]
impl crate::Writable for SAR_READ_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_READ_CTRL2 to value 0x0007_0902"]
impl crate::Resettable for SAR_READ_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0007_0902
    }
}
