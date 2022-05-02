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
#[doc = "CK8M_D256_OUT divider. 00: div128 01: div256 10: div512 11: div1024.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CK8M_DIV_A {
    #[doc = "0: `0`"]
    DIV128 = 0,
    #[doc = "1: `1`"]
    DIV256 = 1,
    #[doc = "2: `10`"]
    DIV512 = 2,
    #[doc = "3: `11`"]
    DIV1024 = 3,
}
impl From<CK8M_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CK8M_DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CK8M_DIV` reader - CK8M_D256_OUT divider. 00: div128 01: div256 10: div512 11: div1024."]
pub struct CK8M_DIV_R(crate::FieldReader<u8>);
impl CK8M_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CK8M_DIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CK8M_DIV_A {
        match self.bits {
            0 => CK8M_DIV_A::DIV128,
            1 => CK8M_DIV_A::DIV256,
            2 => CK8M_DIV_A::DIV512,
            3 => CK8M_DIV_A::DIV1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        **self == CK8M_DIV_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        **self == CK8M_DIV_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        **self == CK8M_DIV_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        **self == CK8M_DIV_A::DIV1024
    }
}
impl core::ops::Deref for CK8M_DIV_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK8M_DIV` writer - CK8M_D256_OUT divider. 00: div128 01: div256 10: div512 11: div1024."]
pub struct CK8M_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CK8M_DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CK8M_DIV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(CK8M_DIV_A::DIV128)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(CK8M_DIV_A::DIV256)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(CK8M_DIV_A::DIV512)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(CK8M_DIV_A::DIV1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "Field `ENB_CK8M` reader - disable CK8M and CK8M_D256_OUT"]
pub struct ENB_CK8M_R(crate::FieldReader<bool>);
impl ENB_CK8M_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENB_CK8M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENB_CK8M_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENB_CK8M` writer - disable CK8M and CK8M_D256_OUT"]
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
#[doc = "1: CK8M_D256_OUT is actually CK8M 0: CK8M_D256_OUT is CK8M divided by 256\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENB_CK8M_DIV_A {
    #[doc = "0: `0`"]
    CK8M_DIV_256 = 0,
    #[doc = "1: `1`"]
    CK8M = 1,
}
impl From<ENB_CK8M_DIV_A> for bool {
    #[inline(always)]
    fn from(variant: ENB_CK8M_DIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENB_CK8M_DIV` reader - 1: CK8M_D256_OUT is actually CK8M 0: CK8M_D256_OUT is CK8M divided by 256"]
