#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH0_TX_END_INT_RAW` reader - The interrupt raw bit for CHANNEL0. Triggered when transmission done."]
pub struct CH0_TX_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH0_TX_END_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0_TX_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_TX_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_RX_END_INT_RAW` reader - The interrupt raw bit for CHANNEL0. Triggered when reception done."]
pub struct CH0_RX_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH0_RX_END_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0_RX_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_RX_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_ERR_INT_RAW` reader - The interrupt raw bit for CHANNEL0. Triggered when error occurs."]
pub struct CH0_ERR_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH0_ERR_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_TX_END_INT_RAW` reader - The interrupt raw bit for CHANNEL1. Triggered when transmission done."]
pub struct CH1_TX_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH1_TX_END_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1_TX_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_TX_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_RX_END_INT_RAW` reader - The interrupt raw bit for CHANNEL1. Triggered when reception done."]
pub struct CH1_RX_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH1_RX_END_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1_RX_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_RX_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_ERR_INT_RAW` reader - The interrupt raw bit for CHANNEL1. Triggered when error occurs."]
pub struct CH1_ERR_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH1_ERR_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2_TX_END_INT_RAW` reader - The interrupt raw bit for CHANNEL2. Triggered when transmission done."]
pub struct CH2_TX_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH2_TX_END_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2_TX_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_TX_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2_RX_END_INT_RAW` reader - The interrupt raw bit for CHANNEL2. Triggered when reception done."]
pub struct CH2_RX_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH2_RX_END_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2_RX_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_RX_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2_ERR_INT_RAW` reader - The interrupt raw bit for CHANNEL2. Triggered when error occurs."]
pub struct CH2_ERR_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH2_ERR_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3_TX_END_INT_RAW` reader - The interrupt raw bit for CHANNEL3. Triggered when transmission done."]
pub struct CH3_TX_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH3_TX_END_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3_TX_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_TX_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3_RX_END_INT_RAW` reader - The interrupt raw bit for CHANNEL3. Triggered when reception done."]
pub struct CH3_RX_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH3_RX_END_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3_RX_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_RX_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3_ERR_INT_RAW` reader - The interrupt raw bit for CHANNEL3. Triggered when error occurs."]
pub struct CH3_ERR_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH3_ERR_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_TX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for CHANNEL0. Triggered when transmitter sent more data than configured value."]
pub struct CH0_TX_THR_EVENT_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH0_TX_THR_EVENT_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0_TX_THR_EVENT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_TX_THR_EVENT_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_TX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for CHANNEL1. Triggered when transmitter sent more data than configured value."]
pub struct CH1_TX_THR_EVENT_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH1_TX_THR_EVENT_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1_TX_THR_EVENT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_TX_THR_EVENT_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2_TX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for CHANNEL2. Triggered when transmitter sent more data than configured value."]
pub struct CH2_TX_THR_EVENT_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH2_TX_THR_EVENT_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2_TX_THR_EVENT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_TX_THR_EVENT_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3_TX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for CHANNEL3. Triggered when transmitter sent more data than configured value."]
pub struct CH3_TX_THR_EVENT_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH3_TX_THR_EVENT_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3_TX_THR_EVENT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_TX_THR_EVENT_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_TX_LOOP_INT_RAW` reader - The interrupt raw bit for CHANNEL0. Triggered when the loop count reaches the configured threshold value."]
pub struct CH0_TX_LOOP_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH0_TX_LOOP_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0_TX_LOOP_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_TX_LOOP_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_TX_LOOP_INT_RAW` reader - The interrupt raw bit for CHANNEL1. Triggered when the loop count reaches the configured threshold value."]
pub struct CH1_TX_LOOP_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH1_TX_LOOP_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1_TX_LOOP_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_TX_LOOP_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2_TX_LOOP_INT_RAW` reader - The interrupt raw bit for CHANNEL2. Triggered when the loop count reaches the configured threshold value."]
pub struct CH2_TX_LOOP_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH2_TX_LOOP_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2_TX_LOOP_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_TX_LOOP_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3_TX_LOOP_INT_RAW` reader - The interrupt raw bit for CHANNEL3. Triggered when the loop count reaches the configured threshold value."]
pub struct CH3_TX_LOOP_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH3_TX_LOOP_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3_TX_LOOP_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_TX_LOOP_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - The interrupt raw bit for CHANNEL0. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch0_tx_end_int_raw(&self) -> CH0_TX_END_INT_RAW_R {
        CH0_TX_END_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt raw bit for CHANNEL0. Triggered when reception done."]
    #[inline(always)]
    pub fn ch0_rx_end_int_raw(&self) -> CH0_RX_END_INT_RAW_R {
        CH0_RX_END_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt raw bit for CHANNEL0. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch0_err_int_raw(&self) -> CH0_ERR_INT_RAW_R {
        CH0_ERR_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt raw bit for CHANNEL1. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch1_tx_end_int_raw(&self) -> CH1_TX_END_INT_RAW_R {
        CH1_TX_END_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt raw bit for CHANNEL1. Triggered when reception done."]
    #[inline(always)]
    pub fn ch1_rx_end_int_raw(&self) -> CH1_RX_END_INT_RAW_R {
        CH1_RX_END_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt raw bit for CHANNEL1. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch1_err_int_raw(&self) -> CH1_ERR_INT_RAW_R {
        CH1_ERR_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt raw bit for CHANNEL2. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch2_tx_end_int_raw(&self) -> CH2_TX_END_INT_RAW_R {
        CH2_TX_END_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt raw bit for CHANNEL2. Triggered when reception done."]
    #[inline(always)]
    pub fn ch2_rx_end_int_raw(&self) -> CH2_RX_END_INT_RAW_R {
        CH2_RX_END_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt raw bit for CHANNEL2. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch2_err_int_raw(&self) -> CH2_ERR_INT_RAW_R {
        CH2_ERR_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt raw bit for CHANNEL3. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch3_tx_end_int_raw(&self) -> CH3_TX_END_INT_RAW_R {
        CH3_TX_END_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt raw bit for CHANNEL3. Triggered when reception done."]
    #[inline(always)]
    pub fn ch3_rx_end_int_raw(&self) -> CH3_RX_END_INT_RAW_R {
        CH3_RX_END_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt raw bit for CHANNEL3. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch3_err_int_raw(&self) -> CH3_ERR_INT_RAW_R {
        CH3_ERR_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt raw bit for CHANNEL0. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch0_tx_thr_event_int_raw(&self) -> CH0_TX_THR_EVENT_INT_RAW_R {
        CH0_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt raw bit for CHANNEL1. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch1_tx_thr_event_int_raw(&self) -> CH1_TX_THR_EVENT_INT_RAW_R {
        CH1_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt raw bit for CHANNEL2. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch2_tx_thr_event_int_raw(&self) -> CH2_TX_THR_EVENT_INT_RAW_R {
        CH2_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt raw bit for CHANNEL3. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch3_tx_thr_event_int_raw(&self) -> CH3_TX_THR_EVENT_INT_RAW_R {
        CH3_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The interrupt raw bit for CHANNEL0. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch0_tx_loop_int_raw(&self) -> CH0_TX_LOOP_INT_RAW_R {
        CH0_TX_LOOP_INT_RAW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The interrupt raw bit for CHANNEL1. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch1_tx_loop_int_raw(&self) -> CH1_TX_LOOP_INT_RAW_R {
        CH1_TX_LOOP_INT_RAW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The interrupt raw bit for CHANNEL2. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch2_tx_loop_int_raw(&self) -> CH2_TX_LOOP_INT_RAW_R {
        CH2_TX_LOOP_INT_RAW_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The interrupt raw bit for CHANNEL3. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch3_tx_loop_int_raw(&self) -> CH3_TX_LOOP_INT_RAW_R {
        CH3_TX_LOOP_INT_RAW_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw]
(index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R]
(R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
