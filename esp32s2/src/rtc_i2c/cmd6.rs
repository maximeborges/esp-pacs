#[doc = "Register `CMD6` reader"]
pub struct R(crate::R<CMD6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD6` writer"]
pub struct W(crate::W<CMD6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD6_SPEC>;
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
impl From<crate::W<CMD6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND6` reader - Content of command 6. For more information, please refer to the register I2C_COMD6_REG in Chapter I²C Controller."]
pub type COMMAND6_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMMAND6` writer - Content of command 6. For more information, please refer to the register I2C_COMD6_REG in Chapter I²C Controller."]
pub type COMMAND6_W<'a> = crate::FieldWriter<'a, u32, CMD6_SPEC, u16, u16, 14, 0>;
#[doc = "Field `COMMAND6_DONE` reader - When command 6 is done, this bit changes to 1."]
pub type COMMAND6_DONE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:13 - Content of command 6. For more information, please refer to the register I2C_COMD6_REG in Chapter I²C Controller."]
    #[inline(always)]
    pub fn command6(&self) -> COMMAND6_R {
        COMMAND6_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - When command 6 is done, this bit changes to 1."]
    #[inline(always)]
    pub fn command6_done(&self) -> COMMAND6_DONE_R {
        COMMAND6_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - Content of command 6. For more information, please refer to the register I2C_COMD6_REG in Chapter I²C Controller."]
    #[inline(always)]
    pub fn command6(&mut self) -> COMMAND6_W {
        COMMAND6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC I2C Command 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd6](index.html) module"]
pub struct CMD6_SPEC;
impl crate::RegisterSpec for CMD6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd6::R](R) reader structure"]
impl crate::Readable for CMD6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd6::W](W) writer structure"]
impl crate::Writable for CMD6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD6 to value 0x1901"]
impl crate::Resettable for CMD6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1901
    }
}
