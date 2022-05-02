#[doc = "Register `RXEOF_NUM` reader"]
pub struct R(crate::R<RXEOF_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXEOF_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXEOF_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXEOF_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXEOF_NUM` writer"]
pub struct W(crate::W<RXEOF_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXEOF_NUM_SPEC>;
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
impl From<crate::W<RXEOF_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXEOF_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_EOF_NUM` reader - The receive data bit length is (I2S_RX_BITS_MOD\\[4:0\\] + 1) * (REG_RX_EOF_NUM\\[11:0\\] + 1) . It will trigger in_suc_eof interrupt in the configured DMA RX channel."]
pub struct RX_EOF_NUM_R(crate::FieldReader<u16>);
impl RX_EOF_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RX_EOF_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_EOF_NUM_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_EOF_NUM` writer - The receive data bit length is (I2S_RX_BITS_MOD\\[4:0\\] + 1) * (REG_RX_EOF_NUM\\[11:0\\] + 1) . It will trigger in_suc_eof interrupt in the configured DMA RX channel."]
pub struct RX_EOF_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_EOF_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - The receive data bit length is (I2S_RX_BITS_MOD\\[4:0\\] + 1) * (REG_RX_EOF_NUM\\[11:0\\] + 1) . It will trigger in_suc_eof interrupt in the configured DMA RX channel."]
    #[inline(always)]
    pub fn rx_eof_num(&self) -> RX_EOF_NUM_R {
        RX_EOF_NUM_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - The receive data bit length is (I2S_RX_BITS_MOD\\[4:0\\] + 1) * (REG_RX_EOF_NUM\\[11:0\\] + 1) . It will trigger in_suc_eof interrupt in the configured DMA RX channel."]
    #[inline(always)]
    pub fn rx_eof_num(&mut self) -> RX_EOF_NUM_W {
        RX_EOF_NUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S RX data number control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxeof_num](index.html) module"]
pub struct RXEOF_NUM_SPEC;
impl crate::RegisterSpec for RXEOF_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxeof_num::R](R) reader structure"]
impl crate::Readable for RXEOF_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxeof_num::W](W) writer structure"]
impl crate::Writable for RXEOF_NUM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXEOF_NUM to value 0x40"]
impl crate::Resettable for RXEOF_NUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}
