#[doc = "Register `DIN_MODE` reader"]
pub struct R(crate::R<DIN_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIN_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIN_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIN_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIN_MODE` writer"]
pub struct W(crate::W<DIN_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIN_MODE_SPEC>;
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
impl From<crate::W<DIN_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIN_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIN0_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub type DIN0_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIN0_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub type DIN0_MODE_W<'a> = crate::FieldWriter<'a, u32, DIN_MODE_SPEC, u8, u8, 3, 0>;
#[doc = "Field `DIN1_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub type DIN1_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIN1_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub type DIN1_MODE_W<'a> = crate::FieldWriter<'a, u32, DIN_MODE_SPEC, u8, u8, 3, 3>;
#[doc = "Field `DIN2_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub type DIN2_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIN2_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub type DIN2_MODE_W<'a> = crate::FieldWriter<'a, u32, DIN_MODE_SPEC, u8, u8, 3, 6>;
#[doc = "Field `DIN3_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub type DIN3_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIN3_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub type DIN3_MODE_W<'a> = crate::FieldWriter<'a, u32, DIN_MODE_SPEC, u8, u8, 3, 9>;
#[doc = "Field `DIN4_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub type DIN4_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIN4_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub type DIN4_MODE_W<'a> = crate::FieldWriter<'a, u32, DIN_MODE_SPEC, u8, u8, 3, 12>;
#[doc = "Field `DIN5_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub type DIN5_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIN5_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub type DIN5_MODE_W<'a> = crate::FieldWriter<'a, u32, DIN_MODE_SPEC, u8, u8, 3, 15>;
#[doc = "Field `DIN6_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub type DIN6_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIN6_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub type DIN6_MODE_W<'a> = crate::FieldWriter<'a, u32, DIN_MODE_SPEC, u8, u8, 3, 18>;
#[doc = "Field `DIN7_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub type DIN7_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIN7_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub type DIN7_MODE_W<'a> = crate::FieldWriter<'a, u32, DIN_MODE_SPEC, u8, u8, 3, 21>;
#[doc = "Field `TIMING_CLK_ENA` reader - 1:enable hclk in spi_timing.v. 0: disable it. Can be configured in CONF state."]
pub type TIMING_CLK_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TIMING_CLK_ENA` writer - 1:enable hclk in spi_timing.v. 0: disable it. Can be configured in CONF state."]
pub type TIMING_CLK_ENA_W<'a> = crate::BitWriter<'a, u32, DIN_MODE_SPEC, bool, 24>;
impl R {
    #[doc = "Bits 0:2 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din0_mode(&self) -> DIN0_MODE_R {
        DIN0_MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din1_mode(&self) -> DIN1_MODE_R {
        DIN1_MODE_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din2_mode(&self) -> DIN2_MODE_R {
        DIN2_MODE_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din3_mode(&self) -> DIN3_MODE_R {
        DIN3_MODE_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din4_mode(&self) -> DIN4_MODE_R {
        DIN4_MODE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din5_mode(&self) -> DIN5_MODE_R {
        DIN5_MODE_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din6_mode(&self) -> DIN6_MODE_R {
        DIN6_MODE_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din7_mode(&self) -> DIN7_MODE_R {
        DIN7_MODE_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - 1:enable hclk in spi_timing.v. 0: disable it. Can be configured in CONF state."]
    #[inline(always)]
    pub fn timing_clk_ena(&self) -> TIMING_CLK_ENA_R {
        TIMING_CLK_ENA_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din0_mode(&mut self) -> DIN0_MODE_W {
        DIN0_MODE_W::new(self)
    }
    #[doc = "Bits 3:5 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din1_mode(&mut self) -> DIN1_MODE_W {
        DIN1_MODE_W::new(self)
    }
    #[doc = "Bits 6:8 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din2_mode(&mut self) -> DIN2_MODE_W {
        DIN2_MODE_W::new(self)
    }
    #[doc = "Bits 9:11 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din3_mode(&mut self) -> DIN3_MODE_W {
        DIN3_MODE_W::new(self)
    }
    #[doc = "Bits 12:14 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din4_mode(&mut self) -> DIN4_MODE_W {
        DIN4_MODE_W::new(self)
    }
    #[doc = "Bits 15:17 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din5_mode(&mut self) -> DIN5_MODE_W {
        DIN5_MODE_W::new(self)
    }
    #[doc = "Bits 18:20 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din6_mode(&mut self) -> DIN6_MODE_W {
        DIN6_MODE_W::new(self)
    }
    #[doc = "Bits 21:23 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din7_mode(&mut self) -> DIN7_MODE_W {
        DIN7_MODE_W::new(self)
    }
    #[doc = "Bit 24 - 1:enable hclk in spi_timing.v. 0: disable it. Can be configured in CONF state."]
    #[inline(always)]
    pub fn timing_clk_ena(&mut self) -> TIMING_CLK_ENA_W {
        TIMING_CLK_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI input delay mode configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [din_mode](index.html) module"]
pub struct DIN_MODE_SPEC;
impl crate::RegisterSpec for DIN_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [din_mode::R](R) reader structure"]
impl crate::Readable for DIN_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [din_mode::W](W) writer structure"]
impl crate::Writable for DIN_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIN_MODE to value 0"]
impl crate::Resettable for DIN_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
