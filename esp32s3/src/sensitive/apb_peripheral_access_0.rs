#[doc = "Register `APB_PERIPHERAL_ACCESS_0` reader"]
pub struct R(crate::R<APB_PERIPHERAL_ACCESS_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_PERIPHERAL_ACCESS_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_PERIPHERAL_ACCESS_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_PERIPHERAL_ACCESS_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_PERIPHERAL_ACCESS_0` writer"]
pub struct W(crate::W<APB_PERIPHERAL_ACCESS_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_PERIPHERAL_ACCESS_0_SPEC>;
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
impl From<crate::W<APB_PERIPHERAL_ACCESS_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_PERIPHERAL_ACCESS_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APB_PERIPHERAL_ACCESS_LOCK` reader - Set 1 to lock APB peripheral Configuration Register."]
pub struct APB_PERIPHERAL_ACCESS_LOCK_R(crate::FieldReader<bool, bool>);
impl APB_PERIPHERAL_ACCESS_LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_PERIPHERAL_ACCESS_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_PERIPHERAL_ACCESS_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_PERIPHERAL_ACCESS_LOCK` writer - Set 1 to lock APB peripheral Configuration Register."]
pub struct APB_PERIPHERAL_ACCESS_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_PERIPHERAL_ACCESS_LOCK_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Set 1 to lock APB peripheral Configuration Register."]
    #[inline(always)]
    pub fn apb_peripheral_access_lock(&self) -> APB_PERIPHERAL_ACCESS_LOCK_R {
        APB_PERIPHERAL_ACCESS_LOCK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to lock APB peripheral Configuration Register."]
    #[inline(always)]
    pub fn apb_peripheral_access_lock(&mut self) -> APB_PERIPHERAL_ACCESS_LOCK_W {
        APB_PERIPHERAL_ACCESS_LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB peripheral configuration register 0.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_peripheral_access_0]
(index.html) module"]
pub struct APB_PERIPHERAL_ACCESS_0_SPEC;
impl crate::RegisterSpec for APB_PERIPHERAL_ACCESS_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_peripheral_access_0::R]
(R) reader structure"]
impl crate::Readable for APB_PERIPHERAL_ACCESS_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_peripheral_access_0::W]
(W) writer structure"]
impl crate::Writable for APB_PERIPHERAL_ACCESS_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB_PERIPHERAL_ACCESS_0 to value 0"]
impl crate::Resettable for APB_PERIPHERAL_ACCESS_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
