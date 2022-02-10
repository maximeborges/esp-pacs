#[doc = "Register `TZ2_CFG0` reader"]
pub struct R(crate::R<TZ2_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZ2_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZ2_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZ2_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TZ2_CFG0` writer"]
pub struct W(crate::W<TZ2_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZ2_CFG0_SPEC>;
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
impl From<crate::W<TZ2_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZ2_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TZ2_SW_CBC` reader - Enable register for software force cycle-by-cycle mode action. 0: disable, 1: enable"]
pub struct TZ2_SW_CBC_R(crate::FieldReader<bool, bool>);
impl TZ2_SW_CBC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TZ2_SW_CBC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZ2_SW_CBC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZ2_SW_CBC` writer - Enable register for software force cycle-by-cycle mode action. 0: disable, 1: enable"]
pub struct TZ2_SW_CBC_W<'a> {
    w: &'a mut W,
}
impl<'a> TZ2_SW_CBC_W<'a> {
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
#[doc = "Field `TZ2_F2_CBC` reader - event_f2 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
pub struct TZ2_F2_CBC_R(crate::FieldReader<bool, bool>);
impl TZ2_F2_CBC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TZ2_F2_CBC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZ2_F2_CBC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZ2_F2_CBC` writer - event_f2 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
pub struct TZ2_F2_CBC_W<'a> {
    w: &'a mut W,
}
impl<'a> TZ2_F2_CBC_W<'a> {
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
#[doc = "Field `TZ2_F1_CBC` reader - event_f1 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
pub struct TZ2_F1_CBC_R(crate::FieldReader<bool, bool>);
impl TZ2_F1_CBC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TZ2_F1_CBC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZ2_F1_CBC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZ2_F1_CBC` writer - event_f1 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
pub struct TZ2_F1_CBC_W<'a> {
    w: &'a mut W,
}
impl<'a> TZ2_F1_CBC_W<'a> {
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
#[doc = "Field `TZ2_F0_CBC` reader - event_f0 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
pub struct TZ2_F0_CBC_R(crate::FieldReader<bool, bool>);
impl TZ2_F0_CBC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TZ2_F0_CBC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZ2_F0_CBC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZ2_F0_CBC` writer - event_f0 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
pub struct TZ2_F0_CBC_W<'a> {
    w: &'a mut W,
}
impl<'a> TZ2_F0_CBC_W<'a> {
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
#[doc = "Field `TZ2_SW_OST` reader - Enable register for software force one-shot mode action. 0: disable, 1: enable"]
pub struct TZ2_SW_OST_R(crate::FieldReader<bool, bool>);
impl TZ2_SW_OST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TZ2_SW_OST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZ2_SW_OST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZ2_SW_OST` writer - Enable register for software force one-shot mode action. 0: disable, 1: enable"]
pub struct TZ2_SW_OST_W<'a> {
    w: &'a mut W,
}
impl<'a> TZ2_SW_OST_W<'a> {
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
#[doc = "Field `TZ2_F2_OST` reader - event_f2 will trigger one-shot mode action. 0: disable, 1: enable"]
pub struct TZ2_F2_OST_R(crate::FieldReader<bool, bool>);
impl TZ2_F2_OST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TZ2_F2_OST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZ2_F2_OST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZ2_F2_OST` writer - event_f2 will trigger one-shot mode action. 0: disable, 1: enable"]
pub struct TZ2_F2_OST_W<'a> {
    w: &'a mut W,
}
impl<'a> TZ2_F2_OST_W<'a> {
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
#[doc = "Field `TZ2_F1_OST` reader - event_f1 will trigger one-shot mode action. 0: disable, 1: enable"]
pub struct TZ2_F1_OST_R(crate::FieldReader<bool, bool>);
impl TZ2_F1_OST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TZ2_F1_OST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZ2_F1_OST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZ2_F1_OST` writer - event_f1 will trigger one-shot mode action. 0: disable, 1: enable"]
pub struct TZ2_F1_OST_W<'a> {
    w: &'a mut W,
}
impl<'a> TZ2_F1_OST_W<'a> {
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
#[doc = "Field `TZ2_F0_OST` reader - event_f0 will trigger one-shot mode action. 0: disable, 1: enable"]
pub struct TZ2_F0_OST_R(crate::FieldReader<bool, bool>);
impl TZ2_F0_OST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TZ2_F0_OST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZ2_F0_OST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZ2_F0_OST` writer - event_f0 will trigger one-shot mode action. 0: disable, 1: enable"]
pub struct TZ2_F0_OST_W<'a> {
    w: &'a mut W,
}
impl<'a> TZ2_F0_OST_W<'a> {
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
#[doc = "Field `TZ2_A_CBC_D` reader - Cycle-by-cycle mode action on PWM2A when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub struct TZ2_A_CBC_D_R(crate::FieldReader<u8, u8>);
impl TZ2_A_CBC_D_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TZ2_A_CBC_D_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZ2_A_CBC_D_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZ2_A_CBC_D` writer - Cycle-by-cycle mode action on PWM2A when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub struct TZ2_A_CBC_D_W<'a> {
    w: &'a mut W,
}
impl<'a> TZ2_A_CBC_D_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `TZ2_A_CBC_U` reader - Cycle-by-cycle mode action on PWM2A when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub struct TZ2_A_CBC_U_R(crate::FieldReader<u8, u8>);
impl TZ2_A_CBC_U_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TZ2_A_CBC_U_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZ2_A_CBC_U_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZ2_A_CBC_U` writer - Cycle-by-cycle mode action on PWM2A when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub struct TZ2_A_CBC_U_W<'a> {
    w: &'a mut W,
}
impl<'a> TZ2_A_CBC_U_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `TZ2_A_OST_D` reader - One-shot mode action on PWM2A when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub struct TZ2_A_OST_D_R(crate::FieldReader<u8, u8>);
impl TZ2_A_OST_D_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TZ2_A_OST_D_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZ2_A_OST_D_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZ2_A_OST_D` writer - One-shot mode action on PWM2A when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub struct TZ2_A_OST_D_W<'a> {
    w: &'a mut W,
}
impl<'a> TZ2_A_OST_D_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `TZ2_A_OST_U` reader - One-shot mode action on PWM2A when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub struct TZ2_A_OST_U_R(crate::FieldReader<u8, u8>);
impl TZ2_A_OST_U_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TZ2_A_OST_U_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZ2_A_OST_U_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZ2_A_OST_U` writer - One-shot mode action on PWM2A when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub struct TZ2_A_OST_U_W<'a> {
    w: &'a mut W,
}
impl<'a> TZ2_A_OST_U_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `TZ2_B_CBC_D` reader - Cycle-by-cycle mode action on PWM2B when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub struct TZ2_B_CBC_D_R(crate::FieldReader<u8, u8>);
impl TZ2_B_CBC_D_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TZ2_B_CBC_D_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZ2_B_CBC_D_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZ2_B_CBC_D` writer - Cycle-by-cycle mode action on PWM2B when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub struct TZ2_B_CBC_D_W<'a> {
    w: &'a mut W,
}
impl<'a> TZ2_B_CBC_D_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `TZ2_B_CBC_U` reader - Cycle-by-cycle mode action on PWM2B when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub struct TZ2_B_CBC_U_R(crate::FieldReader<u8, u8>);
impl TZ2_B_CBC_U_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TZ2_B_CBC_U_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZ2_B_CBC_U_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZ2_B_CBC_U` writer - Cycle-by-cycle mode action on PWM2B when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub struct TZ2_B_CBC_U_W<'a> {
    w: &'a mut W,
}
impl<'a> TZ2_B_CBC_U_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `TZ2_B_OST_D` reader - One-shot mode action on PWM2B when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub struct TZ2_B_OST_D_R(crate::FieldReader<u8, u8>);
impl TZ2_B_OST_D_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TZ2_B_OST_D_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZ2_B_OST_D_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZ2_B_OST_D` writer - One-shot mode action on PWM2B when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub struct TZ2_B_OST_D_W<'a> {
    w: &'a mut W,
}
impl<'a> TZ2_B_OST_D_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `TZ2_B_OST_U` reader - One-shot mode action on PWM2B when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub struct TZ2_B_OST_U_R(crate::FieldReader<u8, u8>);
impl TZ2_B_OST_U_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TZ2_B_OST_U_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZ2_B_OST_U_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZ2_B_OST_U` writer - One-shot mode action on PWM2B when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
pub struct TZ2_B_OST_U_W<'a> {
    w: &'a mut W,
}
impl<'a> TZ2_B_OST_U_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable register for software force cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz2_sw_cbc(&self) -> TZ2_SW_CBC_R {
        TZ2_SW_CBC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - event_f2 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz2_f2_cbc(&self) -> TZ2_F2_CBC_R {
        TZ2_F2_CBC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - event_f1 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz2_f1_cbc(&self) -> TZ2_F1_CBC_R {
        TZ2_F1_CBC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - event_f0 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz2_f0_cbc(&self) -> TZ2_F0_CBC_R {
        TZ2_F0_CBC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable register for software force one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz2_sw_ost(&self) -> TZ2_SW_OST_R {
        TZ2_SW_OST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - event_f2 will trigger one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz2_f2_ost(&self) -> TZ2_F2_OST_R {
        TZ2_F2_OST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - event_f1 will trigger one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz2_f1_ost(&self) -> TZ2_F1_OST_R {
        TZ2_F1_OST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - event_f0 will trigger one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz2_f0_ost(&self) -> TZ2_F0_OST_R {
        TZ2_F0_OST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Cycle-by-cycle mode action on PWM2A when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz2_a_cbc_d(&self) -> TZ2_A_CBC_D_R {
        TZ2_A_CBC_D_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Cycle-by-cycle mode action on PWM2A when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz2_a_cbc_u(&self) -> TZ2_A_CBC_U_R {
        TZ2_A_CBC_U_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - One-shot mode action on PWM2A when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz2_a_ost_d(&self) -> TZ2_A_OST_D_R {
        TZ2_A_OST_D_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - One-shot mode action on PWM2A when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz2_a_ost_u(&self) -> TZ2_A_OST_U_R {
        TZ2_A_OST_U_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Cycle-by-cycle mode action on PWM2B when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz2_b_cbc_d(&self) -> TZ2_B_CBC_D_R {
        TZ2_B_CBC_D_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Cycle-by-cycle mode action on PWM2B when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz2_b_cbc_u(&self) -> TZ2_B_CBC_U_R {
        TZ2_B_CBC_U_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - One-shot mode action on PWM2B when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz2_b_ost_d(&self) -> TZ2_B_OST_D_R {
        TZ2_B_OST_D_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - One-shot mode action on PWM2B when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz2_b_ost_u(&self) -> TZ2_B_OST_U_R {
        TZ2_B_OST_U_R::new(((self.bits >> 22) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable register for software force cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz2_sw_cbc(&mut self) -> TZ2_SW_CBC_W {
        TZ2_SW_CBC_W { w: self }
    }
    #[doc = "Bit 1 - event_f2 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz2_f2_cbc(&mut self) -> TZ2_F2_CBC_W {
        TZ2_F2_CBC_W { w: self }
    }
    #[doc = "Bit 2 - event_f1 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz2_f1_cbc(&mut self) -> TZ2_F1_CBC_W {
        TZ2_F1_CBC_W { w: self }
    }
    #[doc = "Bit 3 - event_f0 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz2_f0_cbc(&mut self) -> TZ2_F0_CBC_W {
        TZ2_F0_CBC_W { w: self }
    }
    #[doc = "Bit 4 - Enable register for software force one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz2_sw_ost(&mut self) -> TZ2_SW_OST_W {
        TZ2_SW_OST_W { w: self }
    }
    #[doc = "Bit 5 - event_f2 will trigger one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz2_f2_ost(&mut self) -> TZ2_F2_OST_W {
        TZ2_F2_OST_W { w: self }
    }
    #[doc = "Bit 6 - event_f1 will trigger one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz2_f1_ost(&mut self) -> TZ2_F1_OST_W {
        TZ2_F1_OST_W { w: self }
    }
    #[doc = "Bit 7 - event_f0 will trigger one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz2_f0_ost(&mut self) -> TZ2_F0_OST_W {
        TZ2_F0_OST_W { w: self }
    }
    #[doc = "Bits 8:9 - Cycle-by-cycle mode action on PWM2A when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz2_a_cbc_d(&mut self) -> TZ2_A_CBC_D_W {
        TZ2_A_CBC_D_W { w: self }
    }
    #[doc = "Bits 10:11 - Cycle-by-cycle mode action on PWM2A when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz2_a_cbc_u(&mut self) -> TZ2_A_CBC_U_W {
        TZ2_A_CBC_U_W { w: self }
    }
    #[doc = "Bits 12:13 - One-shot mode action on PWM2A when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz2_a_ost_d(&mut self) -> TZ2_A_OST_D_W {
        TZ2_A_OST_D_W { w: self }
    }
    #[doc = "Bits 14:15 - One-shot mode action on PWM2A when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz2_a_ost_u(&mut self) -> TZ2_A_OST_U_W {
        TZ2_A_OST_U_W { w: self }
    }
    #[doc = "Bits 16:17 - Cycle-by-cycle mode action on PWM2B when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz2_b_cbc_d(&mut self) -> TZ2_B_CBC_D_W {
        TZ2_B_CBC_D_W { w: self }
    }
    #[doc = "Bits 18:19 - Cycle-by-cycle mode action on PWM2B when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz2_b_cbc_u(&mut self) -> TZ2_B_CBC_U_W {
        TZ2_B_CBC_U_W { w: self }
    }
    #[doc = "Bits 20:21 - One-shot mode action on PWM2B when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz2_b_ost_d(&mut self) -> TZ2_B_OST_D_W {
        TZ2_B_OST_D_W { w: self }
    }
    #[doc = "Bits 22:23 - One-shot mode action on PWM2B when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle"]
    #[inline(always)]
    pub fn tz2_b_ost_u(&mut self) -> TZ2_B_OST_U_W {
        TZ2_B_OST_U_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Actions on PWM2A and PWM2B trip events\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tz2_cfg0]
(index.html) module"]
pub struct TZ2_CFG0_SPEC;
impl crate::RegisterSpec for TZ2_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tz2_cfg0::R]
(R) reader structure"]
impl crate::Readable for TZ2_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tz2_cfg0::W]
(W) writer structure"]
impl crate::Writable for TZ2_CFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TZ2_CFG0 to value 0"]
impl crate::Resettable for TZ2_CFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
