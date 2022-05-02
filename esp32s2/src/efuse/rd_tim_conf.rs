#[doc = "Register `RD_TIM_CONF` reader"]
pub struct R(crate::R<RD_TIM_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_TIM_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_TIM_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_TIM_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RD_TIM_CONF` writer"]
pub struct W(crate::W<RD_TIM_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RD_TIM_CONF_SPEC>;
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
impl From<crate::W<RD_TIM_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RD_TIM_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THR_A` reader - Configures the hold time of read operation."]
pub struct THR_A_R(crate::FieldReader<u8>);
impl THR_A_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        THR_A_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THR_A_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THR_A` writer - Configures the hold time of read operation."]
pub struct THR_A_W<'a> {
    w: &'a mut W,
}
impl<'a> THR_A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `TRD` reader - Configures the length of pulse of read operation."]
pub struct TRD_R(crate::FieldReader<u8>);
impl TRD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRD_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRD` writer - Configures the length of pulse of read operation."]
pub struct TRD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `TSUR_A` reader - Configures the setup time of read operation."]
pub struct TSUR_A_R(crate::FieldReader<u8>);
impl TSUR_A_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TSUR_A_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSUR_A_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSUR_A` writer - Configures the setup time of read operation."]
pub struct TSUR_A_W<'a> {
    w: &'a mut W,
}
impl<'a> TSUR_A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `READ_INIT_NUM` reader - Configures the initial read time of eFuse."]
pub struct READ_INIT_NUM_R(crate::FieldReader<u8>);
impl READ_INIT_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        READ_INIT_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for READ_INIT_NUM_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READ_INIT_NUM` writer - Configures the initial read time of eFuse."]
pub struct READ_INIT_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_INIT_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Configures the hold time of read operation."]
    #[inline(always)]
    pub fn thr_a(&self) -> THR_A_R {
        THR_A_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Configures the length of pulse of read operation."]
    #[inline(always)]
    pub fn trd(&self) -> TRD_R {
        TRD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Configures the setup time of read operation."]
    #[inline(always)]
    pub fn tsur_a(&self) -> TSUR_A_R {
        TSUR_A_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Configures the initial read time of eFuse."]
    #[inline(always)]
    pub fn read_init_num(&self) -> READ_INIT_NUM_R {
        READ_INIT_NUM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures the hold time of read operation."]
    #[inline(always)]
    pub fn thr_a(&mut self) -> THR_A_W {
        THR_A_W { w: self }
    }
    #[doc = "Bits 8:15 - Configures the length of pulse of read operation."]
    #[inline(always)]
    pub fn trd(&mut self) -> TRD_W {
        TRD_W { w: self }
    }
    #[doc = "Bits 16:23 - Configures the setup time of read operation."]
    #[inline(always)]
    pub fn tsur_a(&mut self) -> TSUR_A_W {
        TSUR_A_W { w: self }
    }
    #[doc = "Bits 24:31 - Configures the initial read time of eFuse."]
    #[inline(always)]
    pub fn read_init_num(&mut self) -> READ_INIT_NUM_W {
        READ_INIT_NUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configures read timing parameters.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_tim_conf](index.html) module"]
pub struct RD_TIM_CONF_SPEC;
impl crate::RegisterSpec for RD_TIM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_tim_conf::R](R) reader structure"]
impl crate::Readable for RD_TIM_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rd_tim_conf::W](W) writer structure"]
impl crate::Writable for RD_TIM_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RD_TIM_CONF to value 0x1201_0101"]
impl crate::Resettable for RD_TIM_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1201_0101
    }
}
