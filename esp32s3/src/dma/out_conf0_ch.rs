#[doc = "Register `OUT_CONF0_CH%s` reader"]
pub struct R(crate::R<OUT_CONF0_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_CONF0_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_CONF0_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_CONF0_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_CONF0_CH%s` writer"]
pub struct W(crate::W<OUT_CONF0_CH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_CONF0_CH_SPEC>;
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
impl From<crate::W<OUT_CONF0_CH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_CONF0_CH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT_RST_CH` reader - This bit is used to reset DMA channel 0 Tx FSM and Tx FIFO pointer."]
pub type OUT_RST_CH_R = crate::BitReader<bool>;
#[doc = "Field `OUT_RST_CH` writer - This bit is used to reset DMA channel 0 Tx FSM and Tx FIFO pointer."]
pub type OUT_RST_CH_W<'a> = crate::BitWriter<'a, u32, OUT_CONF0_CH_SPEC, bool, 0>;
#[doc = "Field `OUT_LOOP_TEST_CH` reader - reserved"]
pub type OUT_LOOP_TEST_CH_R = crate::BitReader<bool>;
#[doc = "Field `OUT_LOOP_TEST_CH` writer - reserved"]
pub type OUT_LOOP_TEST_CH_W<'a> = crate::BitWriter<'a, u32, OUT_CONF0_CH_SPEC, bool, 1>;
#[doc = "Field `OUT_AUTO_WRBACK_CH` reader - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
pub type OUT_AUTO_WRBACK_CH_R = crate::BitReader<bool>;
#[doc = "Field `OUT_AUTO_WRBACK_CH` writer - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
pub type OUT_AUTO_WRBACK_CH_W<'a> = crate::BitWriter<'a, u32, OUT_CONF0_CH_SPEC, bool, 2>;
#[doc = "Field `OUT_EOF_MODE_CH` reader - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 0 is generated when data need to transmit has been popped from FIFO in DMA"]
pub type OUT_EOF_MODE_CH_R = crate::BitReader<bool>;
#[doc = "Field `OUT_EOF_MODE_CH` writer - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 0 is generated when data need to transmit has been popped from FIFO in DMA"]
pub type OUT_EOF_MODE_CH_W<'a> = crate::BitWriter<'a, u32, OUT_CONF0_CH_SPEC, bool, 3>;
#[doc = "Field `OUTDSCR_BURST_EN_CH` reader - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM."]
pub type OUTDSCR_BURST_EN_CH_R = crate::BitReader<bool>;
#[doc = "Field `OUTDSCR_BURST_EN_CH` writer - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM."]
pub type OUTDSCR_BURST_EN_CH_W<'a> = crate::BitWriter<'a, u32, OUT_CONF0_CH_SPEC, bool, 4>;
#[doc = "Field `OUT_DATA_BURST_EN_CH` reader - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 transmitting data when accessing internal SRAM."]
pub type OUT_DATA_BURST_EN_CH_R = crate::BitReader<bool>;
#[doc = "Field `OUT_DATA_BURST_EN_CH` writer - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 transmitting data when accessing internal SRAM."]
pub type OUT_DATA_BURST_EN_CH_W<'a> = crate::BitWriter<'a, u32, OUT_CONF0_CH_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - This bit is used to reset DMA channel 0 Tx FSM and Tx FIFO pointer."]
    #[inline(always)]
    pub fn out_rst_ch(&self) -> OUT_RST_CH_R {
        OUT_RST_CH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn out_loop_test_ch(&self) -> OUT_LOOP_TEST_CH_R {
        OUT_LOOP_TEST_CH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
    #[inline(always)]
    pub fn out_auto_wrback_ch(&self) -> OUT_AUTO_WRBACK_CH_R {
        OUT_AUTO_WRBACK_CH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 0 is generated when data need to transmit has been popped from FIFO in DMA"]
    #[inline(always)]
    pub fn out_eof_mode_ch(&self) -> OUT_EOF_MODE_CH_R {
        OUT_EOF_MODE_CH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    pub fn outdscr_burst_en_ch(&self) -> OUTDSCR_BURST_EN_CH_R {
        OUTDSCR_BURST_EN_CH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 transmitting data when accessing internal SRAM."]
    #[inline(always)]
    pub fn out_data_burst_en_ch(&self) -> OUT_DATA_BURST_EN_CH_R {
        OUT_DATA_BURST_EN_CH_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to reset DMA channel 0 Tx FSM and Tx FIFO pointer."]
    #[inline(always)]
    pub fn out_rst_ch(&mut self) -> OUT_RST_CH_W {
        OUT_RST_CH_W::new(self)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn out_loop_test_ch(&mut self) -> OUT_LOOP_TEST_CH_W {
        OUT_LOOP_TEST_CH_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
    #[inline(always)]
    pub fn out_auto_wrback_ch(&mut self) -> OUT_AUTO_WRBACK_CH_W {
        OUT_AUTO_WRBACK_CH_W::new(self)
    }
    #[doc = "Bit 3 - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 0 is generated when data need to transmit has been popped from FIFO in DMA"]
    #[inline(always)]
    pub fn out_eof_mode_ch(&mut self) -> OUT_EOF_MODE_CH_W {
        OUT_EOF_MODE_CH_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    pub fn outdscr_burst_en_ch(&mut self) -> OUTDSCR_BURST_EN_CH_W {
        OUTDSCR_BURST_EN_CH_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 transmitting data when accessing internal SRAM."]
    #[inline(always)]
    pub fn out_data_burst_en_ch(&mut self) -> OUT_DATA_BURST_EN_CH_W {
        OUT_DATA_BURST_EN_CH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure 0 register of Tx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_conf0_ch](index.html) module"]
pub struct OUT_CONF0_CH_SPEC;
impl crate::RegisterSpec for OUT_CONF0_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_conf0_ch::R](R) reader structure"]
impl crate::Readable for OUT_CONF0_CH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_conf0_ch::W](W) writer structure"]
impl crate::Writable for OUT_CONF0_CH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT_CONF0_CH%s to value 0x08"]
impl crate::Resettable for OUT_CONF0_CH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
