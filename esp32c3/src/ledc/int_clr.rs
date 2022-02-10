#[doc = "Register `INT_CLR` writer"]
pub struct W(crate::W<INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_SPEC>;
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
impl From<crate::W<INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSTIMER0_OVF_INT_CLR` writer - reg_lstimer0_ovf_int_clr."]
pub struct LSTIMER0_OVF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER0_OVF_INT_CLR_W<'a> {
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
#[doc = "Field `LSTIMER1_OVF_INT_CLR` writer - reg_lstimer1_ovf_int_clr."]
pub struct LSTIMER1_OVF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER1_OVF_INT_CLR_W<'a> {
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
#[doc = "Field `LSTIMER2_OVF_INT_CLR` writer - reg_lstimer2_ovf_int_clr."]
pub struct LSTIMER2_OVF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER2_OVF_INT_CLR_W<'a> {
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
#[doc = "Field `LSTIMER3_OVF_INT_CLR` writer - reg_lstimer3_ovf_int_clr."]
pub struct LSTIMER3_OVF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER3_OVF_INT_CLR_W<'a> {
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
#[doc = "Field `DUTY_CHNG_END_LSCH0_INT_CLR` writer - reg_duty_chng_end_lsch0_int_clr."]
pub struct DUTY_CHNG_END_LSCH0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH0_INT_CLR_W<'a> {
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
#[doc = "Field `DUTY_CHNG_END_LSCH1_INT_CLR` writer - reg_duty_chng_end_lsch1_int_clr."]
pub struct DUTY_CHNG_END_LSCH1_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH1_INT_CLR_W<'a> {
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
#[doc = "Field `DUTY_CHNG_END_LSCH2_INT_CLR` writer - reg_duty_chng_end_lsch2_int_clr."]
pub struct DUTY_CHNG_END_LSCH2_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH2_INT_CLR_W<'a> {
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
#[doc = "Field `DUTY_CHNG_END_LSCH3_INT_CLR` writer - reg_duty_chng_end_lsch3_int_clr."]
pub struct DUTY_CHNG_END_LSCH3_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH3_INT_CLR_W<'a> {
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
#[doc = "Field `DUTY_CHNG_END_LSCH4_INT_CLR` writer - reg_duty_chng_end_lsch4_int_clr."]
pub struct DUTY_CHNG_END_LSCH4_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH4_INT_CLR_W<'a> {
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
#[doc = "Field `DUTY_CHNG_END_LSCH5_INT_CLR` writer - reg_duty_chng_end_lsch5_int_clr."]
pub struct DUTY_CHNG_END_LSCH5_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH5_INT_CLR_W<'a> {
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
#[doc = "Field `OVF_CNT_LSCH0_INT_CLR` writer - reg_ovf_cnt_lsch0_int_clr."]
pub struct OVF_CNT_LSCH0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_LSCH0_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `OVF_CNT_LSCH1_INT_CLR` writer - reg_ovf_cnt_lsch1_int_clr."]
pub struct OVF_CNT_LSCH1_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_LSCH1_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `OVF_CNT_LSCH2_INT_CLR` writer - reg_ovf_cnt_lsch2_int_clr."]
pub struct OVF_CNT_LSCH2_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_LSCH2_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `OVF_CNT_LSCH3_INT_CLR` writer - reg_ovf_cnt_lsch3_int_clr."]
pub struct OVF_CNT_LSCH3_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_LSCH3_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `OVF_CNT_LSCH4_INT_CLR` writer - reg_ovf_cnt_lsch4_int_clr."]
pub struct OVF_CNT_LSCH4_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_LSCH4_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `OVF_CNT_LSCH5_INT_CLR` writer - reg_ovf_cnt_lsch5_int_clr."]
pub struct OVF_CNT_LSCH5_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_LSCH5_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - reg_lstimer0_ovf_int_clr."]
    #[inline(always)]
    pub fn lstimer0_ovf_int_clr(&mut self) -> LSTIMER0_OVF_INT_CLR_W {
        LSTIMER0_OVF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1 - reg_lstimer1_ovf_int_clr."]
    #[inline(always)]
    pub fn lstimer1_ovf_int_clr(&mut self) -> LSTIMER1_OVF_INT_CLR_W {
        LSTIMER1_OVF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2 - reg_lstimer2_ovf_int_clr."]
    #[inline(always)]
    pub fn lstimer2_ovf_int_clr(&mut self) -> LSTIMER2_OVF_INT_CLR_W {
        LSTIMER2_OVF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3 - reg_lstimer3_ovf_int_clr."]
    #[inline(always)]
    pub fn lstimer3_ovf_int_clr(&mut self) -> LSTIMER3_OVF_INT_CLR_W {
        LSTIMER3_OVF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4 - reg_duty_chng_end_lsch0_int_clr."]
    #[inline(always)]
    pub fn duty_chng_end_lsch0_int_clr(&mut self) -> DUTY_CHNG_END_LSCH0_INT_CLR_W {
        DUTY_CHNG_END_LSCH0_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5 - reg_duty_chng_end_lsch1_int_clr."]
    #[inline(always)]
    pub fn duty_chng_end_lsch1_int_clr(&mut self) -> DUTY_CHNG_END_LSCH1_INT_CLR_W {
        DUTY_CHNG_END_LSCH1_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6 - reg_duty_chng_end_lsch2_int_clr."]
    #[inline(always)]
    pub fn duty_chng_end_lsch2_int_clr(&mut self) -> DUTY_CHNG_END_LSCH2_INT_CLR_W {
        DUTY_CHNG_END_LSCH2_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7 - reg_duty_chng_end_lsch3_int_clr."]
    #[inline(always)]
    pub fn duty_chng_end_lsch3_int_clr(&mut self) -> DUTY_CHNG_END_LSCH3_INT_CLR_W {
        DUTY_CHNG_END_LSCH3_INT_CLR_W { w: self }
    }
    #[doc = "Bit 8 - reg_duty_chng_end_lsch4_int_clr."]
    #[inline(always)]
    pub fn duty_chng_end_lsch4_int_clr(&mut self) -> DUTY_CHNG_END_LSCH4_INT_CLR_W {
        DUTY_CHNG_END_LSCH4_INT_CLR_W { w: self }
    }
    #[doc = "Bit 9 - reg_duty_chng_end_lsch5_int_clr."]
    #[inline(always)]
    pub fn duty_chng_end_lsch5_int_clr(&mut self) -> DUTY_CHNG_END_LSCH5_INT_CLR_W {
        DUTY_CHNG_END_LSCH5_INT_CLR_W { w: self }
    }
    #[doc = "Bit 10 - reg_ovf_cnt_lsch0_int_clr."]
    #[inline(always)]
    pub fn ovf_cnt_lsch0_int_clr(&mut self) -> OVF_CNT_LSCH0_INT_CLR_W {
        OVF_CNT_LSCH0_INT_CLR_W { w: self }
    }
    #[doc = "Bit 11 - reg_ovf_cnt_lsch1_int_clr."]
    #[inline(always)]
    pub fn ovf_cnt_lsch1_int_clr(&mut self) -> OVF_CNT_LSCH1_INT_CLR_W {
        OVF_CNT_LSCH1_INT_CLR_W { w: self }
    }
    #[doc = "Bit 12 - reg_ovf_cnt_lsch2_int_clr."]
    #[inline(always)]
    pub fn ovf_cnt_lsch2_int_clr(&mut self) -> OVF_CNT_LSCH2_INT_CLR_W {
        OVF_CNT_LSCH2_INT_CLR_W { w: self }
    }
    #[doc = "Bit 13 - reg_ovf_cnt_lsch3_int_clr."]
    #[inline(always)]
    pub fn ovf_cnt_lsch3_int_clr(&mut self) -> OVF_CNT_LSCH3_INT_CLR_W {
        OVF_CNT_LSCH3_INT_CLR_W { w: self }
    }
    #[doc = "Bit 14 - reg_ovf_cnt_lsch4_int_clr."]
    #[inline(always)]
    pub fn ovf_cnt_lsch4_int_clr(&mut self) -> OVF_CNT_LSCH4_INT_CLR_W {
        OVF_CNT_LSCH4_INT_CLR_W { w: self }
    }
    #[doc = "Bit 15 - reg_ovf_cnt_lsch5_int_clr."]
    #[inline(always)]
    pub fn ovf_cnt_lsch5_int_clr(&mut self) -> OVF_CNT_LSCH5_INT_CLR_W {
        OVF_CNT_LSCH5_INT_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LEDC_INT_CLR.\n\nThis register you can [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr]
(index.html) module"]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr::W]
(W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
