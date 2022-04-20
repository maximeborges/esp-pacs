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
#[doc = "Field `RXFIFO_WM_INT_CLR` writer - reg_rxfifo_wm_int_clr"]
pub struct RXFIFO_WM_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_WM_INT_CLR_W<'a> {
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
#[doc = "Field `TXFIFO_WM_INT_CLR` writer - reg_txfifo_wm_int_clr"]
pub struct TXFIFO_WM_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_WM_INT_CLR_W<'a> {
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
#[doc = "Field `RXFIFO_OVF_INT_CLR` writer - reg_rxfifo_ovf_int_clr"]
pub struct RXFIFO_OVF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_OVF_INT_CLR_W<'a> {
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
#[doc = "Field `END_DETECT_INT_CLR` writer - reg_end_detect_int_clr"]
pub struct END_DETECT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> END_DETECT_INT_CLR_W<'a> {
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
#[doc = "Field `BYTE_TRANS_DONE_INT_CLR` writer - reg_byte_trans_done_int_clr"]
pub struct BYTE_TRANS_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE_TRANS_DONE_INT_CLR_W<'a> {
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
#[doc = "Field `ARBITRATION_LOST_INT_CLR` writer - reg_arbitration_lost_int_clr"]
pub struct ARBITRATION_LOST_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBITRATION_LOST_INT_CLR_W<'a> {
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
#[doc = "Field `MST_TXFIFO_UDF_INT_CLR` writer - reg_mst_txfifo_udf_int_clr"]
pub struct MST_TXFIFO_UDF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> MST_TXFIFO_UDF_INT_CLR_W<'a> {
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
#[doc = "Field `TRANS_COMPLETE_INT_CLR` writer - reg_trans_complete_int_clr"]
pub struct TRANS_COMPLETE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_COMPLETE_INT_CLR_W<'a> {
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
#[doc = "Field `TIME_OUT_INT_CLR` writer - reg_time_out_int_clr"]
pub struct TIME_OUT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIME_OUT_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `TRANS_START_INT_CLR` writer - reg_trans_start_int_clr"]
pub struct TRANS_START_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_START_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "Field `NACK_INT_CLR` writer - reg_nack_int_clr"]
pub struct NACK_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> NACK_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "Field `TXFIFO_OVF_INT_CLR` writer - reg_txfifo_ovf_int_clr"]
pub struct TXFIFO_OVF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_OVF_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Field `RXFIFO_UDF_INT_CLR` writer - reg_rxfifo_udf_int_clr"]
pub struct RXFIFO_UDF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_UDF_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
#[doc = "Field `SCL_ST_TO_INT_CLR` writer - reg_scl_st_to_int_clr"]
pub struct SCL_ST_TO_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_ST_TO_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `SCL_MAIN_ST_TO_INT_CLR` writer - reg_scl_main_st_to_int_clr"]
pub struct SCL_MAIN_ST_TO_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_MAIN_ST_TO_INT_CLR_W<'a> {
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
#[doc = "Field `DET_START_INT_CLR` writer - reg_det_start_int_clr"]
pub struct DET_START_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DET_START_INT_CLR_W<'a> {
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
#[doc = "Field `SLAVE_STRETCH_INT_CLR` writer - reg_slave_stretch_int_clr"]
pub struct SLAVE_STRETCH_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_STRETCH_INT_CLR_W<'a> {
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
#[doc = "Field `GENERAL_CALL_INT_CLR` writer - reg_general_call_int_clr"]
pub struct GENERAL_CALL_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GENERAL_CALL_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u32 & 1) << 17);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - reg_rxfifo_wm_int_clr"]
    #[inline(always)]
    pub fn rxfifo_wm_int_clr(&mut self) -> RXFIFO_WM_INT_CLR_W {
        RXFIFO_WM_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1 - reg_txfifo_wm_int_clr"]
    #[inline(always)]
    pub fn txfifo_wm_int_clr(&mut self) -> TXFIFO_WM_INT_CLR_W {
        TXFIFO_WM_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2 - reg_rxfifo_ovf_int_clr"]
    #[inline(always)]
    pub fn rxfifo_ovf_int_clr(&mut self) -> RXFIFO_OVF_INT_CLR_W {
        RXFIFO_OVF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3 - reg_end_detect_int_clr"]
    #[inline(always)]
    pub fn end_detect_int_clr(&mut self) -> END_DETECT_INT_CLR_W {
        END_DETECT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4 - reg_byte_trans_done_int_clr"]
    #[inline(always)]
    pub fn byte_trans_done_int_clr(&mut self) -> BYTE_TRANS_DONE_INT_CLR_W {
        BYTE_TRANS_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5 - reg_arbitration_lost_int_clr"]
    #[inline(always)]
    pub fn arbitration_lost_int_clr(&mut self) -> ARBITRATION_LOST_INT_CLR_W {
        ARBITRATION_LOST_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6 - reg_mst_txfifo_udf_int_clr"]
    #[inline(always)]
    pub fn mst_txfifo_udf_int_clr(&mut self) -> MST_TXFIFO_UDF_INT_CLR_W {
        MST_TXFIFO_UDF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7 - reg_trans_complete_int_clr"]
    #[inline(always)]
    pub fn trans_complete_int_clr(&mut self) -> TRANS_COMPLETE_INT_CLR_W {
        TRANS_COMPLETE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 8 - reg_time_out_int_clr"]
    #[inline(always)]
    pub fn time_out_int_clr(&mut self) -> TIME_OUT_INT_CLR_W {
        TIME_OUT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 9 - reg_trans_start_int_clr"]
    #[inline(always)]
    pub fn trans_start_int_clr(&mut self) -> TRANS_START_INT_CLR_W {
        TRANS_START_INT_CLR_W { w: self }
    }
    #[doc = "Bit 10 - reg_nack_int_clr"]
    #[inline(always)]
    pub fn nack_int_clr(&mut self) -> NACK_INT_CLR_W {
        NACK_INT_CLR_W { w: self }
    }
    #[doc = "Bit 11 - reg_txfifo_ovf_int_clr"]
    #[inline(always)]
    pub fn txfifo_ovf_int_clr(&mut self) -> TXFIFO_OVF_INT_CLR_W {
        TXFIFO_OVF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 12 - reg_rxfifo_udf_int_clr"]
    #[inline(always)]
    pub fn rxfifo_udf_int_clr(&mut self) -> RXFIFO_UDF_INT_CLR_W {
        RXFIFO_UDF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 13 - reg_scl_st_to_int_clr"]
    #[inline(always)]
    pub fn scl_st_to_int_clr(&mut self) -> SCL_ST_TO_INT_CLR_W {
        SCL_ST_TO_INT_CLR_W { w: self }
    }
    #[doc = "Bit 14 - reg_scl_main_st_to_int_clr"]
    #[inline(always)]
    pub fn scl_main_st_to_int_clr(&mut self) -> SCL_MAIN_ST_TO_INT_CLR_W {
        SCL_MAIN_ST_TO_INT_CLR_W { w: self }
    }
    #[doc = "Bit 15 - reg_det_start_int_clr"]
    #[inline(always)]
    pub fn det_start_int_clr(&mut self) -> DET_START_INT_CLR_W {
        DET_START_INT_CLR_W { w: self }
    }
    #[doc = "Bit 16 - reg_slave_stretch_int_clr"]
    #[inline(always)]
    pub fn slave_stretch_int_clr(&mut self) -> SLAVE_STRETCH_INT_CLR_W {
        SLAVE_STRETCH_INT_CLR_W { w: self }
    }
    #[doc = "Bit 17 - reg_general_call_int_clr"]
    #[inline(always)]
    pub fn general_call_int_clr(&mut self) -> GENERAL_CALL_INT_CLR_W {
        GENERAL_CALL_INT_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C_INT_CLR_REG\n\nThis register you can [`write_with_zero`]
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
