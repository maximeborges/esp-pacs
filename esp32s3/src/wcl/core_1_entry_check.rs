#[doc = "Register `Core_1_ENTRY_CHECK` reader"]
pub struct R(crate::R<CORE_1_ENTRY_CHECK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_ENTRY_CHECK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_ENTRY_CHECK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_ENTRY_CHECK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Core_1_ENTRY_CHECK` writer"]
pub struct W(crate::W<CORE_1_ENTRY_CHECK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_ENTRY_CHECK_SPEC>;
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
impl From<crate::W<CORE_1_ENTRY_CHECK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_ENTRY_CHECK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_1_ENTRY_CHECK` reader - This filed is used to enable entry address check"]
pub struct CORE_1_ENTRY_CHECK_R(crate::FieldReader<u16, u16>);
impl CORE_1_ENTRY_CHECK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CORE_1_ENTRY_CHECK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_ENTRY_CHECK_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_ENTRY_CHECK` writer - This filed is used to enable entry address check"]
pub struct CORE_1_ENTRY_CHECK_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_ENTRY_CHECK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 1)) | ((value as u32 & 0x1fff) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:13 - This filed is used to enable entry address check"]
    #[inline(always)]
    pub fn core_1_entry_check(&self) -> CORE_1_ENTRY_CHECK_R {
        CORE_1_ENTRY_CHECK_R::new(((self.bits >> 1) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:13 - This filed is used to enable entry address check"]
    #[inline(always)]
    pub fn core_1_entry_check(&mut self) -> CORE_1_ENTRY_CHECK_W {
        CORE_1_ENTRY_CHECK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core_1 Entry check configuration Register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_entry_check]
(index.html) module"]
pub struct CORE_1_ENTRY_CHECK_SPEC;
impl crate::RegisterSpec for CORE_1_ENTRY_CHECK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_entry_check::R]
(R) reader structure"]
impl crate::Readable for CORE_1_ENTRY_CHECK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_entry_check::W]
(W) writer structure"]
impl crate::Writable for CORE_1_ENTRY_CHECK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Core_1_ENTRY_CHECK to value 0x02"]
impl crate::Resettable for CORE_1_ENTRY_CHECK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
