#[doc = "Register `PAD_HOLD` reader"]
pub struct R(crate::R<PAD_HOLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_HOLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_HOLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_HOLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_HOLD` writer"]
pub struct W(crate::W<PAD_HOLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_HOLD_SPEC>;
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
impl From<crate::W<PAD_HOLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_HOLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_PAD0_HOLD` reader - hold rtc pad0"]
pub struct TOUCH_PAD0_HOLD_R(crate::FieldReader<bool, bool>);
impl TOUCH_PAD0_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_PAD0_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD0_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD0_HOLD` writer - hold rtc pad0"]
pub struct TOUCH_PAD0_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD0_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `TOUCH_PAD1_HOLD` reader - hold rtc pad-1"]
pub struct TOUCH_PAD1_HOLD_R(crate::FieldReader<bool, bool>);
impl TOUCH_PAD1_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_PAD1_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD1_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD1_HOLD` writer - hold rtc pad-1"]
pub struct TOUCH_PAD1_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD1_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `TOUCH_PAD2_HOLD` reader - hold rtc pad-2"]
pub struct TOUCH_PAD2_HOLD_R(crate::FieldReader<bool, bool>);
impl TOUCH_PAD2_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_PAD2_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD2_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD2_HOLD` writer - hold rtc pad-2"]
pub struct TOUCH_PAD2_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD2_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `TOUCH_PAD3_HOLD` reader - hold rtc pad-3"]
pub struct TOUCH_PAD3_HOLD_R(crate::FieldReader<bool, bool>);
impl TOUCH_PAD3_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_PAD3_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD3_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD3_HOLD` writer - hold rtc pad-3"]
pub struct TOUCH_PAD3_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD3_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `TOUCH_PAD4_HOLD` reader - hold rtc pad-4"]
pub struct TOUCH_PAD4_HOLD_R(crate::FieldReader<bool, bool>);
impl TOUCH_PAD4_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_PAD4_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD4_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD4_HOLD` writer - hold rtc pad-4"]
pub struct TOUCH_PAD4_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD4_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `TOUCH_PAD5_HOLD` reader - hold rtc pad-5"]
pub struct TOUCH_PAD5_HOLD_R(crate::FieldReader<bool, bool>);
impl TOUCH_PAD5_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_PAD5_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD5_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD5_HOLD` writer - hold rtc pad-5"]
pub struct TOUCH_PAD5_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD5_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `TOUCH_PAD6_HOLD` reader - hold rtc pad-6"]
pub struct TOUCH_PAD6_HOLD_R(crate::FieldReader<bool, bool>);
impl TOUCH_PAD6_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_PAD6_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD6_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD6_HOLD` writer - hold rtc pad-6"]
pub struct TOUCH_PAD6_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD6_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `TOUCH_PAD7_HOLD` reader - hold rtc pad-7"]
pub struct TOUCH_PAD7_HOLD_R(crate::FieldReader<bool, bool>);
impl TOUCH_PAD7_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_PAD7_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD7_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD7_HOLD` writer - hold rtc pad-7"]
pub struct TOUCH_PAD7_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD7_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `TOUCH_PAD8_HOLD` reader - hold rtc pad-8"]
pub struct TOUCH_PAD8_HOLD_R(crate::FieldReader<bool, bool>);
impl TOUCH_PAD8_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_PAD8_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD8_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD8_HOLD` writer - hold rtc pad-8"]
pub struct TOUCH_PAD8_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD8_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `TOUCH_PAD9_HOLD` reader - hold rtc pad-9"]
pub struct TOUCH_PAD9_HOLD_R(crate::FieldReader<bool, bool>);
impl TOUCH_PAD9_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_PAD9_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD9_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD9_HOLD` writer - hold rtc pad-9"]
pub struct TOUCH_PAD9_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD9_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `TOUCH_PAD10_HOLD` reader - hold rtc pad-10"]
pub struct TOUCH_PAD10_HOLD_R(crate::FieldReader<bool, bool>);
impl TOUCH_PAD10_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_PAD10_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD10_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD10_HOLD` writer - hold rtc pad-10"]
pub struct TOUCH_PAD10_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD10_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `TOUCH_PAD11_HOLD` reader - hold rtc pad-11"]
pub struct TOUCH_PAD11_HOLD_R(crate::FieldReader<bool, bool>);
impl TOUCH_PAD11_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_PAD11_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD11_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD11_HOLD` writer - hold rtc pad-11"]
pub struct TOUCH_PAD11_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD11_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `TOUCH_PAD12_HOLD` reader - hold rtc pad-12"]
pub struct TOUCH_PAD12_HOLD_R(crate::FieldReader<bool, bool>);
impl TOUCH_PAD12_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_PAD12_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD12_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD12_HOLD` writer - hold rtc pad-12"]
pub struct TOUCH_PAD12_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD12_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `TOUCH_PAD13_HOLD` reader - hold rtc pad-13"]
pub struct TOUCH_PAD13_HOLD_R(crate::FieldReader<bool, bool>);
impl TOUCH_PAD13_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_PAD13_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD13_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD13_HOLD` writer - hold rtc pad-13"]
pub struct TOUCH_PAD13_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD13_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `TOUCH_PAD14_HOLD` reader - hold rtc pad-14"]
pub struct TOUCH_PAD14_HOLD_R(crate::FieldReader<bool, bool>);
impl TOUCH_PAD14_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_PAD14_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD14_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD14_HOLD` writer - hold rtc pad-14"]
pub struct TOUCH_PAD14_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD14_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `X32P_HOLD` reader - hold rtc pad-15"]
pub struct X32P_HOLD_R(crate::FieldReader<bool, bool>);
impl X32P_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X32P_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32P_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32P_HOLD` writer - hold rtc pad-15"]
pub struct X32P_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> X32P_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `X32N_HOLD` reader - hold rtc pad-16"]
pub struct X32N_HOLD_R(crate::FieldReader<bool, bool>);
impl X32N_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X32N_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32N_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32N_HOLD` writer - hold rtc pad-16"]
pub struct X32N_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> X32N_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `PDAC1_HOLD` reader - hold rtc pad-17"]
pub struct PDAC1_HOLD_R(crate::FieldReader<bool, bool>);
impl PDAC1_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDAC1_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC1_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC1_HOLD` writer - hold rtc pad-17"]
pub struct PDAC1_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC1_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `PDAC2_HOLD` reader - hold rtc pad-18"]
pub struct PDAC2_HOLD_R(crate::FieldReader<bool, bool>);
impl PDAC2_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDAC2_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC2_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC2_HOLD` writer - hold rtc pad-18"]
pub struct PDAC2_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC2_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `RTC_PAD19_HOLD` reader - hold rtc pad-19"]
pub struct RTC_PAD19_HOLD_R(crate::FieldReader<bool, bool>);
impl RTC_PAD19_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_PAD19_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_PAD19_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_PAD19_HOLD` writer - hold rtc pad-19"]
pub struct RTC_PAD19_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_PAD19_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `RTC_PAD20_HOLD` reader - hold rtc pad-20"]
pub struct RTC_PAD20_HOLD_R(crate::FieldReader<bool, bool>);
impl RTC_PAD20_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_PAD20_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_PAD20_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_PAD20_HOLD` writer - hold rtc pad-20"]
pub struct RTC_PAD20_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_PAD20_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `RTC_PAD21_HOLD` reader - hold rtc pad-21"]
pub struct RTC_PAD21_HOLD_R(crate::FieldReader<bool, bool>);
impl RTC_PAD21_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_PAD21_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_PAD21_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_PAD21_HOLD` writer - hold rtc pad-21"]
pub struct RTC_PAD21_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_PAD21_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - hold rtc pad0"]
    #[inline(always)]
    pub fn touch_pad0_hold(&self) -> TOUCH_PAD0_HOLD_R {
        TOUCH_PAD0_HOLD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - hold rtc pad-1"]
    #[inline(always)]
    pub fn touch_pad1_hold(&self) -> TOUCH_PAD1_HOLD_R {
        TOUCH_PAD1_HOLD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - hold rtc pad-2"]
    #[inline(always)]
    pub fn touch_pad2_hold(&self) -> TOUCH_PAD2_HOLD_R {
        TOUCH_PAD2_HOLD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - hold rtc pad-3"]
    #[inline(always)]
    pub fn touch_pad3_hold(&self) -> TOUCH_PAD3_HOLD_R {
        TOUCH_PAD3_HOLD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - hold rtc pad-4"]
    #[inline(always)]
    pub fn touch_pad4_hold(&self) -> TOUCH_PAD4_HOLD_R {
        TOUCH_PAD4_HOLD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - hold rtc pad-5"]
    #[inline(always)]
    pub fn touch_pad5_hold(&self) -> TOUCH_PAD5_HOLD_R {
        TOUCH_PAD5_HOLD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - hold rtc pad-6"]
    #[inline(always)]
    pub fn touch_pad6_hold(&self) -> TOUCH_PAD6_HOLD_R {
        TOUCH_PAD6_HOLD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - hold rtc pad-7"]
    #[inline(always)]
    pub fn touch_pad7_hold(&self) -> TOUCH_PAD7_HOLD_R {
        TOUCH_PAD7_HOLD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - hold rtc pad-8"]
    #[inline(always)]
    pub fn touch_pad8_hold(&self) -> TOUCH_PAD8_HOLD_R {
        TOUCH_PAD8_HOLD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - hold rtc pad-9"]
    #[inline(always)]
    pub fn touch_pad9_hold(&self) -> TOUCH_PAD9_HOLD_R {
        TOUCH_PAD9_HOLD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - hold rtc pad-10"]
    #[inline(always)]
    pub fn touch_pad10_hold(&self) -> TOUCH_PAD10_HOLD_R {
        TOUCH_PAD10_HOLD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - hold rtc pad-11"]
    #[inline(always)]
    pub fn touch_pad11_hold(&self) -> TOUCH_PAD11_HOLD_R {
        TOUCH_PAD11_HOLD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - hold rtc pad-12"]
    #[inline(always)]
    pub fn touch_pad12_hold(&self) -> TOUCH_PAD12_HOLD_R {
        TOUCH_PAD12_HOLD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - hold rtc pad-13"]
    #[inline(always)]
    pub fn touch_pad13_hold(&self) -> TOUCH_PAD13_HOLD_R {
        TOUCH_PAD13_HOLD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - hold rtc pad-14"]
    #[inline(always)]
    pub fn touch_pad14_hold(&self) -> TOUCH_PAD14_HOLD_R {
        TOUCH_PAD14_HOLD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - hold rtc pad-15"]
    #[inline(always)]
    pub fn x32p_hold(&self) -> X32P_HOLD_R {
        X32P_HOLD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - hold rtc pad-16"]
    #[inline(always)]
    pub fn x32n_hold(&self) -> X32N_HOLD_R {
        X32N_HOLD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - hold rtc pad-17"]
    #[inline(always)]
    pub fn pdac1_hold(&self) -> PDAC1_HOLD_R {
        PDAC1_HOLD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - hold rtc pad-18"]
    #[inline(always)]
    pub fn pdac2_hold(&self) -> PDAC2_HOLD_R {
        PDAC2_HOLD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - hold rtc pad-19"]
    #[inline(always)]
    pub fn rtc_pad19_hold(&self) -> RTC_PAD19_HOLD_R {
        RTC_PAD19_HOLD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - hold rtc pad-20"]
    #[inline(always)]
    pub fn rtc_pad20_hold(&self) -> RTC_PAD20_HOLD_R {
        RTC_PAD20_HOLD_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - hold rtc pad-21"]
    #[inline(always)]
    pub fn rtc_pad21_hold(&self) -> RTC_PAD21_HOLD_R {
        RTC_PAD21_HOLD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - hold rtc pad0"]
    #[inline(always)]
    pub fn touch_pad0_hold(&mut self) -> TOUCH_PAD0_HOLD_W {
        TOUCH_PAD0_HOLD_W { w: self }
    }
    #[doc = "Bit 1 - hold rtc pad-1"]
    #[inline(always)]
    pub fn touch_pad1_hold(&mut self) -> TOUCH_PAD1_HOLD_W {
        TOUCH_PAD1_HOLD_W { w: self }
    }
    #[doc = "Bit 2 - hold rtc pad-2"]
    #[inline(always)]
    pub fn touch_pad2_hold(&mut self) -> TOUCH_PAD2_HOLD_W {
        TOUCH_PAD2_HOLD_W { w: self }
    }
    #[doc = "Bit 3 - hold rtc pad-3"]
    #[inline(always)]
    pub fn touch_pad3_hold(&mut self) -> TOUCH_PAD3_HOLD_W {
        TOUCH_PAD3_HOLD_W { w: self }
    }
    #[doc = "Bit 4 - hold rtc pad-4"]
    #[inline(always)]
    pub fn touch_pad4_hold(&mut self) -> TOUCH_PAD4_HOLD_W {
        TOUCH_PAD4_HOLD_W { w: self }
    }
    #[doc = "Bit 5 - hold rtc pad-5"]
    #[inline(always)]
    pub fn touch_pad5_hold(&mut self) -> TOUCH_PAD5_HOLD_W {
        TOUCH_PAD5_HOLD_W { w: self }
    }
    #[doc = "Bit 6 - hold rtc pad-6"]
    #[inline(always)]
    pub fn touch_pad6_hold(&mut self) -> TOUCH_PAD6_HOLD_W {
        TOUCH_PAD6_HOLD_W { w: self }
    }
    #[doc = "Bit 7 - hold rtc pad-7"]
    #[inline(always)]
    pub fn touch_pad7_hold(&mut self) -> TOUCH_PAD7_HOLD_W {
        TOUCH_PAD7_HOLD_W { w: self }
    }
    #[doc = "Bit 8 - hold rtc pad-8"]
    #[inline(always)]
    pub fn touch_pad8_hold(&mut self) -> TOUCH_PAD8_HOLD_W {
        TOUCH_PAD8_HOLD_W { w: self }
    }
    #[doc = "Bit 9 - hold rtc pad-9"]
    #[inline(always)]
    pub fn touch_pad9_hold(&mut self) -> TOUCH_PAD9_HOLD_W {
        TOUCH_PAD9_HOLD_W { w: self }
    }
    #[doc = "Bit 10 - hold rtc pad-10"]
    #[inline(always)]
    pub fn touch_pad10_hold(&mut self) -> TOUCH_PAD10_HOLD_W {
        TOUCH_PAD10_HOLD_W { w: self }
    }
    #[doc = "Bit 11 - hold rtc pad-11"]
    #[inline(always)]
    pub fn touch_pad11_hold(&mut self) -> TOUCH_PAD11_HOLD_W {
        TOUCH_PAD11_HOLD_W { w: self }
    }
    #[doc = "Bit 12 - hold rtc pad-12"]
    #[inline(always)]
    pub fn touch_pad12_hold(&mut self) -> TOUCH_PAD12_HOLD_W {
        TOUCH_PAD12_HOLD_W { w: self }
    }
    #[doc = "Bit 13 - hold rtc pad-13"]
    #[inline(always)]
    pub fn touch_pad13_hold(&mut self) -> TOUCH_PAD13_HOLD_W {
        TOUCH_PAD13_HOLD_W { w: self }
    }
    #[doc = "Bit 14 - hold rtc pad-14"]
    #[inline(always)]
    pub fn touch_pad14_hold(&mut self) -> TOUCH_PAD14_HOLD_W {
        TOUCH_PAD14_HOLD_W { w: self }
    }
    #[doc = "Bit 15 - hold rtc pad-15"]
    #[inline(always)]
    pub fn x32p_hold(&mut self) -> X32P_HOLD_W {
        X32P_HOLD_W { w: self }
    }
    #[doc = "Bit 16 - hold rtc pad-16"]
    #[inline(always)]
    pub fn x32n_hold(&mut self) -> X32N_HOLD_W {
        X32N_HOLD_W { w: self }
    }
    #[doc = "Bit 17 - hold rtc pad-17"]
    #[inline(always)]
    pub fn pdac1_hold(&mut self) -> PDAC1_HOLD_W {
        PDAC1_HOLD_W { w: self }
    }
    #[doc = "Bit 18 - hold rtc pad-18"]
    #[inline(always)]
    pub fn pdac2_hold(&mut self) -> PDAC2_HOLD_W {
        PDAC2_HOLD_W { w: self }
    }
    #[doc = "Bit 19 - hold rtc pad-19"]
    #[inline(always)]
    pub fn rtc_pad19_hold(&mut self) -> RTC_PAD19_HOLD_W {
        RTC_PAD19_HOLD_W { w: self }
    }
    #[doc = "Bit 20 - hold rtc pad-20"]
    #[inline(always)]
    pub fn rtc_pad20_hold(&mut self) -> RTC_PAD20_HOLD_W {
        RTC_PAD20_HOLD_W { w: self }
    }
    #[doc = "Bit 21 - hold rtc pad-21"]
    #[inline(always)]
    pub fn rtc_pad21_hold(&mut self) -> RTC_PAD21_HOLD_W {
        RTC_PAD21_HOLD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc pad hold configure\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_hold]
(index.html) module"]
pub struct PAD_HOLD_SPEC;
impl crate::RegisterSpec for PAD_HOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_hold::R]
(R) reader structure"]
impl crate::Readable for PAD_HOLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_hold::W]
(W) writer structure"]
impl crate::Writable for PAD_HOLD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAD_HOLD to value 0"]
impl crate::Resettable for PAD_HOLD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
