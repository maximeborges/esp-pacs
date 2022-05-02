#[doc = "Register `TOUCH_SCAN_CTRL` reader"]
pub struct R(crate::R<TOUCH_SCAN_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_SCAN_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_SCAN_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_SCAN_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOUCH_SCAN_CTRL` writer"]
pub struct W(crate::W<TOUCH_SCAN_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_SCAN_CTRL_SPEC>;
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
impl From<crate::W<TOUCH_SCAN_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_SCAN_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_DENOISE_RES` reader - De-noise resolution: 12/10/8/4 bit"]
pub struct TOUCH_DENOISE_RES_R(crate::FieldReader<u8>);
impl TOUCH_DENOISE_RES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_DENOISE_RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_DENOISE_RES_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_DENOISE_RES` writer - De-noise resolution: 12/10/8/4 bit"]
pub struct TOUCH_DENOISE_RES_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_DENOISE_RES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `TOUCH_DENOISE_EN` reader - touch pad0 will be used to de-noise"]
pub struct TOUCH_DENOISE_EN_R(crate::FieldReader<bool>);
impl TOUCH_DENOISE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_DENOISE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_DENOISE_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_DENOISE_EN` writer - touch pad0 will be used to de-noise"]
pub struct TOUCH_DENOISE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_DENOISE_EN_W<'a> {
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
#[doc = "Field `TOUCH_INACTIVE_CONNECTION` reader - inactive touch pads connect to 1: gnd 0: HighZ"]
pub struct TOUCH_INACTIVE_CONNECTION_R(crate::FieldReader<bool>);
impl TOUCH_INACTIVE_CONNECTION_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_INACTIVE_CONNECTION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_INACTIVE_CONNECTION_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_INACTIVE_CONNECTION` writer - inactive touch pads connect to 1: gnd 0: HighZ"]
pub struct TOUCH_INACTIVE_CONNECTION_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_INACTIVE_CONNECTION_W<'a> {
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
#[doc = "Field `TOUCH_SHIELD_PAD_EN` reader - touch pad14 will be used as shield"]
pub struct TOUCH_SHIELD_PAD_EN_R(crate::FieldReader<bool>);
impl TOUCH_SHIELD_PAD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_SHIELD_PAD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_SHIELD_PAD_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_SHIELD_PAD_EN` writer - touch pad14 will be used as shield"]
pub struct TOUCH_SHIELD_PAD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_SHIELD_PAD_EN_W<'a> {
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
#[doc = "Field `TOUCH_SCAN_PAD_MAP` reader - touch scan mode pad enable map"]
pub struct TOUCH_SCAN_PAD_MAP_R(crate::FieldReader<u16>);
impl TOUCH_SCAN_PAD_MAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TOUCH_SCAN_PAD_MAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_SCAN_PAD_MAP_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_SCAN_PAD_MAP` writer - touch scan mode pad enable map"]
pub struct TOUCH_SCAN_PAD_MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_SCAN_PAD_MAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 10)) | ((value as u32 & 0x7fff) << 10);
        self.w
    }
}
#[doc = "Field `TOUCH_BUFDRV` reader - touch7 buffer driver strength"]
pub struct TOUCH_BUFDRV_R(crate::FieldReader<u8>);
impl TOUCH_BUFDRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_BUFDRV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_BUFDRV_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_BUFDRV` writer - touch7 buffer driver strength"]
pub struct TOUCH_BUFDRV_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_BUFDRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 25)) | ((value as u32 & 7) << 25);
        self.w
    }
}
#[doc = "Field `TOUCH_OUT_RING` reader - select out ring pad"]
pub struct TOUCH_OUT_RING_R(crate::FieldReader<u8>);
impl TOUCH_OUT_RING_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_OUT_RING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_OUT_RING_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_OUT_RING` writer - select out ring pad"]
pub struct TOUCH_OUT_RING_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_OUT_RING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - De-noise resolution: 12/10/8/4 bit"]
    #[inline(always)]
    pub fn touch_denoise_res(&self) -> TOUCH_DENOISE_RES_R {
        TOUCH_DENOISE_RES_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - touch pad0 will be used to de-noise"]
    #[inline(always)]
    pub fn touch_denoise_en(&self) -> TOUCH_DENOISE_EN_R {
        TOUCH_DENOISE_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - inactive touch pads connect to 1: gnd 0: HighZ"]
    #[inline(always)]
    pub fn touch_inactive_connection(&self) -> TOUCH_INACTIVE_CONNECTION_R {
        TOUCH_INACTIVE_CONNECTION_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - touch pad14 will be used as shield"]
    #[inline(always)]
    pub fn touch_shield_pad_en(&self) -> TOUCH_SHIELD_PAD_EN_R {
        TOUCH_SHIELD_PAD_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:24 - touch scan mode pad enable map"]
    #[inline(always)]
    pub fn touch_scan_pad_map(&self) -> TOUCH_SCAN_PAD_MAP_R {
        TOUCH_SCAN_PAD_MAP_R::new(((self.bits >> 10) & 0x7fff) as u16)
    }
    #[doc = "Bits 25:27 - touch7 buffer driver strength"]
    #[inline(always)]
    pub fn touch_bufdrv(&self) -> TOUCH_BUFDRV_R {
        TOUCH_BUFDRV_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:31 - select out ring pad"]
    #[inline(always)]
    pub fn touch_out_ring(&self) -> TOUCH_OUT_RING_R {
        TOUCH_OUT_RING_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - De-noise resolution: 12/10/8/4 bit"]
    #[inline(always)]
    pub fn touch_denoise_res(&mut self) -> TOUCH_DENOISE_RES_W {
        TOUCH_DENOISE_RES_W { w: self }
    }
    #[doc = "Bit 2 - touch pad0 will be used to de-noise"]
    #[inline(always)]
    pub fn touch_denoise_en(&mut self) -> TOUCH_DENOISE_EN_W {
        TOUCH_DENOISE_EN_W { w: self }
    }
    #[doc = "Bit 8 - inactive touch pads connect to 1: gnd 0: HighZ"]
    #[inline(always)]
    pub fn touch_inactive_connection(&mut self) -> TOUCH_INACTIVE_CONNECTION_W {
        TOUCH_INACTIVE_CONNECTION_W { w: self }
    }
    #[doc = "Bit 9 - touch pad14 will be used as shield"]
    #[inline(always)]
    pub fn touch_shield_pad_en(&mut self) -> TOUCH_SHIELD_PAD_EN_W {
        TOUCH_SHIELD_PAD_EN_W { w: self }
    }
    #[doc = "Bits 10:24 - touch scan mode pad enable map"]
    #[inline(always)]
    pub fn touch_scan_pad_map(&mut self) -> TOUCH_SCAN_PAD_MAP_W {
        TOUCH_SCAN_PAD_MAP_W { w: self }
    }
    #[doc = "Bits 25:27 - touch7 buffer driver strength"]
    #[inline(always)]
    pub fn touch_bufdrv(&mut self) -> TOUCH_BUFDRV_W {
        TOUCH_BUFDRV_W { w: self }
    }
    #[doc = "Bits 28:31 - select out ring pad"]
    #[inline(always)]
    pub fn touch_out_ring(&mut self) -> TOUCH_OUT_RING_W {
        TOUCH_OUT_RING_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure touch controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_scan_ctrl](index.html) module"]
pub struct TOUCH_SCAN_CTRL_SPEC;
impl crate::RegisterSpec for TOUCH_SCAN_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch_scan_ctrl::R](R) reader structure"]
impl crate::Readable for TOUCH_SCAN_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch_scan_ctrl::W](W) writer structure"]
impl crate::Writable for TOUCH_SCAN_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TOUCH_SCAN_CTRL to value 0xf000_0102"]
impl crate::Resettable for TOUCH_SCAN_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf000_0102
    }
}
