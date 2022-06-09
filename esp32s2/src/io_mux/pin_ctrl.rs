#[doc = "Register `PIN_CTRL` reader"]
pub struct R(crate::R<PIN_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIN_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIN_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIN_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIN_CTRL` writer"]
pub struct W(crate::W<PIN_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIN_CTRL_SPEC>;
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
impl From<crate::W<PIN_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIN_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIN_CLK_OUT1` reader - Configure I2S0 clock output. 0: output I2S0 clock to CLK_OUT1. 15: disabled."]
pub type PIN_CLK_OUT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN_CLK_OUT1` writer - Configure I2S0 clock output. 0: output I2S0 clock to CLK_OUT1. 15: disabled."]
pub type PIN_CLK_OUT1_W<'a> = crate::FieldWriter<'a, u32, PIN_CTRL_SPEC, u8, u8, 4, 0>;
#[doc = "Field `PIN_CLK_OUT2` reader - Configure I2S0 clock output. 0: output I2S0 clock to CLK_OUT2. 15: disabled."]
pub type PIN_CLK_OUT2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN_CLK_OUT2` writer - Configure I2S0 clock output. 0: output I2S0 clock to CLK_OUT2. 15: disabled."]
pub type PIN_CLK_OUT2_W<'a> = crate::FieldWriter<'a, u32, PIN_CTRL_SPEC, u8, u8, 4, 4>;
#[doc = "Field `PIN_CLK_OUT3` reader - Configure I2S0 clock output. 0: output I2S0 clock to CLK_OUT3. 15: disabled."]
pub type PIN_CLK_OUT3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN_CLK_OUT3` writer - Configure I2S0 clock output. 0: output I2S0 clock to CLK_OUT3. 15: disabled."]
pub type PIN_CLK_OUT3_W<'a> = crate::FieldWriter<'a, u32, PIN_CTRL_SPEC, u8, u8, 4, 8>;
#[doc = "Field `SWITCH_PRT_NUM` reader - IO pin power switch delay, delay unit is one APB clock."]
pub type SWITCH_PRT_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SWITCH_PRT_NUM` writer - IO pin power switch delay, delay unit is one APB clock."]
pub type SWITCH_PRT_NUM_W<'a> = crate::FieldWriter<'a, u32, PIN_CTRL_SPEC, u8, u8, 3, 12>;
#[doc = "Field `PAD_POWER_CTRL` reader - Select power voltage for GPIO33 ~ GPIO37. 1: select VDD_SPI 1.8 V. 0: select VDD3P3_CPU 3.3 V."]
pub type PAD_POWER_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `PAD_POWER_CTRL` writer - Select power voltage for GPIO33 ~ GPIO37. 1: select VDD_SPI 1.8 V. 0: select VDD3P3_CPU 3.3 V."]
pub type PAD_POWER_CTRL_W<'a> = crate::BitWriter<'a, u32, PIN_CTRL_SPEC, bool, 15>;
impl R {
    #[doc = "Bits 0:3 - Configure I2S0 clock output. 0: output I2S0 clock to CLK_OUT1. 15: disabled."]
    #[inline(always)]
    pub fn pin_clk_out1(&self) -> PIN_CLK_OUT1_R {
        PIN_CLK_OUT1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Configure I2S0 clock output. 0: output I2S0 clock to CLK_OUT2. 15: disabled."]
    #[inline(always)]
    pub fn pin_clk_out2(&self) -> PIN_CLK_OUT2_R {
        PIN_CLK_OUT2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Configure I2S0 clock output. 0: output I2S0 clock to CLK_OUT3. 15: disabled."]
    #[inline(always)]
    pub fn pin_clk_out3(&self) -> PIN_CLK_OUT3_R {
        PIN_CLK_OUT3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - IO pin power switch delay, delay unit is one APB clock."]
    #[inline(always)]
    pub fn switch_prt_num(&self) -> SWITCH_PRT_NUM_R {
        SWITCH_PRT_NUM_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Select power voltage for GPIO33 ~ GPIO37. 1: select VDD_SPI 1.8 V. 0: select VDD3P3_CPU 3.3 V."]
    #[inline(always)]
    pub fn pad_power_ctrl(&self) -> PAD_POWER_CTRL_R {
        PAD_POWER_CTRL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Configure I2S0 clock output. 0: output I2S0 clock to CLK_OUT1. 15: disabled."]
    #[inline(always)]
    pub fn pin_clk_out1(&mut self) -> PIN_CLK_OUT1_W {
        PIN_CLK_OUT1_W::new(self)
    }
    #[doc = "Bits 4:7 - Configure I2S0 clock output. 0: output I2S0 clock to CLK_OUT2. 15: disabled."]
    #[inline(always)]
    pub fn pin_clk_out2(&mut self) -> PIN_CLK_OUT2_W {
        PIN_CLK_OUT2_W::new(self)
    }
    #[doc = "Bits 8:11 - Configure I2S0 clock output. 0: output I2S0 clock to CLK_OUT3. 15: disabled."]
    #[inline(always)]
    pub fn pin_clk_out3(&mut self) -> PIN_CLK_OUT3_W {
        PIN_CLK_OUT3_W::new(self)
    }
    #[doc = "Bits 12:14 - IO pin power switch delay, delay unit is one APB clock."]
    #[inline(always)]
    pub fn switch_prt_num(&mut self) -> SWITCH_PRT_NUM_W {
        SWITCH_PRT_NUM_W::new(self)
    }
    #[doc = "Bit 15 - Select power voltage for GPIO33 ~ GPIO37. 1: select VDD_SPI 1.8 V. 0: select VDD3P3_CPU 3.3 V."]
    #[inline(always)]
    pub fn pad_power_ctrl(&mut self) -> PAD_POWER_CTRL_W {
        PAD_POWER_CTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock output configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin_ctrl](index.html) module"]
pub struct PIN_CTRL_SPEC;
impl crate::RegisterSpec for PIN_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pin_ctrl::R](R) reader structure"]
impl crate::Readable for PIN_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pin_ctrl::W](W) writer structure"]
impl crate::Writable for PIN_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIN_CTRL to value 0x27ff"]
impl crate::Resettable for PIN_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x27ff
    }
}
