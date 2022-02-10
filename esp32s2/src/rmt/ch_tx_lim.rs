#[doc = "Register `CH%s_TX_LIM` reader"]
pub struct R(crate::R<CH_TX_LIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_TX_LIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_TX_LIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_TX_LIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%s_TX_LIM` writer"]
pub struct W(crate::W<CH_TX_LIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_TX_LIM_SPEC>;
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
impl From<crate::W<CH_TX_LIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_TX_LIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_LIM_CH0` reader - This register is used to configure the maximum entries that CHANNEL%s can send out."]
pub struct TX_LIM_CH0_R(crate::FieldReader<u16, u16>);
impl TX_LIM_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TX_LIM_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_LIM_CH0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_LIM_CH0` writer - This register is used to configure the maximum entries that CHANNEL%s can send out."]
pub struct TX_LIM_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_LIM_CH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
#[doc = "Field `TX_LOOP_NUM_CH0` reader - This register is used to configure the maximum loop count when tx_conti_mode is valid."]
pub struct TX_LOOP_NUM_CH0_R(crate::FieldReader<u16, u16>);
impl TX_LOOP_NUM_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TX_LOOP_NUM_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_LOOP_NUM_CH0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_LOOP_NUM_CH0` writer - This register is used to configure the maximum loop count when tx_conti_mode is valid."]
pub struct TX_LOOP_NUM_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_LOOP_NUM_CH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 9)) | ((value as u32 & 0x03ff) << 9);
        self.w
    }
}
#[doc = "Field `TX_LOOP_CNT_EN_CH0` reader - This register is the enabled bit for loop count."]
pub struct TX_LOOP_CNT_EN_CH0_R(crate::FieldReader<bool, bool>);
impl TX_LOOP_CNT_EN_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_LOOP_CNT_EN_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_LOOP_CNT_EN_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_LOOP_CNT_EN_CH0` writer - This register is the enabled bit for loop count."]
pub struct TX_LOOP_CNT_EN_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_LOOP_CNT_EN_CH0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `LOOP_COUNT_RESET_CH0` writer - This register is used to reset the loop count when tx_conti_mode is valid."]
pub struct LOOP_COUNT_RESET_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOP_COUNT_RESET_CH0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - This register is used to configure the maximum entries that CHANNEL%s can send out."]
    #[inline(always)]
    pub fn tx_lim_ch0(&self) -> TX_LIM_CH0_R {
        TX_LIM_CH0_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:18 - This register is used to configure the maximum loop count when tx_conti_mode is valid."]
    #[inline(always)]
    pub fn tx_loop_num_ch0(&self) -> TX_LOOP_NUM_CH0_R {
        TX_LOOP_NUM_CH0_R::new(((self.bits >> 9) & 0x03ff) as u16)
    }
    #[doc = "Bit 19 - This register is the enabled bit for loop count."]
    #[inline(always)]
    pub fn tx_loop_cnt_en_ch0(&self) -> TX_LOOP_CNT_EN_CH0_R {
        TX_LOOP_CNT_EN_CH0_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - This register is used to configure the maximum entries that CHANNEL%s can send out."]
    #[inline(always)]
    pub fn tx_lim_ch0(&mut self) -> TX_LIM_CH0_W {
        TX_LIM_CH0_W { w: self }
    }
    #[doc = "Bits 9:18 - This register is used to configure the maximum loop count when tx_conti_mode is valid."]
    #[inline(always)]
    pub fn tx_loop_num_ch0(&mut self) -> TX_LOOP_NUM_CH0_W {
        TX_LOOP_NUM_CH0_W { w: self }
    }
    #[doc = "Bit 19 - This register is the enabled bit for loop count."]
    #[inline(always)]
    pub fn tx_loop_cnt_en_ch0(&mut self) -> TX_LOOP_CNT_EN_CH0_W {
        TX_LOOP_CNT_EN_CH0_W { w: self }
    }
    #[doc = "Bit 20 - This register is used to reset the loop count when tx_conti_mode is valid."]
    #[inline(always)]
    pub fn loop_count_reset_ch0(&mut self) -> LOOP_COUNT_RESET_CH0_W {
        LOOP_COUNT_RESET_CH0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel %s Tx event configuration register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_tx_lim]
(index.html) module"]
pub struct CH_TX_LIM_SPEC;
impl crate::RegisterSpec for CH_TX_LIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_tx_lim::R]
(R) reader structure"]
impl crate::Readable for CH_TX_LIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_tx_lim::W]
(W) writer structure"]
impl crate::Writable for CH_TX_LIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH%s_TX_LIM to value 0x80"]
impl crate::Resettable for CH_TX_LIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
