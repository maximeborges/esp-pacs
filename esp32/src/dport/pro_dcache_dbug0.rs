#[doc = "Register `PRO_DCACHE_DBUG0` reader"]
pub struct R(crate::R<PRO_DCACHE_DBUG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_DCACHE_DBUG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_DCACHE_DBUG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_DCACHE_DBUG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_DCACHE_DBUG0` writer"]
pub struct W(crate::W<PRO_DCACHE_DBUG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_DCACHE_DBUG0_SPEC>;
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
impl From<crate::W<PRO_DCACHE_DBUG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_DCACHE_DBUG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_SLAVE_WDATA` reader - "]
pub type PRO_SLAVE_WDATA_R = crate::BitReader<bool>;
#[doc = "Field `PRO_SLAVE_WDATA` writer - "]
pub type PRO_SLAVE_WDATA_W<'a> = crate::BitWriter<'a, u32, PRO_DCACHE_DBUG0_SPEC, bool, 0>;
#[doc = "Field `PRO_CACHE_MMU_IA` reader - "]
pub type PRO_CACHE_MMU_IA_R = crate::BitReader<bool>;
#[doc = "Field `PRO_CACHE_IA` reader - "]
pub type PRO_CACHE_IA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRO_CACHE_STATE` reader - "]
pub type PRO_CACHE_STATE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRO_WR_BAK_TO_READ` reader - "]
pub type PRO_WR_BAK_TO_READ_R = crate::BitReader<bool>;
#[doc = "Field `PRO_TX_END` reader - "]
pub type PRO_TX_END_R = crate::BitReader<bool>;
#[doc = "Field `PRO_SLAVE_WR` reader - "]
pub type PRO_SLAVE_WR_R = crate::BitReader<bool>;
#[doc = "Field `PRO_SLAVE_WDATA_V` reader - "]
pub type PRO_SLAVE_WDATA_V_R = crate::BitReader<bool>;
#[doc = "Field `PRO_RX_END` reader - "]
pub type PRO_RX_END_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_slave_wdata(&self) -> PRO_SLAVE_WDATA_R {
        PRO_SLAVE_WDATA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_cache_mmu_ia(&self) -> PRO_CACHE_MMU_IA_R {
        PRO_CACHE_MMU_IA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6"]
    #[inline(always)]
    pub fn pro_cache_ia(&self) -> PRO_CACHE_IA_R {
        PRO_CACHE_IA_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bits 7:18"]
    #[inline(always)]
    pub fn pro_cache_state(&self) -> PRO_CACHE_STATE_R {
        PRO_CACHE_STATE_R::new(((self.bits >> 7) & 0x0fff) as u16)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pro_wr_bak_to_read(&self) -> PRO_WR_BAK_TO_READ_R {
        PRO_WR_BAK_TO_READ_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pro_tx_end(&self) -> PRO_TX_END_R {
        PRO_TX_END_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pro_slave_wr(&self) -> PRO_SLAVE_WR_R {
        PRO_SLAVE_WR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pro_slave_wdata_v(&self) -> PRO_SLAVE_WDATA_V_R {
        PRO_SLAVE_WDATA_V_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pro_rx_end(&self) -> PRO_RX_END_R {
        PRO_RX_END_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_slave_wdata(&mut self) -> PRO_SLAVE_WDATA_W {
        PRO_SLAVE_WDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_dcache_dbug0](index.html) module"]
pub struct PRO_DCACHE_DBUG0_SPEC;
impl crate::RegisterSpec for PRO_DCACHE_DBUG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_dcache_dbug0::R](R) reader structure"]
impl crate::Readable for PRO_DCACHE_DBUG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_dcache_dbug0::W](W) writer structure"]
impl crate::Writable for PRO_DCACHE_DBUG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_DCACHE_DBUG0 to value 0"]
impl crate::Resettable for PRO_DCACHE_DBUG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
