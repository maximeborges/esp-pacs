#[doc = "Register `SAR_COCPU_INT_CLR` writer"]
pub struct W(crate::W<SAR_COCPU_INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_COCPU_INT_CLR_SPEC>;
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
impl From<crate::W<SAR_COCPU_INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_COCPU_INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COCPU_TOUCH_DONE_INT_CLR` writer - TOUCH_DONE_INT interrupt clear bit"]
pub struct COCPU_TOUCH_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_TOUCH_DONE_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `COCPU_TOUCH_INACTIVE_INT_CLR` writer - TOUCH_INACTIVE_INT interrupt clear bit"]
pub struct COCPU_TOUCH_INACTIVE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_TOUCH_INACTIVE_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `COCPU_TOUCH_ACTIVE_INT_CLR` writer - TOUCH_ACTIVE_INT interrupt clear bit"]
pub struct COCPU_TOUCH_ACTIVE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_TOUCH_ACTIVE_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `COCPU_SARADC1_INT_CLR` writer - SARADC1_DONE_INT interrupt clear bit"]
pub struct COCPU_SARADC1_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_SARADC1_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `COCPU_SARADC2_INT_CLR` writer - SARADC2_DONE_INT interrupt clear bit"]
pub struct COCPU_SARADC2_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_SARADC2_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `COCPU_TSENS_INT_CLR` writer - TSENS_DONE_INT interrupt clear bit"]
pub struct COCPU_TSENS_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_TSENS_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `COCPU_START_INT_CLR` writer - RISCV_START_INT interrupt clear bit"]
pub struct COCPU_START_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_START_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `COCPU_SW_INT_CLR` writer - SW_INT interrupt clear bit"]
pub struct COCPU_SW_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_SW_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `COCPU_SWD_INT_CLR` writer - SWD_INT interrupt clear bit"]
pub struct COCPU_SWD_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_SWD_INT_CLR_W<'a> {
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
impl W {
    #[doc = "Bit 0 - TOUCH_DONE_INT interrupt clear bit"]
    #[inline(always)]
    pub fn cocpu_touch_done_int_clr(&mut self) -> COCPU_TOUCH_DONE_INT_CLR_W {
        COCPU_TOUCH_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1 - TOUCH_INACTIVE_INT interrupt clear bit"]
    #[inline(always)]
    pub fn cocpu_touch_inactive_int_clr(&mut self) -> COCPU_TOUCH_INACTIVE_INT_CLR_W {
        COCPU_TOUCH_INACTIVE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2 - TOUCH_ACTIVE_INT interrupt clear bit"]
    #[inline(always)]
    pub fn cocpu_touch_active_int_clr(&mut self) -> COCPU_TOUCH_ACTIVE_INT_CLR_W {
        COCPU_TOUCH_ACTIVE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3 - SARADC1_DONE_INT interrupt clear bit"]
    #[inline(always)]
    pub fn cocpu_saradc1_int_clr(&mut self) -> COCPU_SARADC1_INT_CLR_W {
        COCPU_SARADC1_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4 - SARADC2_DONE_INT interrupt clear bit"]
    #[inline(always)]
    pub fn cocpu_saradc2_int_clr(&mut self) -> COCPU_SARADC2_INT_CLR_W {
        COCPU_SARADC2_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5 - TSENS_DONE_INT interrupt clear bit"]
    #[inline(always)]
    pub fn cocpu_tsens_int_clr(&mut self) -> COCPU_TSENS_INT_CLR_W {
        COCPU_TSENS_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6 - RISCV_START_INT interrupt clear bit"]
    #[inline(always)]
    pub fn cocpu_start_int_clr(&mut self) -> COCPU_START_INT_CLR_W {
        COCPU_START_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7 - SW_INT interrupt clear bit"]
    #[inline(always)]
    pub fn cocpu_sw_int_clr(&mut self) -> COCPU_SW_INT_CLR_W {
        COCPU_SW_INT_CLR_W { w: self }
    }
    #[doc = "Bit 8 - SWD_INT interrupt clear bit"]
    #[inline(always)]
    pub fn cocpu_swd_int_clr(&mut self) -> COCPU_SWD_INT_CLR_W {
        COCPU_SWD_INT_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear bit of ULP-RISCV\n\nThis register you can [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_cocpu_int_clr]
(index.html) module"]
pub struct SAR_COCPU_INT_CLR_SPEC;
impl crate::RegisterSpec for SAR_COCPU_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sar_cocpu_int_clr::W]
(W) writer structure"]
impl crate::Writable for SAR_COCPU_INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_COCPU_INT_CLR to value 0"]
impl crate::Resettable for SAR_COCPU_INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
