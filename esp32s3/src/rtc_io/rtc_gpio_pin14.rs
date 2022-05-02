#[doc = "Register `RTC_GPIO_PIN14` reader"]
pub struct R(crate::R<RTC_GPIO_PIN14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_GPIO_PIN14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_GPIO_PIN14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_GPIO_PIN14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_GPIO_PIN14` writer"]
pub struct W(crate::W<RTC_GPIO_PIN14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_GPIO_PIN14_SPEC>;
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
impl From<crate::W<RTC_GPIO_PIN14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_GPIO_PIN14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAD_DRIVER` reader - if set to 0: normal output, if set to 1: open drain"]
pub struct PAD_DRIVER_R(crate::FieldReader<bool>);
impl PAD_DRIVER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAD_DRIVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_DRIVER_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_DRIVER` writer - if set to 0: normal output, if set to 1: open drain"]
pub struct PAD_DRIVER_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_DRIVER_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `INT_TYPE` reader - if set to 0: GPIO interrupt disable, if set to 1: rising edge trigger, if set to 2: falling edge trigger, if set to 3: any edge trigger, if set to 4: low level trigger, if set to 5: high level trigger"]
pub struct INT_TYPE_R(crate::FieldReader<u8>);
impl INT_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INT_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_TYPE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_TYPE` writer - if set to 0: GPIO interrupt disable, if set to 1: rising edge trigger, if set to 2: falling edge trigger, if set to 3: any edge trigger, if set to 4: low level trigger, if set to 5: high level trigger"]
pub struct INT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 7)) | ((value as u32 & 7) << 7);
        self.w
    }
}
#[doc = "Field `WAKEUP_ENABLE` reader - RTC GPIO wakeup enable bit"]
pub struct WAKEUP_ENABLE_R(crate::FieldReader<bool>);
impl WAKEUP_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUP_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUP_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUP_ENABLE` writer - RTC GPIO wakeup enable bit"]
pub struct WAKEUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_ENABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - if set to 0: normal output, if set to 1: open drain"]
    #[inline(always)]
    pub fn pad_driver(&self) -> PAD_DRIVER_R {
        PAD_DRIVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 7:9 - if set to 0: GPIO interrupt disable, if set to 1: rising edge trigger, if set to 2: falling edge trigger, if set to 3: any edge trigger, if set to 4: low level trigger, if set to 5: high level trigger"]
    #[inline(always)]
    pub fn int_type(&self) -> INT_TYPE_R {
        INT_TYPE_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - RTC GPIO wakeup enable bit"]
    #[inline(always)]
    pub fn wakeup_enable(&self) -> WAKEUP_ENABLE_R {
        WAKEUP_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - if set to 0: normal output, if set to 1: open drain"]
    #[inline(always)]
    pub fn pad_driver(&mut self) -> PAD_DRIVER_W {
        PAD_DRIVER_W { w: self }
    }
    #[doc = "Bits 7:9 - if set to 0: GPIO interrupt disable, if set to 1: rising edge trigger, if set to 2: falling edge trigger, if set to 3: any edge trigger, if set to 4: low level trigger, if set to 5: high level trigger"]
    #[inline(always)]
    pub fn int_type(&mut self) -> INT_TYPE_W {
        INT_TYPE_W { w: self }
    }
    #[doc = "Bit 10 - RTC GPIO wakeup enable bit"]
    #[inline(always)]
    pub fn wakeup_enable(&mut self) -> WAKEUP_ENABLE_W {
        WAKEUP_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure RTC GPIO14\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_gpio_pin14](index.html) module"]
pub struct RTC_GPIO_PIN14_SPEC;
impl crate::RegisterSpec for RTC_GPIO_PIN14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_gpio_pin14::R](R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin14::W](W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN14_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_GPIO_PIN14 to value 0"]
impl crate::Resettable for RTC_GPIO_PIN14_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
