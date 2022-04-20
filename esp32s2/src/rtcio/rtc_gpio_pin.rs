#[doc = "Register `RTC_GPIO_PIN%s` reader"]
pub struct R(crate::R<RTC_GPIO_PIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_GPIO_PIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_GPIO_PIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_GPIO_PIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_GPIO_PIN%s` writer"]
pub struct W(crate::W<RTC_GPIO_PIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_GPIO_PIN_SPEC>;
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
impl From<crate::W<RTC_GPIO_PIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_GPIO_PIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_PIN0_PAD_DRIVER` reader - Pad driver selection. 0: normal output. 1: open drain."]
pub struct GPIO_PIN0_PAD_DRIVER_R(crate::FieldReader<bool, bool>);
impl GPIO_PIN0_PAD_DRIVER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_PIN0_PAD_DRIVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_PIN0_PAD_DRIVER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_PIN0_PAD_DRIVER` writer - Pad driver selection. 0: normal output. 1: open drain."]
pub struct GPIO_PIN0_PAD_DRIVER_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN0_PAD_DRIVER_W<'a> {
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
#[doc = "Field `GPIO_PIN0_INT_TYPE` reader - GPIO interrupt type selection. 0: GPIO interrupt disabled. 1: rising edge trigger. 2: falling edge trigger. 3: any edge trigger. 4: low level trigger. 5: high level trigger."]
pub struct GPIO_PIN0_INT_TYPE_R(crate::FieldReader<u8, u8>);
impl GPIO_PIN0_INT_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GPIO_PIN0_INT_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_PIN0_INT_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_PIN0_INT_TYPE` writer - GPIO interrupt type selection. 0: GPIO interrupt disabled. 1: rising edge trigger. 2: falling edge trigger. 3: any edge trigger. 4: low level trigger. 5: high level trigger."]
pub struct GPIO_PIN0_INT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN0_INT_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 7)) | ((value as u32 & 7) << 7);
        self.w
    }
}
#[doc = "Field `GPIO_PIN0_WAKEUP_ENABLE` reader - GPIO wake-up enable. This will only wake up ESP32-S2 from Light-sleep."]
pub struct GPIO_PIN0_WAKEUP_ENABLE_R(crate::FieldReader<bool, bool>);
impl GPIO_PIN0_WAKEUP_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_PIN0_WAKEUP_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_PIN0_WAKEUP_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_PIN0_WAKEUP_ENABLE` writer - GPIO wake-up enable. This will only wake up ESP32-S2 from Light-sleep."]
pub struct GPIO_PIN0_WAKEUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN0_WAKEUP_ENABLE_W<'a> {
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
    #[doc = "Bit 2 - Pad driver selection. 0: normal output. 1: open drain."]
    #[inline(always)]
    pub fn gpio_pin0_pad_driver(&self) -> GPIO_PIN0_PAD_DRIVER_R {
        GPIO_PIN0_PAD_DRIVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 7:9 - GPIO interrupt type selection. 0: GPIO interrupt disabled. 1: rising edge trigger. 2: falling edge trigger. 3: any edge trigger. 4: low level trigger. 5: high level trigger."]
    #[inline(always)]
    pub fn gpio_pin0_int_type(&self) -> GPIO_PIN0_INT_TYPE_R {
        GPIO_PIN0_INT_TYPE_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - GPIO wake-up enable. This will only wake up ESP32-S2 from Light-sleep."]
    #[inline(always)]
    pub fn gpio_pin0_wakeup_enable(&self) -> GPIO_PIN0_WAKEUP_ENABLE_R {
        GPIO_PIN0_WAKEUP_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Pad driver selection. 0: normal output. 1: open drain."]
    #[inline(always)]
    pub fn gpio_pin0_pad_driver(&mut self) -> GPIO_PIN0_PAD_DRIVER_W {
        GPIO_PIN0_PAD_DRIVER_W { w: self }
    }
    #[doc = "Bits 7:9 - GPIO interrupt type selection. 0: GPIO interrupt disabled. 1: rising edge trigger. 2: falling edge trigger. 3: any edge trigger. 4: low level trigger. 5: high level trigger."]
    #[inline(always)]
    pub fn gpio_pin0_int_type(&mut self) -> GPIO_PIN0_INT_TYPE_W {
        GPIO_PIN0_INT_TYPE_W { w: self }
    }
    #[doc = "Bit 10 - GPIO wake-up enable. This will only wake up ESP32-S2 from Light-sleep."]
    #[inline(always)]
    pub fn gpio_pin0_wakeup_enable(&mut self) -> GPIO_PIN0_WAKEUP_ENABLE_W {
        GPIO_PIN0_WAKEUP_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC configuration for pin %s\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_gpio_pin]
(index.html) module"]
pub struct RTC_GPIO_PIN_SPEC;
impl crate::RegisterSpec for RTC_GPIO_PIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_gpio_pin::R]
(R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin::W]
(W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_GPIO_PIN%s to value 0"]
impl crate::Resettable for RTC_GPIO_PIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
