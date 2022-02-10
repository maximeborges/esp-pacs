#[doc = "Register `COMD15` reader"]
pub struct R(crate::R<COMD15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMD15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMD15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMD15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMD15` writer"]
pub struct W(crate::W<COMD15_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMD15_SPEC>;
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
impl From<crate::W<COMD15_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMD15_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND15` reader - This is the content of command 15. It consists of three parts: op_code is the command, 0: RSTART. 1: WRITE. 2: READ. 3: STOP. 4: END. byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more information."]
pub struct COMMAND15_R(crate::FieldReader<u16, u16>);
impl COMMAND15_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        COMMAND15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMAND15_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMAND15` writer - This is the content of command 15. It consists of three parts: op_code is the command, 0: RSTART. 1: WRITE. 2: READ. 3: STOP. 4: END. byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more information."]
pub struct COMMAND15_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
#[doc = "Field `COMMAND15_DONE` reader - When command 15 is done in I2C Master mode, this bit changes to high level."]
pub struct COMMAND15_DONE_R(crate::FieldReader<bool, bool>);
impl COMMAND15_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMMAND15_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMAND15_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMAND15_DONE` writer - When command 15 is done in I2C Master mode, this bit changes to high level."]
pub struct COMMAND15_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND15_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - This is the content of command 15. It consists of three parts: op_code is the command, 0: RSTART. 1: WRITE. 2: READ. 3: STOP. 4: END. byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more information."]
    #[inline(always)]
    pub fn command15(&self) -> COMMAND15_R {
        COMMAND15_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - When command 15 is done in I2C Master mode, this bit changes to high level."]
    #[inline(always)]
    pub fn command15_done(&self) -> COMMAND15_DONE_R {
        COMMAND15_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - This is the content of command 15. It consists of three parts: op_code is the command, 0: RSTART. 1: WRITE. 2: READ. 3: STOP. 4: END. byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more information."]
    #[inline(always)]
    pub fn command15(&mut self) -> COMMAND15_W {
        COMMAND15_W { w: self }
    }
    #[doc = "Bit 31 - When command 15 is done in I2C Master mode, this bit changes to high level."]
    #[inline(always)]
    pub fn command15_done(&mut self) -> COMMAND15_DONE_W {
        COMMAND15_DONE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C command register 15\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comd15]
(index.html) module"]
pub struct COMD15_SPEC;
impl crate::RegisterSpec for COMD15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comd15::R]
(R) reader structure"]
impl crate::Readable for COMD15_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comd15::W]
(W) writer structure"]
impl crate::Writable for COMD15_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMD15 to value 0"]
impl crate::Resettable for COMD15_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
