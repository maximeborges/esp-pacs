#[doc = "Register `SDIO_CONF` reader"]
pub struct R(crate::R<SDIO_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIO_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIO_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIO_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDIO_CONF` writer"]
pub struct W(crate::W<SDIO_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIO_CONF_SPEC>;
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
impl From<crate::W<SDIO_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIO_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDIO_TIMER_TARGET` reader - timer count to apply reg_sdio_dcap after sdio power on"]
pub struct SDIO_TIMER_TARGET_R(crate::FieldReader<u8, u8>);
impl SDIO_TIMER_TARGET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SDIO_TIMER_TARGET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_TIMER_TARGET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_TIMER_TARGET` writer - timer count to apply reg_sdio_dcap after sdio power on"]
pub struct SDIO_TIMER_TARGET_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_TIMER_TARGET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `SDIO_DTHDRV` reader - Tieh = 1 mode drive ability. Initially set to 0 to limit charge current"]
pub struct SDIO_DTHDRV_R(crate::FieldReader<u8, u8>);
impl SDIO_DTHDRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SDIO_DTHDRV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_DTHDRV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_DTHDRV` writer - Tieh = 1 mode drive ability. Initially set to 0 to limit charge current"]
pub struct SDIO_DTHDRV_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_DTHDRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 9)) | ((value as u32 & 3) << 9);
        self.w
    }
}
#[doc = "Field `SDIO_DCAP` reader - ability to prevent LDO from overshoot"]
pub struct SDIO_DCAP_R(crate::FieldReader<u8, u8>);
impl SDIO_DCAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SDIO_DCAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_DCAP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_DCAP` writer - ability to prevent LDO from overshoot"]
pub struct SDIO_DCAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_DCAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 11)) | ((value as u32 & 3) << 11);
        self.w
    }
}
#[doc = "Field `SDIO_INITI` reader - add resistor from ldo output to ground. 0: no res"]
pub struct SDIO_INITI_R(crate::FieldReader<u8, u8>);
impl SDIO_INITI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SDIO_INITI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_INITI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_INITI` writer - add resistor from ldo output to ground. 0: no res"]
pub struct SDIO_INITI_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_INITI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 13)) | ((value as u32 & 3) << 13);
        self.w
    }
}
#[doc = "Field `SDIO_EN_INITI` reader - 0 to set init\\[1:0\\]
=0"]
pub struct SDIO_EN_INITI_R(crate::FieldReader<bool, bool>);
impl SDIO_EN_INITI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_EN_INITI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_EN_INITI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_EN_INITI` writer - 0 to set init\\[1:0\\]
=0"]
pub struct SDIO_EN_INITI_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_EN_INITI_W<'a> {
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
#[doc = "Field `SDIO_DCURLIM` reader - tune current limit threshold when tieh = 0. About 800mA/(8+d)"]
pub struct SDIO_DCURLIM_R(crate::FieldReader<u8, u8>);
impl SDIO_DCURLIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SDIO_DCURLIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_DCURLIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_DCURLIM` writer - tune current limit threshold when tieh = 0. About 800mA/(8+d)"]
pub struct SDIO_DCURLIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_DCURLIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 16)) | ((value as u32 & 7) << 16);
        self.w
    }
}
#[doc = "Field `SDIO_MODECURLIM` reader - select current limit mode"]
pub struct SDIO_MODECURLIM_R(crate::FieldReader<bool, bool>);
impl SDIO_MODECURLIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_MODECURLIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_MODECURLIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_MODECURLIM` writer - select current limit mode"]
pub struct SDIO_MODECURLIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_MODECURLIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 19)) | ((value as u32 & 1) << 19);
        self.w
    }
}
#[doc = "Field `SDIO_ENCURLIM` reader - enable current limit"]
pub struct SDIO_ENCURLIM_R(crate::FieldReader<bool, bool>);
impl SDIO_ENCURLIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_ENCURLIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_ENCURLIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_ENCURLIM` writer - enable current limit"]
pub struct SDIO_ENCURLIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_ENCURLIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 20)) | ((value as u32 & 1) << 20);
        self.w
    }
}
#[doc = "Field `SDIO_REG_PD_EN` reader - power down SDIO_REG in sleep. Only active when reg_sdio_force = 0"]
pub struct SDIO_REG_PD_EN_R(crate::FieldReader<bool, bool>);
impl SDIO_REG_PD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_REG_PD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_REG_PD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_REG_PD_EN` writer - power down SDIO_REG in sleep. Only active when reg_sdio_force = 0"]
pub struct SDIO_REG_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_REG_PD_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 21)) | ((value as u32 & 1) << 21);
        self.w
    }
}
#[doc = "Field `SDIO_FORCE` reader - 1: use SW option to control SDIO_REG"]
pub struct SDIO_FORCE_R(crate::FieldReader<bool, bool>);
impl SDIO_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_FORCE` writer - 1: use SW option to control SDIO_REG"]
pub struct SDIO_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 22)) | ((value as u32 & 1) << 22);
        self.w
    }
}
#[doc = "Field `SDIO_TIEH` reader - SW option for SDIO_TIEH. Only active when reg_sdio_force = 1"]
pub struct SDIO_TIEH_R(crate::FieldReader<bool, bool>);
impl SDIO_TIEH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_TIEH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_TIEH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_TIEH` writer - SW option for SDIO_TIEH. Only active when reg_sdio_force = 1"]
pub struct SDIO_TIEH_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_TIEH_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 23)) | ((value as u32 & 1) << 23);
        self.w
    }
}
#[doc = "Field `_1P8_READY` reader - read only register for REG1P8_READY"]
pub struct _1P8_READY_R(crate::FieldReader<bool, bool>);
impl _1P8_READY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        _1P8_READY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for _1P8_READY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DREFL_SDIO` reader - SW option for DREFL_SDIO. Only active when reg_sdio_force = 1"]
pub struct DREFL_SDIO_R(crate::FieldReader<u8, u8>);
impl DREFL_SDIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DREFL_SDIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DREFL_SDIO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DREFL_SDIO` writer - SW option for DREFL_SDIO. Only active when reg_sdio_force = 1"]
pub struct DREFL_SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> DREFL_SDIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 25)) | ((value as u32 & 3) << 25);
        self.w
    }
}
#[doc = "Field `DREFM_SDIO` reader - SW option for DREFM_SDIO. Only active when reg_sdio_force = 1"]
pub struct DREFM_SDIO_R(crate::FieldReader<u8, u8>);
impl DREFM_SDIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DREFM_SDIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DREFM_SDIO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DREFM_SDIO` writer - SW option for DREFM_SDIO. Only active when reg_sdio_force = 1"]
pub struct DREFM_SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> DREFM_SDIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 27)) | ((value as u32 & 3) << 27);
        self.w
    }
}
#[doc = "Field `DREFH_SDIO` reader - SW option for DREFH_SDIO. Only active when reg_sdio_force = 1"]
pub struct DREFH_SDIO_R(crate::FieldReader<u8, u8>);
impl DREFH_SDIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DREFH_SDIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DREFH_SDIO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DREFH_SDIO` writer - SW option for DREFH_SDIO. Only active when reg_sdio_force = 1"]
pub struct DREFH_SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> DREFH_SDIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 29)) | ((value as u32 & 3) << 29);
        self.w
    }
}
#[doc = "Field `XPD_SDIO` reader - "]
pub struct XPD_SDIO_R(crate::FieldReader<bool, bool>);
impl XPD_SDIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XPD_SDIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XPD_SDIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XPD_SDIO` writer - "]
pub struct XPD_SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> XPD_SDIO_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - timer count to apply reg_sdio_dcap after sdio power on"]
    #[inline(always)]
    pub fn sdio_timer_target(&self) -> SDIO_TIMER_TARGET_R {
        SDIO_TIMER_TARGET_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 9:10 - Tieh = 1 mode drive ability. Initially set to 0 to limit charge current"]
    #[inline(always)]
    pub fn sdio_dthdrv(&self) -> SDIO_DTHDRV_R {
        SDIO_DTHDRV_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - ability to prevent LDO from overshoot"]
    #[inline(always)]
    pub fn sdio_dcap(&self) -> SDIO_DCAP_R {
        SDIO_DCAP_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - add resistor from ldo output to ground. 0: no res"]
    #[inline(always)]
    pub fn sdio_initi(&self) -> SDIO_INITI_R {
        SDIO_INITI_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - 0 to set init\\[1:0\\]
=0"]
    #[inline(always)]
    pub fn sdio_en_initi(&self) -> SDIO_EN_INITI_R {
        SDIO_EN_INITI_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - tune current limit threshold when tieh = 0. About 800mA/(8+d)"]
    #[inline(always)]
    pub fn sdio_dcurlim(&self) -> SDIO_DCURLIM_R {
        SDIO_DCURLIM_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - select current limit mode"]
    #[inline(always)]
    pub fn sdio_modecurlim(&self) -> SDIO_MODECURLIM_R {
        SDIO_MODECURLIM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - enable current limit"]
    #[inline(always)]
    pub fn sdio_encurlim(&self) -> SDIO_ENCURLIM_R {
        SDIO_ENCURLIM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - power down SDIO_REG in sleep. Only active when reg_sdio_force = 0"]
    #[inline(always)]
    pub fn sdio_reg_pd_en(&self) -> SDIO_REG_PD_EN_R {
        SDIO_REG_PD_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: use SW option to control SDIO_REG"]
    #[inline(always)]
    pub fn sdio_force(&self) -> SDIO_FORCE_R {
        SDIO_FORCE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SW option for SDIO_TIEH. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn sdio_tieh(&self) -> SDIO_TIEH_R {
        SDIO_TIEH_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - read only register for REG1P8_READY"]
    #[inline(always)]
    pub fn _1p8_ready(&self) -> _1P8_READY_R {
        _1P8_READY_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - SW option for DREFL_SDIO. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn drefl_sdio(&self) -> DREFL_SDIO_R {
        DREFL_SDIO_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:28 - SW option for DREFM_SDIO. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn drefm_sdio(&self) -> DREFM_SDIO_R {
        DREFM_SDIO_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 29:30 - SW option for DREFH_SDIO. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn drefh_sdio(&self) -> DREFH_SDIO_R {
        DREFH_SDIO_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn xpd_sdio(&self) -> XPD_SDIO_R {
        XPD_SDIO_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - timer count to apply reg_sdio_dcap after sdio power on"]
    #[inline(always)]
    pub fn sdio_timer_target(&mut self) -> SDIO_TIMER_TARGET_W {
        SDIO_TIMER_TARGET_W { w: self }
    }
    #[doc = "Bits 9:10 - Tieh = 1 mode drive ability. Initially set to 0 to limit charge current"]
    #[inline(always)]
    pub fn sdio_dthdrv(&mut self) -> SDIO_DTHDRV_W {
        SDIO_DTHDRV_W { w: self }
    }
    #[doc = "Bits 11:12 - ability to prevent LDO from overshoot"]
    #[inline(always)]
    pub fn sdio_dcap(&mut self) -> SDIO_DCAP_W {
        SDIO_DCAP_W { w: self }
    }
    #[doc = "Bits 13:14 - add resistor from ldo output to ground. 0: no res"]
    #[inline(always)]
    pub fn sdio_initi(&mut self) -> SDIO_INITI_W {
        SDIO_INITI_W { w: self }
    }
    #[doc = "Bit 15 - 0 to set init\\[1:0\\]
=0"]
    #[inline(always)]
    pub fn sdio_en_initi(&mut self) -> SDIO_EN_INITI_W {
        SDIO_EN_INITI_W { w: self }
    }
    #[doc = "Bits 16:18 - tune current limit threshold when tieh = 0. About 800mA/(8+d)"]
    #[inline(always)]
    pub fn sdio_dcurlim(&mut self) -> SDIO_DCURLIM_W {
        SDIO_DCURLIM_W { w: self }
    }
    #[doc = "Bit 19 - select current limit mode"]
    #[inline(always)]
    pub fn sdio_modecurlim(&mut self) -> SDIO_MODECURLIM_W {
        SDIO_MODECURLIM_W { w: self }
    }
    #[doc = "Bit 20 - enable current limit"]
    #[inline(always)]
    pub fn sdio_encurlim(&mut self) -> SDIO_ENCURLIM_W {
        SDIO_ENCURLIM_W { w: self }
    }
    #[doc = "Bit 21 - power down SDIO_REG in sleep. Only active when reg_sdio_force = 0"]
    #[inline(always)]
    pub fn sdio_reg_pd_en(&mut self) -> SDIO_REG_PD_EN_W {
        SDIO_REG_PD_EN_W { w: self }
    }
    #[doc = "Bit 22 - 1: use SW option to control SDIO_REG"]
    #[inline(always)]
    pub fn sdio_force(&mut self) -> SDIO_FORCE_W {
        SDIO_FORCE_W { w: self }
    }
    #[doc = "Bit 23 - SW option for SDIO_TIEH. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn sdio_tieh(&mut self) -> SDIO_TIEH_W {
        SDIO_TIEH_W { w: self }
    }
    #[doc = "Bits 25:26 - SW option for DREFL_SDIO. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn drefl_sdio(&mut self) -> DREFL_SDIO_W {
        DREFL_SDIO_W { w: self }
    }
    #[doc = "Bits 27:28 - SW option for DREFM_SDIO. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn drefm_sdio(&mut self) -> DREFM_SDIO_W {
        DREFM_SDIO_W { w: self }
    }
    #[doc = "Bits 29:30 - SW option for DREFH_SDIO. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn drefh_sdio(&mut self) -> DREFH_SDIO_W {
        DREFH_SDIO_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn xpd_sdio(&mut self) -> XPD_SDIO_W {
        XPD_SDIO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdio_conf]
(index.html) module"]
pub struct SDIO_CONF_SPEC;
impl crate::RegisterSpec for SDIO_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdio_conf::R]
(R) reader structure"]
impl crate::Readable for SDIO_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdio_conf::W]
(W) writer structure"]
impl crate::Writable for SDIO_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDIO_CONF to value 0x0ab0_be0a"]
impl crate::Resettable for SDIO_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0ab0_be0a
    }
}
