#[doc = "Register `LC_STATE1` reader"]
pub struct R(crate::R<LC_STATE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LC_STATE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LC_STATE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LC_STATE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INLINK_DSCR_ADDR` reader - I2S DMA in descriptor address."]
pub struct INLINK_DSCR_ADDR_R(crate::FieldReader<u32, u32>);
impl INLINK_DSCR_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        INLINK_DSCR_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INLINK_DSCR_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_DSCR_STATE` reader - I2S DMA in descriptor state."]
pub struct IN_DSCR_STATE_R(crate::FieldReader<u8, u8>);
impl IN_DSCR_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IN_DSCR_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_DSCR_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_STATE` reader - I2S DMA in data state."]
pub struct IN_STATE_R(crate::FieldReader<u8, u8>);
impl IN_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IN_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INFIFO_CNT_DEBUG` reader - The remains of I2S DMA infifo data."]
pub struct INFIFO_CNT_DEBUG_R(crate::FieldReader<u8, u8>);
impl INFIFO_CNT_DEBUG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INFIFO_CNT_DEBUG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INFIFO_CNT_DEBUG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_FULL` reader - I2S DMA infifo is full."]
pub struct IN_FULL_R(crate::FieldReader<bool, bool>);
impl IN_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_EMPTY` reader - I2S DMA infifo is empty."]
pub struct IN_EMPTY_R(crate::FieldReader<bool, bool>);
impl IN_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:17 - I2S DMA in descriptor address."]
    #[inline(always)]
    pub fn inlink_dscr_addr(&self) -> INLINK_DSCR_ADDR_R {
        INLINK_DSCR_ADDR_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 18:19 - I2S DMA in descriptor state."]
    #[inline(always)]
    pub fn in_dscr_state(&self) -> IN_DSCR_STATE_R {
        IN_DSCR_STATE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - I2S DMA in data state."]
    #[inline(always)]
    pub fn in_state(&self) -> IN_STATE_R {
        IN_STATE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:29 - The remains of I2S DMA infifo data."]
    #[inline(always)]
    pub fn infifo_cnt_debug(&self) -> INFIFO_CNT_DEBUG_R {
        INFIFO_CNT_DEBUG_R::new(((self.bits >> 23) & 0x7f) as u8)
    }
    #[doc = "Bit 30 - I2S DMA infifo is full."]
    #[inline(always)]
    pub fn in_full(&self) -> IN_FULL_R {
        IN_FULL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - I2S DMA infifo is empty."]
    #[inline(always)]
    pub fn in_empty(&self) -> IN_EMPTY_R {
        IN_EMPTY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "I2S DMA RX status\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lc_state1]
(index.html) module"]
pub struct LC_STATE1_SPEC;
impl crate::RegisterSpec for LC_STATE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lc_state1::R]
(R) reader structure"]
impl crate::Readable for LC_STATE1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LC_STATE1 to value 0"]
impl crate::Resettable for LC_STATE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
