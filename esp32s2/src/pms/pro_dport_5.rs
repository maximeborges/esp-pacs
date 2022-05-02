#[doc = "Register `PRO_DPORT_5` reader"]
pub struct R(crate::R<PRO_DPORT_5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_DPORT_5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_DPORT_5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_DPORT_5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_DPORT_5` writer"]
pub struct W(crate::W<PRO_DPORT_5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_DPORT_5_SPEC>;
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
impl From<crate::W<PRO_DPORT_5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_DPORT_5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_DPORT_RESERVE_FIFO_3` reader - Configure read-protection address 3."]
pub struct PRO_DPORT_RESERVE_FIFO_3_R(crate::FieldReader<u32>);
impl PRO_DPORT_RESERVE_FIFO_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PRO_DPORT_RESERVE_FIFO_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DPORT_RESERVE_FIFO_3_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DPORT_RESERVE_FIFO_3` writer - Configure read-protection address 3."]
pub struct PRO_DPORT_RESERVE_FIFO_3_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_DPORT_RESERVE_FIFO_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | (value as u32 & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Configure read-protection address 3."]
    #[inline(always)]
    pub fn pro_dport_reserve_fifo_3(&self) -> PRO_DPORT_RESERVE_FIFO_3_R {
        PRO_DPORT_RESERVE_FIFO_3_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - Configure read-protection address 3."]
    #[inline(always)]
    pub fn pro_dport_reserve_fifo_3(&mut self) -> PRO_DPORT_RESERVE_FIFO_3_W {
        PRO_DPORT_RESERVE_FIFO_3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PeriBus1 permission control register 5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_dport_5](index.html) module"]
pub struct PRO_DPORT_5_SPEC;
impl crate::RegisterSpec for PRO_DPORT_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_dport_5::R](R) reader structure"]
impl crate::Readable for PRO_DPORT_5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_dport_5::W](W) writer structure"]
impl crate::Writable for PRO_DPORT_5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_DPORT_5 to value 0"]
impl crate::Resettable for PRO_DPORT_5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
