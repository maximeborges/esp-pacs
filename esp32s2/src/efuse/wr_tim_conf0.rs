#[doc = "Register `WR_TIM_CONF0` reader"]
pub struct R(crate::R<WR_TIM_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WR_TIM_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WR_TIM_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WR_TIM_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WR_TIM_CONF0` writer"]
pub struct W(crate::W<WR_TIM_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WR_TIM_CONF0_SPEC>;
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
impl From<crate::W<WR_TIM_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WR_TIM_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THP_A` reader - Configures the hold time of programming operation."]
pub struct THP_A_R(crate::FieldReader<u8>);
impl THP_A_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        THP_A_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THP_A_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THP_A` writer - Configures the hold time of programming operation."]
pub struct THP_A_W<'a> {
    w: &'a mut W,
}
impl<'a> THP_A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `TPGM_INACTIVE` reader - Configures the length of pulse during programming 0 to eFuse."]
pub struct TPGM_INACTIVE_R(crate::FieldReader<u8>);
impl TPGM_INACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TPGM_INACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPGM_INACTIVE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPGM_INACTIVE` writer - Configures the length of pulse during programming 0 to eFuse."]
pub struct TPGM_INACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> TPGM_INACTIVE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `TPGM` reader - Configures the length of pulse during programming 1 to eFuse."]
pub struct TPGM_R(crate::FieldReader<u16>);
impl TPGM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TPGM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPGM_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPGM` writer - Configures the length of pulse during programming 1 to eFuse."]
pub struct TPGM_W<'a> {
    w: &'a mut W,
}
impl<'a> TPGM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Configures the hold time of programming operation."]
    #[inline(always)]
    pub fn thp_a(&self) -> THP_A_R {
        THP_A_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Configures the length of pulse during programming 0 to eFuse."]
    #[inline(always)]
    pub fn tpgm_inactive(&self) -> TPGM_INACTIVE_R {
        TPGM_INACTIVE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Configures the length of pulse during programming 1 to eFuse."]
    #[inline(always)]
    pub fn tpgm(&self) -> TPGM_R {
        TPGM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures the hold time of programming operation."]
    #[inline(always)]
    pub fn thp_a(&mut self) -> THP_A_W {
        THP_A_W { w: self }
    }
    #[doc = "Bits 8:15 - Configures the length of pulse during programming 0 to eFuse."]
    #[inline(always)]
    pub fn tpgm_inactive(&mut self) -> TPGM_INACTIVE_W {
        TPGM_INACTIVE_W { w: self }
    }
    #[doc = "Bits 16:31 - Configures the length of pulse during programming 1 to eFuse."]
    #[inline(always)]
    pub fn tpgm(&mut self) -> TPGM_W {
        TPGM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register 0 of eFuse programming timing parameters.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr_tim_conf0](index.html) module"]
pub struct WR_TIM_CONF0_SPEC;
impl crate::RegisterSpec for WR_TIM_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wr_tim_conf0::R](R) reader structure"]
impl crate::Readable for WR_TIM_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wr_tim_conf0::W](W) writer structure"]
impl crate::Writable for WR_TIM_CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WR_TIM_CONF0 to value 0x00c8_0101"]
impl crate::Resettable for WR_TIM_CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00c8_0101
    }
}
