#[doc = "Register `OUT_SRAM_SIZE_CH%s` reader"]
pub struct R(crate::R<OUT_SRAM_SIZE_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_SRAM_SIZE_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_SRAM_SIZE_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_SRAM_SIZE_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_SRAM_SIZE_CH%s` writer"]
pub struct W(crate::W<OUT_SRAM_SIZE_CH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_SRAM_SIZE_CH_SPEC>;
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
impl From<crate::W<OUT_SRAM_SIZE_CH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_SRAM_SIZE_CH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT_SIZE_CH` reader - This register is used to configure the size of L2 Tx FIFO for Tx channel 0. 0:16 bytes. 1:24 bytes. 2:32 bytes. 3: 40 bytes. 4: 48 bytes. 5:56 bytes. 6: 64 bytes. 7: 72 bytes. 8: 80 bytes."]
pub type OUT_SIZE_CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUT_SIZE_CH` writer - This register is used to configure the size of L2 Tx FIFO for Tx channel 0. 0:16 bytes. 1:24 bytes. 2:32 bytes. 3: 40 bytes. 4: 48 bytes. 5:56 bytes. 6: 64 bytes. 7: 72 bytes. 8: 80 bytes."]
pub type OUT_SIZE_CH_W<'a> = crate::FieldWriter<'a, u32, OUT_SRAM_SIZE_CH_SPEC, u8, u8, 7, 0>;
impl R {
    #[doc = "Bits 0:6 - This register is used to configure the size of L2 Tx FIFO for Tx channel 0. 0:16 bytes. 1:24 bytes. 2:32 bytes. 3: 40 bytes. 4: 48 bytes. 5:56 bytes. 6: 64 bytes. 7: 72 bytes. 8: 80 bytes."]
    #[inline(always)]
    pub fn out_size_ch(&self) -> OUT_SIZE_CH_R {
        OUT_SIZE_CH_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - This register is used to configure the size of L2 Tx FIFO for Tx channel 0. 0:16 bytes. 1:24 bytes. 2:32 bytes. 3: 40 bytes. 4: 48 bytes. 5:56 bytes. 6: 64 bytes. 7: 72 bytes. 8: 80 bytes."]
    #[inline(always)]
    pub fn out_size_ch(&mut self) -> OUT_SIZE_CH_W {
        OUT_SIZE_CH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit L2 FIFO depth of Tx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_sram_size_ch](index.html) module"]
pub struct OUT_SRAM_SIZE_CH_SPEC;
impl crate::RegisterSpec for OUT_SRAM_SIZE_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_sram_size_ch::R](R) reader structure"]
impl crate::Readable for OUT_SRAM_SIZE_CH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_sram_size_ch::W](W) writer structure"]
impl crate::Writable for OUT_SRAM_SIZE_CH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT_SRAM_SIZE_CH%s to value 0x0e"]
impl crate::Resettable for OUT_SRAM_SIZE_CH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0e
    }
}
