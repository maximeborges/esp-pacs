#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_2` reader"]
pub struct R(crate::R<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_2` writer"]
pub struct W(crate::W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC>;
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
impl From<crate::W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BT` reader - BackUp access bt permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_BT_R(crate::FieldReader<u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_BT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_BT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_BT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BT` writer - BackUp access bt permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_BT_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_BT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0` reader - BackUp access i2c_ext0 permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_R(crate::FieldReader<u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0` writer - BackUp access i2c_ext0 permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_UHCI0` reader - BackUp access uhci0 permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_UHCI0_R(crate::FieldReader<u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_UHCI0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_UHCI0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_UHCI0_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_UHCI0` writer - BackUp access uhci0 permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_UHCI0_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_UHCI0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u32 & 3) << 6);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SLCHOST` reader - BackUp access slchost permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_SLCHOST_R(crate::FieldReader<u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_SLCHOST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_SLCHOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_SLCHOST_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SLCHOST` writer - BackUp access slchost permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_SLCHOST_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_SLCHOST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 8)) | ((value as u32 & 3) << 8);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RMT` reader - BackUp access rmt permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_RMT_R(crate::FieldReader<u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_RMT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_RMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_RMT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RMT` writer - BackUp access rmt permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_RMT_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_RMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 10)) | ((value as u32 & 3) << 10);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_PCNT` reader - BackUp access pcnt permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_PCNT_R(crate::FieldReader<u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_PCNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_PCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_PCNT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_PCNT` writer - BackUp access pcnt permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_PCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_PCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 12)) | ((value as u32 & 3) << 12);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SLC` reader - BackUp access slc permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_SLC_R(crate::FieldReader<u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_SLC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_SLC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_SLC_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SLC` writer - BackUp access slc permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_SLC_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_SLC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 14)) | ((value as u32 & 3) << 14);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_LEDC` reader - BackUp access ledc permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_LEDC_R(crate::FieldReader<u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_LEDC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_LEDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_LEDC_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_LEDC` writer - BackUp access ledc permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_LEDC_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_LEDC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 16)) | ((value as u32 & 3) << 16);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BACKUP` reader - BackUp access backup permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_BACKUP_R(crate::FieldReader<u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_BACKUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_BACKUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_BACKUP_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BACKUP` writer - BackUp access backup permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_BACKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_BACKUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 18)) | ((value as u32 & 3) << 18);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BB` reader - BackUp access bb permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_BB_R(crate::FieldReader<u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_BB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_BB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_BB_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BB` writer - BackUp access bb permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_BB_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_BB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 22)) | ((value as u32 & 3) << 22);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_PWM0` reader - BackUp access pwm0 permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_PWM0_R(crate::FieldReader<u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_PWM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_PWM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_PWM0_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_PWM0` writer - BackUp access pwm0 permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_PWM0_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_PWM0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 24)) | ((value as u32 & 3) << 24);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP` reader - BackUp access timergroup permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_R(crate::FieldReader<u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP` writer - BackUp access timergroup permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 26)) | ((value as u32 & 3) << 26);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1` reader - BackUp access timergroup1 permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_R(crate::FieldReader<u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1` writer - BackUp access timergroup1 permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 28)) | ((value as u32 & 3) << 28);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER` reader - BackUp access systimer permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_R(crate::FieldReader<u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER` writer - BackUp access systimer permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 30)) | ((value as u32 & 3) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - BackUp access bt permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_bt(&self) -> BACKUP_BUS_PMS_CONSTRAIN_BT_R {
        BACKUP_BUS_PMS_CONSTRAIN_BT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - BackUp access i2c_ext0 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_i2c_ext0(&self) -> BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_R {
        BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - BackUp access uhci0 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_uhci0(&self) -> BACKUP_BUS_PMS_CONSTRAIN_UHCI0_R {
        BACKUP_BUS_PMS_CONSTRAIN_UHCI0_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - BackUp access slchost permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_slchost(&self) -> BACKUP_BUS_PMS_CONSTRAIN_SLCHOST_R {
        BACKUP_BUS_PMS_CONSTRAIN_SLCHOST_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - BackUp access rmt permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_rmt(&self) -> BACKUP_BUS_PMS_CONSTRAIN_RMT_R {
        BACKUP_BUS_PMS_CONSTRAIN_RMT_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - BackUp access pcnt permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_pcnt(&self) -> BACKUP_BUS_PMS_CONSTRAIN_PCNT_R {
        BACKUP_BUS_PMS_CONSTRAIN_PCNT_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - BackUp access slc permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_slc(&self) -> BACKUP_BUS_PMS_CONSTRAIN_SLC_R {
        BACKUP_BUS_PMS_CONSTRAIN_SLC_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - BackUp access ledc permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_ledc(&self) -> BACKUP_BUS_PMS_CONSTRAIN_LEDC_R {
        BACKUP_BUS_PMS_CONSTRAIN_LEDC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - BackUp access backup permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_backup(&self) -> BACKUP_BUS_PMS_CONSTRAIN_BACKUP_R {
        BACKUP_BUS_PMS_CONSTRAIN_BACKUP_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 22:23 - BackUp access bb permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_bb(&self) -> BACKUP_BUS_PMS_CONSTRAIN_BB_R {
        BACKUP_BUS_PMS_CONSTRAIN_BB_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - BackUp access pwm0 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_pwm0(&self) -> BACKUP_BUS_PMS_CONSTRAIN_PWM0_R {
        BACKUP_BUS_PMS_CONSTRAIN_PWM0_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - BackUp access timergroup permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_timergroup(&self) -> BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_R {
        BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - BackUp access timergroup1 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_timergroup1(&self) -> BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_R {
        BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - BackUp access systimer permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_systimer(&self) -> BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_R {
        BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - BackUp access bt permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_bt(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_BT_W {
        BACKUP_BUS_PMS_CONSTRAIN_BT_W { w: self }
    }
    #[doc = "Bits 4:5 - BackUp access i2c_ext0 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_i2c_ext0(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_W {
        BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_W { w: self }
    }
    #[doc = "Bits 6:7 - BackUp access uhci0 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_uhci0(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_UHCI0_W {
        BACKUP_BUS_PMS_CONSTRAIN_UHCI0_W { w: self }
    }
    #[doc = "Bits 8:9 - BackUp access slchost permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_slchost(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_SLCHOST_W {
        BACKUP_BUS_PMS_CONSTRAIN_SLCHOST_W { w: self }
    }
    #[doc = "Bits 10:11 - BackUp access rmt permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_rmt(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_RMT_W {
        BACKUP_BUS_PMS_CONSTRAIN_RMT_W { w: self }
    }
    #[doc = "Bits 12:13 - BackUp access pcnt permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_pcnt(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_PCNT_W {
        BACKUP_BUS_PMS_CONSTRAIN_PCNT_W { w: self }
    }
    #[doc = "Bits 14:15 - BackUp access slc permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_slc(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_SLC_W {
        BACKUP_BUS_PMS_CONSTRAIN_SLC_W { w: self }
    }
    #[doc = "Bits 16:17 - BackUp access ledc permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_ledc(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_LEDC_W {
        BACKUP_BUS_PMS_CONSTRAIN_LEDC_W { w: self }
    }
    #[doc = "Bits 18:19 - BackUp access backup permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_backup(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_BACKUP_W {
        BACKUP_BUS_PMS_CONSTRAIN_BACKUP_W { w: self }
    }
    #[doc = "Bits 22:23 - BackUp access bb permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_bb(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_BB_W {
        BACKUP_BUS_PMS_CONSTRAIN_BB_W { w: self }
    }
    #[doc = "Bits 24:25 - BackUp access pwm0 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_pwm0(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_PWM0_W {
        BACKUP_BUS_PMS_CONSTRAIN_PWM0_W { w: self }
    }
    #[doc = "Bits 26:27 - BackUp access timergroup permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_timergroup(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_W {
        BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_W { w: self }
    }
    #[doc = "Bits 28:29 - BackUp access timergroup1 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_timergroup1(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_W {
        BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_W { w: self }
    }
    #[doc = "Bits 30:31 - BackUp access systimer permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_systimer(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_W {
        BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BackUp access peripherals permission configuration register 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backup_bus_pms_constrain_2](index.html) module"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_2_SPEC;
impl crate::RegisterSpec for BACKUP_BUS_PMS_CONSTRAIN_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [backup_bus_pms_constrain_2::R](R) reader structure"]
impl crate::Readable for BACKUP_BUS_PMS_CONSTRAIN_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [backup_bus_pms_constrain_2::W](W) writer structure"]
impl crate::Writable for BACKUP_BUS_PMS_CONSTRAIN_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BACKUP_BUS_PMS_CONSTRAIN_2 to value 0xffcf_fff3"]
impl crate::Resettable for BACKUP_BUS_PMS_CONSTRAIN_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffcf_fff3
    }
}
