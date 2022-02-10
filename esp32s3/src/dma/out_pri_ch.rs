#[doc = "Register `OUT_PRI_CH%s` reader"]
pub struct R(crate::R<OUT_PRI_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_PRI_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_PRI_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_PRI_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_PRI_CH%s` writer"]
pub struct W(crate::W<OUT_PRI_CH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_PRI_CH_SPEC>;
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
impl From<crate::W<OUT_PRI_CH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_PRI_CH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_PRI_CH` reader - The priority of Tx channel 0. The larger of the value, the higher of the priority."]
pub struct TX_PRI_CH_R(crate::FieldReader<u8, u8>);
impl TX_PRI_CH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_PRI_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_PRI_CH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_PRI_CH` writer - The priority of Tx channel 0. The larger of the value, the higher of the priority."]
pub struct TX_PRI_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PRI_CH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - The priority of Tx channel 0. The larger of the value, the higher of the priority."]
    #[inline(always)]
    pub fn tx_pri_ch(&self) -> TX_PRI_CH_R {
        TX_PRI_CH_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - The priority of Tx channel 0. The larger of the value, the higher of the priority."]
    #[inline(always)]
    pub fn tx_pri_ch(&mut self) -> TX_PRI_CH_W {
        TX_PRI_CH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Priority register of Tx channel 0.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_pri_ch]
(index.html) module"]
pub struct OUT_PRI_CH_SPEC;
impl crate::RegisterSpec for OUT_PRI_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_pri_ch::R]
(R) reader structure"]
impl crate::Readable for OUT_PRI_CH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_pri_ch::W]
(W) writer structure"]
impl crate::Writable for OUT_PRI_CH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT_PRI_CH%s to value 0"]
impl crate::Resettable for OUT_PRI_CH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
