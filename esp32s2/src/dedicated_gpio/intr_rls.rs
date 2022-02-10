#[doc = "Register `INTR_RLS` reader"]
pub struct R(crate::R<INTR_RLS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_RLS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_RLS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_RLS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_RLS` writer"]
pub struct W(crate::W<INTR_RLS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_RLS_SPEC>;
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
impl From<crate::W<INTR_RLS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_RLS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO0_INT_ENA` reader - The enable bit for DEDIC_GPIO0_INT_ST register."]
pub struct GPIO0_INT_ENA_R(crate::FieldReader<bool, bool>);
impl GPIO0_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO0_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO0_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO0_INT_ENA` writer - The enable bit for DEDIC_GPIO0_INT_ST register."]
pub struct GPIO0_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO0_INT_ENA_W<'a> {
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
#[doc = "Field `GPIO1_INT_ENA` reader - The enable bit for DEDIC_GPIO1_INT_ST register."]
pub struct GPIO1_INT_ENA_R(crate::FieldReader<bool, bool>);
impl GPIO1_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO1_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO1_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO1_INT_ENA` writer - The enable bit for DEDIC_GPIO1_INT_ST register."]
pub struct GPIO1_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO1_INT_ENA_W<'a> {
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
#[doc = "Field `GPIO2_INT_ENA` reader - The enable bit for DEDIC_GPIO2_INT_ST register."]
pub struct GPIO2_INT_ENA_R(crate::FieldReader<bool, bool>);
impl GPIO2_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO2_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO2_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO2_INT_ENA` writer - The enable bit for DEDIC_GPIO2_INT_ST register."]
pub struct GPIO2_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO2_INT_ENA_W<'a> {
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
#[doc = "Field `GPIO3_INT_ENA` reader - The enable bit for DEDIC_GPIO3_INT_ST register."]
pub struct GPIO3_INT_ENA_R(crate::FieldReader<bool, bool>);
impl GPIO3_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO3_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO3_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO3_INT_ENA` writer - The enable bit for DEDIC_GPIO3_INT_ST register."]
pub struct GPIO3_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO3_INT_ENA_W<'a> {
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
#[doc = "Field `GPIO4_INT_ENA` reader - The enable bit for DEDIC_GPIO4_INT_ST register."]
pub struct GPIO4_INT_ENA_R(crate::FieldReader<bool, bool>);
impl GPIO4_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO4_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO4_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO4_INT_ENA` writer - The enable bit for DEDIC_GPIO4_INT_ST register."]
pub struct GPIO4_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO4_INT_ENA_W<'a> {
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
#[doc = "Field `GPIO5_INT_ENA` reader - The enable bit for DEDIC_GPIO5_INT_ST register."]
pub struct GPIO5_INT_ENA_R(crate::FieldReader<bool, bool>);
impl GPIO5_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO5_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO5_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO5_INT_ENA` writer - The enable bit for DEDIC_GPIO5_INT_ST register."]
pub struct GPIO5_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO5_INT_ENA_W<'a> {
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
#[doc = "Field `GPIO6_INT_ENA` reader - The enable bit for DEDIC_GPIO6_INT_ST register."]
pub struct GPIO6_INT_ENA_R(crate::FieldReader<bool, bool>);
impl GPIO6_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO6_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO6_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO6_INT_ENA` writer - The enable bit for DEDIC_GPIO6_INT_ST register."]
pub struct GPIO6_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO6_INT_ENA_W<'a> {
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
#[doc = "Field `GPIO7_INT_ENA` reader - The enable bit for DEDIC_GPIO7_INT_ST register."]
pub struct GPIO7_INT_ENA_R(crate::FieldReader<bool, bool>);
impl GPIO7_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO7_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO7_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO7_INT_ENA` writer - The enable bit for DEDIC_GPIO7_INT_ST register."]
pub struct GPIO7_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO7_INT_ENA_W<'a> {
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
impl R {
    #[doc = "Bit 0 - The enable bit for DEDIC_GPIO0_INT_ST register."]
    #[inline(always)]
    pub fn gpio0_int_ena(&self) -> GPIO0_INT_ENA_R {
        GPIO0_INT_ENA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The enable bit for DEDIC_GPIO1_INT_ST register."]
    #[inline(always)]
    pub fn gpio1_int_ena(&self) -> GPIO1_INT_ENA_R {
        GPIO1_INT_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The enable bit for DEDIC_GPIO2_INT_ST register."]
    #[inline(always)]
    pub fn gpio2_int_ena(&self) -> GPIO2_INT_ENA_R {
        GPIO2_INT_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The enable bit for DEDIC_GPIO3_INT_ST register."]
    #[inline(always)]
    pub fn gpio3_int_ena(&self) -> GPIO3_INT_ENA_R {
        GPIO3_INT_ENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The enable bit for DEDIC_GPIO4_INT_ST register."]
    #[inline(always)]
    pub fn gpio4_int_ena(&self) -> GPIO4_INT_ENA_R {
        GPIO4_INT_ENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - The enable bit for DEDIC_GPIO5_INT_ST register."]
    #[inline(always)]
    pub fn gpio5_int_ena(&self) -> GPIO5_INT_ENA_R {
        GPIO5_INT_ENA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The enable bit for DEDIC_GPIO6_INT_ST register."]
    #[inline(always)]
    pub fn gpio6_int_ena(&self) -> GPIO6_INT_ENA_R {
        GPIO6_INT_ENA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The enable bit for DEDIC_GPIO7_INT_ST register."]
    #[inline(always)]
    pub fn gpio7_int_ena(&self) -> GPIO7_INT_ENA_R {
        GPIO7_INT_ENA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The enable bit for DEDIC_GPIO0_INT_ST register."]
    #[inline(always)]
    pub fn gpio0_int_ena(&mut self) -> GPIO0_INT_ENA_W {
        GPIO0_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1 - The enable bit for DEDIC_GPIO1_INT_ST register."]
    #[inline(always)]
    pub fn gpio1_int_ena(&mut self) -> GPIO1_INT_ENA_W {
        GPIO1_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2 - The enable bit for DEDIC_GPIO2_INT_ST register."]
    #[inline(always)]
    pub fn gpio2_int_ena(&mut self) -> GPIO2_INT_ENA_W {
        GPIO2_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3 - The enable bit for DEDIC_GPIO3_INT_ST register."]
    #[inline(always)]
    pub fn gpio3_int_ena(&mut self) -> GPIO3_INT_ENA_W {
        GPIO3_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4 - The enable bit for DEDIC_GPIO4_INT_ST register."]
    #[inline(always)]
    pub fn gpio4_int_ena(&mut self) -> GPIO4_INT_ENA_W {
        GPIO4_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5 - The enable bit for DEDIC_GPIO5_INT_ST register."]
    #[inline(always)]
    pub fn gpio5_int_ena(&mut self) -> GPIO5_INT_ENA_W {
        GPIO5_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6 - The enable bit for DEDIC_GPIO6_INT_ST register."]
    #[inline(always)]
    pub fn gpio6_int_ena(&mut self) -> GPIO6_INT_ENA_W {
        GPIO6_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7 - The enable bit for DEDIC_GPIO7_INT_ST register."]
    #[inline(always)]
    pub fn gpio7_int_ena(&mut self) -> GPIO7_INT_ENA_W {
        GPIO7_INT_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable bits\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_rls]
(index.html) module"]
pub struct INTR_RLS_SPEC;
impl crate::RegisterSpec for INTR_RLS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_rls::R]
(R) reader structure"]
impl crate::Readable for INTR_RLS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_rls::W]
(W) writer structure"]
impl crate::Writable for INTR_RLS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_RLS to value 0"]
impl crate::Resettable for INTR_RLS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
