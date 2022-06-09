#[doc = "Register `CTR` reader"]
pub struct R(crate::R<CTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTR` writer"]
pub struct W(crate::W<CTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTR_SPEC>;
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
impl From<crate::W<CTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDA_FORCE_OUT` reader - reg_sda_force_out"]
pub type SDA_FORCE_OUT_R = crate::BitReader<bool>;
#[doc = "Field `SDA_FORCE_OUT` writer - reg_sda_force_out"]
pub type SDA_FORCE_OUT_W<'a> = crate::BitWriter<'a, u32, CTR_SPEC, bool, 0>;
#[doc = "Field `SCL_FORCE_OUT` reader - reg_scl_force_out"]
pub type SCL_FORCE_OUT_R = crate::BitReader<bool>;
#[doc = "Field `SCL_FORCE_OUT` writer - reg_scl_force_out"]
pub type SCL_FORCE_OUT_W<'a> = crate::BitWriter<'a, u32, CTR_SPEC, bool, 1>;
#[doc = "Field `SAMPLE_SCL_LEVEL` reader - reg_sample_scl_level"]
pub type SAMPLE_SCL_LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `SAMPLE_SCL_LEVEL` writer - reg_sample_scl_level"]
pub type SAMPLE_SCL_LEVEL_W<'a> = crate::BitWriter<'a, u32, CTR_SPEC, bool, 2>;
#[doc = "Field `RX_FULL_ACK_LEVEL` reader - reg_rx_full_ack_level"]
pub type RX_FULL_ACK_LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `RX_FULL_ACK_LEVEL` writer - reg_rx_full_ack_level"]
pub type RX_FULL_ACK_LEVEL_W<'a> = crate::BitWriter<'a, u32, CTR_SPEC, bool, 3>;
#[doc = "Field `MS_MODE` reader - reg_ms_mode"]
pub type MS_MODE_R = crate::BitReader<bool>;
#[doc = "Field `MS_MODE` writer - reg_ms_mode"]
pub type MS_MODE_W<'a> = crate::BitWriter<'a, u32, CTR_SPEC, bool, 4>;
#[doc = "Field `TRANS_START` writer - reg_trans_start"]
pub type TRANS_START_W<'a> = crate::BitWriter<'a, u32, CTR_SPEC, bool, 5>;
#[doc = "Field `TX_LSB_FIRST` reader - reg_tx_lsb_first"]
pub type TX_LSB_FIRST_R = crate::BitReader<bool>;
#[doc = "Field `TX_LSB_FIRST` writer - reg_tx_lsb_first"]
pub type TX_LSB_FIRST_W<'a> = crate::BitWriter<'a, u32, CTR_SPEC, bool, 6>;
#[doc = "Field `RX_LSB_FIRST` reader - reg_rx_lsb_first"]
pub type RX_LSB_FIRST_R = crate::BitReader<bool>;
#[doc = "Field `RX_LSB_FIRST` writer - reg_rx_lsb_first"]
pub type RX_LSB_FIRST_W<'a> = crate::BitWriter<'a, u32, CTR_SPEC, bool, 7>;
#[doc = "Field `CLK_EN` reader - reg_clk_en"]
pub type CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `CLK_EN` writer - reg_clk_en"]
pub type CLK_EN_W<'a> = crate::BitWriter<'a, u32, CTR_SPEC, bool, 8>;
#[doc = "Field `ARBITRATION_EN` reader - reg_arbitration_en"]
pub type ARBITRATION_EN_R = crate::BitReader<bool>;
#[doc = "Field `ARBITRATION_EN` writer - reg_arbitration_en"]
pub type ARBITRATION_EN_W<'a> = crate::BitWriter<'a, u32, CTR_SPEC, bool, 9>;
#[doc = "Field `FSM_RST` writer - reg_fsm_rst"]
pub type FSM_RST_W<'a> = crate::BitWriter<'a, u32, CTR_SPEC, bool, 10>;
#[doc = "Field `CONF_UPGATE` writer - reg_conf_upgate"]
pub type CONF_UPGATE_W<'a> = crate::BitWriter<'a, u32, CTR_SPEC, bool, 11>;
#[doc = "Field `SLV_TX_AUTO_START_EN` reader - reg_slv_tx_auto_start_en"]
pub type SLV_TX_AUTO_START_EN_R = crate::BitReader<bool>;
#[doc = "Field `SLV_TX_AUTO_START_EN` writer - reg_slv_tx_auto_start_en"]
pub type SLV_TX_AUTO_START_EN_W<'a> = crate::BitWriter<'a, u32, CTR_SPEC, bool, 12>;
#[doc = "Field `ADDR_10BIT_RW_CHECK_EN` reader - reg_addr_10bit_rw_check_en"]
pub type ADDR_10BIT_RW_CHECK_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADDR_10BIT_RW_CHECK_EN` writer - reg_addr_10bit_rw_check_en"]
pub type ADDR_10BIT_RW_CHECK_EN_W<'a> = crate::BitWriter<'a, u32, CTR_SPEC, bool, 13>;
#[doc = "Field `ADDR_BROADCASTING_EN` reader - reg_addr_broadcasting_en"]
pub type ADDR_BROADCASTING_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADDR_BROADCASTING_EN` writer - reg_addr_broadcasting_en"]
pub type ADDR_BROADCASTING_EN_W<'a> = crate::BitWriter<'a, u32, CTR_SPEC, bool, 14>;
impl R {
    #[doc = "Bit 0 - reg_sda_force_out"]
    #[inline(always)]
    pub fn sda_force_out(&self) -> SDA_FORCE_OUT_R {
        SDA_FORCE_OUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_scl_force_out"]
    #[inline(always)]
    pub fn scl_force_out(&self) -> SCL_FORCE_OUT_R {
        SCL_FORCE_OUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - reg_sample_scl_level"]
    #[inline(always)]
    pub fn sample_scl_level(&self) -> SAMPLE_SCL_LEVEL_R {
        SAMPLE_SCL_LEVEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reg_rx_full_ack_level"]
    #[inline(always)]
    pub fn rx_full_ack_level(&self) -> RX_FULL_ACK_LEVEL_R {
        RX_FULL_ACK_LEVEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reg_ms_mode"]
    #[inline(always)]
    pub fn ms_mode(&self) -> MS_MODE_R {
        MS_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - reg_tx_lsb_first"]
    #[inline(always)]
    pub fn tx_lsb_first(&self) -> TX_LSB_FIRST_R {
        TX_LSB_FIRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reg_rx_lsb_first"]
    #[inline(always)]
    pub fn rx_lsb_first(&self) -> RX_LSB_FIRST_R {
        RX_LSB_FIRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - reg_clk_en"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - reg_arbitration_en"]
    #[inline(always)]
    pub fn arbitration_en(&self) -> ARBITRATION_EN_R {
        ARBITRATION_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - reg_slv_tx_auto_start_en"]
    #[inline(always)]
    pub fn slv_tx_auto_start_en(&self) -> SLV_TX_AUTO_START_EN_R {
        SLV_TX_AUTO_START_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - reg_addr_10bit_rw_check_en"]
    #[inline(always)]
    pub fn addr_10bit_rw_check_en(&self) -> ADDR_10BIT_RW_CHECK_EN_R {
        ADDR_10BIT_RW_CHECK_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - reg_addr_broadcasting_en"]
    #[inline(always)]
    pub fn addr_broadcasting_en(&self) -> ADDR_BROADCASTING_EN_R {
        ADDR_BROADCASTING_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - reg_sda_force_out"]
    #[inline(always)]
    pub fn sda_force_out(&mut self) -> SDA_FORCE_OUT_W {
        SDA_FORCE_OUT_W::new(self)
    }
    #[doc = "Bit 1 - reg_scl_force_out"]
    #[inline(always)]
    pub fn scl_force_out(&mut self) -> SCL_FORCE_OUT_W {
        SCL_FORCE_OUT_W::new(self)
    }
    #[doc = "Bit 2 - reg_sample_scl_level"]
    #[inline(always)]
    pub fn sample_scl_level(&mut self) -> SAMPLE_SCL_LEVEL_W {
        SAMPLE_SCL_LEVEL_W::new(self)
    }
    #[doc = "Bit 3 - reg_rx_full_ack_level"]
    #[inline(always)]
    pub fn rx_full_ack_level(&mut self) -> RX_FULL_ACK_LEVEL_W {
        RX_FULL_ACK_LEVEL_W::new(self)
    }
    #[doc = "Bit 4 - reg_ms_mode"]
    #[inline(always)]
    pub fn ms_mode(&mut self) -> MS_MODE_W {
        MS_MODE_W::new(self)
    }
    #[doc = "Bit 5 - reg_trans_start"]
    #[inline(always)]
    pub fn trans_start(&mut self) -> TRANS_START_W {
        TRANS_START_W::new(self)
    }
    #[doc = "Bit 6 - reg_tx_lsb_first"]
    #[inline(always)]
    pub fn tx_lsb_first(&mut self) -> TX_LSB_FIRST_W {
        TX_LSB_FIRST_W::new(self)
    }
    #[doc = "Bit 7 - reg_rx_lsb_first"]
    #[inline(always)]
    pub fn rx_lsb_first(&mut self) -> RX_LSB_FIRST_W {
        RX_LSB_FIRST_W::new(self)
    }
    #[doc = "Bit 8 - reg_clk_en"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W::new(self)
    }
    #[doc = "Bit 9 - reg_arbitration_en"]
    #[inline(always)]
    pub fn arbitration_en(&mut self) -> ARBITRATION_EN_W {
        ARBITRATION_EN_W::new(self)
    }
    #[doc = "Bit 10 - reg_fsm_rst"]
    #[inline(always)]
    pub fn fsm_rst(&mut self) -> FSM_RST_W {
        FSM_RST_W::new(self)
    }
    #[doc = "Bit 11 - reg_conf_upgate"]
    #[inline(always)]
    pub fn conf_upgate(&mut self) -> CONF_UPGATE_W {
        CONF_UPGATE_W::new(self)
    }
    #[doc = "Bit 12 - reg_slv_tx_auto_start_en"]
    #[inline(always)]
    pub fn slv_tx_auto_start_en(&mut self) -> SLV_TX_AUTO_START_EN_W {
        SLV_TX_AUTO_START_EN_W::new(self)
    }
    #[doc = "Bit 13 - reg_addr_10bit_rw_check_en"]
    #[inline(always)]
    pub fn addr_10bit_rw_check_en(&mut self) -> ADDR_10BIT_RW_CHECK_EN_W {
        ADDR_10BIT_RW_CHECK_EN_W::new(self)
    }
    #[doc = "Bit 14 - reg_addr_broadcasting_en"]
    #[inline(always)]
    pub fn addr_broadcasting_en(&mut self) -> ADDR_BROADCASTING_EN_W {
        ADDR_BROADCASTING_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C_CTR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctr](index.html) module"]
pub struct CTR_SPEC;
impl crate::RegisterSpec for CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctr::R](R) reader structure"]
impl crate::Readable for CTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctr::W](W) writer structure"]
impl crate::Writable for CTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTR to value 0x020b"]
impl crate::Resettable for CTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x020b
    }
}
