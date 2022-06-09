#[doc = "Register `CMD12` reader"]
pub struct R(crate::R<CMD12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD12` writer"]
pub struct W(crate::W<CMD12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD12_SPEC>;
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
impl From<crate::W<CMD12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND12` reader - command12"]
pub type COMMAND12_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMMAND12` writer - command12"]
pub type COMMAND12_W<'a> = crate::FieldWriter<'a, u32, CMD12_SPEC, u16, u16, 14, 0>;
#[doc = "Field `COMMAND12_DONE` reader - command12_done"]
pub type COMMAND12_DONE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:13 - command12"]
    #[inline(always)]
    pub fn command12(&self) -> COMMAND12_R {
        COMMAND12_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - command12_done"]
    #[inline(always)]
    pub fn command12_done(&self) -> COMMAND12_DONE_R {
        COMMAND12_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - command12"]
    #[inline(always)]
    pub fn command12(&mut self) -> COMMAND12_W {
        COMMAND12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2c commond12 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd12](index.html) module"]
pub struct CMD12_SPEC;
impl crate::RegisterSpec for CMD12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd12::R](R) reader structure"]
impl crate::Readable for CMD12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd12::W](W) writer structure"]
impl crate::Writable for CMD12_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD12 to value 0x1701"]
impl crate::Resettable for CMD12_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1701
    }
}
