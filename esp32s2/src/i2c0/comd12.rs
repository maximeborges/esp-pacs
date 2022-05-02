#[doc = "Register `COMD12` reader"]
pub struct R(crate::R<COMD12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMD12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMD12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMD12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMD12` writer"]
pub struct W(crate::W<COMD12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMD12_SPEC>;
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
impl From<crate::W<COMD12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMD12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND12` reader - This is the content of command 12. It consists of three parts: op_code is the command, 0: RSTART. 1: WRITE. 2: READ. 3: STOP. 4: END. byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more information."]
pub struct COMMAND12_R(crate::FieldReader<u16>);
impl COMMAND12_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        COMMAND12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMAND12_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMAND12` writer - This is the content of command 12. It consists of three parts: op_code is the command, 0: RSTART. 1: WRITE. 2: READ. 3: STOP. 4: END. byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more information."]
pub struct COMMAND12_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
#[doc = "Field `COMMAND12_DONE` reader - When command 12 is done in I2C Master mode, this bit changes to high level."]
pub struct COMMAND12_DONE_R(crate::FieldReader<bool>);
impl COMMAND12_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMMAND12_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMAND12_DONE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMAND12_DONE` writer - When command 12 is done in I2C Master mode, this bit changes to high level."]
pub struct COMMAND12_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND12_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - This is the content of command 12. It consists of three parts: op_code is the command, 0: RSTART. 1: WRITE. 2: READ. 3: STOP. 4: END. byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more information."]
    #[inline(always)]
    pub fn command12(&self) -> COMMAND12_R {
        COMMAND12_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - When command 12 is done in I2C Master mode, this bit changes to high level."]
    #[inline(always)]
    pub fn command12_done(&self) -> COMMAND12_DONE_R {
        COMMAND12_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - This is the content of command 12. It consists of three parts: op_code is the command, 0: RSTART. 1: WRITE. 2: READ. 3: STOP. 4: END. byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more information."]
    #[inline(always)]
    pub fn command12(&mut self) -> COMMAND12_W {
        COMMAND12_W { w: self }
    }
    #[doc = "Bit 31 - When command 12 is done in I2C Master mode, this bit changes to high level."]
    #[inline(always)]
    pub fn command12_done(&mut self) -> COMMAND12_DONE_W {
        COMMAND12_DONE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C command register 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comd12](index.html) module"]
pub struct COMD12_SPEC;
impl crate::RegisterSpec for COMD12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comd12::R](R) reader structure"]
impl crate::Readable for COMD12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comd12::W](W) writer structure"]
impl crate::Writable for COMD12_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMD12 to value 0"]
impl crate::Resettable for COMD12_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
