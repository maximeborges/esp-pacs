#[doc = "Register `LC_STATE0` reader"]
pub struct R(crate::R<LC_STATE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LC_STATE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LC_STATE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LC_STATE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUTLINK_DSCR_ADDR` reader - I2S DMA out descriptor address."]
pub struct OUTLINK_DSCR_ADDR_R(crate::FieldReader<u32>);
impl OUTLINK_DSCR_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        OUTLINK_DSCR_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTLINK_DSCR_ADDR_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_DSCR_STATE` reader - I2S DMA out descriptor state."]
pub struct OUT_DSCR_STATE_R(crate::FieldReader<u8>);
impl OUT_DSCR_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OUT_DSCR_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_DSCR_STATE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_STATE` reader - I2S DMA out data state."]
pub struct OUT_STATE_R(crate::FieldReader<u8>);
impl OUT_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OUT_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_STATE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTFIFO_CNT` reader - The remains of I2S DMA outfifo data."]
pub struct OUTFIFO_CNT_R(crate::FieldReader<u8>);
impl OUTFIFO_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OUTFIFO_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTFIFO_CNT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_FULL` reader - I2S DMA outfifo is full."]
pub struct OUT_FULL_R(crate::FieldReader<bool>);
impl OUT_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_FULL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_EMPTY` reader - I2S DMA outfifo is empty."]
pub struct OUT_EMPTY_R(crate::FieldReader<bool>);
impl OUT_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_EMPTY_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:17 - I2S DMA out descriptor address."]
    #[inline(always)]
    pub fn outlink_dscr_addr(&self) -> OUTLINK_DSCR_ADDR_R {
        OUTLINK_DSCR_ADDR_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 18:19 - I2S DMA out descriptor state."]
    #[inline(always)]
    pub fn out_dscr_state(&self) -> OUT_DSCR_STATE_R {
        OUT_DSCR_STATE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - I2S DMA out data state."]
    #[inline(always)]
    pub fn out_state(&self) -> OUT_STATE_R {
        OUT_STATE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:29 - The remains of I2S DMA outfifo data."]
    #[inline(always)]
    pub fn outfifo_cnt(&self) -> OUTFIFO_CNT_R {
        OUTFIFO_CNT_R::new(((self.bits >> 23) & 0x7f) as u8)
    }
    #[doc = "Bit 30 - I2S DMA outfifo is full."]
    #[inline(always)]
    pub fn out_full(&self) -> OUT_FULL_R {
        OUT_FULL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - I2S DMA outfifo is empty."]
    #[inline(always)]
    pub fn out_empty(&self) -> OUT_EMPTY_R {
        OUT_EMPTY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "I2S DMA TX status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lc_state0](index.html) module"]
pub struct LC_STATE0_SPEC;
impl crate::RegisterSpec for LC_STATE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lc_state0::R](R) reader structure"]
impl crate::Readable for LC_STATE0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LC_STATE0 to value 0"]
impl crate::Resettable for LC_STATE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
