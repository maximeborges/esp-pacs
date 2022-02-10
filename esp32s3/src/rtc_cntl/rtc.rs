#[doc = "Register `RTC` reader"]
pub struct R(crate::R<RTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC` writer"]
pub struct W(crate::W<RTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_SPEC>;
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
impl From<crate::W<RTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIG_REG_CAL_EN` reader - enable dig regulator cali"]
pub struct DIG_REG_CAL_EN_R(crate::FieldReader<bool, bool>);
impl DIG_REG_CAL_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIG_REG_CAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIG_REG_CAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIG_REG_CAL_EN` writer - enable dig regulator cali"]
pub struct DIG_REG_CAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_REG_CAL_EN_W<'a> {
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
#[doc = "Field `SCK_DCAP` reader - SCK_DCAP"]
pub struct SCK_DCAP_R(crate::FieldReader<u8, u8>);
impl SCK_DCAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCK_DCAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCK_DCAP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCK_DCAP` writer - SCK_DCAP"]
pub struct SCK_DCAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SCK_DCAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 14)) | ((value as u32 & 0xff) << 14);
        self.w
    }
}
#[doc = "Field `DBOOST_FORCE_PD` reader - RTC_DBOOST force power down"]
pub struct DBOOST_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl DBOOST_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBOOST_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBOOST_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBOOST_FORCE_PD` writer - RTC_DBOOST force power down"]
pub struct DBOOST_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> DBOOST_FORCE_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `DBOOST_FORCE_PU` reader - RTC_DBOOST force power up"]
pub struct DBOOST_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl DBOOST_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBOOST_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBOOST_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBOOST_FORCE_PU` writer - RTC_DBOOST force power up"]
pub struct DBOOST_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> DBOOST_FORCE_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `REGULATOR_FORCE_PD` reader - RTC_REG force power down (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
pub struct REGULATOR_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl REGULATOR_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGULATOR_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGULATOR_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGULATOR_FORCE_PD` writer - RTC_REG force power down (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
pub struct REGULATOR_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> REGULATOR_FORCE_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `REGULATOR_FORCE_PU` reader - RTC_REG force power on (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
pub struct REGULATOR_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl REGULATOR_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGULATOR_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGULATOR_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGULATOR_FORCE_PU` writer - RTC_REG force power on (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
pub struct REGULATOR_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> REGULATOR_FORCE_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - enable dig regulator cali"]
    #[inline(always)]
    pub fn dig_reg_cal_en(&self) -> DIG_REG_CAL_EN_R {
        DIG_REG_CAL_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 14:21 - SCK_DCAP"]
    #[inline(always)]
    pub fn sck_dcap(&self) -> SCK_DCAP_R {
        SCK_DCAP_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bit 28 - RTC_DBOOST force power down"]
    #[inline(always)]
    pub fn dboost_force_pd(&self) -> DBOOST_FORCE_PD_R {
        DBOOST_FORCE_PD_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - RTC_DBOOST force power up"]
    #[inline(always)]
    pub fn dboost_force_pu(&self) -> DBOOST_FORCE_PU_R {
        DBOOST_FORCE_PU_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - RTC_REG force power down (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
    #[inline(always)]
    pub fn regulator_force_pd(&self) -> REGULATOR_FORCE_PD_R {
        REGULATOR_FORCE_PD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - RTC_REG force power on (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
    #[inline(always)]
    pub fn regulator_force_pu(&self) -> REGULATOR_FORCE_PU_R {
        REGULATOR_FORCE_PU_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - enable dig regulator cali"]
    #[inline(always)]
    pub fn dig_reg_cal_en(&mut self) -> DIG_REG_CAL_EN_W {
        DIG_REG_CAL_EN_W { w: self }
    }
    #[doc = "Bits 14:21 - SCK_DCAP"]
    #[inline(always)]
    pub fn sck_dcap(&mut self) -> SCK_DCAP_W {
        SCK_DCAP_W { w: self }
    }
    #[doc = "Bit 28 - RTC_DBOOST force power down"]
    #[inline(always)]
    pub fn dboost_force_pd(&mut self) -> DBOOST_FORCE_PD_W {
        DBOOST_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 29 - RTC_DBOOST force power up"]
    #[inline(always)]
    pub fn dboost_force_pu(&mut self) -> DBOOST_FORCE_PU_W {
        DBOOST_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 30 - RTC_REG force power down (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
    #[inline(always)]
    pub fn regulator_force_pd(&mut self) -> REGULATOR_FORCE_PD_W {
        REGULATOR_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 31 - RTC_REG force power on (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
    #[inline(always)]
    pub fn regulator_force_pu(&mut self) -> REGULATOR_FORCE_PU_W {
        REGULATOR_FORCE_PU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure rtc regulator\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc]
(index.html) module"]
pub struct RTC_SPEC;
impl crate::RegisterSpec for RTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc::R]
(R) reader structure"]
impl crate::Readable for RTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc::W]
(W) writer structure"]
impl crate::Writable for RTC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC to value 0xa000_0000"]
impl crate::Resettable for RTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa000_0000
    }
}
