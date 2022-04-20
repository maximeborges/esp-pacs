#[doc = "Register `CMD` reader"]
pub struct R(crate::R<CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READ_CMD` reader - Set this bit to send read command."]
pub struct READ_CMD_R(crate::FieldReader<bool, bool>);
impl READ_CMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        READ_CMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for READ_CMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READ_CMD` writer - Set this bit to send read command."]
pub struct READ_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_CMD_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `PGM_CMD` reader - Set this bit to send programming command."]
pub struct PGM_CMD_R(crate::FieldReader<bool, bool>);
impl PGM_CMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGM_CMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGM_CMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGM_CMD` writer - Set this bit to send programming command."]
pub struct PGM_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> PGM_CMD_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `BLK_NUM` reader - The serial number of the block to be programmed. Value 0-10 corresponds to block number 0-10, respectively."]
pub struct BLK_NUM_R(crate::FieldReader<u8, u8>);
impl BLK_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BLK_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLK_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLK_NUM` writer - The serial number of the block to be programmed. Value 0-10 corresponds to block number 0-10, respectively."]
pub struct BLK_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> BLK_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | ((value as u32 & 0x0f) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set this bit to send read command."]
    #[inline(always)]
    pub fn read_cmd(&self) -> READ_CMD_R {
        READ_CMD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to send programming command."]
    #[inline(always)]
    pub fn pgm_cmd(&self) -> PGM_CMD_R {
        PGM_CMD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - The serial number of the block to be programmed. Value 0-10 corresponds to block number 0-10, respectively."]
    #[inline(always)]
    pub fn blk_num(&self) -> BLK_NUM_R {
        BLK_NUM_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to send read command."]
    #[inline(always)]
    pub fn read_cmd(&mut self) -> READ_CMD_W {
        READ_CMD_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to send programming command."]
    #[inline(always)]
    pub fn pgm_cmd(&mut self) -> PGM_CMD_W {
        PGM_CMD_W { w: self }
    }
    #[doc = "Bits 2:5 - The serial number of the block to be programmed. Value 0-10 corresponds to block number 0-10, respectively."]
    #[inline(always)]
    pub fn blk_num(&mut self) -> BLK_NUM_W {
        BLK_NUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eFuse command register.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd]
(index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd::R]
(R) reader structure"]
impl crate::Readable for CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd::W]
(W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
