#[doc = "Register `CH%sCONF0` reader"]
pub struct R(crate::R<CHCONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%sCONF0` writer"]
pub struct W(crate::W<CHCONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCONF0_SPEC>;
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
impl From<crate::W<CHCONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV_CNT_CH` reader - This register is used to configure the divider for clock of CHANNEL%s."]
pub struct DIV_CNT_CH_R(crate::FieldReader<u8, u8>);
impl DIV_CNT_CH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIV_CNT_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_CNT_CH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV_CNT_CH` writer - This register is used to configure the divider for clock of CHANNEL%s."]
pub struct DIV_CNT_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_CNT_CH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `IDLE_THRES_CH` reader - When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished."]
pub struct IDLE_THRES_CH_R(crate::FieldReader<u16, u16>);
impl IDLE_THRES_CH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        IDLE_THRES_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLE_THRES_CH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLE_THRES_CH` writer - When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished."]
pub struct IDLE_THRES_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_THRES_CH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 8)) | ((value as u32 & 0xffff) << 8);
        self.w
    }
}
#[doc = "Field `MEM_SIZE_CH` reader - This register is used to configure the maximum size of memory allocated to CHANNEL%s."]
pub struct MEM_SIZE_CH_R(crate::FieldReader<u8, u8>);
impl MEM_SIZE_CH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MEM_SIZE_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_SIZE_CH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_SIZE_CH` writer - This register is used to configure the maximum size of memory allocated to CHANNEL%s."]
pub struct MEM_SIZE_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_SIZE_CH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 24)) | ((value as u32 & 7) << 24);
        self.w
    }
}
#[doc = "Field `CARRIER_EFF_EN_CH` reader - 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1."]
pub struct CARRIER_EFF_EN_CH_R(crate::FieldReader<bool, bool>);
impl CARRIER_EFF_EN_CH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARRIER_EFF_EN_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARRIER_EFF_EN_CH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARRIER_EFF_EN_CH` writer - 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1."]
pub struct CARRIER_EFF_EN_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_EFF_EN_CH_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 27)) | ((value as u32 & 1) << 27);
        self.w
    }
}
#[doc = "Field `CARRIER_EN_CH` reader - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out."]
pub struct CARRIER_EN_CH_R(crate::FieldReader<bool, bool>);
impl CARRIER_EN_CH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARRIER_EN_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARRIER_EN_CH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARRIER_EN_CH` writer - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out."]
pub struct CARRIER_EN_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_EN_CH_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 28)) | ((value as u32 & 1) << 28);
        self.w
    }
}
#[doc = "Field `CARRIER_OUT_LV_CH` reader - This bit is used to configure the position of carrier wave for CHANNEL%s. 1'h0: add carrier wave on low level. 1'h1: add carrier wave on high level."]
pub struct CARRIER_OUT_LV_CH_R(crate::FieldReader<bool, bool>);
impl CARRIER_OUT_LV_CH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARRIER_OUT_LV_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARRIER_OUT_LV_CH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARRIER_OUT_LV_CH` writer - This bit is used to configure the position of carrier wave for CHANNEL%s. 1'h0: add carrier wave on low level. 1'h1: add carrier wave on high level."]
pub struct CARRIER_OUT_LV_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_OUT_LV_CH_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 29)) | ((value as u32 & 1) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - This register is used to configure the divider for clock of CHANNEL%s."]
    #[inline(always)]
    pub fn div_cnt_ch(&self) -> DIV_CNT_CH_R {
        DIV_CNT_CH_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished."]
    #[inline(always)]
    pub fn idle_thres_ch(&self) -> IDLE_THRES_CH_R {
        IDLE_THRES_CH_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:26 - This register is used to configure the maximum size of memory allocated to CHANNEL%s."]
    #[inline(always)]
    pub fn mem_size_ch(&self) -> MEM_SIZE_CH_R {
        MEM_SIZE_CH_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1."]
    #[inline(always)]
    pub fn carrier_eff_en_ch(&self) -> CARRIER_EFF_EN_CH_R {
        CARRIER_EFF_EN_CH_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out."]
    #[inline(always)]
    pub fn carrier_en_ch(&self) -> CARRIER_EN_CH_R {
        CARRIER_EN_CH_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit is used to configure the position of carrier wave for CHANNEL%s. 1'h0: add carrier wave on low level. 1'h1: add carrier wave on high level."]
    #[inline(always)]
    pub fn carrier_out_lv_ch(&self) -> CARRIER_OUT_LV_CH_R {
        CARRIER_OUT_LV_CH_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is used to configure the divider for clock of CHANNEL%s."]
    #[inline(always)]
    pub fn div_cnt_ch(&mut self) -> DIV_CNT_CH_W {
        DIV_CNT_CH_W { w: self }
    }
    #[doc = "Bits 8:23 - When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished."]
    #[inline(always)]
    pub fn idle_thres_ch(&mut self) -> IDLE_THRES_CH_W {
        IDLE_THRES_CH_W { w: self }
    }
    #[doc = "Bits 24:26 - This register is used to configure the maximum size of memory allocated to CHANNEL%s."]
    #[inline(always)]
    pub fn mem_size_ch(&mut self) -> MEM_SIZE_CH_W {
        MEM_SIZE_CH_W { w: self }
    }
    #[doc = "Bit 27 - 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1."]
    #[inline(always)]
    pub fn carrier_eff_en_ch(&mut self) -> CARRIER_EFF_EN_CH_W {
        CARRIER_EFF_EN_CH_W { w: self }
    }
    #[doc = "Bit 28 - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out."]
    #[inline(always)]
    pub fn carrier_en_ch(&mut self) -> CARRIER_EN_CH_W {
        CARRIER_EN_CH_W { w: self }
    }
    #[doc = "Bit 29 - This bit is used to configure the position of carrier wave for CHANNEL%s. 1'h0: add carrier wave on low level. 1'h1: add carrier wave on high level."]
    #[inline(always)]
    pub fn carrier_out_lv_ch(&mut self) -> CARRIER_OUT_LV_CH_W {
        CARRIER_OUT_LV_CH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel %s configure register 0\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chconf0]
(index.html) module"]
pub struct CHCONF0_SPEC;
impl crate::RegisterSpec for CHCONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chconf0::R]
(R) reader structure"]
impl crate::Readable for CHCONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chconf0::W]
(W) writer structure"]
impl crate::Writable for CHCONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH%sCONF0 to value 0x3910_0002"]
impl crate::Resettable for CHCONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3910_0002
    }
}
