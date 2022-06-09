#[doc = "Register `CPUSDIO_INT1` reader"]
pub struct R(crate::R<CPUSDIO_INT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUSDIO_INT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUSDIO_INT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUSDIO_INT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SDIO1_INT` reader - GPIO32~53 CPU SDIO interrupt status."]
pub type SDIO1_INT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:21 - GPIO32~53 CPU SDIO interrupt status."]
    #[inline(always)]
    pub fn sdio1_int(&self) -> SDIO1_INT_R {
        SDIO1_INT_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
#[doc = "GPIO32 ~ 53 CPU SDIO interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpusdio_int1](index.html) module"]
pub struct CPUSDIO_INT1_SPEC;
impl crate::RegisterSpec for CPUSDIO_INT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpusdio_int1::R](R) reader structure"]
impl crate::Readable for CPUSDIO_INT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CPUSDIO_INT1 to value 0"]
impl crate::Resettable for CPUSDIO_INT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
