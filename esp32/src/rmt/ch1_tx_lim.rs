#[doc = "Register `CH1_TX_LIM` reader"]
pub struct R(crate::R<CH1_TX_LIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH1_TX_LIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH1_TX_LIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH1_TX_LIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH1_TX_LIM` writer"]
pub struct W(crate::W<CH1_TX_LIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH1_TX_LIM_SPEC>;
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
impl From<crate::W<CH1_TX_LIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH1_TX_LIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_LIM_CH1` reader - When channel1 sends more than reg_rmt_tx_lim_ch1 datas then channel1 produce the relative interrupt."]
pub struct TX_LIM_CH1_R(crate::FieldReader<u16, u16>);
impl TX_LIM_CH1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TX_LIM_CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_LIM_CH1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_LIM_CH1` writer - When channel1 sends more than reg_rmt_tx_lim_ch1 datas then channel1 produce the relative interrupt."]
pub struct TX_LIM_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_LIM_CH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - When channel1 sends more than reg_rmt_tx_lim_ch1 datas then channel1 produce the relative interrupt."]
    #[inline(always)]
    pub fn tx_lim_ch1(&self) -> TX_LIM_CH1_R {
        TX_LIM_CH1_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - When channel1 sends more than reg_rmt_tx_lim_ch1 datas then channel1 produce the relative interrupt."]
    #[inline(always)]
    pub fn tx_lim_ch1(&mut self) -> TX_LIM_CH1_W {
        TX_LIM_CH1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_tx_lim]
(index.html) module"]
pub struct CH1_TX_LIM_SPEC;
impl crate::RegisterSpec for CH1_TX_LIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch1_tx_lim::R]
(R) reader structure"]
impl crate::Readable for CH1_TX_LIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch1_tx_lim::W]
(W) writer structure"]
impl crate::Writable for CH1_TX_LIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH1_TX_LIM to value 0x80"]
impl crate::Resettable for CH1_TX_LIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
