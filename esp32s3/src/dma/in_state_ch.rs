#[doc = "Register `IN_STATE_CH%s` reader"]
pub struct R(crate::R<IN_STATE_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_STATE_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_STATE_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_STATE_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INLINK_DSCR_ADDR_CH` reader - This register stores the current inlink descriptor's address."]
pub struct INLINK_DSCR_ADDR_CH_R(crate::FieldReader<u32>);
impl INLINK_DSCR_ADDR_CH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        INLINK_DSCR_ADDR_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INLINK_DSCR_ADDR_CH_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_DSCR_STATE_CH` reader - reserved"]
pub struct IN_DSCR_STATE_CH_R(crate::FieldReader<u8>);
impl IN_DSCR_STATE_CH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IN_DSCR_STATE_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_DSCR_STATE_CH_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_STATE_CH` reader - reserved"]
pub struct IN_STATE_CH_R(crate::FieldReader<u8>);
impl IN_STATE_CH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IN_STATE_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_STATE_CH_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:17 - This register stores the current inlink descriptor's address."]
    #[inline(always)]
    pub fn inlink_dscr_addr_ch(&self) -> INLINK_DSCR_ADDR_CH_R {
        INLINK_DSCR_ADDR_CH_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 18:19 - reserved"]
    #[inline(always)]
    pub fn in_dscr_state_ch(&self) -> IN_DSCR_STATE_CH_R {
        IN_DSCR_STATE_CH_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - reserved"]
    #[inline(always)]
    pub fn in_state_ch(&self) -> IN_STATE_CH_R {
        IN_STATE_CH_R::new(((self.bits >> 20) & 7) as u8)
    }
}
#[doc = "Receive status of Rx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_state_ch](index.html) module"]
pub struct IN_STATE_CH_SPEC;
impl crate::RegisterSpec for IN_STATE_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_state_ch::R](R) reader structure"]
impl crate::Readable for IN_STATE_CH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IN_STATE_CH%s to value 0"]
impl crate::Resettable for IN_STATE_CH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
