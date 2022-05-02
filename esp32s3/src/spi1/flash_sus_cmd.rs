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
#[doc = "Field `FLASH_PER` reader - program erase resume bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub struct FLASH_PER_R(crate::FieldReader<bool>);
impl FLASH_PER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_PER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_PER_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_PER` writer - program erase resume bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub struct FLASH_PER_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_PER_W<'a> {
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
#[doc = "Field `FLASH_PES` reader - program erase suspend bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub struct FLASH_PES_R(crate::FieldReader<bool>);
impl FLASH_PES_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_PES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_PES_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_PES` writer - program erase suspend bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub struct FLASH_PES_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_PES_W<'a> {
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
#[doc = "Field `FLASH_PER_WAIT_EN` reader - Set this bit to add delay time after program erase resume(PER) is sent."]
pub struct FLASH_PER_WAIT_EN_R(crate::FieldReader<bool>);
impl FLASH_PER_WAIT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_PER_WAIT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_PER_WAIT_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_PER_WAIT_EN` writer - Set this bit to add delay time after program erase resume(PER) is sent."]
pub struct FLASH_PER_WAIT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_PER_WAIT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `FLASH_PES_WAIT_EN` reader - Set this bit to add delay time after program erase suspend(PES) command is sent."]
pub struct FLASH_PES_WAIT_EN_R(crate::FieldReader<bool>);
impl FLASH_PES_WAIT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_PES_WAIT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_PES_WAIT_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_PES_WAIT_EN` writer - Set this bit to add delay time after program erase suspend(PES) command is sent."]
pub struct FLASH_PES_WAIT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_PES_WAIT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `PES_PER_EN` reader - Set this bit to enable PES transfer trigger PES transfer option."]
pub struct PES_PER_EN_R(crate::FieldReader<bool>);
impl PES_PER_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PES_PER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PES_PER_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PES_PER_EN` writer - Set this bit to enable PES transfer trigger PES transfer option."]
pub struct PES_PER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PES_PER_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `PESR_IDLE_EN` reader - 1: Separate PER flash wait idle and PES flash wait idle. 0: Not separate."]
pub struct PESR_IDLE_EN_R(crate::FieldReader<bool>);
impl PESR_IDLE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PESR_IDLE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PESR_IDLE_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PESR_IDLE_EN` writer - 1: Separate PER flash wait idle and PES flash wait idle. 0: Not separate."]
pub struct PESR_IDLE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PESR_IDLE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - program erase resume bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_per(&self) -> FLASH_PER_R {
        FLASH_PER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - program erase suspend bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_pes(&self) -> FLASH_PES_R {
        FLASH_PES_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to add delay time after program erase resume(PER) is sent."]
    #[inline(always)]
    pub fn flash_per_wait_en(&self) -> FLASH_PER_WAIT_EN_R {
        FLASH_PER_WAIT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to add delay time after program erase suspend(PES) command is sent."]
    #[inline(always)]
    pub fn flash_pes_wait_en(&self) -> FLASH_PES_WAIT_EN_R {
        FLASH_PES_WAIT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable PES transfer trigger PES transfer option."]
    #[inline(always)]
    pub fn pes_per_en(&self) -> PES_PER_EN_R {
        PES_PER_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Separate PER flash wait idle and PES flash wait idle. 0: Not separate."]
    #[inline(always)]
    pub fn pesr_idle_en(&self) -> PESR_IDLE_EN_R {
        PESR_IDLE_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - program erase resume bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_per(&mut self) -> FLASH_PER_W {
        FLASH_PER_W { w: self }
    }
    #[doc = "Bit 1 - program erase suspend bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_pes(&mut self) -> FLASH_PES_W {
        FLASH_PES_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to add delay time after program erase resume(PER) is sent."]
    #[inline(always)]
    pub fn flash_per_wait_en(&mut self) -> FLASH_PER_WAIT_EN_W {
        FLASH_PER_WAIT_EN_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to add delay time after program erase suspend(PES) command is sent."]
    #[inline(always)]
    pub fn flash_pes_wait_en(&mut self) -> FLASH_PES_WAIT_EN_W {
        FLASH_PES_WAIT_EN_W { w: self }
    }
    #[doc = "Bit 4 - Set this bit to enable PES transfer trigger PES transfer option."]
    #[inline(always)]
    pub fn pes_per_en(&mut self) -> PES_PER_EN_W {
        PES_PER_EN_W { w: self }
    }
    #[doc = "Bit 5 - 1: Separate PER flash wait idle and PES flash wait idle. 0: Not separate."]
    #[inline(always)]
    pub fn pesr_idle_en(&mut self) -> PESR_IDLE_EN_W {
        PESR_IDLE_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 flash suspend control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_sus_cmd](index.html) module"]
pub struct FLASH_SUS_CMD_SPEC;
impl crate::RegisterSpec for FLASH_SUS_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_sus_cmd::R](R) reader structure"]
impl crate::Readable for FLASH_SUS_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_sus_cmd::W](W) writer structure"]
impl crate::Writable for FLASH_SUS_CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_SUS_CMD to value 0"]
impl crate::Resettable for FLASH_SUS_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
