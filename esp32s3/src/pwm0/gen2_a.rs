#[doc = "Register `GEN2_A` reader"]
pub struct R(crate::R<GEN2_A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GEN2_A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GEN2_A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GEN2_A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GEN2_A` writer"]
pub struct W(crate::W<GEN2_A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GEN2_A_SPEC>;
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
impl From<crate::W<GEN2_A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GEN2_A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UTEZ` reader - Action on PWM2A triggered by event TEZ when timer increasing"]
pub struct UTEZ_R(crate::FieldReader<u8, u8>);
impl UTEZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UTEZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UTEZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UTEZ` writer - Action on PWM2A triggered by event TEZ when timer increasing"]
pub struct UTEZ_W<'a> {
    w: &'a mut W,
}
impl<'a> UTEZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `UTEP` reader - Action on PWM2A triggered by event TEP when timer increasing"]
pub struct UTEP_R(crate::FieldReader<u8, u8>);
impl UTEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UTEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UTEP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UTEP` writer - Action on PWM2A triggered by event TEP when timer increasing"]
pub struct UTEP_W<'a> {
    w: &'a mut W,
}
impl<'a> UTEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 2)) | ((value as u32 & 3) << 2);
        self.w
    }
}
#[doc = "Field `UTEA` reader - Action on PWM2A triggered by event TEA when timer increasing"]
pub struct UTEA_R(crate::FieldReader<u8, u8>);
impl UTEA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UTEA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UTEA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UTEA` writer - Action on PWM2A triggered by event TEA when timer increasing"]
pub struct UTEA_W<'a> {
    w: &'a mut W,
}
impl<'a> UTEA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "Field `UTEB` reader - Action on PWM2A triggered by event TEB when timer increasing"]
pub struct UTEB_R(crate::FieldReader<u8, u8>);
impl UTEB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UTEB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UTEB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UTEB` writer - Action on PWM2A triggered by event TEB when timer increasing"]
pub struct UTEB_W<'a> {
    w: &'a mut W,
}
impl<'a> UTEB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u32 & 3) << 6);
        self.w
    }
}
#[doc = "Field `UT0` reader - Action on PWM2A triggered by event_t0 when timer increasing"]
pub struct UT0_R(crate::FieldReader<u8, u8>);
impl UT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UT0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UT0` writer - Action on PWM2A triggered by event_t0 when timer increasing"]
pub struct UT0_W<'a> {
    w: &'a mut W,
}
impl<'a> UT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 8)) | ((value as u32 & 3) << 8);
        self.w
    }
}
#[doc = "Field `UT1` reader - Action on PWM2A triggered by event_t1 when timer increasing"]
pub struct UT1_R(crate::FieldReader<u8, u8>);
impl UT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UT1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UT1` writer - Action on PWM2A triggered by event_t1 when timer increasing"]
pub struct UT1_W<'a> {
    w: &'a mut W,
}
impl<'a> UT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 10)) | ((value as u32 & 3) << 10);
        self.w
    }
}
#[doc = "Field `DTEZ` reader - Action on PWM2A triggered by event TEZ when timer decreasing"]
pub struct DTEZ_R(crate::FieldReader<u8, u8>);
impl DTEZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DTEZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTEZ` writer - Action on PWM2A triggered by event TEZ when timer decreasing"]
pub struct DTEZ_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 12)) | ((value as u32 & 3) << 12);
        self.w
    }
}
#[doc = "Field `DTEP` reader - Action on PWM2A triggered by event TEP when timer decreasing"]
pub struct DTEP_R(crate::FieldReader<u8, u8>);
impl DTEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DTEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTEP` writer - Action on PWM2A triggered by event TEP when timer decreasing"]
pub struct DTEP_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 14)) | ((value as u32 & 3) << 14);
        self.w
    }
}
#[doc = "Field `DTEA` reader - Action on PWM2A triggered by event TEA when timer decreasing"]
pub struct DTEA_R(crate::FieldReader<u8, u8>);
impl DTEA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DTEA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTEA` writer - Action on PWM2A triggered by event TEA when timer decreasing"]
pub struct DTEA_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 16)) | ((value as u32 & 3) << 16);
        self.w
    }
}
#[doc = "Field `DTEB` reader - Action on PWM2A triggered by event TEB when timer decreasing"]
pub struct DTEB_R(crate::FieldReader<u8, u8>);
impl DTEB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DTEB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTEB` writer - Action on PWM2A triggered by event TEB when timer decreasing"]
pub struct DTEB_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 18)) | ((value as u32 & 3) << 18);
        self.w
    }
}
#[doc = "Field `DT0` reader - Action on PWM2A triggered by event_t0 when timer decreasing"]
pub struct DT0_R(crate::FieldReader<u8, u8>);
impl DT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT0` writer - Action on PWM2A triggered by event_t0 when timer decreasing"]
pub struct DT0_W<'a> {
    w: &'a mut W,
}
impl<'a> DT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 20)) | ((value as u32 & 3) << 20);
        self.w
    }
}
#[doc = "Field `DT1` reader - Action on PWM2A triggered by event_t1 when timer decreasing. 0: no change, 1: low, 2: high, 3: toggle"]
pub struct DT1_R(crate::FieldReader<u8, u8>);
impl DT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT1` writer - Action on PWM2A triggered by event_t1 when timer decreasing. 0: no change, 1: low, 2: high, 3: toggle"]
pub struct DT1_W<'a> {
    w: &'a mut W,
}
impl<'a> DT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 22)) | ((value as u32 & 3) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Action on PWM2A triggered by event TEZ when timer increasing"]
    #[inline(always)]
    pub fn utez(&self) -> UTEZ_R {
        UTEZ_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Action on PWM2A triggered by event TEP when timer increasing"]
    #[inline(always)]
    pub fn utep(&self) -> UTEP_R {
        UTEP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Action on PWM2A triggered by event TEA when timer increasing"]
    #[inline(always)]
    pub fn utea(&self) -> UTEA_R {
        UTEA_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Action on PWM2A triggered by event TEB when timer increasing"]
    #[inline(always)]
    pub fn uteb(&self) -> UTEB_R {
        UTEB_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Action on PWM2A triggered by event_t0 when timer increasing"]
    #[inline(always)]
    pub fn ut0(&self) -> UT0_R {
        UT0_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Action on PWM2A triggered by event_t1 when timer increasing"]
    #[inline(always)]
    pub fn ut1(&self) -> UT1_R {
        UT1_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Action on PWM2A triggered by event TEZ when timer decreasing"]
    #[inline(always)]
    pub fn dtez(&self) -> DTEZ_R {
        DTEZ_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Action on PWM2A triggered by event TEP when timer decreasing"]
    #[inline(always)]
    pub fn dtep(&self) -> DTEP_R {
        DTEP_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Action on PWM2A triggered by event TEA when timer decreasing"]
    #[inline(always)]
    pub fn dtea(&self) -> DTEA_R {
        DTEA_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Action on PWM2A triggered by event TEB when timer decreasing"]
    #[inline(always)]
    pub fn dteb(&self) -> DTEB_R {
        DTEB_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Action on PWM2A triggered by event_t0 when timer decreasing"]
    #[inline(always)]
    pub fn dt0(&self) -> DT0_R {
        DT0_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Action on PWM2A triggered by event_t1 when timer decreasing. 0: no change, 1: low, 2: high, 3: toggle"]
    #[inline(always)]
    pub fn dt1(&self) -> DT1_R {
        DT1_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Action on PWM2A triggered by event TEZ when timer increasing"]
    #[inline(always)]
    pub fn utez(&mut self) -> UTEZ_W {
        UTEZ_W { w: self }
    }
    #[doc = "Bits 2:3 - Action on PWM2A triggered by event TEP when timer increasing"]
    #[inline(always)]
    pub fn utep(&mut self) -> UTEP_W {
        UTEP_W { w: self }
    }
    #[doc = "Bits 4:5 - Action on PWM2A triggered by event TEA when timer increasing"]
    #[inline(always)]
    pub fn utea(&mut self) -> UTEA_W {
        UTEA_W { w: self }
    }
    #[doc = "Bits 6:7 - Action on PWM2A triggered by event TEB when timer increasing"]
    #[inline(always)]
    pub fn uteb(&mut self) -> UTEB_W {
        UTEB_W { w: self }
    }
    #[doc = "Bits 8:9 - Action on PWM2A triggered by event_t0 when timer increasing"]
    #[inline(always)]
    pub fn ut0(&mut self) -> UT0_W {
        UT0_W { w: self }
    }
    #[doc = "Bits 10:11 - Action on PWM2A triggered by event_t1 when timer increasing"]
    #[inline(always)]
    pub fn ut1(&mut self) -> UT1_W {
        UT1_W { w: self }
    }
    #[doc = "Bits 12:13 - Action on PWM2A triggered by event TEZ when timer decreasing"]
    #[inline(always)]
    pub fn dtez(&mut self) -> DTEZ_W {
        DTEZ_W { w: self }
    }
    #[doc = "Bits 14:15 - Action on PWM2A triggered by event TEP when timer decreasing"]
    #[inline(always)]
    pub fn dtep(&mut self) -> DTEP_W {
        DTEP_W { w: self }
    }
    #[doc = "Bits 16:17 - Action on PWM2A triggered by event TEA when timer decreasing"]
    #[inline(always)]
    pub fn dtea(&mut self) -> DTEA_W {
        DTEA_W { w: self }
    }
    #[doc = "Bits 18:19 - Action on PWM2A triggered by event TEB when timer decreasing"]
    #[inline(always)]
    pub fn dteb(&mut self) -> DTEB_W {
        DTEB_W { w: self }
    }
    #[doc = "Bits 20:21 - Action on PWM2A triggered by event_t0 when timer decreasing"]
    #[inline(always)]
    pub fn dt0(&mut self) -> DT0_W {
        DT0_W { w: self }
    }
    #[doc = "Bits 22:23 - Action on PWM2A triggered by event_t1 when timer decreasing. 0: no change, 1: low, 2: high, 3: toggle"]
    #[inline(always)]
    pub fn dt1(&mut self) -> DT1_W {
        DT1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Actions triggered by events on PWM2A\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gen2_a]
(index.html) module"]
pub struct GEN2_A_SPEC;
impl crate::RegisterSpec for GEN2_A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gen2_a::R]
(R) reader structure"]
impl crate::Readable for GEN2_A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gen2_a::W]
(W) writer structure"]
impl crate::Writable for GEN2_A_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GEN2_A to value 0"]
impl crate::Resettable for GEN2_A_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
