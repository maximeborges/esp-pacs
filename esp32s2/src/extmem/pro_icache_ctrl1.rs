#[doc = "Register `PRO_ICACHE_CTRL1` reader"]
pub struct R(crate::R<PRO_ICACHE_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_ICACHE_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_ICACHE_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_ICACHE_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_ICACHE_CTRL1` writer"]
pub struct W(crate::W<PRO_ICACHE_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_ICACHE_CTRL1_SPEC>;
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
impl From<crate::W<PRO_ICACHE_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_ICACHE_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_ICACHE_MASK_BUS0` reader - The bit is used to disable ibus0, 0: enable, 1: disable"]
pub struct PRO_ICACHE_MASK_BUS0_R(crate::FieldReader<bool, bool>);
impl PRO_ICACHE_MASK_BUS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_ICACHE_MASK_BUS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_ICACHE_MASK_BUS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_ICACHE_MASK_BUS0` writer - The bit is used to disable ibus0, 0: enable, 1: disable"]
pub struct PRO_ICACHE_MASK_BUS0_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_ICACHE_MASK_BUS0_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `PRO_ICACHE_MASK_BUS1` reader - The bit is used to disable ibus1, 0: enable, 1: disable"]
pub struct PRO_ICACHE_MASK_BUS1_R(crate::FieldReader<bool, bool>);
impl PRO_ICACHE_MASK_BUS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_ICACHE_MASK_BUS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_ICACHE_MASK_BUS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_ICACHE_MASK_BUS1` writer - The bit is used to disable ibus1, 0: enable, 1: disable"]
pub struct PRO_ICACHE_MASK_BUS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_ICACHE_MASK_BUS1_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `PRO_ICACHE_MASK_BUS2` reader - The bit is used to disable ibus2, 0: enable, 1: disable"]
pub struct PRO_ICACHE_MASK_BUS2_R(crate::FieldReader<bool, bool>);
impl PRO_ICACHE_MASK_BUS2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_ICACHE_MASK_BUS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_ICACHE_MASK_BUS2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_ICACHE_MASK_BUS2` writer - The bit is used to disable ibus2, 0: enable, 1: disable"]
pub struct PRO_ICACHE_MASK_BUS2_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_ICACHE_MASK_BUS2_W<'a> {
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
impl R {
    #[doc = "Bit 0 - The bit is used to disable ibus0, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn pro_icache_mask_bus0(&self) -> PRO_ICACHE_MASK_BUS0_R {
        PRO_ICACHE_MASK_BUS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to disable ibus1, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn pro_icache_mask_bus1(&self) -> PRO_ICACHE_MASK_BUS1_R {
        PRO_ICACHE_MASK_BUS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to disable ibus2, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn pro_icache_mask_bus2(&self) -> PRO_ICACHE_MASK_BUS2_R {
        PRO_ICACHE_MASK_BUS2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to disable ibus0, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn pro_icache_mask_bus0(&mut self) -> PRO_ICACHE_MASK_BUS0_W {
        PRO_ICACHE_MASK_BUS0_W { w: self }
    }
    #[doc = "Bit 1 - The bit is used to disable ibus1, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn pro_icache_mask_bus1(&mut self) -> PRO_ICACHE_MASK_BUS1_W {
        PRO_ICACHE_MASK_BUS1_W { w: self }
    }
    #[doc = "Bit 2 - The bit is used to disable ibus2, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn pro_icache_mask_bus2(&mut self) -> PRO_ICACHE_MASK_BUS2_W {
        PRO_ICACHE_MASK_BUS2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_icache_ctrl1]
(index.html) module"]
pub struct PRO_ICACHE_CTRL1_SPEC;
impl crate::RegisterSpec for PRO_ICACHE_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_icache_ctrl1::R]
(R) reader structure"]
impl crate::Readable for PRO_ICACHE_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_icache_ctrl1::W]
(W) writer structure"]
impl crate::Writable for PRO_ICACHE_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_ICACHE_CTRL1 to value 0x07"]
impl crate::Resettable for PRO_ICACHE_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
