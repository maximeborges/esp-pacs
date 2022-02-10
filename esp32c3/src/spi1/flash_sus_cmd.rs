#[doc = "Register `FLASH_SUS_CMD` reader"]
pub struct R(crate::R<FLASH_SUS_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_SUS_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_SUS_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_SUS_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_SUS_CMD` writer"]
pub struct W(crate::W<FLASH_SUS_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_SUS_CMD_SPEC>;
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
impl From<crate::W<FLASH_SUS_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_SUS_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_PER_COMMAND` reader - Program/Erase resume command."]
pub struct FLASH_PER_COMMAND_R(crate::FieldReader<u8, u8>);
impl FLASH_PER_COMMAND_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FLASH_PER_COMMAND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_PER_COMMAND_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_PER_COMMAND` writer - Program/Erase resume command."]
pub struct FLASH_PER_COMMAND_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_PER_COMMAND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `FLASH_PES_COMMAND` reader - Program/Erase suspend command."]
pub struct FLASH_PES_COMMAND_R(crate::FieldReader<u8, u8>);
impl FLASH_PES_COMMAND_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FLASH_PES_COMMAND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_PES_COMMAND_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_PES_COMMAND` writer - Program/Erase suspend command."]
pub struct FLASH_PES_COMMAND_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_PES_COMMAND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `WAIT_PESR_COMMAND` reader - Flash SUS/SUS1/SUS2 status bit read command. The command should be sent when SUS/SUS1/SUS2 bit should be checked to insure the suspend or resume status of flash."]
pub struct WAIT_PESR_COMMAND_R(crate::FieldReader<u16, u16>);
impl WAIT_PESR_COMMAND_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        WAIT_PESR_COMMAND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAIT_PESR_COMMAND_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAIT_PESR_COMMAND` writer - Flash SUS/SUS1/SUS2 status bit read command. The command should be sent when SUS/SUS1/SUS2 bit should be checked to insure the suspend or resume status of flash."]
pub struct WAIT_PESR_COMMAND_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_PESR_COMMAND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Program/Erase resume command."]
    #[inline(always)]
    pub fn flash_per_command(&self) -> FLASH_PER_COMMAND_R {
        FLASH_PER_COMMAND_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Program/Erase suspend command."]
    #[inline(always)]
    pub fn flash_pes_command(&self) -> FLASH_PES_COMMAND_R {
        FLASH_PES_COMMAND_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Flash SUS/SUS1/SUS2 status bit read command. The command should be sent when SUS/SUS1/SUS2 bit should be checked to insure the suspend or resume status of flash."]
    #[inline(always)]
    pub fn wait_pesr_command(&self) -> WAIT_PESR_COMMAND_R {
        WAIT_PESR_COMMAND_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Program/Erase resume command."]
    #[inline(always)]
    pub fn flash_per_command(&mut self) -> FLASH_PER_COMMAND_W {
        FLASH_PER_COMMAND_W { w: self }
    }
    #[doc = "Bits 8:15 - Program/Erase suspend command."]
    #[inline(always)]
    pub fn flash_pes_command(&mut self) -> FLASH_PES_COMMAND_W {
        FLASH_PES_COMMAND_W { w: self }
    }
    #[doc = "Bits 16:31 - Flash SUS/SUS1/SUS2 status bit read command. The command should be sent when SUS/SUS1/SUS2 bit should be checked to insure the suspend or resume status of flash."]
    #[inline(always)]
    pub fn wait_pesr_command(&mut self) -> WAIT_PESR_COMMAND_W {
        WAIT_PESR_COMMAND_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 flash suspend command register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_sus_cmd]
(index.html) module"]
pub struct FLASH_SUS_CMD_SPEC;
impl crate::RegisterSpec for FLASH_SUS_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_sus_cmd::R]
(R) reader structure"]
impl crate::Readable for FLASH_SUS_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_sus_cmd::W]
(W) writer structure"]
impl crate::Writable for FLASH_SUS_CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_SUS_CMD to value 0x0005_757a"]
impl crate::Resettable for FLASH_SUS_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0005_757a
    }
}
