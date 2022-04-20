#[doc = "Register `CPU_PERI_CLK_EN` reader"]
pub struct R(crate::R<CPU_PERI_CLK_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_PERI_CLK_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_PERI_CLK_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_PERI_CLK_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_PERI_CLK_EN` writer"]
pub struct W(crate::W<CPU_PERI_CLK_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_PERI_CLK_EN_SPEC>;
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
impl From<crate::W<CPU_PERI_CLK_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_PERI_CLK_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_EN_DEDICATED_GPIO` reader - Set this bit to enable clock of DEDICATED GPIO module."]
pub struct CLK_EN_DEDICATED_GPIO_R(crate::FieldReader<bool, bool>);
impl CLK_EN_DEDICATED_GPIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_EN_DEDICATED_GPIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_EN_DEDICATED_GPIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_EN_DEDICATED_GPIO` writer - Set this bit to enable clock of DEDICATED GPIO module."]
pub struct CLK_EN_DEDICATED_GPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EN_DEDICATED_GPIO_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - Set this bit to enable clock of DEDICATED GPIO module."]
    #[inline(always)]
    pub fn clk_en_dedicated_gpio(&self) -> CLK_EN_DEDICATED_GPIO_R {
        CLK_EN_DEDICATED_GPIO_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Set this bit to enable clock of DEDICATED GPIO module."]
    #[inline(always)]
    pub fn clk_en_dedicated_gpio(&mut self) -> CLK_EN_DEDICATED_GPIO_W {
        CLK_EN_DEDICATED_GPIO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU peripheral clock enable register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_peri_clk_en]
(index.html) module"]
pub struct CPU_PERI_CLK_EN_SPEC;
impl crate::RegisterSpec for CPU_PERI_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_peri_clk_en::R]
(R) reader structure"]
impl crate::Readable for CPU_PERI_CLK_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_peri_clk_en::W]
(W) writer structure"]
impl crate::Writable for CPU_PERI_CLK_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPU_PERI_CLK_EN to value 0"]
impl crate::Resettable for CPU_PERI_CLK_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
