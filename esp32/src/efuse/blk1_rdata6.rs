#[doc = "Register `BLK1_RDATA6` reader"]
pub struct R(crate::R<BLK1_RDATA6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK1_RDATA6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK1_RDATA6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK1_RDATA6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLK1_DOUT6` reader - read for BLOCK1"]
pub struct BLK1_DOUT6_R(crate::FieldReader<u32>);
impl BLK1_DOUT6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BLK1_DOUT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLK1_DOUT6_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - read for BLOCK1"]
    #[inline(always)]
    pub fn blk1_dout6(&self) -> BLK1_DOUT6_R {
        BLK1_DOUT6_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk1_rdata6](index.html) module"]
pub struct BLK1_RDATA6_SPEC;
impl crate::RegisterSpec for BLK1_RDATA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk1_rdata6::R](R) reader structure"]
impl crate::Readable for BLK1_RDATA6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK1_RDATA6 to value 0"]
impl crate::Resettable for BLK1_RDATA6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