pub struct ENB_CK8M_DIV_R(crate::FieldReader<bool>);
impl ENB_CK8M_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENB_CK8M_DIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENB_CK8M_DIV_A {
        match self.bits {
            false => ENB_CK8M_DIV_A::CK8M_DIV_256,
            true => ENB_CK8M_DIV_A::CK8M,
        }
    }
    #[doc = "Checks if the value of the field is `CK8M_DIV_256`"]
    #[inline(always)]
    pub fn is_ck8m_div_256(&self) -> bool {
        **self == ENB_CK8M_DIV_A::CK8M_DIV_256
    }
    #[doc = "Checks if the value of the field is `CK8M`"]
    #[inline(always)]
    pub fn is_ck8m(&self) -> bool {
        **self == ENB_CK8M_DIV_A::CK8M
    }
}
impl core::ops::Deref for ENB_CK8M_DIV_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENB_CK8M_DIV` writer - 1: CK8M_D256_OUT is actually CK8M 0: CK8M_D256_OUT is CK8M divided by 256"]
pub struct ENB_CK8M_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ENB_CK8M_DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENB_CK8M_DIV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ck8m_div_256(self) -> &'a mut W {
        self.variant(ENB_CK8M_DIV_A::CK8M_DIV_256)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ck8m(self) -> &'a mut W {
        self.variant(ENB_CK8M_DIV_A::CK8M)
    }
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
#[doc = "Field `DIG_XTAL32K_EN` reader - enable CK_XTAL_32K for digital core (no relationship with RTC core)"]
pub struct DIG_XTAL32K_EN_R(crate::FieldReader<bool>);
impl DIG_XTAL32K_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIG_XTAL32K_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIG_XTAL32K_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIG_XTAL32K_EN` writer - enable CK_XTAL_32K for digital core (no relationship with RTC core)"]
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
#[doc = "Field `DIG_CLK8M_D256_EN` reader - enable CK8M_D256_OUT for digital core (no relationship with RTC core)"]
pub struct DIG_CLK8M_D256_EN_R(crate::FieldReader<bool>);
impl DIG_CLK8M_D256_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIG_CLK8M_D256_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIG_CLK8M_D256_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIG_CLK8M_D256_EN` writer - enable CK8M_D256_OUT for digital core (no relationship with RTC core)"]
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
#[doc = "Field `DIG_CLK8M_EN` reader - enable CK8M for digital core (no relationship with RTC core)"]
pub struct DIG_CLK8M_EN_R(crate::FieldReader<bool>);
impl DIG_CLK8M_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIG_CLK8M_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIG_CLK8M_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIG_CLK8M_EN` writer - enable CK8M for digital core (no relationship with RTC core)"]
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
#[doc = "Field `CK8M_DFREQ_FORCE` reader - "]
pub struct CK8M_DFREQ_FORCE_R(crate::FieldReader<bool>);
impl CK8M_DFREQ_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CK8M_DFREQ_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CK8M_DFREQ_FORCE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK8M_DFREQ_FORCE` writer - "]
pub struct CK8M_DFREQ_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CK8M_DFREQ_FORCE_W<'a> {
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
#[doc = "Field `CK8M_DIV_SEL` reader - divider = reg_ck8m_div_sel + 1"]
pub struct CK8M_DIV_SEL_R(crate::FieldReader<u8>);
impl CK8M_DIV_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CK8M_DIV_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CK8M_DIV_SEL_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK8M_DIV_SEL` writer - divider = reg_ck8m_div_sel + 1"]
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
#[doc = "Field `XTAL_FORCE_NOGATING` reader - XTAL force no gating during sleep"]
pub struct XTAL_FORCE_NOGATING_R(crate::FieldReader<bool>);
impl XTAL_FORCE_NOGATING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTAL_FORCE_NOGATING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL_FORCE_NOGATING_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL_FORCE_NOGATING` writer - XTAL force no gating during sleep"]
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
#[doc = "Field `CK8M_FORCE_NOGATING` reader - CK8M force no gating during sleep"]
pub struct CK8M_FORCE_NOGATING_R(crate::FieldReader<bool>);
impl CK8M_FORCE_NOGATING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CK8M_FORCE_NOGATING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CK8M_FORCE_NOGATING_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK8M_FORCE_NOGATING` writer - CK8M force no gating during sleep"]
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
pub struct CK8M_DFREQ_R(crate::FieldReader<u8>);
impl CK8M_DFREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CK8M_DFREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CK8M_DFREQ_R {
    type Target = crate::FieldReader<u8>;
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
#[doc = "Field `CK8M_FORCE_PD` reader - CK8M force power down"]
pub struct CK8M_FORCE_PD_R(crate::FieldReader<bool>);
impl CK8M_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CK8M_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CK8M_FORCE_PD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK8M_FORCE_PD` writer - CK8M force power down"]
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
#[doc = "Field `CK8M_FORCE_PU` reader - CK8M force power up"]
pub struct CK8M_FORCE_PU_R(crate::FieldReader<bool>);
impl CK8M_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CK8M_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CK8M_FORCE_PU_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK8M_FORCE_PU` writer - CK8M force power up"]
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
#[doc = "SOC clock sel. 0: XTAL 1: PLL 2: CK8M 3: APLL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SOC_CLK_SEL_A {
    #[doc = "0: `0`"]
    XTAL = 0,
    #[doc = "1: `1`"]
    PLL = 1,
    #[doc = "2: `10`"]
    CK8M = 2,
    #[doc = "3: `11`"]
    APLL = 3,
}
impl From<SOC_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SOC_CLK_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SOC_CLK_SEL` reader - SOC clock sel. 0: XTAL 1: PLL 2: CK8M 3: APLL"]
pub struct SOC_CLK_SEL_R(crate::FieldReader<u8>);
impl SOC_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SOC_CLK_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOC_CLK_SEL_A {
        match self.bits {
            0 => SOC_CLK_SEL_A::XTAL,
            1 => SOC_CLK_SEL_A::PLL,
            2 => SOC_CLK_SEL_A::CK8M,
            3 => SOC_CLK_SEL_A::APLL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        **self == SOC_CLK_SEL_A::XTAL
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        **self == SOC_CLK_SEL_A::PLL
    }
    #[doc = "Checks if the value of the field is `CK8M`"]
    #[inline(always)]
    pub fn is_ck8m(&self) -> bool {
        **self == SOC_CLK_SEL_A::CK8M
    }
    #[doc = "Checks if the value of the field is `APLL`"]
    #[inline(always)]
    pub fn is_apll(&self) -> bool {
        **self == SOC_CLK_SEL_A::APLL
    }
}
impl core::ops::Deref for SOC_CLK_SEL_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOC_CLK_SEL` writer - SOC clock sel. 0: XTAL 1: PLL 2: CK8M 3: APLL"]
pub struct SOC_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SOC_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOC_CLK_SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut W {
        self.variant(SOC_CLK_SEL_A::XTAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(SOC_CLK_SEL_A::PLL)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ck8m(self) -> &'a mut W {
        self.variant(SOC_CLK_SEL_A::CK8M)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn apll(self) -> &'a mut W {
        self.variant(SOC_CLK_SEL_A::APLL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 27)) | ((value as u32 & 3) << 27);
        self.w
    }
}
#[doc = "fast_clk_rtc sel. 0: XTAL div 4 1: CK8M\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAST_CLK_RTC_SEL_A {
    #[doc = "0: `0`"]
    XTAL_DIV_4 = 0,
    #[doc = "1: `1`"]
    CK8M = 1,
}
impl From<FAST_CLK_RTC_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: FAST_CLK_RTC_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAST_CLK_RTC_SEL` reader - fast_clk_rtc sel. 0: XTAL div 4 1: CK8M"]
pub struct FAST_CLK_RTC_SEL_R(crate::FieldReader<bool>);
impl FAST_CLK_RTC_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAST_CLK_RTC_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAST_CLK_RTC_SEL_A {
        match self.bits {
            false => FAST_CLK_RTC_SEL_A::XTAL_DIV_4,
            true => FAST_CLK_RTC_SEL_A::CK8M,
        }
    }
    #[doc = "Checks if the value of the field is `XTAL_DIV_4`"]
    #[inline(always)]
    pub fn is_xtal_div_4(&self) -> bool {
        **self == FAST_CLK_RTC_SEL_A::XTAL_DIV_4
    }
    #[doc = "Checks if the value of the field is `CK8M`"]
    #[inline(always)]
    pub fn is_ck8m(&self) -> bool {
        **self == FAST_CLK_RTC_SEL_A::CK8M
    }
}
impl core::ops::Deref for FAST_CLK_RTC_SEL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAST_CLK_RTC_SEL` writer - fast_clk_rtc sel. 0: XTAL div 4 1: CK8M"]
pub struct FAST_CLK_RTC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FAST_CLK_RTC_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAST_CLK_RTC_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn xtal_div_4(self) -> &'a mut W {
        self.variant(FAST_CLK_RTC_SEL_A::XTAL_DIV_4)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ck8m(self) -> &'a mut W {
        self.variant(FAST_CLK_RTC_SEL_A::CK8M)
    }
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
#[doc = "slow_clk_rtc sel. 0: SLOW_CK 1: CK_XTAL_32K 2: CK8M_D256_OUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ANA_CLK_RTC_SEL_A {
    #[doc = "0: `0`"]
    SLOW_CK = 0,
    #[doc = "1: `1`"]
    CK_XTAL_32K = 1,
    #[doc = "2: `10`"]
    CK8M_D256_OUT = 2,
}
impl From<ANA_CLK_RTC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ANA_CLK_RTC_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ANA_CLK_RTC_SEL` reader - slow_clk_rtc sel. 0: SLOW_CK 1: CK_XTAL_32K 2: CK8M_D256_OUT"]
pub struct ANA_CLK_RTC_SEL_R(crate::FieldReader<u8>);
impl ANA_CLK_RTC_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ANA_CLK_RTC_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ANA_CLK_RTC_SEL_A> {
        match self.bits {
            0 => Some(ANA_CLK_RTC_SEL_A::SLOW_CK),
            1 => Some(ANA_CLK_RTC_SEL_A::CK_XTAL_32K),
            2 => Some(ANA_CLK_RTC_SEL_A::CK8M_D256_OUT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SLOW_CK`"]
    #[inline(always)]
    pub fn is_slow_ck(&self) -> bool {
        **self == ANA_CLK_RTC_SEL_A::SLOW_CK
    }
    #[doc = "Checks if the value of the field is `CK_XTAL_32K`"]
    #[inline(always)]
    pub fn is_ck_xtal_32k(&self) -> bool {
        **self == ANA_CLK_RTC_SEL_A::CK_XTAL_32K
    }
    #[doc = "Checks if the value of the field is `CK8M_D256_OUT`"]
    #[inline(always)]
    pub fn is_ck8m_d256_out(&self) -> bool {
        **self == ANA_CLK_RTC_SEL_A::CK8M_D256_OUT
    }
}
impl core::ops::Deref for ANA_CLK_RTC_SEL_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANA_CLK_RTC_SEL` writer - slow_clk_rtc sel. 0: SLOW_CK 1: CK_XTAL_32K 2: CK8M_D256_OUT"]
pub struct ANA_CLK_RTC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ANA_CLK_RTC_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ANA_CLK_RTC_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn slow_ck(self) -> &'a mut W {
        self.variant(ANA_CLK_RTC_SEL_A::SLOW_CK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ck_xtal_32k(self) -> &'a mut W {
        self.variant(ANA_CLK_RTC_SEL_A::CK_XTAL_32K)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ck8m_d256_out(self) -> &'a mut W {
        self.variant(ANA_CLK_RTC_SEL_A::CK8M_D256_OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 30)) | ((value as u32 & 3) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:5 - CK8M_D256_OUT divider. 00: div128 01: div256 10: div512 11: div1024."]
    #[inline(always)]
    pub fn ck8m_div(&self) -> CK8M_DIV_R {
        CK8M_DIV_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - disable CK8M and CK8M_D256_OUT"]
    #[inline(always)]
    pub fn enb_ck8m(&self) -> ENB_CK8M_R {
        ENB_CK8M_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: CK8M_D256_OUT is actually CK8M 0: CK8M_D256_OUT is CK8M divided by 256"]
    #[inline(always)]
    pub fn enb_ck8m_div(&self) -> ENB_CK8M_DIV_R {
        ENB_CK8M_DIV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - enable CK_XTAL_32K for digital core (no relationship with RTC core)"]
    #[inline(always)]
    pub fn dig_xtal32k_en(&self) -> DIG_XTAL32K_EN_R {
        DIG_XTAL32K_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - enable CK8M_D256_OUT for digital core (no relationship with RTC core)"]
    #[inline(always)]
    pub fn dig_clk8m_d256_en(&self) -> DIG_CLK8M_D256_EN_R {
        DIG_CLK8M_D256_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - enable CK8M for digital core (no relationship with RTC core)"]
    #[inline(always)]
    pub fn dig_clk8m_en(&self) -> DIG_CLK8M_EN_R {
        DIG_CLK8M_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ck8m_dfreq_force(&self) -> CK8M_DFREQ_FORCE_R {
        CK8M_DFREQ_FORCE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - divider = reg_ck8m_div_sel + 1"]
    #[inline(always)]
    pub fn ck8m_div_sel(&self) -> CK8M_DIV_SEL_R {
        CK8M_DIV_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - XTAL force no gating during sleep"]
    #[inline(always)]
    pub fn xtal_force_nogating(&self) -> XTAL_FORCE_NOGATING_R {
        XTAL_FORCE_NOGATING_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CK8M force no gating during sleep"]
    #[inline(always)]
    pub fn ck8m_force_nogating(&self) -> CK8M_FORCE_NOGATING_R {
        CK8M_FORCE_NOGATING_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:24 - CK8M_DFREQ"]
    #[inline(always)]
    pub fn ck8m_dfreq(&self) -> CK8M_DFREQ_R {
        CK8M_DFREQ_R::new(((self.bits >> 17) & 0xff) as u8)
    }
    #[doc = "Bit 25 - CK8M force power down"]
    #[inline(always)]
    pub fn ck8m_force_pd(&self) -> CK8M_FORCE_PD_R {
        CK8M_FORCE_PD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CK8M force power up"]
    #[inline(always)]
    pub fn ck8m_force_pu(&self) -> CK8M_FORCE_PU_R {
        CK8M_FORCE_PU_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - SOC clock sel. 0: XTAL 1: PLL 2: CK8M 3: APLL"]
    #[inline(always)]
    pub fn soc_clk_sel(&self) -> SOC_CLK_SEL_R {
        SOC_CLK_SEL_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - fast_clk_rtc sel. 0: XTAL div 4 1: CK8M"]
    #[inline(always)]
    pub fn fast_clk_rtc_sel(&self) -> FAST_CLK_RTC_SEL_R {
        FAST_CLK_RTC_SEL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - slow_clk_rtc sel. 0: SLOW_CK 1: CK_XTAL_32K 2: CK8M_D256_OUT"]
    #[inline(always)]
    pub fn ana_clk_rtc_sel(&self) -> ANA_CLK_RTC_SEL_R {
        ANA_CLK_RTC_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - CK8M_D256_OUT divider. 00: div128 01: div256 10: div512 11: div1024."]
    #[inline(always)]
    pub fn ck8m_div(&mut self) -> CK8M_DIV_W {
        CK8M_DIV_W { w: self }
    }
    #[doc = "Bit 6 - disable CK8M and CK8M_D256_OUT"]
    #[inline(always)]
    pub fn enb_ck8m(&mut self) -> ENB_CK8M_W {
        ENB_CK8M_W { w: self }
    }
    #[doc = "Bit 7 - 1: CK8M_D256_OUT is actually CK8M 0: CK8M_D256_OUT is CK8M divided by 256"]
    #[inline(always)]
    pub fn enb_ck8m_div(&mut self) -> ENB_CK8M_DIV_W {
        ENB_CK8M_DIV_W { w: self }
    }
    #[doc = "Bit 8 - enable CK_XTAL_32K for digital core (no relationship with RTC core)"]
    #[inline(always)]
    pub fn dig_xtal32k_en(&mut self) -> DIG_XTAL32K_EN_W {
        DIG_XTAL32K_EN_W { w: self }
    }
    #[doc = "Bit 9 - enable CK8M_D256_OUT for digital core (no relationship with RTC core)"]
    #[inline(always)]
    pub fn dig_clk8m_d256_en(&mut self) -> DIG_CLK8M_D256_EN_W {
        DIG_CLK8M_D256_EN_W { w: self }
    }
    #[doc = "Bit 10 - enable CK8M for digital core (no relationship with RTC core)"]
    #[inline(always)]
    pub fn dig_clk8m_en(&mut self) -> DIG_CLK8M_EN_W {
        DIG_CLK8M_EN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ck8m_dfreq_force(&mut self) -> CK8M_DFREQ_FORCE_W {
        CK8M_DFREQ_FORCE_W { w: self }
    }
    #[doc = "Bits 12:14 - divider = reg_ck8m_div_sel + 1"]
    #[inline(always)]
    pub fn ck8m_div_sel(&mut self) -> CK8M_DIV_SEL_W {
        CK8M_DIV_SEL_W { w: self }
    }
    #[doc = "Bit 15 - XTAL force no gating during sleep"]
    #[inline(always)]
    pub fn xtal_force_nogating(&mut self) -> XTAL_FORCE_NOGATING_W {
        XTAL_FORCE_NOGATING_W { w: self }
    }
    #[doc = "Bit 16 - CK8M force no gating during sleep"]
    #[inline(always)]
    pub fn ck8m_force_nogating(&mut self) -> CK8M_FORCE_NOGATING_W {
        CK8M_FORCE_NOGATING_W { w: self }
    }
    #[doc = "Bits 17:24 - CK8M_DFREQ"]
    #[inline(always)]
    pub fn ck8m_dfreq(&mut self) -> CK8M_DFREQ_W {
        CK8M_DFREQ_W { w: self }
    }
    #[doc = "Bit 25 - CK8M force power down"]
    #[inline(always)]
    pub fn ck8m_force_pd(&mut self) -> CK8M_FORCE_PD_W {
        CK8M_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 26 - CK8M force power up"]
    #[inline(always)]
    pub fn ck8m_force_pu(&mut self) -> CK8M_FORCE_PU_W {
        CK8M_FORCE_PU_W { w: self }
    }
    #[doc = "Bits 27:28 - SOC clock sel. 0: XTAL 1: PLL 2: CK8M 3: APLL"]
    #[inline(always)]
    pub fn soc_clk_sel(&mut self) -> SOC_CLK_SEL_W {
        SOC_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 29 - fast_clk_rtc sel. 0: XTAL div 4 1: CK8M"]
    #[inline(always)]
    pub fn fast_clk_rtc_sel(&mut self) -> FAST_CLK_RTC_SEL_W {
        FAST_CLK_RTC_SEL_W { w: self }
    }
    #[doc = "Bits 30:31 - slow_clk_rtc sel. 0: SLOW_CK 1: CK_XTAL_32K 2: CK8M_D256_OUT"]
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_conf](index.html) module"]
pub struct CLK_CONF_SPEC;
impl crate::RegisterSpec for CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_conf::R](R) reader structure"]
impl crate::Readable for CLK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_conf::W](W) writer structure"]
impl crate::Writable for CLK_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CONF to value 0x2210"]
impl crate::Resettable for CLK_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2210
    }
}
