#[doc = "Register `OUT_CPU` reader"]
pub struct R(crate::R<OUT_CPU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_CPU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_CPU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_CPU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_CPU` writer"]
pub struct W(crate::W<OUT_CPU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_CPU_SPEC>;
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
impl From<crate::W<OUT_CPU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_CPU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL0` reader - Select GPIO out value configured by registers or CPU instructions for channel 0. 0: Configured by registers. 1: configured by CPU instructions."]
pub struct SEL0_R(crate::FieldReader<bool>);
impl SEL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL0_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL0` writer - Select GPIO out value configured by registers or CPU instructions for channel 0. 0: Configured by registers. 1: configured by CPU instructions."]
pub struct SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL0_W<'a> {
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
#[doc = "Field `SEL1` reader - Select GPIO out value configured by registers or CPU instructions for channel 1. 0: Configured by registers. 1: configured by CPU instructions."]
pub struct SEL1_R(crate::FieldReader<bool>);
impl SEL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL1_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL1` writer - Select GPIO out value configured by registers or CPU instructions for channel 1. 0: Configured by registers. 1: configured by CPU instructions."]
pub struct SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL1_W<'a> {
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
#[doc = "Field `SEL2` reader - Select GPIO out value configured by registers or CPU instructions for channel 2. 0: Configured by registers. 1: configured by CPU instructions."]
pub struct SEL2_R(crate::FieldReader<bool>);
impl SEL2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL2_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL2` writer - Select GPIO out value configured by registers or CPU instructions for channel 2. 0: Configured by registers. 1: configured by CPU instructions."]
pub struct SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL2_W<'a> {
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
#[doc = "Field `SEL3` reader - Select GPIO out value configured by registers or CPU instructions for channel 3. 0: Configured by registers. 1: configured by CPU instructions."]
pub struct SEL3_R(crate::FieldReader<bool>);
impl SEL3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL3_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL3` writer - Select GPIO out value configured by registers or CPU instructions for channel 3. 0: Configured by registers. 1: configured by CPU instructions."]
pub struct SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL3_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `SEL4` reader - Select GPIO out value configured by registers or CPU instructions for channel 4. 0: Configured by registers. 1: configured by CPU instructions."]
pub struct SEL4_R(crate::FieldReader<bool>);
impl SEL4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL4_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL4` writer - Select GPIO out value configured by registers or CPU instructions for channel 4. 0: Configured by registers. 1: configured by CPU instructions."]
pub struct SEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL4_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `SEL5` reader - Select GPIO out value configured by registers or CPU instructions for channel 5. 0: Configured by registers. 1: configured by CPU instructions."]
pub struct SEL5_R(crate::FieldReader<bool>);
impl SEL5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEL5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL5_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL5` writer - Select GPIO out value configured by registers or CPU instructions for channel 5. 0: Configured by registers. 1: configured by CPU instructions."]
pub struct SEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL5_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `SEL6` reader - Select GPIO out value configured by registers or CPU instructions for channel 6. 0: Configured by registers. 1: configured by CPU instructions."]
pub struct SEL6_R(crate::FieldReader<bool>);
impl SEL6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEL6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL6_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL6` writer - Select GPIO out value configured by registers or CPU instructions for channel 6. 0: Configured by registers. 1: configured by CPU instructions."]
pub struct SEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL6_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `SEL7` reader - Select GPIO out value configured by registers or CPU instructions for channel 7. 0: Configured by registers. 1: configured by CPU instructions."]
pub struct SEL7_R(crate::FieldReader<bool>);
impl SEL7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEL7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL7_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL7` writer - Select GPIO out value configured by registers or CPU instructions for channel 7. 0: Configured by registers. 1: configured by CPU instructions."]
pub struct SEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL7_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Select GPIO out value configured by registers or CPU instructions for channel 0. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel0(&self) -> SEL0_R {
        SEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select GPIO out value configured by registers or CPU instructions for channel 1. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel1(&self) -> SEL1_R {
        SEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select GPIO out value configured by registers or CPU instructions for channel 2. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel2(&self) -> SEL2_R {
        SEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Select GPIO out value configured by registers or CPU instructions for channel 3. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel3(&self) -> SEL3_R {
        SEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Select GPIO out value configured by registers or CPU instructions for channel 4. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel4(&self) -> SEL4_R {
        SEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Select GPIO out value configured by registers or CPU instructions for channel 5. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel5(&self) -> SEL5_R {
        SEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Select GPIO out value configured by registers or CPU instructions for channel 6. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel6(&self) -> SEL6_R {
        SEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Select GPIO out value configured by registers or CPU instructions for channel 7. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel7(&self) -> SEL7_R {
        SEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select GPIO out value configured by registers or CPU instructions for channel 0. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel0(&mut self) -> SEL0_W {
        SEL0_W { w: self }
    }
    #[doc = "Bit 1 - Select GPIO out value configured by registers or CPU instructions for channel 1. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel1(&mut self) -> SEL1_W {
        SEL1_W { w: self }
    }
    #[doc = "Bit 2 - Select GPIO out value configured by registers or CPU instructions for channel 2. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel2(&mut self) -> SEL2_W {
        SEL2_W { w: self }
    }
    #[doc = "Bit 3 - Select GPIO out value configured by registers or CPU instructions for channel 3. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel3(&mut self) -> SEL3_W {
        SEL3_W { w: self }
    }
    #[doc = "Bit 4 - Select GPIO out value configured by registers or CPU instructions for channel 4. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel4(&mut self) -> SEL4_W {
        SEL4_W { w: self }
    }
    #[doc = "Bit 5 - Select GPIO out value configured by registers or CPU instructions for channel 5. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel5(&mut self) -> SEL5_W {
        SEL5_W { w: self }
    }
    #[doc = "Bit 6 - Select GPIO out value configured by registers or CPU instructions for channel 6. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel6(&mut self) -> SEL6_W {
        SEL6_W { w: self }
    }
    #[doc = "Bit 7 - Select GPIO out value configured by registers or CPU instructions for channel 7. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel7(&mut self) -> SEL7_W {
        SEL7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dedicated GPIO output mode selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_cpu](index.html) module"]
pub struct OUT_CPU_SPEC;
impl crate::RegisterSpec for OUT_CPU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_cpu::R](R) reader structure"]
impl crate::Readable for OUT_CPU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_cpu::W](W) writer structure"]
impl crate::Writable for OUT_CPU_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT_CPU to value 0"]
impl crate::Resettable for OUT_CPU_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
