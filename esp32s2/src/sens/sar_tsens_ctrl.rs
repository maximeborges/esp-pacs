#[doc = "Register `SAR_TSENS_CTRL` reader"]
pub struct R(crate::R<SAR_TSENS_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TSENS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TSENS_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TSENS_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_TSENS_CTRL` writer"]
pub struct W(crate::W<SAR_TSENS_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_TSENS_CTRL_SPEC>;
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
impl From<crate::W<SAR_TSENS_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_TSENS_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSENS_OUT` reader - Temperature sensor data out."]
pub struct TSENS_OUT_R(crate::FieldReader<u8>);
impl TSENS_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TSENS_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENS_OUT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENS_READY` reader - Indicate temperature sensor out ready."]
pub struct TSENS_READY_R(crate::FieldReader<bool>);
impl TSENS_READY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSENS_READY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENS_READY_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENS_INT_EN` reader - Enable temperature sensor to send out interrupt."]
pub struct TSENS_INT_EN_R(crate::FieldReader<bool>);
impl TSENS_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSENS_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENS_INT_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENS_INT_EN` writer - Enable temperature sensor to send out interrupt."]
pub struct TSENS_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
#[doc = "Field `TSENS_IN_INV` reader - Invert temperature sensor data."]
pub struct TSENS_IN_INV_R(crate::FieldReader<bool>);
impl TSENS_IN_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSENS_IN_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENS_IN_INV_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENS_IN_INV` writer - Invert temperature sensor data."]
pub struct TSENS_IN_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_IN_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `TSENS_CLK_DIV` reader - Temperature sensor clock divider."]
pub struct TSENS_CLK_DIV_R(crate::FieldReader<u8>);
impl TSENS_CLK_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TSENS_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENS_CLK_DIV_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENS_CLK_DIV` writer - Temperature sensor clock divider."]
pub struct TSENS_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 14)) | ((value as u32 & 0xff) << 14);
        self.w
    }
}
#[doc = "Field `TSENS_POWER_UP` reader - Temperature sensor power up."]
pub struct TSENS_POWER_UP_R(crate::FieldReader<bool>);
impl TSENS_POWER_UP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSENS_POWER_UP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENS_POWER_UP_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENS_POWER_UP` writer - Temperature sensor power up."]
pub struct TSENS_POWER_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_POWER_UP_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 22)) | ((value as u32 & 1) << 22);
        self.w
    }
}
#[doc = "Field `TSENS_POWER_UP_FORCE` reader - 1: dump out and power up controlled by software. 0: by FSM."]
pub struct TSENS_POWER_UP_FORCE_R(crate::FieldReader<bool>);
impl TSENS_POWER_UP_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSENS_POWER_UP_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENS_POWER_UP_FORCE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENS_POWER_UP_FORCE` writer - 1: dump out and power up controlled by software. 0: by FSM."]
pub struct TSENS_POWER_UP_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_POWER_UP_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 23)) | ((value as u32 & 1) << 23);
        self.w
    }
}
#[doc = "Field `TSENS_DUMP_OUT` reader - Temperature sensor dump out only active when SENS_TSENS_POWER_UP_FORCE = 1."]
pub struct TSENS_DUMP_OUT_R(crate::FieldReader<bool>);
impl TSENS_DUMP_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSENS_DUMP_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENS_DUMP_OUT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENS_DUMP_OUT` writer - Temperature sensor dump out only active when SENS_TSENS_POWER_UP_FORCE = 1."]
pub struct TSENS_DUMP_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_DUMP_OUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 24)) | ((value as u32 & 1) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Temperature sensor data out."]
    #[inline(always)]
    pub fn tsens_out(&self) -> TSENS_OUT_R {
        TSENS_OUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Indicate temperature sensor out ready."]
    #[inline(always)]
    pub fn tsens_ready(&self) -> TSENS_READY_R {
        TSENS_READY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable temperature sensor to send out interrupt."]
    #[inline(always)]
    pub fn tsens_int_en(&self) -> TSENS_INT_EN_R {
        TSENS_INT_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Invert temperature sensor data."]
    #[inline(always)]
    pub fn tsens_in_inv(&self) -> TSENS_IN_INV_R {
        TSENS_IN_INV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:21 - Temperature sensor clock divider."]
    #[inline(always)]
    pub fn tsens_clk_div(&self) -> TSENS_CLK_DIV_R {
        TSENS_CLK_DIV_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bit 22 - Temperature sensor power up."]
    #[inline(always)]
    pub fn tsens_power_up(&self) -> TSENS_POWER_UP_R {
        TSENS_POWER_UP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: dump out and power up controlled by software. 0: by FSM."]
    #[inline(always)]
    pub fn tsens_power_up_force(&self) -> TSENS_POWER_UP_FORCE_R {
        TSENS_POWER_UP_FORCE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Temperature sensor dump out only active when SENS_TSENS_POWER_UP_FORCE = 1."]
    #[inline(always)]
    pub fn tsens_dump_out(&self) -> TSENS_DUMP_OUT_R {
        TSENS_DUMP_OUT_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Enable temperature sensor to send out interrupt."]
    #[inline(always)]
    pub fn tsens_int_en(&mut self) -> TSENS_INT_EN_W {
        TSENS_INT_EN_W { w: self }
    }
    #[doc = "Bit 13 - Invert temperature sensor data."]
    #[inline(always)]
    pub fn tsens_in_inv(&mut self) -> TSENS_IN_INV_W {
        TSENS_IN_INV_W { w: self }
    }
    #[doc = "Bits 14:21 - Temperature sensor clock divider."]
    #[inline(always)]
    pub fn tsens_clk_div(&mut self) -> TSENS_CLK_DIV_W {
        TSENS_CLK_DIV_W { w: self }
    }
    #[doc = "Bit 22 - Temperature sensor power up."]
    #[inline(always)]
    pub fn tsens_power_up(&mut self) -> TSENS_POWER_UP_W {
        TSENS_POWER_UP_W { w: self }
    }
    #[doc = "Bit 23 - 1: dump out and power up controlled by software. 0: by FSM."]
    #[inline(always)]
    pub fn tsens_power_up_force(&mut self) -> TSENS_POWER_UP_FORCE_W {
        TSENS_POWER_UP_FORCE_W { w: self }
    }
    #[doc = "Bit 24 - Temperature sensor dump out only active when SENS_TSENS_POWER_UP_FORCE = 1."]
    #[inline(always)]
    pub fn tsens_dump_out(&mut self) -> TSENS_DUMP_OUT_W {
        TSENS_DUMP_OUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Temperature sensor data control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_tsens_ctrl](index.html) module"]
pub struct SAR_TSENS_CTRL_SPEC;
impl crate::RegisterSpec for SAR_TSENS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_tsens_ctrl::R](R) reader structure"]
impl crate::Readable for SAR_TSENS_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_tsens_ctrl::W](W) writer structure"]
impl crate::Writable for SAR_TSENS_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_TSENS_CTRL to value 0x0001_9000"]
impl crate::Resettable for SAR_TSENS_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_9000
    }
}
