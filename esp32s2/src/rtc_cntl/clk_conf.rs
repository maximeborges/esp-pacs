#[doc = "Register `CLK_CONF` reader"]
pub struct R(crate::R<CLK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CONF` writer"]
pub struct W(crate::W<CLK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CONF_SPEC>;
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
impl From<crate::W<CLK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CK8M_DIV_SEL_VLD` reader - Synchronizes the reg_ck8m_div_sel. Not that you have to invalidate the bus before switching clock, and validate the new clock."]
pub struct CK8M_DIV_SEL_VLD_R(crate::FieldReader<bool, bool>);
impl CK8M_DIV_SEL_VLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CK8M_DIV_SEL_VLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CK8M_DIV_SEL_VLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK8M_DIV_SEL_VLD` writer - Synchronizes the reg_ck8m_div_sel. Not that you have to invalidate the bus before switching clock, and validate the new clock."]
pub struct CK8M_DIV_SEL_VLD_W<'a> {
    w: &'a mut W,
}
impl<'a> CK8M_DIV_SEL_VLD_W<'a> {
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
#[doc = "Field `CK8M_DIV` reader - Set the CK8M_D256_OUT divider. 00: divided by 128 01: divided by 256 10: divided by 512 11: divided by 1024."]
pub struct CK8M_DIV_R(crate::FieldReader<u8, u8>);
impl CK8M_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CK8M_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CK8M_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK8M_DIV` writer - Set the CK8M_D256_OUT divider. 00: divided by 128 01: divided by 256 10: divided by 512 11: divided by 1024."]
pub struct CK8M_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CK8M_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "Field `ENB_CK8M` reader - Set this bit to disable CK8M and CK8M_D256_OUT."]
pub struct ENB_CK8M_R(crate::FieldReader<bool, bool>);
impl ENB_CK8M_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENB_CK8M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENB_CK8M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENB_CK8M` writer - Set this bit to disable CK8M and CK8M_D256_OUT."]
pub struct ENB_CK8M_W<'a> {
    w: &'a mut W,
}
impl<'a> ENB_CK8M_W<'a> {
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
#[doc = "Field `ENB_CK8M_DIV` reader - Selects the CK8M_D256_OUT. 1: CK8M 0: CK8M divided by 256."]
pub struct ENB_CK8M_DIV_R(crate::FieldReader<bool, bool>);
impl ENB_CK8M_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENB_CK8M_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENB_CK8M_DIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENB_CK8M_DIV` writer - Selects the CK8M_D256_OUT. 1: CK8M 0: CK8M divided by 256."]
pub struct ENB_CK8M_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ENB_CK8M_DIV_W<'a> {
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
#[doc = "Field `DIG_XTAL32K_EN` reader - Set this bit to enable CK_XTAL_32K clock for the digital core."]
pub struct DIG_XTAL32K_EN_R(crate::FieldReader<bool, bool>);
impl DIG_XTAL32K_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIG_XTAL32K_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIG_XTAL32K_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIG_XTAL32K_EN` writer - Set this bit to enable CK_XTAL_32K clock for the digital core."]
pub struct DIG_XTAL32K_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_XTAL32K_EN_W<'a> {
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
#[doc = "Field `DIG_CLK8M_D256_EN` reader - Set this bit to enable CK8M_D256_OUT clock for the digital core."]
pub struct DIG_CLK8M_D256_EN_R(crate::FieldReader<bool, bool>);
impl DIG_CLK8M_D256_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIG_CLK8M_D256_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIG_CLK8M_D256_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIG_CLK8M_D256_EN` writer - Set this bit to enable CK8M_D256_OUT clock for the digital core."]
pub struct DIG_CLK8M_D256_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_CLK8M_D256_EN_W<'a> {
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
#[doc = "Field `DIG_CLK8M_EN` reader - Set this bit to enable 8 MHz clock for the digital core."]
pub struct DIG_CLK8M_EN_R(crate::FieldReader<bool, bool>);
impl DIG_CLK8M_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIG_CLK8M_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIG_CLK8M_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIG_CLK8M_EN` writer - Set this bit to enable 8 MHz clock for the digital core."]
pub struct DIG_CLK8M_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_CLK8M_EN_W<'a> {
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
#[doc = "Field `CK8M_DIV_SEL` reader - Stores the 8 MHz divider, which is reg_ck8m_div_sel + 1"]
pub struct CK8M_DIV_SEL_R(crate::FieldReader<u8, u8>);
impl CK8M_DIV_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CK8M_DIV_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CK8M_DIV_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK8M_DIV_SEL` writer - Stores the 8 MHz divider, which is reg_ck8m_div_sel + 1"]
pub struct CK8M_DIV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CK8M_DIV_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 12)) | ((value as u32 & 7) << 12);
        self.w
    }
}
#[doc = "Field `XTAL_FORCE_NOGATING` reader - Set this bit to force no gating to crystal during sleep"]
pub struct XTAL_FORCE_NOGATING_R(crate::FieldReader<bool, bool>);
impl XTAL_FORCE_NOGATING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTAL_FORCE_NOGATING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL_FORCE_NOGATING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL_FORCE_NOGATING` writer - Set this bit to force no gating to crystal during sleep"]
pub struct XTAL_FORCE_NOGATING_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_FORCE_NOGATING_W<'a> {
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
#[doc = "Field `CK8M_FORCE_NOGATING` reader - Set this bit to disable force gating to 8 MHz crystal during sleep."]
pub struct CK8M_FORCE_NOGATING_R(crate::FieldReader<bool, bool>);
impl CK8M_FORCE_NOGATING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CK8M_FORCE_NOGATING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CK8M_FORCE_NOGATING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK8M_FORCE_NOGATING` writer - Set this bit to disable force gating to 8 MHz crystal during sleep."]
pub struct CK8M_FORCE_NOGATING_W<'a> {
    w: &'a mut W,
}
impl<'a> CK8M_FORCE_NOGATING_W<'a> {
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
#[doc = "Field `CK8M_DFREQ` reader - CK8M_DFREQ"]
pub struct CK8M_DFREQ_R(crate::FieldReader<u8, u8>);
impl CK8M_DFREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CK8M_DFREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CK8M_DFREQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK8M_DFREQ` writer - CK8M_DFREQ"]
pub struct CK8M_DFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CK8M_DFREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 17)) | ((value as u32 & 0xff) << 17);
        self.w
    }
}
#[doc = "Field `CK8M_FORCE_PD` reader - Set this bit to FPD the 8 MHz clock."]
pub struct CK8M_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl CK8M_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CK8M_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CK8M_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK8M_FORCE_PD` writer - Set this bit to FPD the 8 MHz clock."]
pub struct CK8M_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> CK8M_FORCE_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 25)) | ((value as u32 & 1) << 25);
        self.w
    }
}
#[doc = "Field `CK8M_FORCE_PU` reader - Set this bit to FPU the 8 MHz clock."]
pub struct CK8M_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl CK8M_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CK8M_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CK8M_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK8M_FORCE_PU` writer - Set this bit to FPU the 8 MHz clock."]
pub struct CK8M_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> CK8M_FORCE_PU_W<'a> {
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
#[doc = "Field `FAST_CLK_RTC_SEL` reader - Set this bit to select the RTC fast clock. 0: XTAL div 4, 1: CK8M."]
pub struct FAST_CLK_RTC_SEL_R(crate::FieldReader<bool, bool>);
impl FAST_CLK_RTC_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAST_CLK_RTC_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAST_CLK_RTC_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAST_CLK_RTC_SEL` writer - Set this bit to select the RTC fast clock. 0: XTAL div 4, 1: CK8M."]
pub struct FAST_CLK_RTC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FAST_CLK_RTC_SEL_W<'a> {
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
#[doc = "Field `ANA_CLK_RTC_SEL` reader - Set this bit to select the RTC slow clock. 0: 90K rtc_clk 1: 32k XTAL 2: 8md256."]
pub struct ANA_CLK_RTC_SEL_R(crate::FieldReader<u8, u8>);
impl ANA_CLK_RTC_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ANA_CLK_RTC_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANA_CLK_RTC_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANA_CLK_RTC_SEL` writer - Set this bit to select the RTC slow clock. 0: 90K rtc_clk 1: 32k XTAL 2: 8md256."]
pub struct ANA_CLK_RTC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ANA_CLK_RTC_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 30)) | ((value as u32 & 3) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - Synchronizes the reg_ck8m_div_sel. Not that you have to invalidate the bus before switching clock, and validate the new clock."]
    #[inline(always)]
    pub fn ck8m_div_sel_vld(&self) -> CK8M_DIV_SEL_VLD_R {
        CK8M_DIV_SEL_VLD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Set the CK8M_D256_OUT divider. 00: divided by 128 01: divided by 256 10: divided by 512 11: divided by 1024."]
    #[inline(always)]
    pub fn ck8m_div(&self) -> CK8M_DIV_R {
        CK8M_DIV_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Set this bit to disable CK8M and CK8M_D256_OUT."]
    #[inline(always)]
    pub fn enb_ck8m(&self) -> ENB_CK8M_R {
        ENB_CK8M_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects the CK8M_D256_OUT. 1: CK8M 0: CK8M divided by 256."]
    #[inline(always)]
    pub fn enb_ck8m_div(&self) -> ENB_CK8M_DIV_R {
        ENB_CK8M_DIV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set this bit to enable CK_XTAL_32K clock for the digital core."]
    #[inline(always)]
    pub fn dig_xtal32k_en(&self) -> DIG_XTAL32K_EN_R {
        DIG_XTAL32K_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set this bit to enable CK8M_D256_OUT clock for the digital core."]
    #[inline(always)]
    pub fn dig_clk8m_d256_en(&self) -> DIG_CLK8M_D256_EN_R {
        DIG_CLK8M_D256_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set this bit to enable 8 MHz clock for the digital core."]
    #[inline(always)]
    pub fn dig_clk8m_en(&self) -> DIG_CLK8M_EN_R {
        DIG_CLK8M_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Stores the 8 MHz divider, which is reg_ck8m_div_sel + 1"]
    #[inline(always)]
    pub fn ck8m_div_sel(&self) -> CK8M_DIV_SEL_R {
        CK8M_DIV_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Set this bit to force no gating to crystal during sleep"]
    #[inline(always)]
    pub fn xtal_force_nogating(&self) -> XTAL_FORCE_NOGATING_R {
        XTAL_FORCE_NOGATING_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Set this bit to disable force gating to 8 MHz crystal during sleep."]
    #[inline(always)]
    pub fn ck8m_force_nogating(&self) -> CK8M_FORCE_NOGATING_R {
        CK8M_FORCE_NOGATING_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:24 - CK8M_DFREQ"]
    #[inline(always)]
    pub fn ck8m_dfreq(&self) -> CK8M_DFREQ_R {
        CK8M_DFREQ_R::new(((self.bits >> 17) & 0xff) as u8)
    }
    #[doc = "Bit 25 - Set this bit to FPD the 8 MHz clock."]
    #[inline(always)]
    pub fn ck8m_force_pd(&self) -> CK8M_FORCE_PD_R {
        CK8M_FORCE_PD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Set this bit to FPU the 8 MHz clock."]
    #[inline(always)]
    pub fn ck8m_force_pu(&self) -> CK8M_FORCE_PU_R {
        CK8M_FORCE_PU_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - Set this bit to select the RTC fast clock. 0: XTAL div 4, 1: CK8M."]
    #[inline(always)]
    pub fn fast_clk_rtc_sel(&self) -> FAST_CLK_RTC_SEL_R {
        FAST_CLK_RTC_SEL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Set this bit to select the RTC slow clock. 0: 90K rtc_clk 1: 32k XTAL 2: 8md256."]
    #[inline(always)]
    pub fn ana_clk_rtc_sel(&self) -> ANA_CLK_RTC_SEL_R {
        ANA_CLK_RTC_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 3 - Synchronizes the reg_ck8m_div_sel. Not that you have to invalidate the bus before switching clock, and validate the new clock."]
    #[inline(always)]
    pub fn ck8m_div_sel_vld(&mut self) -> CK8M_DIV_SEL_VLD_W {
        CK8M_DIV_SEL_VLD_W { w: self }
    }
    #[doc = "Bits 4:5 - Set the CK8M_D256_OUT divider. 00: divided by 128 01: divided by 256 10: divided by 512 11: divided by 1024."]
    #[inline(always)]
    pub fn ck8m_div(&mut self) -> CK8M_DIV_W {
        CK8M_DIV_W { w: self }
    }
    #[doc = "Bit 6 - Set this bit to disable CK8M and CK8M_D256_OUT."]
    #[inline(always)]
    pub fn enb_ck8m(&mut self) -> ENB_CK8M_W {
        ENB_CK8M_W { w: self }
    }
    #[doc = "Bit 7 - Selects the CK8M_D256_OUT. 1: CK8M 0: CK8M divided by 256."]
    #[inline(always)]
    pub fn enb_ck8m_div(&mut self) -> ENB_CK8M_DIV_W {
        ENB_CK8M_DIV_W { w: self }
    }
    #[doc = "Bit 8 - Set this bit to enable CK_XTAL_32K clock for the digital core."]
    #[inline(always)]
    pub fn dig_xtal32k_en(&mut self) -> DIG_XTAL32K_EN_W {
        DIG_XTAL32K_EN_W { w: self }
    }
    #[doc = "Bit 9 - Set this bit to enable CK8M_D256_OUT clock for the digital core."]
    #[inline(always)]
    pub fn dig_clk8m_d256_en(&mut self) -> DIG_CLK8M_D256_EN_W {
        DIG_CLK8M_D256_EN_W { w: self }
    }
    #[doc = "Bit 10 - Set this bit to enable 8 MHz clock for the digital core."]
    #[inline(always)]
    pub fn dig_clk8m_en(&mut self) -> DIG_CLK8M_EN_W {
        DIG_CLK8M_EN_W { w: self }
    }
    #[doc = "Bits 12:14 - Stores the 8 MHz divider, which is reg_ck8m_div_sel + 1"]
    #[inline(always)]
    pub fn ck8m_div_sel(&mut self) -> CK8M_DIV_SEL_W {
        CK8M_DIV_SEL_W { w: self }
    }
    #[doc = "Bit 15 - Set this bit to force no gating to crystal during sleep"]
    #[inline(always)]
    pub fn xtal_force_nogating(&mut self) -> XTAL_FORCE_NOGATING_W {
        XTAL_FORCE_NOGATING_W { w: self }
    }
    #[doc = "Bit 16 - Set this bit to disable force gating to 8 MHz crystal during sleep."]
    #[inline(always)]
    pub fn ck8m_force_nogating(&mut self) -> CK8M_FORCE_NOGATING_W {
        CK8M_FORCE_NOGATING_W { w: self }
    }
    #[doc = "Bits 17:24 - CK8M_DFREQ"]
    #[inline(always)]
    pub fn ck8m_dfreq(&mut self) -> CK8M_DFREQ_W {
        CK8M_DFREQ_W { w: self }
    }
    #[doc = "Bit 25 - Set this bit to FPD the 8 MHz clock."]
    #[inline(always)]
    pub fn ck8m_force_pd(&mut self) -> CK8M_FORCE_PD_W {
        CK8M_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 26 - Set this bit to FPU the 8 MHz clock."]
    #[inline(always)]
    pub fn ck8m_force_pu(&mut self) -> CK8M_FORCE_PU_W {
        CK8M_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 29 - Set this bit to select the RTC fast clock. 0: XTAL div 4, 1: CK8M."]
    #[inline(always)]
    pub fn fast_clk_rtc_sel(&mut self) -> FAST_CLK_RTC_SEL_W {
        FAST_CLK_RTC_SEL_W { w: self }
    }
    #[doc = "Bits 30:31 - Set this bit to select the RTC slow clock. 0: 90K rtc_clk 1: 32k XTAL 2: 8md256."]
    #[inline(always)]
    pub fn ana_clk_rtc_sel(&mut self) -> ANA_CLK_RTC_SEL_W {
        ANA_CLK_RTC_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC clock configuration register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_conf]
(index.html) module"]
pub struct CLK_CONF_SPEC;
impl crate::RegisterSpec for CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_conf::R]
(R) reader structure"]
impl crate::Readable for CLK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_conf::W]
(W) writer structure"]
impl crate::Writable for CLK_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CONF to value 0x0158_3218"]
impl crate::Resettable for CLK_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0158_3218
    }
}
