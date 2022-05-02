#[doc = "Register `_0RXFIFO_PUSH` reader"]
pub struct R(crate::R<_0RXFIFO_PUSH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_0RXFIFO_PUSH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_0RXFIFO_PUSH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_0RXFIFO_PUSH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_0RXFIFO_PUSH` writer"]
pub struct W(crate::W<_0RXFIFO_PUSH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_0RXFIFO_PUSH_SPEC>;
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
impl From<crate::W<_0RXFIFO_PUSH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_0RXFIFO_PUSH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC0_RXFIFO_WDATA` reader - "]
pub struct SLC0_RXFIFO_WDATA_R(crate::FieldReader<u16>);
impl SLC0_RXFIFO_WDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SLC0_RXFIFO_WDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_RXFIFO_WDATA_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_RXFIFO_WDATA` writer - "]
pub struct SLC0_RXFIFO_WDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RXFIFO_WDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
#[doc = "Field `SLC0_RXFIFO_PUSH` reader - "]
pub struct SLC0_RXFIFO_PUSH_R(crate::FieldReader<bool>);
impl SLC0_RXFIFO_PUSH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_RXFIFO_PUSH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_RXFIFO_PUSH_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_RXFIFO_PUSH` writer - "]
pub struct SLC0_RXFIFO_PUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RXFIFO_PUSH_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn slc0_rxfifo_wdata(&self) -> SLC0_RXFIFO_WDATA_R {
        SLC0_RXFIFO_WDATA_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc0_rxfifo_push(&self) -> SLC0_RXFIFO_PUSH_R {
        SLC0_RXFIFO_PUSH_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn slc0_rxfifo_wdata(&mut self) -> SLC0_RXFIFO_WDATA_W {
        SLC0_RXFIFO_WDATA_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc0_rxfifo_push(&mut self) -> SLC0_RXFIFO_PUSH_W {
        SLC0_RXFIFO_PUSH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0rxfifo_push](index.html) module"]
pub struct _0RXFIFO_PUSH_SPEC;
impl crate::RegisterSpec for _0RXFIFO_PUSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_0rxfifo_push::R](R) reader structure"]
impl crate::Readable for _0RXFIFO_PUSH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_0rxfifo_push::W](W) writer structure"]
impl crate::Writable for _0RXFIFO_PUSH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets _0RXFIFO_PUSH to value 0"]
impl crate::Resettable for _0RXFIFO_PUSH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
