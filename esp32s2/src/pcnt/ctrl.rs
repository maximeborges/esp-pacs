#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PULSE_CNT_RST_U0` reader - Set this bit to clear unit 0's counter."]
pub struct PULSE_CNT_RST_U0_R(crate::FieldReader<bool>);
impl PULSE_CNT_RST_U0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PULSE_CNT_RST_U0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULSE_CNT_RST_U0_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULSE_CNT_RST_U0` writer - Set this bit to clear unit 0's counter."]
pub struct PULSE_CNT_RST_U0_W<'a> {
    w: &'a mut W,
}
impl<'a> PULSE_CNT_RST_U0_W<'a> {
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
#[doc = "Field `CNT_PAUSE_U0` reader - Set this bit to freeze unit 1's counter."]
pub struct CNT_PAUSE_U0_R(crate::FieldReader<bool>);
impl CNT_PAUSE_U0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CNT_PAUSE_U0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_PAUSE_U0_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT_PAUSE_U0` writer - Set this bit to freeze unit 1's counter."]
pub struct CNT_PAUSE_U0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_PAUSE_U0_W<'a> {
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
#[doc = "Field `PULSE_CNT_RST_U1` reader - Set this bit to clear unit 2's counter."]
pub struct PULSE_CNT_RST_U1_R(crate::FieldReader<bool>);
impl PULSE_CNT_RST_U1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PULSE_CNT_RST_U1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULSE_CNT_RST_U1_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULSE_CNT_RST_U1` writer - Set this bit to clear unit 2's counter."]
pub struct PULSE_CNT_RST_U1_W<'a> {
    w: &'a mut W,
}
impl<'a> PULSE_CNT_RST_U1_W<'a> {
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
#[doc = "Field `CNT_PAUSE_U1` reader - Set this bit to freeze unit 3's counter."]
pub struct CNT_PAUSE_U1_R(crate::FieldReader<bool>);
impl CNT_PAUSE_U1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CNT_PAUSE_U1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_PAUSE_U1_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT_PAUSE_U1` writer - Set this bit to freeze unit 3's counter."]
pub struct CNT_PAUSE_U1_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_PAUSE_U1_W<'a> {
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
#[doc = "Field `PULSE_CNT_RST_U2` reader - Set this bit to clear unit 4's counter."]
pub struct PULSE_CNT_RST_U2_R(crate::FieldReader<bool>);
impl PULSE_CNT_RST_U2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PULSE_CNT_RST_U2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULSE_CNT_RST_U2_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULSE_CNT_RST_U2` writer - Set this bit to clear unit 4's counter."]
pub struct PULSE_CNT_RST_U2_W<'a> {
    w: &'a mut W,
}
impl<'a> PULSE_CNT_RST_U2_W<'a> {
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
#[doc = "Field `CNT_PAUSE_U2` reader - Set this bit to freeze unit 5's counter."]
pub struct CNT_PAUSE_U2_R(crate::FieldReader<bool>);
impl CNT_PAUSE_U2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CNT_PAUSE_U2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_PAUSE_U2_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT_PAUSE_U2` writer - Set this bit to freeze unit 5's counter."]
pub struct CNT_PAUSE_U2_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_PAUSE_U2_W<'a> {
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
#[doc = "Field `PULSE_CNT_RST_U3` reader - Set this bit to clear unit 6's counter."]
pub struct PULSE_CNT_RST_U3_R(crate::FieldReader<bool>);
impl PULSE_CNT_RST_U3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PULSE_CNT_RST_U3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULSE_CNT_RST_U3_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULSE_CNT_RST_U3` writer - Set this bit to clear unit 6's counter."]
pub struct PULSE_CNT_RST_U3_W<'a> {
    w: &'a mut W,
}
impl<'a> PULSE_CNT_RST_U3_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `CNT_PAUSE_U3` reader - Set this bit to freeze unit 7's counter."]
pub struct CNT_PAUSE_U3_R(crate::FieldReader<bool>);
impl CNT_PAUSE_U3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CNT_PAUSE_U3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_PAUSE_U3_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT_PAUSE_U3` writer - Set this bit to freeze unit 7's counter."]
pub struct CNT_PAUSE_U3_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_PAUSE_U3_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `CLK_EN` reader - The registers clock gate enable signal of PCNT module. 1: the registers can be read and written by application. 0: the registers can not be read or written by application"]
pub struct CLK_EN_R(crate::FieldReader<bool>);
impl CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_EN` writer - The registers clock gate enable signal of PCNT module. 1: the registers can be read and written by application. 0: the registers can not be read or written by application"]
pub struct CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set this bit to clear unit 0's counter."]
    #[inline(always)]
    pub fn pulse_cnt_rst_u0(&self) -> PULSE_CNT_RST_U0_R {
        PULSE_CNT_RST_U0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to freeze unit 1's counter."]
    #[inline(always)]
    pub fn cnt_pause_u0(&self) -> CNT_PAUSE_U0_R {
        CNT_PAUSE_U0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to clear unit 2's counter."]
    #[inline(always)]
    pub fn pulse_cnt_rst_u1(&self) -> PULSE_CNT_RST_U1_R {
        PULSE_CNT_RST_U1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to freeze unit 3's counter."]
    #[inline(always)]
    pub fn cnt_pause_u1(&self) -> CNT_PAUSE_U1_R {
        CNT_PAUSE_U1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to clear unit 4's counter."]
    #[inline(always)]
    pub fn pulse_cnt_rst_u2(&self) -> PULSE_CNT_RST_U2_R {
        PULSE_CNT_RST_U2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to freeze unit 5's counter."]
    #[inline(always)]
    pub fn cnt_pause_u2(&self) -> CNT_PAUSE_U2_R {
        CNT_PAUSE_U2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to clear unit 6's counter."]
    #[inline(always)]
    pub fn pulse_cnt_rst_u3(&self) -> PULSE_CNT_RST_U3_R {
        PULSE_CNT_RST_U3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to freeze unit 7's counter."]
    #[inline(always)]
    pub fn cnt_pause_u3(&self) -> CNT_PAUSE_U3_R {
        CNT_PAUSE_U3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - The registers clock gate enable signal of PCNT module. 1: the registers can be read and written by application. 0: the registers can not be read or written by application"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear unit 0's counter."]
    #[inline(always)]
    pub fn pulse_cnt_rst_u0(&mut self) -> PULSE_CNT_RST_U0_W {
        PULSE_CNT_RST_U0_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to freeze unit 1's counter."]
    #[inline(always)]
    pub fn cnt_pause_u0(&mut self) -> CNT_PAUSE_U0_W {
        CNT_PAUSE_U0_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to clear unit 2's counter."]
    #[inline(always)]
    pub fn pulse_cnt_rst_u1(&mut self) -> PULSE_CNT_RST_U1_W {
        PULSE_CNT_RST_U1_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to freeze unit 3's counter."]
    #[inline(always)]
    pub fn cnt_pause_u1(&mut self) -> CNT_PAUSE_U1_W {
        CNT_PAUSE_U1_W { w: self }
    }
    #[doc = "Bit 4 - Set this bit to clear unit 4's counter."]
    #[inline(always)]
    pub fn pulse_cnt_rst_u2(&mut self) -> PULSE_CNT_RST_U2_W {
        PULSE_CNT_RST_U2_W { w: self }
    }
    #[doc = "Bit 5 - Set this bit to freeze unit 5's counter."]
    #[inline(always)]
    pub fn cnt_pause_u2(&mut self) -> CNT_PAUSE_U2_W {
        CNT_PAUSE_U2_W { w: self }
    }
    #[doc = "Bit 6 - Set this bit to clear unit 6's counter."]
    #[inline(always)]
    pub fn pulse_cnt_rst_u3(&mut self) -> PULSE_CNT_RST_U3_W {
        PULSE_CNT_RST_U3_W { w: self }
    }
    #[doc = "Bit 7 - Set this bit to freeze unit 7's counter."]
    #[inline(always)]
    pub fn cnt_pause_u3(&mut self) -> CNT_PAUSE_U3_W {
        CNT_PAUSE_U3_W { w: self }
    }
    #[doc = "Bit 16 - The registers clock gate enable signal of PCNT module. 1: the registers can be read and written by application. 0: the registers can not be read or written by application"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register for all counters\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0x55"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x55
    }
}
