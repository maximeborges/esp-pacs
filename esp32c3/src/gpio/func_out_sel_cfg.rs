#[doc = "Register `FUNC%s_OUT_SEL_CFG` reader"]
pub struct R(crate::R<FUNC_OUT_SEL_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FUNC_OUT_SEL_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FUNC_OUT_SEL_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FUNC_OUT_SEL_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FUNC%s_OUT_SEL_CFG` writer"]
pub struct W(crate::W<FUNC_OUT_SEL_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FUNC_OUT_SEL_CFG_SPEC>;
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
impl From<crate::W<FUNC_OUT_SEL_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FUNC_OUT_SEL_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT_SEL` reader - The value of the bits: <=s<=256. Set the value to select output signal. s=-255: output of GPIO\\[n\\] equals input of peripheral\\[s\\]. s=256: output of GPIO\\[n\\] equals GPIO_OUT_REG\\[n\\]."]
pub type OUT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUT_SEL` writer - The value of the bits: <=s<=256. Set the value to select output signal. s=-255: output of GPIO\\[n\\] equals input of peripheral\\[s\\]. s=256: output of GPIO\\[n\\] equals GPIO_OUT_REG\\[n\\]."]
pub type OUT_SEL_W<'a> = crate::FieldWriter<'a, u32, FUNC_OUT_SEL_CFG_SPEC, u8, u8, 8, 0>;
#[doc = "Field `INV_SEL` reader - set this bit to invert output signal.1:invert.:not invert."]
pub type INV_SEL_R = crate::BitReader<bool>;
#[doc = "Field `INV_SEL` writer - set this bit to invert output signal.1:invert.:not invert."]
pub type INV_SEL_W<'a> = crate::BitWriter<'a, u32, FUNC_OUT_SEL_CFG_SPEC, bool, 8>;
#[doc = "Field `OEN_SEL` reader - set this bit to select output enable signal.1:use GPIO_ENABLE_REG\\[n\\] as output enable signal.:use peripheral output enable signal."]
pub type OEN_SEL_R = crate::BitReader<bool>;
#[doc = "Field `OEN_SEL` writer - set this bit to select output enable signal.1:use GPIO_ENABLE_REG\\[n\\] as output enable signal.:use peripheral output enable signal."]
pub type OEN_SEL_W<'a> = crate::BitWriter<'a, u32, FUNC_OUT_SEL_CFG_SPEC, bool, 9>;
#[doc = "Field `OEN_INV_SEL` reader - set this bit to invert output enable signal.1:invert.:not invert."]
pub type OEN_INV_SEL_R = crate::BitReader<bool>;
#[doc = "Field `OEN_INV_SEL` writer - set this bit to invert output enable signal.1:invert.:not invert."]
pub type OEN_INV_SEL_W<'a> = crate::BitWriter<'a, u32, FUNC_OUT_SEL_CFG_SPEC, bool, 10>;
impl R {
    #[doc = "Bits 0:7 - The value of the bits: <=s<=256. Set the value to select output signal. s=-255: output of GPIO\\[n\\] equals input of peripheral\\[s\\]. s=256: output of GPIO\\[n\\] equals GPIO_OUT_REG\\[n\\]."]
    #[inline(always)]
    pub fn out_sel(&self) -> OUT_SEL_R {
        OUT_SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - set this bit to invert output signal.1:invert.:not invert."]
    #[inline(always)]
    pub fn inv_sel(&self) -> INV_SEL_R {
        INV_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - set this bit to select output enable signal.1:use GPIO_ENABLE_REG\\[n\\] as output enable signal.:use peripheral output enable signal."]
    #[inline(always)]
    pub fn oen_sel(&self) -> OEN_SEL_R {
        OEN_SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - set this bit to invert output enable signal.1:invert.:not invert."]
    #[inline(always)]
    pub fn oen_inv_sel(&self) -> OEN_INV_SEL_R {
        OEN_INV_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - The value of the bits: <=s<=256. Set the value to select output signal. s=-255: output of GPIO\\[n\\] equals input of peripheral\\[s\\]. s=256: output of GPIO\\[n\\] equals GPIO_OUT_REG\\[n\\]."]
    #[inline(always)]
    pub fn out_sel(&mut self) -> OUT_SEL_W {
        OUT_SEL_W::new(self)
    }
    #[doc = "Bit 8 - set this bit to invert output signal.1:invert.:not invert."]
    #[inline(always)]
    pub fn inv_sel(&mut self) -> INV_SEL_W {
        INV_SEL_W::new(self)
    }
    #[doc = "Bit 9 - set this bit to select output enable signal.1:use GPIO_ENABLE_REG\\[n\\] as output enable signal.:use peripheral output enable signal."]
    #[inline(always)]
    pub fn oen_sel(&mut self) -> OEN_SEL_W {
        OEN_SEL_W::new(self)
    }
    #[doc = "Bit 10 - set this bit to invert output enable signal.1:invert.:not invert."]
    #[inline(always)]
    pub fn oen_inv_sel(&mut self) -> OEN_INV_SEL_W {
        OEN_INV_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO output function select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [func_out_sel_cfg](index.html) module"]
pub struct FUNC_OUT_SEL_CFG_SPEC;
impl crate::RegisterSpec for FUNC_OUT_SEL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [func_out_sel_cfg::R](R) reader structure"]
impl crate::Readable for FUNC_OUT_SEL_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [func_out_sel_cfg::W](W) writer structure"]
impl crate::Writable for FUNC_OUT_SEL_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FUNC%s_OUT_SEL_CFG to value 0x80"]
impl crate::Resettable for FUNC_OUT_SEL_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
