#[doc = "Register `CMD14` reader"]
pub struct R(crate::R<CMD14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD14` writer"]
pub struct W(crate::W<CMD14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD14_SPEC>;
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
impl From<crate::W<CMD14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND14` reader - Content of command 14. For more information, please refer to the register I2C_COMD14_REG in Chapter I²C Controller."]
pub type COMMAND14_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMMAND14` writer - Content of command 14. For more information, please refer to the register I2C_COMD14_REG in Chapter I²C Controller."]
pub type COMMAND14_W<'a> = crate::FieldWriter<'a, u32, CMD14_SPEC, u16, u16, 14, 0>;
#[doc = "Field `COMMAND14_DONE` reader - When command 14 is done, this bit changes to 1."]
pub type COMMAND14_DONE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:13 - Content of command 14. For more information, please refer to the register I2C_COMD14_REG in Chapter I²C Controller."]
    #[inline(always)]
    pub fn command14(&self) -> COMMAND14_R {
        COMMAND14_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - When command 14 is done, this bit changes to 1."]
    #[inline(always)]
    pub fn command14_done(&self) -> COMMAND14_DONE_R {
        COMMAND14_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - Content of command 14. For more information, please refer to the register I2C_COMD14_REG in Chapter I²C Controller."]
    #[inline(always)]
    pub fn command14(&mut self) -> COMMAND14_W {
        COMMAND14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC I2C Command 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd14](index.html) module"]
pub struct CMD14_SPEC;
impl crate::RegisterSpec for CMD14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd14::R](R) reader structure"]
impl crate::Readable for CMD14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd14::W](W) writer structure"]
impl crate::Writable for CMD14_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD14 to value 0"]
impl crate::Resettable for CMD14_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
