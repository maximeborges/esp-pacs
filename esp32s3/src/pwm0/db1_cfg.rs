#[doc = "Register `DB1_CFG` reader"]
pub struct R(crate::R<DB1_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DB1_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DB1_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DB1_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DB1_CFG` writer"]
pub struct W(crate::W<DB1_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DB1_CFG_SPEC>;
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
impl From<crate::W<DB1_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DB1_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DB1_FED_UPMETHOD` reader - Update method for FED (falling edge delay) active register. 0: immediate, bit0: tez, bit1: tep, bit2: sync, bit3: freeze"]
pub struct DB1_FED_UPMETHOD_R(crate::FieldReader<u8, u8>);
impl DB1_FED_UPMETHOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DB1_FED_UPMETHOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DB1_FED_UPMETHOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DB1_FED_UPMETHOD` writer - Update method for FED (falling edge delay) active register. 0: immediate, bit0: tez, bit1: tep, bit2: sync, bit3: freeze"]
pub struct DB1_FED_UPMETHOD_W<'a> {
    w: &'a mut W,
}
impl<'a> DB1_FED_UPMETHOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `DB1_RED_UPMETHOD` reader - Update method for RED (rising edge delay) active register. 0: immediate, bit0: tez, bit1: tep, bit2: sync, bit3: freeze"]
pub struct DB1_RED_UPMETHOD_R(crate::FieldReader<u8, u8>);
impl DB1_RED_UPMETHOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DB1_RED_UPMETHOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DB1_RED_UPMETHOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DB1_RED_UPMETHOD` writer - Update method for RED (rising edge delay) active register. 0: immediate, bit0: tez, bit1: tep, bit2: sync, bit3: freeze"]
pub struct DB1_RED_UPMETHOD_W<'a> {
    w: &'a mut W,
}
impl<'a> DB1_RED_UPMETHOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `DB1_DEB_MODE` reader - S8 in documentation, dual-edge B mode, 0: fed/red take effect on different path separately, 1: fed/red take effect on B path, A out is in bypass or dulpB mode"]
pub struct DB1_DEB_MODE_R(crate::FieldReader<bool, bool>);
impl DB1_DEB_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DB1_DEB_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DB1_DEB_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DB1_DEB_MODE` writer - S8 in documentation, dual-edge B mode, 0: fed/red take effect on different path separately, 1: fed/red take effect on B path, A out is in bypass or dulpB mode"]
pub struct DB1_DEB_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DB1_DEB_MODE_W<'a> {
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
#[doc = "Field `DB1_A_OUTSWAP` reader - S6 in documentation"]
pub struct DB1_A_OUTSWAP_R(crate::FieldReader<bool, bool>);
impl DB1_A_OUTSWAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DB1_A_OUTSWAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DB1_A_OUTSWAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DB1_A_OUTSWAP` writer - S6 in documentation"]
pub struct DB1_A_OUTSWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> DB1_A_OUTSWAP_W<'a> {
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
#[doc = "Field `DB1_B_OUTSWAP` reader - S7 in documentation"]
pub struct DB1_B_OUTSWAP_R(crate::FieldReader<bool, bool>);
impl DB1_B_OUTSWAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DB1_B_OUTSWAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DB1_B_OUTSWAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DB1_B_OUTSWAP` writer - S7 in documentation"]
pub struct DB1_B_OUTSWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> DB1_B_OUTSWAP_W<'a> {
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
#[doc = "Field `DB1_RED_INSEL` reader - S4 in documentation"]
pub struct DB1_RED_INSEL_R(crate::FieldReader<bool, bool>);
impl DB1_RED_INSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DB1_RED_INSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DB1_RED_INSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DB1_RED_INSEL` writer - S4 in documentation"]
pub struct DB1_RED_INSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DB1_RED_INSEL_W<'a> {
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
#[doc = "Field `DB1_FED_INSEL` reader - S5 in documentation"]
pub struct DB1_FED_INSEL_R(crate::FieldReader<bool, bool>);
impl DB1_FED_INSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DB1_FED_INSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DB1_FED_INSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DB1_FED_INSEL` writer - S5 in documentation"]
pub struct DB1_FED_INSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DB1_FED_INSEL_W<'a> {
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
#[doc = "Field `DB1_RED_OUTINVERT` reader - S2 in documentation"]
pub struct DB1_RED_OUTINVERT_R(crate::FieldReader<bool, bool>);
impl DB1_RED_OUTINVERT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DB1_RED_OUTINVERT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DB1_RED_OUTINVERT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DB1_RED_OUTINVERT` writer - S2 in documentation"]
pub struct DB1_RED_OUTINVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> DB1_RED_OUTINVERT_W<'a> {
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
#[doc = "Field `DB1_FED_OUTINVERT` reader - S3 in documentation"]
pub struct DB1_FED_OUTINVERT_R(crate::FieldReader<bool, bool>);
impl DB1_FED_OUTINVERT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DB1_FED_OUTINVERT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DB1_FED_OUTINVERT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DB1_FED_OUTINVERT` writer - S3 in documentation"]
pub struct DB1_FED_OUTINVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> DB1_FED_OUTINVERT_W<'a> {
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
#[doc = "Field `DB1_A_OUTBYPASS` reader - S1 in documentation"]
pub struct DB1_A_OUTBYPASS_R(crate::FieldReader<bool, bool>);
impl DB1_A_OUTBYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DB1_A_OUTBYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DB1_A_OUTBYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DB1_A_OUTBYPASS` writer - S1 in documentation"]
pub struct DB1_A_OUTBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> DB1_A_OUTBYPASS_W<'a> {
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
#[doc = "Field `DB1_B_OUTBYPASS` reader - S0 in documentation"]
pub struct DB1_B_OUTBYPASS_R(crate::FieldReader<bool, bool>);
impl DB1_B_OUTBYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DB1_B_OUTBYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DB1_B_OUTBYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DB1_B_OUTBYPASS` writer - S0 in documentation"]
pub struct DB1_B_OUTBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> DB1_B_OUTBYPASS_W<'a> {
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
#[doc = "Field `DB1_CLK_SEL` reader - Dead time generator 1 clock selection. 0: PWM_clk, 1: PT_clk"]
pub struct DB1_CLK_SEL_R(crate::FieldReader<bool, bool>);
impl DB1_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DB1_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DB1_CLK_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DB1_CLK_SEL` writer - Dead time generator 1 clock selection. 0: PWM_clk, 1: PT_clk"]
pub struct DB1_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DB1_CLK_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Update method for FED (falling edge delay) active register. 0: immediate, bit0: tez, bit1: tep, bit2: sync, bit3: freeze"]
    #[inline(always)]
    pub fn db1_fed_upmethod(&self) -> DB1_FED_UPMETHOD_R {
        DB1_FED_UPMETHOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Update method for RED (rising edge delay) active register. 0: immediate, bit0: tez, bit1: tep, bit2: sync, bit3: freeze"]
    #[inline(always)]
    pub fn db1_red_upmethod(&self) -> DB1_RED_UPMETHOD_R {
        DB1_RED_UPMETHOD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - S8 in documentation, dual-edge B mode, 0: fed/red take effect on different path separately, 1: fed/red take effect on B path, A out is in bypass or dulpB mode"]
    #[inline(always)]
    pub fn db1_deb_mode(&self) -> DB1_DEB_MODE_R {
        DB1_DEB_MODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - S6 in documentation"]
    #[inline(always)]
    pub fn db1_a_outswap(&self) -> DB1_A_OUTSWAP_R {
        DB1_A_OUTSWAP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - S7 in documentation"]
    #[inline(always)]
    pub fn db1_b_outswap(&self) -> DB1_B_OUTSWAP_R {
        DB1_B_OUTSWAP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - S4 in documentation"]
    #[inline(always)]
    pub fn db1_red_insel(&self) -> DB1_RED_INSEL_R {
        DB1_RED_INSEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - S5 in documentation"]
    #[inline(always)]
    pub fn db1_fed_insel(&self) -> DB1_FED_INSEL_R {
        DB1_FED_INSEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - S2 in documentation"]
    #[inline(always)]
    pub fn db1_red_outinvert(&self) -> DB1_RED_OUTINVERT_R {
        DB1_RED_OUTINVERT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - S3 in documentation"]
    #[inline(always)]
    pub fn db1_fed_outinvert(&self) -> DB1_FED_OUTINVERT_R {
        DB1_FED_OUTINVERT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - S1 in documentation"]
    #[inline(always)]
    pub fn db1_a_outbypass(&self) -> DB1_A_OUTBYPASS_R {
        DB1_A_OUTBYPASS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - S0 in documentation"]
    #[inline(always)]
    pub fn db1_b_outbypass(&self) -> DB1_B_OUTBYPASS_R {
        DB1_B_OUTBYPASS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Dead time generator 1 clock selection. 0: PWM_clk, 1: PT_clk"]
    #[inline(always)]
    pub fn db1_clk_sel(&self) -> DB1_CLK_SEL_R {
        DB1_CLK_SEL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Update method for FED (falling edge delay) active register. 0: immediate, bit0: tez, bit1: tep, bit2: sync, bit3: freeze"]
    #[inline(always)]
    pub fn db1_fed_upmethod(&mut self) -> DB1_FED_UPMETHOD_W {
        DB1_FED_UPMETHOD_W { w: self }
    }
    #[doc = "Bits 4:7 - Update method for RED (rising edge delay) active register. 0: immediate, bit0: tez, bit1: tep, bit2: sync, bit3: freeze"]
    #[inline(always)]
    pub fn db1_red_upmethod(&mut self) -> DB1_RED_UPMETHOD_W {
        DB1_RED_UPMETHOD_W { w: self }
    }
    #[doc = "Bit 8 - S8 in documentation, dual-edge B mode, 0: fed/red take effect on different path separately, 1: fed/red take effect on B path, A out is in bypass or dulpB mode"]
    #[inline(always)]
    pub fn db1_deb_mode(&mut self) -> DB1_DEB_MODE_W {
        DB1_DEB_MODE_W { w: self }
    }
    #[doc = "Bit 9 - S6 in documentation"]
    #[inline(always)]
    pub fn db1_a_outswap(&mut self) -> DB1_A_OUTSWAP_W {
        DB1_A_OUTSWAP_W { w: self }
    }
    #[doc = "Bit 10 - S7 in documentation"]
    #[inline(always)]
    pub fn db1_b_outswap(&mut self) -> DB1_B_OUTSWAP_W {
        DB1_B_OUTSWAP_W { w: self }
    }
    #[doc = "Bit 11 - S4 in documentation"]
    #[inline(always)]
    pub fn db1_red_insel(&mut self) -> DB1_RED_INSEL_W {
        DB1_RED_INSEL_W { w: self }
    }
    #[doc = "Bit 12 - S5 in documentation"]
    #[inline(always)]
    pub fn db1_fed_insel(&mut self) -> DB1_FED_INSEL_W {
        DB1_FED_INSEL_W { w: self }
    }
    #[doc = "Bit 13 - S2 in documentation"]
    #[inline(always)]
    pub fn db1_red_outinvert(&mut self) -> DB1_RED_OUTINVERT_W {
        DB1_RED_OUTINVERT_W { w: self }
    }
    #[doc = "Bit 14 - S3 in documentation"]
    #[inline(always)]
    pub fn db1_fed_outinvert(&mut self) -> DB1_FED_OUTINVERT_W {
        DB1_FED_OUTINVERT_W { w: self }
    }
    #[doc = "Bit 15 - S1 in documentation"]
    #[inline(always)]
    pub fn db1_a_outbypass(&mut self) -> DB1_A_OUTBYPASS_W {
        DB1_A_OUTBYPASS_W { w: self }
    }
    #[doc = "Bit 16 - S0 in documentation"]
    #[inline(always)]
    pub fn db1_b_outbypass(&mut self) -> DB1_B_OUTBYPASS_W {
        DB1_B_OUTBYPASS_W { w: self }
    }
    #[doc = "Bit 17 - Dead time generator 1 clock selection. 0: PWM_clk, 1: PT_clk"]
    #[inline(always)]
    pub fn db1_clk_sel(&mut self) -> DB1_CLK_SEL_W {
        DB1_CLK_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dead time type selection and configuration\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [db1_cfg]
(index.html) module"]
pub struct DB1_CFG_SPEC;
impl crate::RegisterSpec for DB1_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [db1_cfg::R]
(R) reader structure"]
impl crate::Readable for DB1_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [db1_cfg::W]
(W) writer structure"]
impl crate::Writable for DB1_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DB1_CFG to value 0x0001_8000"]
impl crate::Resettable for DB1_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_8000
    }
}
