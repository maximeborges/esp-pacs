#[doc = "Register `TSENS_CTRL2` reader"]
pub struct R(crate::R<TSENS_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSENS_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSENS_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSENS_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSENS_CTRL2` writer"]
pub struct W(crate::W<TSENS_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSENS_CTRL2_SPEC>;
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
impl From<crate::W<TSENS_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSENS_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSENS_XPD_WAIT` reader - the time that power up tsens need wait"]
pub struct TSENS_XPD_WAIT_R(crate::FieldReader<u16, u16>);
impl TSENS_XPD_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TSENS_XPD_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENS_XPD_WAIT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENS_XPD_WAIT` writer - the time that power up tsens need wait"]
pub struct TSENS_XPD_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_XPD_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `TSENS_XPD_FORCE` reader - force power up tsens"]
pub struct TSENS_XPD_FORCE_R(crate::FieldReader<u8, u8>);
impl TSENS_XPD_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TSENS_XPD_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENS_XPD_FORCE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENS_XPD_FORCE` writer - force power up tsens"]
pub struct TSENS_XPD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_XPD_FORCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 12)) | ((value as u32 & 3) << 12);
        self.w
    }
}
#[doc = "Field `TSENS_CLK_INV` reader - inv tsens clk"]
pub struct TSENS_CLK_INV_R(crate::FieldReader<bool, bool>);
impl TSENS_CLK_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSENS_CLK_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENS_CLK_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENS_CLK_INV` writer - inv tsens clk"]
pub struct TSENS_CLK_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_CLK_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u32 & 1) << 14);
        self.w
    }
}
#[doc = "Field `TSENS_CLK_SEL` reader - tsens clk select"]
pub struct TSENS_CLK_SEL_R(crate::FieldReader<bool, bool>);
impl TSENS_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSENS_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENS_CLK_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENS_CLK_SEL` writer - tsens clk select"]
pub struct TSENS_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_CLK_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - the time that power up tsens need wait"]
    #[inline(always)]
    pub fn tsens_xpd_wait(&self) -> TSENS_XPD_WAIT_R {
        TSENS_XPD_WAIT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13 - force power up tsens"]
    #[inline(always)]
    pub fn tsens_xpd_force(&self) -> TSENS_XPD_FORCE_R {
        TSENS_XPD_FORCE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - inv tsens clk"]
    #[inline(always)]
    pub fn tsens_clk_inv(&self) -> TSENS_CLK_INV_R {
        TSENS_CLK_INV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - tsens clk select"]
    #[inline(always)]
    pub fn tsens_clk_sel(&self) -> TSENS_CLK_SEL_R {
        TSENS_CLK_SEL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - the time that power up tsens need wait"]
    #[inline(always)]
    pub fn tsens_xpd_wait(&mut self) -> TSENS_XPD_WAIT_W {
        TSENS_XPD_WAIT_W { w: self }
    }
    #[doc = "Bits 12:13 - force power up tsens"]
    #[inline(always)]
    pub fn tsens_xpd_force(&mut self) -> TSENS_XPD_FORCE_W {
        TSENS_XPD_FORCE_W { w: self }
    }
    #[doc = "Bit 14 - inv tsens clk"]
    #[inline(always)]
    pub fn tsens_clk_inv(&mut self) -> TSENS_CLK_INV_W {
        TSENS_CLK_INV_W { w: self }
    }
    #[doc = "Bit 15 - tsens clk select"]
    #[inline(always)]
    pub fn tsens_clk_sel(&mut self) -> TSENS_CLK_SEL_W {
        TSENS_CLK_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "digital tsens configure register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsens_ctrl2]
(index.html) module"]
pub struct TSENS_CTRL2_SPEC;
impl crate::RegisterSpec for TSENS_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsens_ctrl2::R]
(R) reader structure"]
impl crate::Readable for TSENS_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsens_ctrl2::W]
(W) writer structure"]
impl crate::Writable for TSENS_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSENS_CTRL2 to value 0x4002"]
impl crate::Resettable for TSENS_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4002
    }
}
