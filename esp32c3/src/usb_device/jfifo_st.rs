#[doc = "Register `JFIFO_ST` reader"]
pub struct R(crate::R<JFIFO_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JFIFO_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JFIFO_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JFIFO_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JFIFO_ST` writer"]
pub struct W(crate::W<JFIFO_ST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JFIFO_ST_SPEC>;
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
impl From<crate::W<JFIFO_ST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JFIFO_ST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN_FIFO_CNT` reader - JTAT in fifo counter."]
pub struct IN_FIFO_CNT_R(crate::FieldReader<u8, u8>);
impl IN_FIFO_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IN_FIFO_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_FIFO_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_FIFO_EMPTY` reader - 1: JTAG in fifo is empty."]
pub struct IN_FIFO_EMPTY_R(crate::FieldReader<bool, bool>);
impl IN_FIFO_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_FIFO_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_FIFO_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_FIFO_FULL` reader - 1: JTAG in fifo is full."]
pub struct IN_FIFO_FULL_R(crate::FieldReader<bool, bool>);
impl IN_FIFO_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_FIFO_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_FIFO_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_FIFO_CNT` reader - JTAT out fifo counter."]
pub struct OUT_FIFO_CNT_R(crate::FieldReader<u8, u8>);
impl OUT_FIFO_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OUT_FIFO_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_FIFO_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_FIFO_EMPTY` reader - 1: JTAG out fifo is empty."]
pub struct OUT_FIFO_EMPTY_R(crate::FieldReader<bool, bool>);
impl OUT_FIFO_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_FIFO_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_FIFO_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_FIFO_FULL` reader - 1: JTAG out fifo is full."]
pub struct OUT_FIFO_FULL_R(crate::FieldReader<bool, bool>);
impl OUT_FIFO_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_FIFO_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_FIFO_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_FIFO_RESET` reader - Write 1 to reset JTAG in fifo."]
pub struct IN_FIFO_RESET_R(crate::FieldReader<bool, bool>);
impl IN_FIFO_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_FIFO_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_FIFO_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_FIFO_RESET` writer - Write 1 to reset JTAG in fifo."]
pub struct IN_FIFO_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_FIFO_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `OUT_FIFO_RESET` reader - Write 1 to reset JTAG out fifo."]
pub struct OUT_FIFO_RESET_R(crate::FieldReader<bool, bool>);
impl OUT_FIFO_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_FIFO_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_FIFO_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_FIFO_RESET` writer - Write 1 to reset JTAG out fifo."]
pub struct OUT_FIFO_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_FIFO_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - JTAT in fifo counter."]
    #[inline(always)]
    pub fn in_fifo_cnt(&self) -> IN_FIFO_CNT_R {
        IN_FIFO_CNT_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - 1: JTAG in fifo is empty."]
    #[inline(always)]
    pub fn in_fifo_empty(&self) -> IN_FIFO_EMPTY_R {
        IN_FIFO_EMPTY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1: JTAG in fifo is full."]
    #[inline(always)]
    pub fn in_fifo_full(&self) -> IN_FIFO_FULL_R {
        IN_FIFO_FULL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - JTAT out fifo counter."]
    #[inline(always)]
    pub fn out_fifo_cnt(&self) -> OUT_FIFO_CNT_R {
        OUT_FIFO_CNT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - 1: JTAG out fifo is empty."]
    #[inline(always)]
    pub fn out_fifo_empty(&self) -> OUT_FIFO_EMPTY_R {
        OUT_FIFO_EMPTY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 1: JTAG out fifo is full."]
    #[inline(always)]
    pub fn out_fifo_full(&self) -> OUT_FIFO_FULL_R {
        OUT_FIFO_FULL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Write 1 to reset JTAG in fifo."]
    #[inline(always)]
    pub fn in_fifo_reset(&self) -> IN_FIFO_RESET_R {
        IN_FIFO_RESET_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Write 1 to reset JTAG out fifo."]
    #[inline(always)]
    pub fn out_fifo_reset(&self) -> OUT_FIFO_RESET_R {
        OUT_FIFO_RESET_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Write 1 to reset JTAG in fifo."]
    #[inline(always)]
    pub fn in_fifo_reset(&mut self) -> IN_FIFO_RESET_W {
        IN_FIFO_RESET_W { w: self }
    }
    #[doc = "Bit 9 - Write 1 to reset JTAG out fifo."]
    #[inline(always)]
    pub fn out_fifo_reset(&mut self) -> OUT_FIFO_RESET_W {
        OUT_FIFO_RESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB_DEVICE_JFIFO_ST_REG.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jfifo_st]
(index.html) module"]
pub struct JFIFO_ST_SPEC;
impl crate::RegisterSpec for JFIFO_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jfifo_st::R]
(R) reader structure"]
impl crate::Readable for JFIFO_ST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jfifo_st::W]
(W) writer structure"]
impl crate::Writable for JFIFO_ST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets JFIFO_ST to value 0x44"]
impl crate::Resettable for JFIFO_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x44
    }
}
