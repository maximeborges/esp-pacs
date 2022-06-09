#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDA_FORCE_OUT` reader - SDA output mode. 0: open drain. 1: push pull."]
pub type SDA_FORCE_OUT_R = crate::BitReader<bool>;
#[doc = "Field `SDA_FORCE_OUT` writer - SDA output mode. 0: open drain. 1: push pull."]
pub type SDA_FORCE_OUT_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 0>;
#[doc = "Field `SCL_FORCE_OUT` reader - SCL output mode. 0: open drain. 1: push pull."]
pub type SCL_FORCE_OUT_R = crate::BitReader<bool>;
#[doc = "Field `SCL_FORCE_OUT` writer - SCL output mode. 0: open drain. 1: push pull."]
pub type SCL_FORCE_OUT_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 1>;
#[doc = "Field `MS_MODE` reader - Set this bit to configure RTC I²C as a master."]
pub type MS_MODE_R = crate::BitReader<bool>;
#[doc = "Field `MS_MODE` writer - Set this bit to configure RTC I²C as a master."]
pub type MS_MODE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 2>;
#[doc = "Field `TRANS_START` reader - Set this bit to 1, RTC I2C starts sending data."]
pub type TRANS_START_R = crate::BitReader<bool>;
#[doc = "Field `TRANS_START` writer - Set this bit to 1, RTC I2C starts sending data."]
pub type TRANS_START_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 3>;
#[doc = "Field `TX_LSB_FIRST` reader - This bit is used to control the sending mode. 0: send data from the most significant bit. 1: send data from the least significant bit."]
pub type TX_LSB_FIRST_R = crate::BitReader<bool>;
#[doc = "Field `TX_LSB_FIRST` writer - This bit is used to control the sending mode. 0: send data from the most significant bit. 1: send data from the least significant bit."]
pub type TX_LSB_FIRST_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 4>;
#[doc = "Field `RX_LSB_FIRST` reader - This bit is used to control the storage mode for received data. 0: receive data from the most significant bit. 1: receive data from the least significant bit."]
pub type RX_LSB_FIRST_R = crate::BitReader<bool>;
#[doc = "Field `RX_LSB_FIRST` writer - This bit is used to control the storage mode for received data. 0: receive data from the most significant bit. 1: receive data from the least significant bit."]
pub type RX_LSB_FIRST_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 5>;
#[doc = "Field `CLK_GATE_EN` reader - RTC I²C controller clock gate."]
pub type CLK_GATE_EN_R = crate::BitReader<bool>;
#[doc = "Field `CLK_GATE_EN` writer - RTC I²C controller clock gate."]
pub type CLK_GATE_EN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 29>;
#[doc = "Field `RESET` reader - RTC I²C software reset."]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET` writer - RTC I²C software reset."]
pub type RESET_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 30>;
#[doc = "Field `CLK_EN` reader - rtc i2c reg clk gating"]
pub type CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `CLK_EN` writer - rtc i2c reg clk gating"]
pub type CLK_EN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - SDA output mode. 0: open drain. 1: push pull."]
    #[inline(always)]
    pub fn sda_force_out(&self) -> SDA_FORCE_OUT_R {
        SDA_FORCE_OUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCL output mode. 0: open drain. 1: push pull."]
    #[inline(always)]
    pub fn scl_force_out(&self) -> SCL_FORCE_OUT_R {
        SCL_FORCE_OUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to configure RTC I²C as a master."]
    #[inline(always)]
    pub fn ms_mode(&self) -> MS_MODE_R {
        MS_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to 1, RTC I2C starts sending data."]
    #[inline(always)]
    pub fn trans_start(&self) -> TRANS_START_R {
        TRANS_START_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit is used to control the sending mode. 0: send data from the most significant bit. 1: send data from the least significant bit."]
    #[inline(always)]
    pub fn tx_lsb_first(&self) -> TX_LSB_FIRST_R {
        TX_LSB_FIRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit is used to control the storage mode for received data. 0: receive data from the most significant bit. 1: receive data from the least significant bit."]
    #[inline(always)]
    pub fn rx_lsb_first(&self) -> RX_LSB_FIRST_R {
        RX_LSB_FIRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 29 - RTC I²C controller clock gate."]
    #[inline(always)]
    pub fn clk_gate_en(&self) -> CLK_GATE_EN_R {
        CLK_GATE_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - RTC I²C software reset."]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - rtc i2c reg clk gating"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDA output mode. 0: open drain. 1: push pull."]
    #[inline(always)]
    pub fn sda_force_out(&mut self) -> SDA_FORCE_OUT_W {
        SDA_FORCE_OUT_W::new(self)
    }
    #[doc = "Bit 1 - SCL output mode. 0: open drain. 1: push pull."]
    #[inline(always)]
    pub fn scl_force_out(&mut self) -> SCL_FORCE_OUT_W {
        SCL_FORCE_OUT_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to configure RTC I²C as a master."]
    #[inline(always)]
    pub fn ms_mode(&mut self) -> MS_MODE_W {
        MS_MODE_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to 1, RTC I2C starts sending data."]
    #[inline(always)]
    pub fn trans_start(&mut self) -> TRANS_START_W {
        TRANS_START_W::new(self)
    }
    #[doc = "Bit 4 - This bit is used to control the sending mode. 0: send data from the most significant bit. 1: send data from the least significant bit."]
    #[inline(always)]
    pub fn tx_lsb_first(&mut self) -> TX_LSB_FIRST_W {
        TX_LSB_FIRST_W::new(self)
    }
    #[doc = "Bit 5 - This bit is used to control the storage mode for received data. 0: receive data from the most significant bit. 1: receive data from the least significant bit."]
    #[inline(always)]
    pub fn rx_lsb_first(&mut self) -> RX_LSB_FIRST_W {
        RX_LSB_FIRST_W::new(self)
    }
    #[doc = "Bit 29 - RTC I²C controller clock gate."]
    #[inline(always)]
    pub fn clk_gate_en(&mut self) -> CLK_GATE_EN_W {
        CLK_GATE_EN_W::new(self)
    }
    #[doc = "Bit 30 - RTC I²C software reset."]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W::new(self)
    }
    #[doc = "Bit 31 - rtc i2c reg clk gating"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmission setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
