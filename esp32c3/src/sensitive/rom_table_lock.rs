#[doc = "Register `ROM_TABLE_LOCK` reader"]
pub struct R(crate::R<ROM_TABLE_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROM_TABLE_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROM_TABLE_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROM_TABLE_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROM_TABLE_LOCK` writer"]
pub struct W(crate::W<ROM_TABLE_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROM_TABLE_LOCK_SPEC>;
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
impl From<crate::W<ROM_TABLE_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROM_TABLE_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROM_TABLE_LOCK` reader - rom_table_lock"]
pub struct ROM_TABLE_LOCK_R(crate::FieldReader<bool, bool>);
impl ROM_TABLE_LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROM_TABLE_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_TABLE_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM_TABLE_LOCK` writer - rom_table_lock"]
pub struct ROM_TABLE_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_TABLE_LOCK_W<'a> {
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
    #[doc = "Bit 0 - rom_table_lock"]
    #[inline(always)]
    pub fn rom_table_lock(&self) -> ROM_TABLE_LOCK_R {
        ROM_TABLE_LOCK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - rom_table_lock"]
    #[inline(always)]
    pub fn rom_table_lock(&mut self) -> ROM_TABLE_LOCK_W {
        ROM_TABLE_LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_ROM_TABLE_LOCK_REG\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_table_lock]
(index.html) module"]
pub struct ROM_TABLE_LOCK_SPEC;
impl crate::RegisterSpec for ROM_TABLE_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rom_table_lock::R]
(R) reader structure"]
impl crate::Readable for ROM_TABLE_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rom_table_lock::W]
(W) writer structure"]
impl crate::Writable for ROM_TABLE_LOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROM_TABLE_LOCK to value 0"]
impl crate::Resettable for ROM_TABLE_LOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
