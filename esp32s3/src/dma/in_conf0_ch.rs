#[doc = "Register `IN_CONF0_CH%s` reader"]
pub struct R(crate::R<IN_CONF0_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_CONF0_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_CONF0_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_CONF0_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN_CONF0_CH%s` writer"]
pub struct W(crate::W<IN_CONF0_CH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN_CONF0_CH_SPEC>;
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
impl From<crate::W<IN_CONF0_CH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN_CONF0_CH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN_RST_CH` reader - This bit is used to reset DMA channel 0 Rx FSM and Rx FIFO pointer."]
pub type IN_RST_CH_R = crate::BitReader<bool>;
#[doc = "Field `IN_RST_CH` writer - This bit is used to reset DMA channel 0 Rx FSM and Rx FIFO pointer."]
pub type IN_RST_CH_W<'a> = crate::BitWriter<'a, u32, IN_CONF0_CH_SPEC, bool, 0>;
#[doc = "Field `IN_LOOP_TEST_CH` reader - reserved"]
pub type IN_LOOP_TEST_CH_R = crate::BitReader<bool>;
#[doc = "Field `IN_LOOP_TEST_CH` writer - reserved"]
pub type IN_LOOP_TEST_CH_W<'a> = crate::BitWriter<'a, u32, IN_CONF0_CH_SPEC, bool, 1>;
#[doc = "Field `INDSCR_BURST_EN_CH` reader - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM."]
pub type INDSCR_BURST_EN_CH_R = crate::BitReader<bool>;
#[doc = "Field `INDSCR_BURST_EN_CH` writer - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM."]
pub type INDSCR_BURST_EN_CH_W<'a> = crate::BitWriter<'a, u32, IN_CONF0_CH_SPEC, bool, 2>;
#[doc = "Field `IN_DATA_BURST_EN_CH` reader - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 receiving data when accessing internal SRAM."]
pub type IN_DATA_BURST_EN_CH_R = crate::BitReader<bool>;
#[doc = "Field `IN_DATA_BURST_EN_CH` writer - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 receiving data when accessing internal SRAM."]
pub type IN_DATA_BURST_EN_CH_W<'a> = crate::BitWriter<'a, u32, IN_CONF0_CH_SPEC, bool, 3>;
#[doc = "Field `MEM_TRANS_EN_CH` reader - Set this bit 1 to enable automatic transmitting data from memory to memory via DMA."]
pub type MEM_TRANS_EN_CH_R = crate::BitReader<bool>;
#[doc = "Field `MEM_TRANS_EN_CH` writer - Set this bit 1 to enable automatic transmitting data from memory to memory via DMA."]
pub type MEM_TRANS_EN_CH_W<'a> = crate::BitWriter<'a, u32, IN_CONF0_CH_SPEC, bool, 4>;
impl R {
    #[doc = "Bit 0 - This bit is used to reset DMA channel 0 Rx FSM and Rx FIFO pointer."]
    #[inline(always)]
    pub fn in_rst_ch(&self) -> IN_RST_CH_R {
        IN_RST_CH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn in_loop_test_ch(&self) -> IN_LOOP_TEST_CH_R {
        IN_LOOP_TEST_CH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    pub fn indscr_burst_en_ch(&self) -> INDSCR_BURST_EN_CH_R {
        INDSCR_BURST_EN_CH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 receiving data when accessing internal SRAM."]
    #[inline(always)]
    pub fn in_data_burst_en_ch(&self) -> IN_DATA_BURST_EN_CH_R {
        IN_DATA_BURST_EN_CH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit 1 to enable automatic transmitting data from memory to memory via DMA."]
    #[inline(always)]
    pub fn mem_trans_en_ch(&self) -> MEM_TRANS_EN_CH_R {
        MEM_TRANS_EN_CH_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to reset DMA channel 0 Rx FSM and Rx FIFO pointer."]
    #[inline(always)]
    pub fn in_rst_ch(&mut self) -> IN_RST_CH_W {
        IN_RST_CH_W::new(self)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn in_loop_test_ch(&mut self) -> IN_LOOP_TEST_CH_W {
        IN_LOOP_TEST_CH_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    pub fn indscr_burst_en_ch(&mut self) -> INDSCR_BURST_EN_CH_W {
        INDSCR_BURST_EN_CH_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 receiving data when accessing internal SRAM."]
    #[inline(always)]
    pub fn in_data_burst_en_ch(&mut self) -> IN_DATA_BURST_EN_CH_W {
        IN_DATA_BURST_EN_CH_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit 1 to enable automatic transmitting data from memory to memory via DMA."]
    #[inline(always)]
    pub fn mem_trans_en_ch(&mut self) -> MEM_TRANS_EN_CH_W {
        MEM_TRANS_EN_CH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure 0 register of Rx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_conf0_ch](index.html) module"]
pub struct IN_CONF0_CH_SPEC;
impl crate::RegisterSpec for IN_CONF0_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_conf0_ch::R](R) reader structure"]
impl crate::Readable for IN_CONF0_CH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in_conf0_ch::W](W) writer structure"]
impl crate::Writable for IN_CONF0_CH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IN_CONF0_CH%s to value 0"]
impl crate::Resettable for IN_CONF0_CH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
