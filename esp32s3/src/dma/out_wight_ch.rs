#[doc = "Register `OUT_WIGHT_CH%s` reader"]
pub struct R(crate::R<OUT_WIGHT_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_WIGHT_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_WIGHT_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_WIGHT_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_WIGHT_CH%s` writer"]
pub struct W(crate::W<OUT_WIGHT_CH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_WIGHT_CH_SPEC>;
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
impl From<crate::W<OUT_WIGHT_CH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_WIGHT_CH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_WEIGHT_CH` reader - The weight of Tx channel 0."]
pub struct TX_WEIGHT_CH_R(crate::FieldReader<u8>);
impl TX_WEIGHT_CH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_WEIGHT_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_WEIGHT_CH_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_WEIGHT_CH` writer - The weight of Tx channel 0."]
pub struct TX_WEIGHT_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_WEIGHT_CH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:11 - The weight of Tx channel 0."]
    #[inline(always)]
    pub fn tx_weight_ch(&self) -> TX_WEIGHT_CH_R {
        TX_WEIGHT_CH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - The weight of Tx channel 0."]
    #[inline(always)]
    pub fn tx_weight_ch(&mut self) -> TX_WEIGHT_CH_W {
        TX_WEIGHT_CH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Weight register of Rx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_wight_ch](index.html) module"]
pub struct OUT_WIGHT_CH_SPEC;
impl crate::RegisterSpec for OUT_WIGHT_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_wight_ch::R](R) reader structure"]
impl crate::Readable for OUT_WIGHT_CH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_wight_ch::W](W) writer structure"]
impl crate::Writable for OUT_WIGHT_CH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT_WIGHT_CH%s to value 0x0f00"]
impl crate::Resettable for OUT_WIGHT_CH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f00
    }
}
