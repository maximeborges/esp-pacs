#[doc = "Register `LSCH0_CONF0` reader"]
pub struct R(crate::R<LSCH0_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSCH0_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSCH0_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSCH0_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LSCH0_CONF0` writer"]
pub struct W(crate::W<LSCH0_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LSCH0_CONF0_SPEC>;
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
impl From<crate::W<LSCH0_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LSCH0_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_SEL_LSCH0` reader - reg_timer_sel_lsch0."]
pub struct TIMER_SEL_LSCH0_R(crate::FieldReader<u8, u8>);
impl TIMER_SEL_LSCH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER_SEL_LSCH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_SEL_LSCH0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER_SEL_LSCH0` writer - reg_timer_sel_lsch0."]
pub struct TIMER_SEL_LSCH0_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_SEL_LSCH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `SIG_OUT_EN_LSCH0` reader - reg_sig_out_en_lsch0."]
pub struct SIG_OUT_EN_LSCH0_R(crate::FieldReader<bool, bool>);
impl SIG_OUT_EN_LSCH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIG_OUT_EN_LSCH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIG_OUT_EN_LSCH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIG_OUT_EN_LSCH0` writer - reg_sig_out_en_lsch0."]
pub struct SIG_OUT_EN_LSCH0_W<'a> {
    w: &'a mut W,
}
impl<'a> SIG_OUT_EN_LSCH0_W<'a> {
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
#[doc = "Field `IDLE_LV_LSCH0` reader - reg_idle_lv_lsch0."]
pub struct IDLE_LV_LSCH0_R(crate::FieldReader<bool, bool>);
impl IDLE_LV_LSCH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IDLE_LV_LSCH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLE_LV_LSCH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLE_LV_LSCH0` writer - reg_idle_lv_lsch0."]
pub struct IDLE_LV_LSCH0_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_LV_LSCH0_W<'a> {
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
#[doc = "Field `PARA_UP_LSCH0` writer - reg_para_up_lsch0."]
pub struct PARA_UP_LSCH0_W<'a> {
    w: &'a mut W,
}
impl<'a> PARA_UP_LSCH0_W<'a> {
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
#[doc = "Field `OVF_NUM_LSCH0` reader - reg_ovf_num_lsch0."]
pub struct OVF_NUM_LSCH0_R(crate::FieldReader<u16, u16>);
impl OVF_NUM_LSCH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        OVF_NUM_LSCH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVF_NUM_LSCH0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVF_NUM_LSCH0` writer - reg_ovf_num_lsch0."]
pub struct OVF_NUM_LSCH0_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_NUM_LSCH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 5)) | ((value as u32 & 0x03ff) << 5);
        self.w
    }
}
#[doc = "Field `OVF_CNT_EN_LSCH0` reader - reg_ovf_cnt_en_lsch0."]
pub struct OVF_CNT_EN_LSCH0_R(crate::FieldReader<bool, bool>);
impl OVF_CNT_EN_LSCH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVF_CNT_EN_LSCH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVF_CNT_EN_LSCH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVF_CNT_EN_LSCH0` writer - reg_ovf_cnt_en_lsch0."]
pub struct OVF_CNT_EN_LSCH0_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_EN_LSCH0_W<'a> {
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
#[doc = "Field `OVF_CNT_RESET_LSCH0` writer - reg_ovf_cnt_reset_lsch0."]
pub struct OVF_CNT_RESET_LSCH0_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_RESET_LSCH0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - reg_timer_sel_lsch0."]
    #[inline(always)]
    pub fn timer_sel_lsch0(&self) -> TIMER_SEL_LSCH0_R {
        TIMER_SEL_LSCH0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - reg_sig_out_en_lsch0."]
    #[inline(always)]
    pub fn sig_out_en_lsch0(&self) -> SIG_OUT_EN_LSCH0_R {
        SIG_OUT_EN_LSCH0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - reg_idle_lv_lsch0."]
    #[inline(always)]
    pub fn idle_lv_lsch0(&self) -> IDLE_LV_LSCH0_R {
        IDLE_LV_LSCH0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 5:14 - reg_ovf_num_lsch0."]
    #[inline(always)]
    pub fn ovf_num_lsch0(&self) -> OVF_NUM_LSCH0_R {
        OVF_NUM_LSCH0_R::new(((self.bits >> 5) & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - reg_ovf_cnt_en_lsch0."]
    #[inline(always)]
    pub fn ovf_cnt_en_lsch0(&self) -> OVF_CNT_EN_LSCH0_R {
        OVF_CNT_EN_LSCH0_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - reg_timer_sel_lsch0."]
    #[inline(always)]
    pub fn timer_sel_lsch0(&mut self) -> TIMER_SEL_LSCH0_W {
        TIMER_SEL_LSCH0_W { w: self }
    }
    #[doc = "Bit 2 - reg_sig_out_en_lsch0."]
    #[inline(always)]
    pub fn sig_out_en_lsch0(&mut self) -> SIG_OUT_EN_LSCH0_W {
        SIG_OUT_EN_LSCH0_W { w: self }
    }
    #[doc = "Bit 3 - reg_idle_lv_lsch0."]
    #[inline(always)]
    pub fn idle_lv_lsch0(&mut self) -> IDLE_LV_LSCH0_W {
        IDLE_LV_LSCH0_W { w: self }
    }
    #[doc = "Bit 4 - reg_para_up_lsch0."]
    #[inline(always)]
    pub fn para_up_lsch0(&mut self) -> PARA_UP_LSCH0_W {
        PARA_UP_LSCH0_W { w: self }
    }
    #[doc = "Bits 5:14 - reg_ovf_num_lsch0."]
    #[inline(always)]
    pub fn ovf_num_lsch0(&mut self) -> OVF_NUM_LSCH0_W {
        OVF_NUM_LSCH0_W { w: self }
    }
    #[doc = "Bit 15 - reg_ovf_cnt_en_lsch0."]
    #[inline(always)]
    pub fn ovf_cnt_en_lsch0(&mut self) -> OVF_CNT_EN_LSCH0_W {
        OVF_CNT_EN_LSCH0_W { w: self }
    }
    #[doc = "Bit 16 - reg_ovf_cnt_reset_lsch0."]
    #[inline(always)]
    pub fn ovf_cnt_reset_lsch0(&mut self) -> OVF_CNT_RESET_LSCH0_W {
        OVF_CNT_RESET_LSCH0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LEDC_LSCH0_CONF0.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch0_conf0]
(index.html) module"]
pub struct LSCH0_CONF0_SPEC;
impl crate::RegisterSpec for LSCH0_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lsch0_conf0::R]
(R) reader structure"]
impl crate::Readable for LSCH0_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lsch0_conf0::W]
(W) writer structure"]
impl crate::Writable for LSCH0_CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LSCH0_CONF0 to value 0"]
impl crate::Resettable for LSCH0_CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
