#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_RAW` writer"]
pub struct W(crate::W<INT_RAW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_RAW_SPEC>;
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
impl From<crate::W<INT_RAW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_RAW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLAVE_TRANS_COMPLETE_INT_RAW` reader - Slave accepted 1 byte and address matched"]
pub type SLAVE_TRANS_COMPLETE_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLAVE_TRANS_COMPLETE_INT_RAW` writer - Slave accepted 1 byte and address matched"]
pub type SLAVE_TRANS_COMPLETE_INT_RAW_W<'a> = crate::BitWriter<'a, u32, INT_RAW_SPEC, bool, 3>;
#[doc = "Field `ARBITRATION_LOST_INT_RAW` reader - Master lost arbitration"]
pub type ARBITRATION_LOST_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `ARBITRATION_LOST_INT_RAW` writer - Master lost arbitration"]
pub type ARBITRATION_LOST_INT_RAW_W<'a> = crate::BitWriter<'a, u32, INT_RAW_SPEC, bool, 4>;
#[doc = "Field `MASTER_TRANS_COMPLETE_INT_RAW` reader - "]
pub type MASTER_TRANS_COMPLETE_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `MASTER_TRANS_COMPLETE_INT_RAW` writer - "]
pub type MASTER_TRANS_COMPLETE_INT_RAW_W<'a> = crate::BitWriter<'a, u32, INT_RAW_SPEC, bool, 5>;
#[doc = "Field `TRANS_COMPLETE_INT_RAW` reader - Stop condition has been detected interrupt raw status"]
pub type TRANS_COMPLETE_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `TRANS_COMPLETE_INT_RAW` writer - Stop condition has been detected interrupt raw status"]
pub type TRANS_COMPLETE_INT_RAW_W<'a> = crate::BitWriter<'a, u32, INT_RAW_SPEC, bool, 6>;
#[doc = "Field `TIME_OUT_INT_RAW` reader - time out interrupt raw status"]
pub type TIME_OUT_INT_RAW_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 3 - Slave accepted 1 byte and address matched"]
    #[inline(always)]
    pub fn slave_trans_complete_int_raw(&self) -> SLAVE_TRANS_COMPLETE_INT_RAW_R {
        SLAVE_TRANS_COMPLETE_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master lost arbitration"]
    #[inline(always)]
    pub fn arbitration_lost_int_raw(&self) -> ARBITRATION_LOST_INT_RAW_R {
        ARBITRATION_LOST_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn master_trans_complete_int_raw(&self) -> MASTER_TRANS_COMPLETE_INT_RAW_R {
        MASTER_TRANS_COMPLETE_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stop condition has been detected interrupt raw status"]
    #[inline(always)]
    pub fn trans_complete_int_raw(&self) -> TRANS_COMPLETE_INT_RAW_R {
        TRANS_COMPLETE_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - time out interrupt raw status"]
    #[inline(always)]
    pub fn time_out_int_raw(&self) -> TIME_OUT_INT_RAW_R {
        TIME_OUT_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Slave accepted 1 byte and address matched"]
    #[inline(always)]
    pub fn slave_trans_complete_int_raw(&mut self) -> SLAVE_TRANS_COMPLETE_INT_RAW_W {
        SLAVE_TRANS_COMPLETE_INT_RAW_W::new(self)
    }
    #[doc = "Bit 4 - Master lost arbitration"]
    #[inline(always)]
    pub fn arbitration_lost_int_raw(&mut self) -> ARBITRATION_LOST_INT_RAW_W {
        ARBITRATION_LOST_INT_RAW_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn master_trans_complete_int_raw(&mut self) -> MASTER_TRANS_COMPLETE_INT_RAW_W {
        MASTER_TRANS_COMPLETE_INT_RAW_W::new(self)
    }
    #[doc = "Bit 6 - Stop condition has been detected interrupt raw status"]
    #[inline(always)]
    pub fn trans_complete_int_raw(&mut self) -> TRANS_COMPLETE_INT_RAW_W {
        TRANS_COMPLETE_INT_RAW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_raw::W](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
