#[doc = "Register `PHYSICAL_ADDRESS` reader"]
pub struct R(crate::R<PHYSICAL_ADDRESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHYSICAL_ADDRESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHYSICAL_ADDRESS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHYSICAL_ADDRESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PHYSICAL_ADDRESS` writer"]
pub struct W(crate::W<PHYSICAL_ADDRESS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHYSICAL_ADDRESS_SPEC>;
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
impl From<crate::W<PHYSICAL_ADDRESS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHYSICAL_ADDRESS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHYSICAL_ADDRESS` reader - Those bits stores the physical address. If linesize is 16-byte, the physical address should be aligned of 16 bytes. If linesize is 32-byte, the physical address should be aligned of 32 bytes."]
pub struct PHYSICAL_ADDRESS_R(crate::FieldReader<u32, u32>);
impl PHYSICAL_ADDRESS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PHYSICAL_ADDRESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHYSICAL_ADDRESS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHYSICAL_ADDRESS` writer - Those bits stores the physical address. If linesize is 16-byte, the physical address should be aligned of 16 bytes. If linesize is 32-byte, the physical address should be aligned of 32 bytes."]
pub struct PHYSICAL_ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYSICAL_ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | (value as u32 & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29 - Those bits stores the physical address. If linesize is 16-byte, the physical address should be aligned of 16 bytes. If linesize is 32-byte, the physical address should be aligned of 32 bytes."]
    #[inline(always)]
    pub fn physical_address(&self) -> PHYSICAL_ADDRESS_R {
        PHYSICAL_ADDRESS_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29 - Those bits stores the physical address. If linesize is 16-byte, the physical address should be aligned of 16 bytes. If linesize is 32-byte, the physical address should be aligned of 32 bytes."]
    #[inline(always)]
    pub fn physical_address(&mut self) -> PHYSICAL_ADDRESS_W {
        PHYSICAL_ADDRESS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XTS-AES physical address register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [physical_address]
(index.html) module"]
pub struct PHYSICAL_ADDRESS_SPEC;
impl crate::RegisterSpec for PHYSICAL_ADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [physical_address::R]
(R) reader structure"]
impl crate::Readable for PHYSICAL_ADDRESS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [physical_address::W]
(W) writer structure"]
impl crate::Writable for PHYSICAL_ADDRESS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PHYSICAL_ADDRESS to value 0"]
impl crate::Resettable for PHYSICAL_ADDRESS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
