#[doc = "Register `CLK_CONF` reader"]
pub struct R(crate::R<CLK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CONF` writer"]
pub struct W(crate::W<CLK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CONF_SPEC>;
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
impl From<crate::W<CLK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCLK_DIV_B` reader - The denominator of the frequency divider factor."]
pub struct SCLK_DIV_B_R(crate::FieldReader<u8, u8>);
impl SCLK_DIV_B_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCLK_DIV_B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLK_DIV_B_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLK_DIV_B` writer - The denominator of the frequency divider factor."]
pub struct SCLK_DIV_B_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_DIV_B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `SCLK_DIV_A` reader - The numerator of the frequency divider factor."]
pub struct SCLK_DIV_A_R(crate::FieldReader<u8, u8>);
impl SCLK_DIV_A_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCLK_DIV_A_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLK_DIV_A_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLK_DIV_A` writer - The numerator of the frequency divider factor."]
pub struct SCLK_DIV_A_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_DIV_A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | ((value as u32 & 0x3f) << 6);
        self.w
    }
}
#[doc = "Field `SCLK_DIV_NUM` reader - The integral part of the frequency divider factor."]
pub struct SCLK_DIV_NUM_R(crate::FieldReader<u8, u8>);
impl SCLK_DIV_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCLK_DIV_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLK_DIV_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLK_DIV_NUM` writer - The integral part of the frequency divider factor."]
pub struct SCLK_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_DIV_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 12)) | ((value as u32 & 0xff) << 12);
        self.w
    }
}
#[doc = "Field `SCLK_SEL` reader - UART clock source select. 1: 80Mhz, 2: 8Mhz, 3: XTAL."]
pub struct SCLK_SEL_R(crate::FieldReader<u8, u8>);
impl SCLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLK_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLK_SEL` writer - UART clock source select. 1: 80Mhz, 2: 8Mhz, 3: XTAL."]
pub struct SCLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 20)) | ((value as u32 & 3) << 20);
        self.w
    }
}
#[doc = "Field `SCLK_EN` reader - Set this bit to enable UART Tx/Rx clock."]
pub struct SCLK_EN_R(crate::FieldReader<bool, bool>);
impl SCLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLK_EN` writer - Set this bit to enable UART Tx/Rx clock."]
pub struct SCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 22)) | ((value as u32 & 1) << 22);
        self.w
    }
}
#[doc = "Field `RST_CORE` reader - Write 1 then write 0 to this bit, reset UART Tx/Rx."]
pub struct RST_CORE_R(crate::FieldReader<bool, bool>);
impl RST_CORE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RST_CORE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RST_CORE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RST_CORE` writer - Write 1 then write 0 to this bit, reset UART Tx/Rx."]
pub struct RST_CORE_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_CORE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 23)) | ((value as u32 & 1) << 23);
        self.w
    }
}
#[doc = "Field `TX_SCLK_EN` reader - Set this bit to enable UART Tx clock."]
pub struct TX_SCLK_EN_R(crate::FieldReader<bool, bool>);
impl TX_SCLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_SCLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_SCLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_SCLK_EN` writer - Set this bit to enable UART Tx clock."]
pub struct TX_SCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SCLK_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 24)) | ((value as u32 & 1) << 24);
        self.w
    }
}
#[doc = "Field `RX_SCLK_EN` reader - Set this bit to enable UART Rx clock."]
pub struct RX_SCLK_EN_R(crate::FieldReader<bool, bool>);
impl RX_SCLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_SCLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_SCLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_SCLK_EN` writer - Set this bit to enable UART Rx clock."]
pub struct RX_SCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_SCLK_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 25)) | ((value as u32 & 1) << 25);
        self.w
    }
}
#[doc = "Field `TX_RST_CORE` reader - Write 1 then write 0 to this bit, reset UART Tx."]
pub struct TX_RST_CORE_R(crate::FieldReader<bool, bool>);
impl TX_RST_CORE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_RST_CORE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_RST_CORE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_RST_CORE` writer - Write 1 then write 0 to this bit, reset UART Tx."]
pub struct TX_RST_CORE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_RST_CORE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 26)) | ((value as u32 & 1) << 26);
        self.w
    }
}
#[doc = "Field `RX_RST_CORE` reader - Write 1 then write 0 to this bit, reset UART Rx."]
pub struct RX_RST_CORE_R(crate::FieldReader<bool, bool>);
impl RX_RST_CORE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_RST_CORE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_RST_CORE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_RST_CORE` writer - Write 1 then write 0 to this bit, reset UART Rx."]
pub struct RX_RST_CORE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_RST_CORE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 27)) | ((value as u32 & 1) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - The denominator of the frequency divider factor."]
    #[inline(always)]
    pub fn sclk_div_b(&self) -> SCLK_DIV_B_R {
        SCLK_DIV_B_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - The numerator of the frequency divider factor."]
    #[inline(always)]
    pub fn sclk_div_a(&self) -> SCLK_DIV_A_R {
        SCLK_DIV_A_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:19 - The integral part of the frequency divider factor."]
    #[inline(always)]
    pub fn sclk_div_num(&self) -> SCLK_DIV_NUM_R {
        SCLK_DIV_NUM_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:21 - UART clock source select. 1: 80Mhz, 2: 8Mhz, 3: XTAL."]
    #[inline(always)]
    pub fn sclk_sel(&self) -> SCLK_SEL_R {
        SCLK_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Set this bit to enable UART Tx/Rx clock."]
    #[inline(always)]
    pub fn sclk_en(&self) -> SCLK_EN_R {
        SCLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Write 1 then write 0 to this bit, reset UART Tx/Rx."]
    #[inline(always)]
    pub fn rst_core(&self) -> RST_CORE_R {
        RST_CORE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Set this bit to enable UART Tx clock."]
    #[inline(always)]
    pub fn tx_sclk_en(&self) -> TX_SCLK_EN_R {
        TX_SCLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Set this bit to enable UART Rx clock."]
    #[inline(always)]
    pub fn rx_sclk_en(&self) -> RX_SCLK_EN_R {
        RX_SCLK_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Write 1 then write 0 to this bit, reset UART Tx."]
    #[inline(always)]
    pub fn tx_rst_core(&self) -> TX_RST_CORE_R {
        TX_RST_CORE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Write 1 then write 0 to this bit, reset UART Rx."]
    #[inline(always)]
    pub fn rx_rst_core(&self) -> RX_RST_CORE_R {
        RX_RST_CORE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - The denominator of the frequency divider factor."]
    #[inline(always)]
    pub fn sclk_div_b(&mut self) -> SCLK_DIV_B_W {
        SCLK_DIV_B_W { w: self }
    }
    #[doc = "Bits 6:11 - The numerator of the frequency divider factor."]
    #[inline(always)]
    pub fn sclk_div_a(&mut self) -> SCLK_DIV_A_W {
        SCLK_DIV_A_W { w: self }
    }
    #[doc = "Bits 12:19 - The integral part of the frequency divider factor."]
    #[inline(always)]
    pub fn sclk_div_num(&mut self) -> SCLK_DIV_NUM_W {
        SCLK_DIV_NUM_W { w: self }
    }
    #[doc = "Bits 20:21 - UART clock source select. 1: 80Mhz, 2: 8Mhz, 3: XTAL."]
    #[inline(always)]
    pub fn sclk_sel(&mut self) -> SCLK_SEL_W {
        SCLK_SEL_W { w: self }
    }
    #[doc = "Bit 22 - Set this bit to enable UART Tx/Rx clock."]
    #[inline(always)]
    pub fn sclk_en(&mut self) -> SCLK_EN_W {
        SCLK_EN_W { w: self }
    }
    #[doc = "Bit 23 - Write 1 then write 0 to this bit, reset UART Tx/Rx."]
    #[inline(always)]
    pub fn rst_core(&mut self) -> RST_CORE_W {
        RST_CORE_W { w: self }
    }
    #[doc = "Bit 24 - Set this bit to enable UART Tx clock."]
    #[inline(always)]
    pub fn tx_sclk_en(&mut self) -> TX_SCLK_EN_W {
        TX_SCLK_EN_W { w: self }
    }
    #[doc = "Bit 25 - Set this bit to enable UART Rx clock."]
    #[inline(always)]
    pub fn rx_sclk_en(&mut self) -> RX_SCLK_EN_W {
        RX_SCLK_EN_W { w: self }
    }
    #[doc = "Bit 26 - Write 1 then write 0 to this bit, reset UART Tx."]
    #[inline(always)]
    pub fn tx_rst_core(&mut self) -> TX_RST_CORE_W {
        TX_RST_CORE_W { w: self }
    }
    #[doc = "Bit 27 - Write 1 then write 0 to this bit, reset UART Rx."]
    #[inline(always)]
    pub fn rx_rst_core(&mut self) -> RX_RST_CORE_W {
        RX_RST_CORE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART core clock configuration\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_conf]
(index.html) module"]
pub struct CLK_CONF_SPEC;
impl crate::RegisterSpec for CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_conf::R]
(R) reader structure"]
impl crate::Readable for CLK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_conf::W]
(W) writer structure"]
impl crate::Writable for CLK_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CONF to value 0x0370_1000"]
impl crate::Resettable for CLK_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0370_1000
    }
}
