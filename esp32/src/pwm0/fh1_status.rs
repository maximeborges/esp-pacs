#[doc = "Register `FH1_STATUS` reader"]
pub struct R(crate::R<FH1_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FH1_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FH1_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FH1_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FH1_CBC_ON` reader - "]
pub struct FH1_CBC_ON_R(crate::FieldReader<bool, bool>);
impl FH1_CBC_ON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FH1_CBC_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FH1_CBC_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FH1_OST_ON` reader - "]
pub struct FH1_OST_ON_R(crate::FieldReader<bool, bool>);
impl FH1_OST_ON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FH1_OST_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FH1_OST_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fh1_cbc_on(&self) -> FH1_CBC_ON_R {
        FH1_CBC_ON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fh1_ost_on(&self) -> FH1_OST_ON_R {
        FH1_OST_ON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fh1_status]
(index.html) module"]
pub struct FH1_STATUS_SPEC;
impl crate::RegisterSpec for FH1_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fh1_status::R]
(R) reader structure"]
impl crate::Readable for FH1_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FH1_STATUS to value 0"]
impl crate::Resettable for FH1_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
