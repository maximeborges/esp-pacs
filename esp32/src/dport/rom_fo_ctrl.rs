#[doc = "Register `ROM_FO_CTRL` reader"]
pub struct R(crate::R<ROM_FO_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROM_FO_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROM_FO_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROM_FO_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROM_FO_CTRL` writer"]
pub struct W(crate::W<ROM_FO_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROM_FO_CTRL_SPEC>;
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
impl From<crate::W<ROM_FO_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROM_FO_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_ROM_FO` reader - "]
pub type PRO_ROM_FO_R = crate::BitReader<bool>;
#[doc = "Field `PRO_ROM_FO` writer - "]
pub type PRO_ROM_FO_W<'a> = crate::BitWriter<'a, u32, ROM_FO_CTRL_SPEC, bool, 0>;
#[doc = "Field `APP_ROM_FO` reader - "]
pub type APP_ROM_FO_R = crate::BitReader<bool>;
#[doc = "Field `APP_ROM_FO` writer - "]
pub type APP_ROM_FO_W<'a> = crate::BitWriter<'a, u32, ROM_FO_CTRL_SPEC, bool, 1>;
#[doc = "Field `SHARE_ROM_FO` reader - "]
pub type SHARE_ROM_FO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHARE_ROM_FO` writer - "]
pub type SHARE_ROM_FO_W<'a> = crate::FieldWriter<'a, u32, ROM_FO_CTRL_SPEC, u8, u8, 6, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_rom_fo(&self) -> PRO_ROM_FO_R {
        PRO_ROM_FO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn app_rom_fo(&self) -> APP_ROM_FO_R {
        APP_ROM_FO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn share_rom_fo(&self) -> SHARE_ROM_FO_R {
        SHARE_ROM_FO_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_rom_fo(&mut self) -> PRO_ROM_FO_W {
        PRO_ROM_FO_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn app_rom_fo(&mut self) -> APP_ROM_FO_W {
        APP_ROM_FO_W::new(self)
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn share_rom_fo(&mut self) -> SHARE_ROM_FO_W {
        SHARE_ROM_FO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_fo_ctrl](index.html) module"]
pub struct ROM_FO_CTRL_SPEC;
impl crate::RegisterSpec for ROM_FO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rom_fo_ctrl::R](R) reader structure"]
impl crate::Readable for ROM_FO_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rom_fo_ctrl::W](W) writer structure"]
impl crate::Writable for ROM_FO_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROM_FO_CTRL to value 0x03"]
impl crate::Resettable for ROM_FO_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
