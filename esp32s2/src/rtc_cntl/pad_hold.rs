#[doc = "Register `PAD_HOLD` reader"]
pub struct R(crate::R<PAD_HOLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_HOLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_HOLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_HOLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_HOLD` writer"]
pub struct W(crate::W<PAD_HOLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_HOLD_SPEC>;
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
impl From<crate::W<PAD_HOLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_HOLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_PAD0_HOLD` reader - Sets the touch GPIO 0 to hold."]
pub type TOUCH_PAD0_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `TOUCH_PAD0_HOLD` writer - Sets the touch GPIO 0 to hold."]
pub type TOUCH_PAD0_HOLD_W<'a> = crate::BitWriter<'a, u32, PAD_HOLD_SPEC, bool, 0>;
#[doc = "Field `TOUCH_PAD1_HOLD` reader - Sets the touch GPIO 1 to hold."]
pub type TOUCH_PAD1_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `TOUCH_PAD1_HOLD` writer - Sets the touch GPIO 1 to hold."]
pub type TOUCH_PAD1_HOLD_W<'a> = crate::BitWriter<'a, u32, PAD_HOLD_SPEC, bool, 1>;
#[doc = "Field `TOUCH_PAD2_HOLD` reader - Sets the touch GPIO 2 to hold."]
pub type TOUCH_PAD2_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `TOUCH_PAD2_HOLD` writer - Sets the touch GPIO 2 to hold."]
pub type TOUCH_PAD2_HOLD_W<'a> = crate::BitWriter<'a, u32, PAD_HOLD_SPEC, bool, 2>;
#[doc = "Field `TOUCH_PAD3_HOLD` reader - Sets the touch GPIO 3 to hold."]
pub type TOUCH_PAD3_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `TOUCH_PAD3_HOLD` writer - Sets the touch GPIO 3 to hold."]
pub type TOUCH_PAD3_HOLD_W<'a> = crate::BitWriter<'a, u32, PAD_HOLD_SPEC, bool, 3>;
#[doc = "Field `TOUCH_PAD4_HOLD` reader - Sets the touch GPIO 4 to hold."]
pub type TOUCH_PAD4_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `TOUCH_PAD4_HOLD` writer - Sets the touch GPIO 4 to hold."]
pub type TOUCH_PAD4_HOLD_W<'a> = crate::BitWriter<'a, u32, PAD_HOLD_SPEC, bool, 4>;
#[doc = "Field `TOUCH_PAD5_HOLD` reader - Sets the touch GPIO 5 to hold."]
pub type TOUCH_PAD5_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `TOUCH_PAD5_HOLD` writer - Sets the touch GPIO 5 to hold."]
pub type TOUCH_PAD5_HOLD_W<'a> = crate::BitWriter<'a, u32, PAD_HOLD_SPEC, bool, 5>;
#[doc = "Field `TOUCH_PAD6_HOLD` reader - Sets the touch GPIO 6 to hold."]
pub type TOUCH_PAD6_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `TOUCH_PAD6_HOLD` writer - Sets the touch GPIO 6 to hold."]
pub type TOUCH_PAD6_HOLD_W<'a> = crate::BitWriter<'a, u32, PAD_HOLD_SPEC, bool, 6>;
#[doc = "Field `TOUCH_PAD7_HOLD` reader - Sets the touch GPIO 7 to hold."]
pub type TOUCH_PAD7_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `TOUCH_PAD7_HOLD` writer - Sets the touch GPIO 7 to hold."]
pub type TOUCH_PAD7_HOLD_W<'a> = crate::BitWriter<'a, u32, PAD_HOLD_SPEC, bool, 7>;
#[doc = "Field `TOUCH_PAD8_HOLD` reader - Sets the touch GPIO 8 to hold."]
pub type TOUCH_PAD8_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `TOUCH_PAD8_HOLD` writer - Sets the touch GPIO 8 to hold."]
pub type TOUCH_PAD8_HOLD_W<'a> = crate::BitWriter<'a, u32, PAD_HOLD_SPEC, bool, 8>;
#[doc = "Field `TOUCH_PAD9_HOLD` reader - Sets the touch GPIO 9 to hold."]
pub type TOUCH_PAD9_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `TOUCH_PAD9_HOLD` writer - Sets the touch GPIO 9 to hold."]
pub type TOUCH_PAD9_HOLD_W<'a> = crate::BitWriter<'a, u32, PAD_HOLD_SPEC, bool, 9>;
#[doc = "Field `TOUCH_PAD10_HOLD` reader - Sets the touch GPIO 10 to hold."]
pub type TOUCH_PAD10_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `TOUCH_PAD10_HOLD` writer - Sets the touch GPIO 10 to hold."]
pub type TOUCH_PAD10_HOLD_W<'a> = crate::BitWriter<'a, u32, PAD_HOLD_SPEC, bool, 10>;
#[doc = "Field `TOUCH_PAD11_HOLD` reader - Sets the touch GPIO 11 to hold."]
pub type TOUCH_PAD11_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `TOUCH_PAD11_HOLD` writer - Sets the touch GPIO 11 to hold."]
pub type TOUCH_PAD11_HOLD_W<'a> = crate::BitWriter<'a, u32, PAD_HOLD_SPEC, bool, 11>;
#[doc = "Field `TOUCH_PAD12_HOLD` reader - Sets the touch GPIO 12 to hold."]
pub type TOUCH_PAD12_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `TOUCH_PAD12_HOLD` writer - Sets the touch GPIO 12 to hold."]
pub type TOUCH_PAD12_HOLD_W<'a> = crate::BitWriter<'a, u32, PAD_HOLD_SPEC, bool, 12>;
#[doc = "Field `TOUCH_PAD13_HOLD` reader - Sets the touch GPIO 13 to hold."]
pub type TOUCH_PAD13_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `TOUCH_PAD13_HOLD` writer - Sets the touch GPIO 13 to hold."]
pub type TOUCH_PAD13_HOLD_W<'a> = crate::BitWriter<'a, u32, PAD_HOLD_SPEC, bool, 13>;
#[doc = "Field `TOUCH_PAD14_HOLD` reader - Sets the touch GPIO 14 to hold."]
pub type TOUCH_PAD14_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `TOUCH_PAD14_HOLD` writer - Sets the touch GPIO 14 to hold."]
pub type TOUCH_PAD14_HOLD_W<'a> = crate::BitWriter<'a, u32, PAD_HOLD_SPEC, bool, 14>;
#[doc = "Field `X32P_HOLD` reader - Sets the x32p to hold."]
pub type X32P_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `X32P_HOLD` writer - Sets the x32p to hold."]
pub type X32P_HOLD_W<'a> = crate::BitWriter<'a, u32, PAD_HOLD_SPEC, bool, 15>;
#[doc = "Field `X32N_HOLD` reader - Sets the x32n to hold."]
pub type X32N_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `X32N_HOLD` writer - Sets the x32n to hold."]
pub type X32N_HOLD_W<'a> = crate::BitWriter<'a, u32, PAD_HOLD_SPEC, bool, 16>;
#[doc = "Field `PDAC1_HOLD` reader - Sets the pdac1 to hold."]
pub type PDAC1_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `PDAC1_HOLD` writer - Sets the pdac1 to hold."]
pub type PDAC1_HOLD_W<'a> = crate::BitWriter<'a, u32, PAD_HOLD_SPEC, bool, 17>;
#[doc = "Field `PDAC2_HOLD` reader - Sets the pdac2 to hold."]
pub type PDAC2_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `PDAC2_HOLD` writer - Sets the pdac2 to hold."]
pub type PDAC2_HOLD_W<'a> = crate::BitWriter<'a, u32, PAD_HOLD_SPEC, bool, 18>;
#[doc = "Field `PAD19_HOLD` reader - Sets the RTG GPIO 19 to hold."]
pub type PAD19_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `PAD19_HOLD` writer - Sets the RTG GPIO 19 to hold."]
pub type PAD19_HOLD_W<'a> = crate::BitWriter<'a, u32, PAD_HOLD_SPEC, bool, 19>;
#[doc = "Field `PAD20_HOLD` reader - Sets the RTG GPIO 20 to hold."]
pub type PAD20_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `PAD20_HOLD` writer - Sets the RTG GPIO 20 to hold."]
pub type PAD20_HOLD_W<'a> = crate::BitWriter<'a, u32, PAD_HOLD_SPEC, bool, 20>;
#[doc = "Field `PAD21_HOLD` reader - Sets the RTG GPIO 21 to hold."]
pub type PAD21_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `PAD21_HOLD` writer - Sets the RTG GPIO 21 to hold."]
pub type PAD21_HOLD_W<'a> = crate::BitWriter<'a, u32, PAD_HOLD_SPEC, bool, 21>;
impl R {
    #[doc = "Bit 0 - Sets the touch GPIO 0 to hold."]
    #[inline(always)]
    pub fn touch_pad0_hold(&self) -> TOUCH_PAD0_HOLD_R {
        TOUCH_PAD0_HOLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sets the touch GPIO 1 to hold."]
    #[inline(always)]
    pub fn touch_pad1_hold(&self) -> TOUCH_PAD1_HOLD_R {
        TOUCH_PAD1_HOLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sets the touch GPIO 2 to hold."]
    #[inline(always)]
    pub fn touch_pad2_hold(&self) -> TOUCH_PAD2_HOLD_R {
        TOUCH_PAD2_HOLD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sets the touch GPIO 3 to hold."]
    #[inline(always)]
    pub fn touch_pad3_hold(&self) -> TOUCH_PAD3_HOLD_R {
        TOUCH_PAD3_HOLD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sets the touch GPIO 4 to hold."]
    #[inline(always)]
    pub fn touch_pad4_hold(&self) -> TOUCH_PAD4_HOLD_R {
        TOUCH_PAD4_HOLD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sets the touch GPIO 5 to hold."]
    #[inline(always)]
    pub fn touch_pad5_hold(&self) -> TOUCH_PAD5_HOLD_R {
        TOUCH_PAD5_HOLD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sets the touch GPIO 6 to hold."]
    #[inline(always)]
    pub fn touch_pad6_hold(&self) -> TOUCH_PAD6_HOLD_R {
        TOUCH_PAD6_HOLD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sets the touch GPIO 7 to hold."]
    #[inline(always)]
    pub fn touch_pad7_hold(&self) -> TOUCH_PAD7_HOLD_R {
        TOUCH_PAD7_HOLD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Sets the touch GPIO 8 to hold."]
    #[inline(always)]
    pub fn touch_pad8_hold(&self) -> TOUCH_PAD8_HOLD_R {
        TOUCH_PAD8_HOLD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Sets the touch GPIO 9 to hold."]
    #[inline(always)]
    pub fn touch_pad9_hold(&self) -> TOUCH_PAD9_HOLD_R {
        TOUCH_PAD9_HOLD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Sets the touch GPIO 10 to hold."]
    #[inline(always)]
    pub fn touch_pad10_hold(&self) -> TOUCH_PAD10_HOLD_R {
        TOUCH_PAD10_HOLD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Sets the touch GPIO 11 to hold."]
    #[inline(always)]
    pub fn touch_pad11_hold(&self) -> TOUCH_PAD11_HOLD_R {
        TOUCH_PAD11_HOLD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Sets the touch GPIO 12 to hold."]
    #[inline(always)]
    pub fn touch_pad12_hold(&self) -> TOUCH_PAD12_HOLD_R {
        TOUCH_PAD12_HOLD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Sets the touch GPIO 13 to hold."]
    #[inline(always)]
    pub fn touch_pad13_hold(&self) -> TOUCH_PAD13_HOLD_R {
        TOUCH_PAD13_HOLD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Sets the touch GPIO 14 to hold."]
    #[inline(always)]
    pub fn touch_pad14_hold(&self) -> TOUCH_PAD14_HOLD_R {
        TOUCH_PAD14_HOLD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Sets the x32p to hold."]
    #[inline(always)]
    pub fn x32p_hold(&self) -> X32P_HOLD_R {
        X32P_HOLD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Sets the x32n to hold."]
    #[inline(always)]
    pub fn x32n_hold(&self) -> X32N_HOLD_R {
        X32N_HOLD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Sets the pdac1 to hold."]
    #[inline(always)]
    pub fn pdac1_hold(&self) -> PDAC1_HOLD_R {
        PDAC1_HOLD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Sets the pdac2 to hold."]
    #[inline(always)]
    pub fn pdac2_hold(&self) -> PDAC2_HOLD_R {
        PDAC2_HOLD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Sets the RTG GPIO 19 to hold."]
    #[inline(always)]
    pub fn pad19_hold(&self) -> PAD19_HOLD_R {
        PAD19_HOLD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Sets the RTG GPIO 20 to hold."]
    #[inline(always)]
    pub fn pad20_hold(&self) -> PAD20_HOLD_R {
        PAD20_HOLD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Sets the RTG GPIO 21 to hold."]
    #[inline(always)]
    pub fn pad21_hold(&self) -> PAD21_HOLD_R {
        PAD21_HOLD_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sets the touch GPIO 0 to hold."]
    #[inline(always)]
    pub fn touch_pad0_hold(&mut self) -> TOUCH_PAD0_HOLD_W {
        TOUCH_PAD0_HOLD_W::new(self)
    }
    #[doc = "Bit 1 - Sets the touch GPIO 1 to hold."]
    #[inline(always)]
    pub fn touch_pad1_hold(&mut self) -> TOUCH_PAD1_HOLD_W {
        TOUCH_PAD1_HOLD_W::new(self)
    }
    #[doc = "Bit 2 - Sets the touch GPIO 2 to hold."]
    #[inline(always)]
    pub fn touch_pad2_hold(&mut self) -> TOUCH_PAD2_HOLD_W {
        TOUCH_PAD2_HOLD_W::new(self)
    }
    #[doc = "Bit 3 - Sets the touch GPIO 3 to hold."]
    #[inline(always)]
    pub fn touch_pad3_hold(&mut self) -> TOUCH_PAD3_HOLD_W {
        TOUCH_PAD3_HOLD_W::new(self)
    }
    #[doc = "Bit 4 - Sets the touch GPIO 4 to hold."]
    #[inline(always)]
    pub fn touch_pad4_hold(&mut self) -> TOUCH_PAD4_HOLD_W {
        TOUCH_PAD4_HOLD_W::new(self)
    }
    #[doc = "Bit 5 - Sets the touch GPIO 5 to hold."]
    #[inline(always)]
    pub fn touch_pad5_hold(&mut self) -> TOUCH_PAD5_HOLD_W {
        TOUCH_PAD5_HOLD_W::new(self)
    }
    #[doc = "Bit 6 - Sets the touch GPIO 6 to hold."]
    #[inline(always)]
    pub fn touch_pad6_hold(&mut self) -> TOUCH_PAD6_HOLD_W {
        TOUCH_PAD6_HOLD_W::new(self)
    }
    #[doc = "Bit 7 - Sets the touch GPIO 7 to hold."]
    #[inline(always)]
    pub fn touch_pad7_hold(&mut self) -> TOUCH_PAD7_HOLD_W {
        TOUCH_PAD7_HOLD_W::new(self)
    }
    #[doc = "Bit 8 - Sets the touch GPIO 8 to hold."]
    #[inline(always)]
    pub fn touch_pad8_hold(&mut self) -> TOUCH_PAD8_HOLD_W {
        TOUCH_PAD8_HOLD_W::new(self)
    }
    #[doc = "Bit 9 - Sets the touch GPIO 9 to hold."]
    #[inline(always)]
    pub fn touch_pad9_hold(&mut self) -> TOUCH_PAD9_HOLD_W {
        TOUCH_PAD9_HOLD_W::new(self)
    }
    #[doc = "Bit 10 - Sets the touch GPIO 10 to hold."]
    #[inline(always)]
    pub fn touch_pad10_hold(&mut self) -> TOUCH_PAD10_HOLD_W {
        TOUCH_PAD10_HOLD_W::new(self)
    }
    #[doc = "Bit 11 - Sets the touch GPIO 11 to hold."]
    #[inline(always)]
    pub fn touch_pad11_hold(&mut self) -> TOUCH_PAD11_HOLD_W {
        TOUCH_PAD11_HOLD_W::new(self)
    }
    #[doc = "Bit 12 - Sets the touch GPIO 12 to hold."]
    #[inline(always)]
    pub fn touch_pad12_hold(&mut self) -> TOUCH_PAD12_HOLD_W {
        TOUCH_PAD12_HOLD_W::new(self)
    }
    #[doc = "Bit 13 - Sets the touch GPIO 13 to hold."]
    #[inline(always)]
    pub fn touch_pad13_hold(&mut self) -> TOUCH_PAD13_HOLD_W {
        TOUCH_PAD13_HOLD_W::new(self)
    }
    #[doc = "Bit 14 - Sets the touch GPIO 14 to hold."]
    #[inline(always)]
    pub fn touch_pad14_hold(&mut self) -> TOUCH_PAD14_HOLD_W {
        TOUCH_PAD14_HOLD_W::new(self)
    }
    #[doc = "Bit 15 - Sets the x32p to hold."]
    #[inline(always)]
    pub fn x32p_hold(&mut self) -> X32P_HOLD_W {
        X32P_HOLD_W::new(self)
    }
    #[doc = "Bit 16 - Sets the x32n to hold."]
    #[inline(always)]
    pub fn x32n_hold(&mut self) -> X32N_HOLD_W {
        X32N_HOLD_W::new(self)
    }
    #[doc = "Bit 17 - Sets the pdac1 to hold."]
    #[inline(always)]
    pub fn pdac1_hold(&mut self) -> PDAC1_HOLD_W {
        PDAC1_HOLD_W::new(self)
    }
    #[doc = "Bit 18 - Sets the pdac2 to hold."]
    #[inline(always)]
    pub fn pdac2_hold(&mut self) -> PDAC2_HOLD_W {
        PDAC2_HOLD_W::new(self)
    }
    #[doc = "Bit 19 - Sets the RTG GPIO 19 to hold."]
    #[inline(always)]
    pub fn pad19_hold(&mut self) -> PAD19_HOLD_W {
        PAD19_HOLD_W::new(self)
    }
    #[doc = "Bit 20 - Sets the RTG GPIO 20 to hold."]
    #[inline(always)]
    pub fn pad20_hold(&mut self) -> PAD20_HOLD_W {
        PAD20_HOLD_W::new(self)
    }
    #[doc = "Bit 21 - Sets the RTG GPIO 21 to hold."]
    #[inline(always)]
    pub fn pad21_hold(&mut self) -> PAD21_HOLD_W {
        PAD21_HOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configures the hold options for RTC GPIOs\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_hold](index.html) module"]
pub struct PAD_HOLD_SPEC;
impl crate::RegisterSpec for PAD_HOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_hold::R](R) reader structure"]
impl crate::Readable for PAD_HOLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_hold::W](W) writer structure"]
impl crate::Writable for PAD_HOLD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAD_HOLD to value 0"]
impl crate::Resettable for PAD_HOLD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
