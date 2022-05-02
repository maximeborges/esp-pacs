#[doc = "Register `PCPU_INT1` reader"]
pub struct R(crate::R<PCPU_INT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCPU_INT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCPU_INT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCPU_INT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PROCPU1_INT` reader - GPIO32 ~ 53 PRO_CPU interrupt status. This interrupt status is corresponding to the bit in GPIO_STATUS1_REG when assert (high) enable signal (bit 13 of GPIO_PINn_REG)."]
pub struct PROCPU1_INT_R(crate::FieldReader<u32>);
impl PROCPU1_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PROCPU1_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROCPU1_INT_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:21 - GPIO32 ~ 53 PRO_CPU interrupt status. This interrupt status is corresponding to the bit in GPIO_STATUS1_REG when assert (high) enable signal (bit 13 of GPIO_PINn_REG)."]
    #[inline(always)]
    pub fn procpu1_int(&self) -> PROCPU1_INT_R {
        PROCPU1_INT_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
#[doc = "GPIO32 ~ 53 PRO_CPU interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcpu_int1](index.html) module"]
pub struct PCPU_INT1_SPEC;
impl crate::RegisterSpec for PCPU_INT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcpu_int1::R](R) reader structure"]
impl crate::Readable for PCPU_INT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PCPU_INT1 to value 0"]
impl crate::Resettable for PCPU_INT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
