#[doc = "Register `HOST_SLCHOST_GPIO_IN0` reader"]
pub struct R(crate::R<HOST_SLCHOST_GPIO_IN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLCHOST_GPIO_IN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLCHOST_GPIO_IN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLCHOST_GPIO_IN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HOST_GPIO_SDIO_IN0` reader - "]
pub type HOST_GPIO_SDIO_IN0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn host_gpio_sdio_in0(&self) -> HOST_GPIO_SDIO_IN0_R {
        HOST_GPIO_SDIO_IN0_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slchost_gpio_in0](index.html) module"]
pub struct HOST_SLCHOST_GPIO_IN0_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_GPIO_IN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slchost_gpio_in0::R](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_GPIO_IN0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HOST_SLCHOST_GPIO_IN0 to value 0"]
impl crate::Resettable for HOST_SLCHOST_GPIO_IN0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
