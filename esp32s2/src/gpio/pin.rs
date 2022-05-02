#[doc = "Register `PIN%s` reader"]
pub struct R(crate::R<PIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIN%s` writer"]
pub struct W(crate::W<PIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIN_SPEC>;
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
impl From<crate::W<PIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIN_SYNC2_BYPASS` reader - For the second stage synchronization, GPIO input data can be syn- chronized on either edge of the APB clock. 0: no synchronization; 1: synchronized on falling edge; 2 and 3: synchronized on rising edge."]
pub struct PIN_SYNC2_BYPASS_R(crate::FieldReader<u8>);
impl PIN_SYNC2_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PIN_SYNC2_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_SYNC2_BYPASS_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN_SYNC2_BYPASS` writer - For the second stage synchronization, GPIO input data can be syn- chronized on either edge of the APB clock. 0: no synchronization; 1: synchronized on falling edge; 2 and 3: synchronized on rising edge."]
pub struct PIN_SYNC2_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_SYNC2_BYPASS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `PIN_PAD_DRIVER` reader - Pad driver selection. 0: normal output; 1: open drain output.."]
pub struct PIN_PAD_DRIVER_R(crate::FieldReader<bool>);
impl PIN_PAD_DRIVER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN_PAD_DRIVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_PAD_DRIVER_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN_PAD_DRIVER` writer - Pad driver selection. 0: normal output; 1: open drain output.."]
pub struct PIN_PAD_DRIVER_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_PAD_DRIVER_W<'a> {
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
#[doc = "Field `PIN_SYNC1_BYPASS` reader - For the first stage synchronization, GPIO input data can be synchro- nized on either edge of the APB clock. 0: no synchronization; 1: synchronized on falling edge; 2 and 3: synchronized on rising edge."]
pub struct PIN_SYNC1_BYPASS_R(crate::FieldReader<u8>);
impl PIN_SYNC1_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PIN_SYNC1_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_SYNC1_BYPASS_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN_SYNC1_BYPASS` writer - For the first stage synchronization, GPIO input data can be synchro- nized on either edge of the APB clock. 0: no synchronization; 1: synchronized on falling edge; 2 and 3: synchronized on rising edge."]
pub struct PIN_SYNC1_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_SYNC1_BYPASS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 3)) | ((value as u32 & 3) << 3);
        self.w
    }
}
#[doc = "Field `PIN_INT_TYPE` reader - Interrupt type selection. 0: GPIO interrupt disabled; 1: rising edge trigger; 2: falling edge trigger; 3: any edge trigger; 4: low level trigger; 5: high level trigger. (R/W)"]
pub struct PIN_INT_TYPE_R(crate::FieldReader<u8>);
impl PIN_INT_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PIN_INT_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_INT_TYPE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN_INT_TYPE` writer - Interrupt type selection. 0: GPIO interrupt disabled; 1: rising edge trigger; 2: falling edge trigger; 3: any edge trigger; 4: low level trigger; 5: high level trigger. (R/W)"]
pub struct PIN_INT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_INT_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 7)) | ((value as u32 & 7) << 7);
        self.w
    }
}
#[doc = "Field `PIN_WAKEUP_ENABLE` reader - GPIO wake-up enable bit, only wakes up the CPU from Light-sleep."]
pub struct PIN_WAKEUP_ENABLE_R(crate::FieldReader<bool>);
impl PIN_WAKEUP_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN_WAKEUP_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_WAKEUP_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN_WAKEUP_ENABLE` writer - GPIO wake-up enable bit, only wakes up the CPU from Light-sleep."]
pub struct PIN_WAKEUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_WAKEUP_ENABLE_W<'a> {
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
#[doc = "Field `PIN_CONFIG` reader - Reserved"]
pub struct PIN_CONFIG_R(crate::FieldReader<u8>);
impl PIN_CONFIG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PIN_CONFIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_CONFIG_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN_CONFIG` writer - Reserved"]
pub struct PIN_CONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_CONFIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 11)) | ((value as u32 & 3) << 11);
        self.w
    }
}
#[doc = "Field `PIN_INT_ENA` reader - Interrupt enable bits. bit13: CPU interrupt enabled; bit14: CPU non-maskable interrupt enabled."]
pub struct PIN_INT_ENA_R(crate::FieldReader<u8>);
impl PIN_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PIN_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_INT_ENA_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN_INT_ENA` writer - Interrupt enable bits. bit13: CPU interrupt enabled; bit14: CPU non-maskable interrupt enabled."]
pub struct PIN_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_INT_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 13)) | ((value as u32 & 0x1f) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - For the second stage synchronization, GPIO input data can be syn- chronized on either edge of the APB clock. 0: no synchronization; 1: synchronized on falling edge; 2 and 3: synchronized on rising edge."]
    #[inline(always)]
    pub fn pin_sync2_bypass(&self) -> PIN_SYNC2_BYPASS_R {
        PIN_SYNC2_BYPASS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Pad driver selection. 0: normal output; 1: open drain output.."]
    #[inline(always)]
    pub fn pin_pad_driver(&self) -> PIN_PAD_DRIVER_R {
        PIN_PAD_DRIVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - For the first stage synchronization, GPIO input data can be synchro- nized on either edge of the APB clock. 0: no synchronization; 1: synchronized on falling edge; 2 and 3: synchronized on rising edge."]
    #[inline(always)]
    pub fn pin_sync1_bypass(&self) -> PIN_SYNC1_BYPASS_R {
        PIN_SYNC1_BYPASS_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 7:9 - Interrupt type selection. 0: GPIO interrupt disabled; 1: rising edge trigger; 2: falling edge trigger; 3: any edge trigger; 4: low level trigger; 5: high level trigger. (R/W)"]
    #[inline(always)]
    pub fn pin_int_type(&self) -> PIN_INT_TYPE_R {
        PIN_INT_TYPE_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - GPIO wake-up enable bit, only wakes up the CPU from Light-sleep."]
    #[inline(always)]
    pub fn pin_wakeup_enable(&self) -> PIN_WAKEUP_ENABLE_R {
        PIN_WAKEUP_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Reserved"]
    #[inline(always)]
    pub fn pin_config(&self) -> PIN_CONFIG_R {
        PIN_CONFIG_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:17 - Interrupt enable bits. bit13: CPU interrupt enabled; bit14: CPU non-maskable interrupt enabled."]
    #[inline(always)]
    pub fn pin_int_ena(&self) -> PIN_INT_ENA_R {
        PIN_INT_ENA_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - For the second stage synchronization, GPIO input data can be syn- chronized on either edge of the APB clock. 0: no synchronization; 1: synchronized on falling edge; 2 and 3: synchronized on rising edge."]
    #[inline(always)]
    pub fn pin_sync2_bypass(&mut self) -> PIN_SYNC2_BYPASS_W {
        PIN_SYNC2_BYPASS_W { w: self }
    }
    #[doc = "Bit 2 - Pad driver selection. 0: normal output; 1: open drain output.."]
    #[inline(always)]
    pub fn pin_pad_driver(&mut self) -> PIN_PAD_DRIVER_W {
        PIN_PAD_DRIVER_W { w: self }
    }
    #[doc = "Bits 3:4 - For the first stage synchronization, GPIO input data can be synchro- nized on either edge of the APB clock. 0: no synchronization; 1: synchronized on falling edge; 2 and 3: synchronized on rising edge."]
    #[inline(always)]
    pub fn pin_sync1_bypass(&mut self) -> PIN_SYNC1_BYPASS_W {
        PIN_SYNC1_BYPASS_W { w: self }
    }
    #[doc = "Bits 7:9 - Interrupt type selection. 0: GPIO interrupt disabled; 1: rising edge trigger; 2: falling edge trigger; 3: any edge trigger; 4: low level trigger; 5: high level trigger. (R/W)"]
    #[inline(always)]
    pub fn pin_int_type(&mut self) -> PIN_INT_TYPE_W {
        PIN_INT_TYPE_W { w: self }
    }
    #[doc = "Bit 10 - GPIO wake-up enable bit, only wakes up the CPU from Light-sleep."]
    #[inline(always)]
    pub fn pin_wakeup_enable(&mut self) -> PIN_WAKEUP_ENABLE_W {
        PIN_WAKEUP_ENABLE_W { w: self }
    }
    #[doc = "Bits 11:12 - Reserved"]
    #[inline(always)]
    pub fn pin_config(&mut self) -> PIN_CONFIG_W {
        PIN_CONFIG_W { w: self }
    }
    #[doc = "Bits 13:17 - Interrupt enable bits. bit13: CPU interrupt enabled; bit14: CPU non-maskable interrupt enabled."]
    #[inline(always)]
    pub fn pin_int_ena(&mut self) -> PIN_INT_ENA_W {
        PIN_INT_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration for GPIO pin %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin](index.html) module"]
pub struct PIN_SPEC;
impl crate::RegisterSpec for PIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pin::R](R) reader structure"]
impl crate::Readable for PIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pin::W](W) writer structure"]
impl crate::Writable for PIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIN%s to value 0"]
impl crate::Resettable for PIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
