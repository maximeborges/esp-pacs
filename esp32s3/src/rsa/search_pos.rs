#[doc = "Register `SEARCH_POS` reader"]
pub struct R(crate::R<SEARCH_POS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEARCH_POS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEARCH_POS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEARCH_POS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEARCH_POS` writer"]
pub struct W(crate::W<SEARCH_POS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEARCH_POS_SPEC>;
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
impl From<crate::W<SEARCH_POS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEARCH_POS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEARCH_POS` reader - This field is used to configure the starting search position when the acceleration option of SEARCH is used."]
pub struct SEARCH_POS_R(crate::FieldReader<u16, u16>);
impl SEARCH_POS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SEARCH_POS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEARCH_POS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEARCH_POS` writer - This field is used to configure the starting search position when the acceleration option of SEARCH is used."]
pub struct SEARCH_POS_W<'a> {
    w: &'a mut W,
}
impl<'a> SEARCH_POS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - This field is used to configure the starting search position when the acceleration option of SEARCH is used."]
    #[inline(always)]
    pub fn search_pos(&self) -> SEARCH_POS_R {
        SEARCH_POS_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - This field is used to configure the starting search position when the acceleration option of SEARCH is used."]
    #[inline(always)]
    pub fn search_pos(&mut self) -> SEARCH_POS_W {
        SEARCH_POS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RSA search position configure register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [search_pos]
(index.html) module"]
pub struct SEARCH_POS_SPEC;
impl crate::RegisterSpec for SEARCH_POS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [search_pos::R]
(R) reader structure"]
impl crate::Readable for SEARCH_POS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [search_pos::W]
(W) writer structure"]
impl crate::Writable for SEARCH_POS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEARCH_POS to value 0"]
impl crate::Resettable for SEARCH_POS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
