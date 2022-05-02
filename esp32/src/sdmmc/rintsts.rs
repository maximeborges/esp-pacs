#[doc = "Register `RINTSTS` reader"]
pub struct R(crate::R<RINTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RINTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RINTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RINTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RINTSTS` writer"]
pub struct W(crate::W<RINTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RINTSTS_SPEC>;
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
impl From<crate::W<RINTSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RINTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_STATUS_RAW` reader - Setting a bit clears the corresponding interrupt and writing 0 has no effect. Bits are logged regardless of interrupt mask status. Bit 15 (EBE): End-bit error/no CRC error; Bit 14 (ACD): Auto command done; Bit 13 (SBE/BCI): RX Start Bit Error; Bit 12 (HLE): Hardware locked write error; Bit 11 (FRUN): FIFO underrun/overrun error; Bit 10 (HTO): Data starvation by host timeout (HTO); Bit 9 (DTRO): Data read timeout; Bit 8 (RTO): Response timeout; Bit 7 (DCRC): Data CRC error; Bit 6 (RCRC): Response CRC error; Bit 5 (RXDR): Receive FIFO data request; Bit 4 (TXDR): Transmit FIFO data request; Bit 3 (DTO): Data transfer over; Bit 2 (CD): Command done; Bit 1 (RE): Response error; Bit 0 (CD): Card detect."]
pub struct INT_STATUS_RAW_R(crate::FieldReader<u16>);
impl INT_STATUS_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        INT_STATUS_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_STATUS_RAW_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_STATUS_RAW` writer - Setting a bit clears the corresponding interrupt and writing 0 has no effect. Bits are logged regardless of interrupt mask status. Bit 15 (EBE): End-bit error/no CRC error; Bit 14 (ACD): Auto command done; Bit 13 (SBE/BCI): RX Start Bit Error; Bit 12 (HLE): Hardware locked write error; Bit 11 (FRUN): FIFO underrun/overrun error; Bit 10 (HTO): Data starvation by host timeout (HTO); Bit 9 (DTRO): Data read timeout; Bit 8 (RTO): Response timeout; Bit 7 (DCRC): Data CRC error; Bit 6 (RCRC): Response CRC error; Bit 5 (RXDR): Receive FIFO data request; Bit 4 (TXDR): Transmit FIFO data request; Bit 3 (DTO): Data transfer over; Bit 2 (CD): Command done; Bit 1 (RE): Response error; Bit 0 (CD): Card detect."]
pub struct INT_STATUS_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_STATUS_RAW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `SDIO_INTERRUPT_RAW` reader - Interrupt from SDIO card, one bit for each card. Bit\\[17:16\\] correspond to card1 and card0, respectively. Setting a bit clears the corresponding interrupt bit and writing 0 has no effect. 0: No SDIO interrupt from card; 1: SDIO interrupt from card."]
pub struct SDIO_INTERRUPT_RAW_R(crate::FieldReader<u8>);
impl SDIO_INTERRUPT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SDIO_INTERRUPT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_INTERRUPT_RAW_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_INTERRUPT_RAW` writer - Interrupt from SDIO card, one bit for each card. Bit\\[17:16\\] correspond to card1 and card0, respectively. Setting a bit clears the corresponding interrupt bit and writing 0 has no effect. 0: No SDIO interrupt from card; 1: SDIO interrupt from card."]
pub struct SDIO_INTERRUPT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_INTERRUPT_RAW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 16)) | ((value as u32 & 3) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Setting a bit clears the corresponding interrupt and writing 0 has no effect. Bits are logged regardless of interrupt mask status. Bit 15 (EBE): End-bit error/no CRC error; Bit 14 (ACD): Auto command done; Bit 13 (SBE/BCI): RX Start Bit Error; Bit 12 (HLE): Hardware locked write error; Bit 11 (FRUN): FIFO underrun/overrun error; Bit 10 (HTO): Data starvation by host timeout (HTO); Bit 9 (DTRO): Data read timeout; Bit 8 (RTO): Response timeout; Bit 7 (DCRC): Data CRC error; Bit 6 (RCRC): Response CRC error; Bit 5 (RXDR): Receive FIFO data request; Bit 4 (TXDR): Transmit FIFO data request; Bit 3 (DTO): Data transfer over; Bit 2 (CD): Command done; Bit 1 (RE): Response error; Bit 0 (CD): Card detect."]
    #[inline(always)]
    pub fn int_status_raw(&self) -> INT_STATUS_RAW_R {
        INT_STATUS_RAW_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - Interrupt from SDIO card, one bit for each card. Bit\\[17:16\\] correspond to card1 and card0, respectively. Setting a bit clears the corresponding interrupt bit and writing 0 has no effect. 0: No SDIO interrupt from card; 1: SDIO interrupt from card."]
    #[inline(always)]
    pub fn sdio_interrupt_raw(&self) -> SDIO_INTERRUPT_RAW_R {
        SDIO_INTERRUPT_RAW_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Setting a bit clears the corresponding interrupt and writing 0 has no effect. Bits are logged regardless of interrupt mask status. Bit 15 (EBE): End-bit error/no CRC error; Bit 14 (ACD): Auto command done; Bit 13 (SBE/BCI): RX Start Bit Error; Bit 12 (HLE): Hardware locked write error; Bit 11 (FRUN): FIFO underrun/overrun error; Bit 10 (HTO): Data starvation by host timeout (HTO); Bit 9 (DTRO): Data read timeout; Bit 8 (RTO): Response timeout; Bit 7 (DCRC): Data CRC error; Bit 6 (RCRC): Response CRC error; Bit 5 (RXDR): Receive FIFO data request; Bit 4 (TXDR): Transmit FIFO data request; Bit 3 (DTO): Data transfer over; Bit 2 (CD): Command done; Bit 1 (RE): Response error; Bit 0 (CD): Card detect."]
    #[inline(always)]
    pub fn int_status_raw(&mut self) -> INT_STATUS_RAW_W {
        INT_STATUS_RAW_W { w: self }
    }
    #[doc = "Bits 16:17 - Interrupt from SDIO card, one bit for each card. Bit\\[17:16\\] correspond to card1 and card0, respectively. Setting a bit clears the corresponding interrupt bit and writing 0 has no effect. 0: No SDIO interrupt from card; 1: SDIO interrupt from card."]
    #[inline(always)]
    pub fn sdio_interrupt_raw(&mut self) -> SDIO_INTERRUPT_RAW_W {
        SDIO_INTERRUPT_RAW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Raw interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rintsts](index.html) module"]
pub struct RINTSTS_SPEC;
impl crate::RegisterSpec for RINTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rintsts::R](R) reader structure"]
impl crate::Readable for RINTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rintsts::W](W) writer structure"]
impl crate::Writable for RINTSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RINTSTS to value 0"]
impl crate::Resettable for RINTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
