#[doc = "Register `COMD2` reader"]
pub struct R(crate::R<COMD2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMD2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMD2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMD2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMD2` writer"]
pub struct W(crate::W<COMD2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMD2_SPEC>;
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
impl From<crate::W<COMD2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMD2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND2` reader - This is the content of command 2. It consists of three parts: op_code is the command, 0: RSTART. 1: WRITE. 2: READ. 3: STOP. 4: END. byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more information."]
pub struct COMMAND2_R(crate::FieldReader<u16, u16>);
impl COMMAND2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        COMMAND2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMAND2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMAND2` writer - This is the content of command 2. It consists of three parts: op_code is the command, 0: RSTART. 1: WRITE. 2: READ. 3: STOP. 4: END. byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more information."]
pub struct COMMAND2_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
#[doc = "Field `COMMAND2_DONE` reader - When command 2 is done in I2C Master mode, this bit changes to high level."]
pub struct COMMAND2_DONE_R(crate::FieldReader<bool, bool>);
impl COMMAND2_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMMAND2_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMAND2_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMAND2_DONE` writer - When command 2 is done in I2C Master mode, this bit changes to high level."]
pub struct COMMAND2_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND2_DONE_W<'a> {
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
    #[doc = "Bits 0:13 - This is the content of command 2. It consists of three parts: op_code is the command, 0: RSTART. 1: WRITE. 2: READ. 3: STOP. 4: END. byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more information."]
    #[inline(always)]
    pub fn command2(&self) -> COMMAND2_R {
        COMMAND2_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - When command 2 is done in I2C Master mode, this bit changes to high level."]
    #[inline(always)]
    pub fn command2_done(&self) -> COMMAND2_DONE_R {
        COMMAND2_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - This is the content of command 2. It consists of three parts: op_code is the command, 0: RSTART. 1: WRITE. 2: READ. 3: STOP. 4: END. byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more information."]
    #[inline(always)]
    pub fn command2(&mut self) -> COMMAND2_W {
        COMMAND2_W { w: self }
    }
    #[doc = "Bit 31 - When command 2 is done in I2C Master mode, this bit changes to high level."]
    #[inline(always)]
    pub fn command2_done(&mut self) -> COMMAND2_DONE_W {
        COMMAND2_DONE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C command register 2\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comd2]
(index.html) module"]
pub struct COMD2_SPEC;
impl crate::RegisterSpec for COMD2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comd2::R]
(R) reader structure"]
impl crate::Readable for COMD2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comd2::W]
(W) writer structure"]
impl crate::Writable for COMD2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMD2 to value 0"]
impl crate::Resettable for COMD2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
