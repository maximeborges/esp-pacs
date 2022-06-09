#[doc = "Register `HOST_SLCHOST_CONF_W1` reader"]
pub struct R(crate::R<HOST_SLCHOST_CONF_W1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLCHOST_CONF_W1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLCHOST_CONF_W1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLCHOST_CONF_W1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_SLCHOST_CONF_W1` writer"]
pub struct W(crate::W<HOST_SLCHOST_CONF_W1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_SLCHOST_CONF_W1_SPEC>;
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
impl From<crate::W<HOST_SLCHOST_CONF_W1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_SLCHOST_CONF_W1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_SLCHOST_CONF4` reader - "]
pub type HOST_SLCHOST_CONF4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOST_SLCHOST_CONF4` writer - "]
pub type HOST_SLCHOST_CONF4_W<'a> =
    crate::FieldWriter<'a, u32, HOST_SLCHOST_CONF_W1_SPEC, u8, u8, 8, 0>;
#[doc = "Field `HOST_SLCHOST_CONF5` reader - "]
pub type HOST_SLCHOST_CONF5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOST_SLCHOST_CONF5` writer - "]
pub type HOST_SLCHOST_CONF5_W<'a> =
    crate::FieldWriter<'a, u32, HOST_SLCHOST_CONF_W1_SPEC, u8, u8, 8, 8>;
#[doc = "Field `HOST_SLCHOST_CONF6` reader - "]
pub type HOST_SLCHOST_CONF6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOST_SLCHOST_CONF6` writer - "]
pub type HOST_SLCHOST_CONF6_W<'a> =
    crate::FieldWriter<'a, u32, HOST_SLCHOST_CONF_W1_SPEC, u8, u8, 8, 16>;
#[doc = "Field `HOST_SLCHOST_CONF7` reader - "]
pub type HOST_SLCHOST_CONF7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOST_SLCHOST_CONF7` writer - "]
pub type HOST_SLCHOST_CONF7_W<'a> =
    crate::FieldWriter<'a, u32, HOST_SLCHOST_CONF_W1_SPEC, u8, u8, 8, 24>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf4(&self) -> HOST_SLCHOST_CONF4_R {
        HOST_SLCHOST_CONF4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf5(&self) -> HOST_SLCHOST_CONF5_R {
        HOST_SLCHOST_CONF5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf6(&self) -> HOST_SLCHOST_CONF6_R {
        HOST_SLCHOST_CONF6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf7(&self) -> HOST_SLCHOST_CONF7_R {
        HOST_SLCHOST_CONF7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf4(&mut self) -> HOST_SLCHOST_CONF4_W {
        HOST_SLCHOST_CONF4_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf5(&mut self) -> HOST_SLCHOST_CONF5_W {
        HOST_SLCHOST_CONF5_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf6(&mut self) -> HOST_SLCHOST_CONF6_W {
        HOST_SLCHOST_CONF6_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf7(&mut self) -> HOST_SLCHOST_CONF7_W {
        HOST_SLCHOST_CONF7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slchost_conf_w1](index.html) module"]
pub struct HOST_SLCHOST_CONF_W1_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_CONF_W1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slchost_conf_w1::R](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w1::W](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_SLCHOST_CONF_W1 to value 0"]
impl crate::Resettable for HOST_SLCHOST_CONF_W1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
