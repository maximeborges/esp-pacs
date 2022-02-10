#[doc = "Register `Core_0_World_UPDATE` writer"]
pub struct W(crate::W<CORE_0_WORLD_UPDATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_WORLD_UPDATE_SPEC>;
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
impl From<crate::W<CORE_0_WORLD_UPDATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_WORLD_UPDATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_UPDATE` writer - This field is used to update configuration completed, can write any value,the hardware only checks the write operation of this register and does not case about its value"]
pub struct CORE_0_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_UPDATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - This field is used to update configuration completed, can write any value,the hardware only checks the write operation of this register and does not case about its value"]
    #[inline(always)]
    pub fn core_0_update(&mut self) -> CORE_0_UPDATE_W {
        CORE_0_UPDATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core_0 configuration update register\n\nThis register you can [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_world_update]
(index.html) module"]
pub struct CORE_0_WORLD_UPDATE_SPEC;
impl crate::RegisterSpec for CORE_0_WORLD_UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [core_0_world_update::W]
(W) writer structure"]
impl crate::Writable for CORE_0_WORLD_UPDATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Core_0_World_UPDATE to value 0"]
impl crate::Resettable for CORE_0_WORLD_UPDATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
