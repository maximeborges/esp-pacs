#[doc = "Register `COMB_PVT_HVT_CONF` reader"]
pub struct R(crate::R<COMB_PVT_HVT_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMB_PVT_HVT_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMB_PVT_HVT_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMB_PVT_HVT_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMB_PVT_HVT_CONF` writer"]
pub struct W(crate::W<COMB_PVT_HVT_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMB_PVT_HVT_CONF_SPEC>;
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
impl From<crate::W<COMB_PVT_HVT_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMB_PVT_HVT_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMB_PATH_LEN_HVT` reader - reg_comb_path_len_hvt"]
pub struct COMB_PATH_LEN_HVT_R(crate::FieldReader<u8, u8>);
impl COMB_PATH_LEN_HVT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COMB_PATH_LEN_HVT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMB_PATH_LEN_HVT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMB_PATH_LEN_HVT` writer - reg_comb_path_len_hvt"]
pub struct COMB_PATH_LEN_HVT_W<'a> {
    w: &'a mut W,
}
impl<'a> COMB_PATH_LEN_HVT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `COMB_ERR_CNT_CLR_HVT` writer - reg_comb_err_cnt_clr_hvt"]
pub struct COMB_ERR_CNT_CLR_HVT_W<'a> {
    w: &'a mut W,
}
impl<'a> COMB_ERR_CNT_CLR_HVT_W<'a> {
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
#[doc = "Field `COMB_PVT_MONITOR_EN_HVT` reader - reg_comb_pvt_monitor_en_hvt"]
pub struct COMB_PVT_MONITOR_EN_HVT_R(crate::FieldReader<bool, bool>);
impl COMB_PVT_MONITOR_EN_HVT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMB_PVT_MONITOR_EN_HVT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMB_PVT_MONITOR_EN_HVT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMB_PVT_MONITOR_EN_HVT` writer - reg_comb_pvt_monitor_en_hvt"]
pub struct COMB_PVT_MONITOR_EN_HVT_W<'a> {
    w: &'a mut W,
}
impl<'a> COMB_PVT_MONITOR_EN_HVT_W<'a> {
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
impl R {
    #[doc = "Bits 0:4 - reg_comb_path_len_hvt"]
    #[inline(always)]
    pub fn comb_path_len_hvt(&self) -> COMB_PATH_LEN_HVT_R {
        COMB_PATH_LEN_HVT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - reg_comb_pvt_monitor_en_hvt"]
    #[inline(always)]
    pub fn comb_pvt_monitor_en_hvt(&self) -> COMB_PVT_MONITOR_EN_HVT_R {
        COMB_PVT_MONITOR_EN_HVT_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - reg_comb_path_len_hvt"]
    #[inline(always)]
    pub fn comb_path_len_hvt(&mut self) -> COMB_PATH_LEN_HVT_W {
        COMB_PATH_LEN_HVT_W { w: self }
    }
    #[doc = "Bit 5 - reg_comb_err_cnt_clr_hvt"]
    #[inline(always)]
    pub fn comb_err_cnt_clr_hvt(&mut self) -> COMB_ERR_CNT_CLR_HVT_W {
        COMB_ERR_CNT_CLR_HVT_W { w: self }
    }
    #[doc = "Bit 6 - reg_comb_pvt_monitor_en_hvt"]
    #[inline(always)]
    pub fn comb_pvt_monitor_en_hvt(&mut self) -> COMB_PVT_MONITOR_EN_HVT_W {
        COMB_PVT_MONITOR_EN_HVT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mem pvt register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comb_pvt_hvt_conf]
(index.html) module"]
pub struct COMB_PVT_HVT_CONF_SPEC;
impl crate::RegisterSpec for COMB_PVT_HVT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comb_pvt_hvt_conf::R]
(R) reader structure"]
impl crate::Readable for COMB_PVT_HVT_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comb_pvt_hvt_conf::W]
(W) writer structure"]
impl crate::Writable for COMB_PVT_HVT_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMB_PVT_HVT_CONF to value 0x03"]
impl crate::Resettable for COMB_PVT_HVT_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
