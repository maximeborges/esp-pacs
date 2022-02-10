#[doc = "Register `STATE1` reader"]
pub struct R(crate::R<STATE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUTLINK_DSCR_ADDR` reader - This register stores the current transmit descriptor's address."]
pub struct OUTLINK_DSCR_ADDR_R(crate::FieldReader<u32, u32>);
impl OUTLINK_DSCR_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        OUTLINK_DSCR_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTLINK_DSCR_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_DSCR_STATE` reader - Reserved."]
pub struct OUT_DSCR_STATE_R(crate::FieldReader<u8, u8>);
impl OUT_DSCR_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OUT_DSCR_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_DSCR_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_STATE` reader - Reserved."]
pub struct OUT_STATE_R(crate::FieldReader<u8, u8>);
impl OUT_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OUT_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTFIFO_CNT` reader - This register stores the number of data bytes in TX FIFO."]
pub struct OUTFIFO_CNT_R(crate::FieldReader<u8, u8>);
impl OUTFIFO_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OUTFIFO_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTFIFO_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENCODE_STATE` reader - UHCI encoder status."]
pub struct ENCODE_STATE_R(crate::FieldReader<u8, u8>);
impl ENCODE_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ENCODE_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENCODE_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:17 - This register stores the current transmit descriptor's address."]
    #[inline(always)]
    pub fn outlink_dscr_addr(&self) -> OUTLINK_DSCR_ADDR_R {
        OUTLINK_DSCR_ADDR_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 18:19 - Reserved."]
    #[inline(always)]
    pub fn out_dscr_state(&self) -> OUT_DSCR_STATE_R {
        OUT_DSCR_STATE_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:22 - Reserved."]
    #[inline(always)]
    pub fn out_state(&self) -> OUT_STATE_R {
        OUT_STATE_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 23:27 - This register stores the number of data bytes in TX FIFO."]
    #[inline(always)]
    pub fn outfifo_cnt(&self) -> OUTFIFO_CNT_R {
        OUTFIFO_CNT_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:30 - UHCI encoder status."]
    #[inline(always)]
    pub fn encode_state(&self) -> ENCODE_STATE_R {
        ENCODE_STATE_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
#[doc = "UHCI encoder status register\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [state1]
(index.html) module"]
pub struct STATE1_SPEC;
impl crate::RegisterSpec for STATE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [state1::R]
(R) reader structure"]
impl crate::Readable for STATE1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATE1 to value 0"]
impl crate::Resettable for STATE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
