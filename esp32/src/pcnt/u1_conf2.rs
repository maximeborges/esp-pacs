#[doc = "Register `U1_CONF2` reader"]
pub struct R(crate::R<U1_CONF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U1_CONF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<U1_CONF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<U1_CONF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U1_CONF2` writer"]
pub struct W(crate::W<U1_CONF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U1_CONF2_SPEC>;
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
impl From<crate::W<U1_CONF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<U1_CONF2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT_H_LIM_U1` reader - This register is used to configure thr_h_lim value for unit1."]
pub struct CNT_H_LIM_U1_R(crate::FieldReader<u16, u16>);
impl CNT_H_LIM_U1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CNT_H_LIM_U1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_H_LIM_U1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT_H_LIM_U1` writer - This register is used to configure thr_h_lim value for unit1."]
pub struct CNT_H_LIM_U1_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_H_LIM_U1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `CNT_L_LIM_U1` reader - This register is used to confiugre thr_l_lim value for unit1."]
pub struct CNT_L_LIM_U1_R(crate::FieldReader<u16, u16>);
impl CNT_L_LIM_U1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CNT_L_LIM_U1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_L_LIM_U1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT_L_LIM_U1` writer - This register is used to confiugre thr_l_lim value for unit1."]
pub struct CNT_L_LIM_U1_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_L_LIM_U1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This register is used to configure thr_h_lim value for unit1."]
    #[inline(always)]
    pub fn cnt_h_lim_u1(&self) -> CNT_H_LIM_U1_R {
        CNT_H_LIM_U1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This register is used to confiugre thr_l_lim value for unit1."]
    #[inline(always)]
    pub fn cnt_l_lim_u1(&self) -> CNT_L_LIM_U1_R {
        CNT_L_LIM_U1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is used to configure thr_h_lim value for unit1."]
    #[inline(always)]
    pub fn cnt_h_lim_u1(&mut self) -> CNT_H_LIM_U1_W {
        CNT_H_LIM_U1_W { w: self }
    }
    #[doc = "Bits 16:31 - This register is used to confiugre thr_l_lim value for unit1."]
    #[inline(always)]
    pub fn cnt_l_lim_u1(&mut self) -> CNT_L_LIM_U1_W {
        CNT_L_LIM_U1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1_conf2]
(index.html) module"]
pub struct U1_CONF2_SPEC;
impl crate::RegisterSpec for U1_CONF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u1_conf2::R]
(R) reader structure"]
impl crate::Readable for U1_CONF2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u1_conf2::W]
(W) writer structure"]
impl crate::Writable for U1_CONF2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets U1_CONF2 to value 0"]
impl crate::Resettable for U1_CONF2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
