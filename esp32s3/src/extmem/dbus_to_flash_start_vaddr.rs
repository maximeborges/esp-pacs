#[doc = "Register `DBUS_TO_FLASH_START_VADDR` reader"]
pub struct R(crate::R<DBUS_TO_FLASH_START_VADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBUS_TO_FLASH_START_VADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBUS_TO_FLASH_START_VADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBUS_TO_FLASH_START_VADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBUS_TO_FLASH_START_VADDR` writer"]
pub struct W(crate::W<DBUS_TO_FLASH_START_VADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBUS_TO_FLASH_START_VADDR_SPEC>;
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
impl From<crate::W<DBUS_TO_FLASH_START_VADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBUS_TO_FLASH_START_VADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBUS_TO_FLASH_START_VADDR` reader - The bits are used to configure the start virtual address of dbus to access flash. The register is used to give constraints to dbus access counter."]
pub struct DBUS_TO_FLASH_START_VADDR_R(crate::FieldReader<u32>);
impl DBUS_TO_FLASH_START_VADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DBUS_TO_FLASH_START_VADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBUS_TO_FLASH_START_VADDR_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBUS_TO_FLASH_START_VADDR` writer - The bits are used to configure the start virtual address of dbus to access flash. The register is used to give constraints to dbus access counter."]
pub struct DBUS_TO_FLASH_START_VADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DBUS_TO_FLASH_START_VADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address of dbus to access flash. The register is used to give constraints to dbus access counter."]
    #[inline(always)]
    pub fn dbus_to_flash_start_vaddr(&self) -> DBUS_TO_FLASH_START_VADDR_R {
        DBUS_TO_FLASH_START_VADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address of dbus to access flash. The register is used to give constraints to dbus access counter."]
    #[inline(always)]
    pub fn dbus_to_flash_start_vaddr(&mut self) -> DBUS_TO_FLASH_START_VADDR_W {
        DBUS_TO_FLASH_START_VADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbus_to_flash_start_vaddr](index.html) module"]
pub struct DBUS_TO_FLASH_START_VADDR_SPEC;
impl crate::RegisterSpec for DBUS_TO_FLASH_START_VADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbus_to_flash_start_vaddr::R](R) reader structure"]
impl crate::Readable for DBUS_TO_FLASH_START_VADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbus_to_flash_start_vaddr::W](W) writer structure"]
impl crate::Writable for DBUS_TO_FLASH_START_VADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DBUS_TO_FLASH_START_VADDR to value 0"]
impl crate::Resettable for DBUS_TO_FLASH_START_VADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
