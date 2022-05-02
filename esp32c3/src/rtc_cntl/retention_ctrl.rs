#[doc = "Register `RETENTION_CTRL` reader"]
pub struct R(crate::R<RETENTION_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RETENTION_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RETENTION_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RETENTION_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RETENTION_CTRL` writer"]
pub struct W(crate::W<RETENTION_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RETENTION_CTRL_SPEC>;
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
impl From<crate::W<RETENTION_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RETENTION_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RETENTION_CLK_SEL` reader - Retention clk sel"]
pub struct RETENTION_CLK_SEL_R(crate::FieldReader<bool>);
impl RETENTION_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RETENTION_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RETENTION_CLK_SEL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RETENTION_CLK_SEL` writer - Retention clk sel"]
pub struct RETENTION_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RETENTION_CLK_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 18)) | ((value as u32 & 1) << 18);
        self.w
    }
}
#[doc = "Field `RETENTION_DONE_WAIT` reader - Retention done wait time"]
pub struct RETENTION_DONE_WAIT_R(crate::FieldReader<u8>);
impl RETENTION_DONE_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RETENTION_DONE_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RETENTION_DONE_WAIT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RETENTION_DONE_WAIT` writer - Retention done wait time"]
pub struct RETENTION_DONE_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RETENTION_DONE_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 19)) | ((value as u32 & 7) << 19);
        self.w
    }
}
#[doc = "Field `RETENTION_CLKOFF_WAIT` reader - Retention clkoff wait time"]
pub struct RETENTION_CLKOFF_WAIT_R(crate::FieldReader<u8>);
impl RETENTION_CLKOFF_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RETENTION_CLKOFF_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RETENTION_CLKOFF_WAIT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RETENTION_CLKOFF_WAIT` writer - Retention clkoff wait time"]
pub struct RETENTION_CLKOFF_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RETENTION_CLKOFF_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 22)) | ((value as u32 & 0x0f) << 22);
        self.w
    }
}
#[doc = "Field `RETENTION_EN` reader - enable cpu retention when light sleep"]
pub struct RETENTION_EN_R(crate::FieldReader<bool>);
impl RETENTION_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RETENTION_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RETENTION_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RETENTION_EN` writer - enable cpu retention when light sleep"]
pub struct RETENTION_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RETENTION_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 26)) | ((value as u32 & 1) << 26);
        self.w
    }
}
#[doc = "Field `RETENTION_WAIT` reader - wait cycles for rention operation"]
pub struct RETENTION_WAIT_R(crate::FieldReader<u8>);
impl RETENTION_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RETENTION_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RETENTION_WAIT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RETENTION_WAIT` writer - wait cycles for rention operation"]
pub struct RETENTION_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RETENTION_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | ((value as u32 & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 18 - Retention clk sel"]
    #[inline(always)]
    pub fn retention_clk_sel(&self) -> RETENTION_CLK_SEL_R {
        RETENTION_CLK_SEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - Retention done wait time"]
    #[inline(always)]
    pub fn retention_done_wait(&self) -> RETENTION_DONE_WAIT_R {
        RETENTION_DONE_WAIT_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:25 - Retention clkoff wait time"]
    #[inline(always)]
    pub fn retention_clkoff_wait(&self) -> RETENTION_CLKOFF_WAIT_R {
        RETENTION_CLKOFF_WAIT_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 26 - enable cpu retention when light sleep"]
    #[inline(always)]
    pub fn retention_en(&self) -> RETENTION_EN_R {
        RETENTION_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - wait cycles for rention operation"]
    #[inline(always)]
    pub fn retention_wait(&self) -> RETENTION_WAIT_R {
        RETENTION_WAIT_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 18 - Retention clk sel"]
    #[inline(always)]
    pub fn retention_clk_sel(&mut self) -> RETENTION_CLK_SEL_W {
        RETENTION_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 19:21 - Retention done wait time"]
    #[inline(always)]
    pub fn retention_done_wait(&mut self) -> RETENTION_DONE_WAIT_W {
        RETENTION_DONE_WAIT_W { w: self }
    }
    #[doc = "Bits 22:25 - Retention clkoff wait time"]
    #[inline(always)]
    pub fn retention_clkoff_wait(&mut self) -> RETENTION_CLKOFF_WAIT_W {
        RETENTION_CLKOFF_WAIT_W { w: self }
    }
    #[doc = "Bit 26 - enable cpu retention when light sleep"]
    #[inline(always)]
    pub fn retention_en(&mut self) -> RETENTION_EN_W {
        RETENTION_EN_W { w: self }
    }
    #[doc = "Bits 27:31 - wait cycles for rention operation"]
    #[inline(always)]
    pub fn retention_wait(&mut self) -> RETENTION_WAIT_W {
        RETENTION_WAIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [retention_ctrl](index.html) module"]
pub struct RETENTION_CTRL_SPEC;
impl crate::RegisterSpec for RETENTION_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [retention_ctrl::R](R) reader structure"]
impl crate::Readable for RETENTION_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [retention_ctrl::W](W) writer structure"]
impl crate::Writable for RETENTION_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RETENTION_CTRL to value 0xa0d0_0000"]
impl crate::Resettable for RETENTION_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa0d0_0000
    }
}
