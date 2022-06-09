#[doc = "Register `TIMER1_SYNC` reader"]
pub struct R(crate::R<TIMER1_SYNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER1_SYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER1_SYNC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER1_SYNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER1_SYNC` writer"]
pub struct W(crate::W<TIMER1_SYNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER1_SYNC_SPEC>;
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
impl From<crate::W<TIMER1_SYNC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER1_SYNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER1_SYNCI_EN` reader - "]
pub type TIMER1_SYNCI_EN_R = crate::BitReader<bool>;
#[doc = "Field `TIMER1_SYNCI_EN` writer - "]
pub type TIMER1_SYNCI_EN_W<'a> = crate::BitWriter<'a, u32, TIMER1_SYNC_SPEC, bool, 0>;
#[doc = "Field `SW` reader - "]
pub type SW_R = crate::BitReader<bool>;
#[doc = "Field `SW` writer - "]
pub type SW_W<'a> = crate::BitWriter<'a, u32, TIMER1_SYNC_SPEC, bool, 1>;
#[doc = "Field `TIMER1_SYNCO_SEL` reader - "]
pub type TIMER1_SYNCO_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMER1_SYNCO_SEL` writer - "]
pub type TIMER1_SYNCO_SEL_W<'a> = crate::FieldWriter<'a, u32, TIMER1_SYNC_SPEC, u8, u8, 2, 2>;
#[doc = "Field `TIMER1_PHASE` reader - "]
pub type TIMER1_PHASE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIMER1_PHASE` writer - "]
pub type TIMER1_PHASE_W<'a> = crate::FieldWriter<'a, u32, TIMER1_SYNC_SPEC, u16, u16, 16, 4>;
#[doc = "Field `TIMER1_PHASE_DIRECTION` reader - "]
pub type TIMER1_PHASE_DIRECTION_R = crate::BitReader<bool>;
#[doc = "Field `TIMER1_PHASE_DIRECTION` writer - "]
pub type TIMER1_PHASE_DIRECTION_W<'a> = crate::BitWriter<'a, u32, TIMER1_SYNC_SPEC, bool, 20>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timer1_synci_en(&self) -> TIMER1_SYNCI_EN_R {
        TIMER1_SYNCI_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn timer1_synco_sel(&self) -> TIMER1_SYNCO_SEL_R {
        TIMER1_SYNCO_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:19"]
    #[inline(always)]
    pub fn timer1_phase(&self) -> TIMER1_PHASE_R {
        TIMER1_PHASE_R::new(((self.bits >> 4) & 0xffff) as u16)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn timer1_phase_direction(&self) -> TIMER1_PHASE_DIRECTION_R {
        TIMER1_PHASE_DIRECTION_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timer1_synci_en(&mut self) -> TIMER1_SYNCI_EN_W {
        TIMER1_SYNCI_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W {
        SW_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn timer1_synco_sel(&mut self) -> TIMER1_SYNCO_SEL_W {
        TIMER1_SYNCO_SEL_W::new(self)
    }
    #[doc = "Bits 4:19"]
    #[inline(always)]
    pub fn timer1_phase(&mut self) -> TIMER1_PHASE_W {
        TIMER1_PHASE_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn timer1_phase_direction(&mut self) -> TIMER1_PHASE_DIRECTION_W {
        TIMER1_PHASE_DIRECTION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer1_sync](index.html) module"]
pub struct TIMER1_SYNC_SPEC;
impl crate::RegisterSpec for TIMER1_SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer1_sync::R](R) reader structure"]
impl crate::Readable for TIMER1_SYNC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer1_sync::W](W) writer structure"]
impl crate::Writable for TIMER1_SYNC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER1_SYNC to value 0"]
impl crate::Resettable for TIMER1_SYNC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
