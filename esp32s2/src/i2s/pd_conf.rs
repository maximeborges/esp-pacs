#[doc = "Register `PD_CONF` reader"]
pub struct R(crate::R<PD_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PD_CONF` writer"]
pub struct W(crate::W<PD_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD_CONF_SPEC>;
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
impl From<crate::W<PD_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFO_FORCE_PD` reader - Force FIFO power-down."]
pub type FIFO_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_FORCE_PD` writer - Force FIFO power-down."]
pub type FIFO_FORCE_PD_W<'a> = crate::BitWriter<'a, u32, PD_CONF_SPEC, bool, 0>;
#[doc = "Field `FIFO_FORCE_PU` reader - Force FIFO power-up."]
pub type FIFO_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_FORCE_PU` writer - Force FIFO power-up."]
pub type FIFO_FORCE_PU_W<'a> = crate::BitWriter<'a, u32, PD_CONF_SPEC, bool, 1>;
#[doc = "Field `PLC_MEM_FORCE_PD` reader - Force I2S memory power-down."]
pub type PLC_MEM_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `PLC_MEM_FORCE_PD` writer - Force I2S memory power-down."]
pub type PLC_MEM_FORCE_PD_W<'a> = crate::BitWriter<'a, u32, PD_CONF_SPEC, bool, 2>;
#[doc = "Field `PLC_MEM_FORCE_PU` reader - Force I2S memory power-up."]
pub type PLC_MEM_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `PLC_MEM_FORCE_PU` writer - Force I2S memory power-up."]
pub type PLC_MEM_FORCE_PU_W<'a> = crate::BitWriter<'a, u32, PD_CONF_SPEC, bool, 3>;
#[doc = "Field `DMA_RAM_FORCE_PD` reader - Force DMA FIFO power-down."]
pub type DMA_RAM_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `DMA_RAM_FORCE_PD` writer - Force DMA FIFO power-down."]
pub type DMA_RAM_FORCE_PD_W<'a> = crate::BitWriter<'a, u32, PD_CONF_SPEC, bool, 4>;
#[doc = "Field `DMA_RAM_FORCE_PU` reader - Force DMA FIFO power-up."]
pub type DMA_RAM_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `DMA_RAM_FORCE_PU` writer - Force DMA FIFO power-up."]
pub type DMA_RAM_FORCE_PU_W<'a> = crate::BitWriter<'a, u32, PD_CONF_SPEC, bool, 5>;
#[doc = "Field `DMA_RAM_CLK_FO` reader - Set this bit to force on DMA RAM clock."]
pub type DMA_RAM_CLK_FO_R = crate::BitReader<bool>;
#[doc = "Field `DMA_RAM_CLK_FO` writer - Set this bit to force on DMA RAM clock."]
pub type DMA_RAM_CLK_FO_W<'a> = crate::BitWriter<'a, u32, PD_CONF_SPEC, bool, 6>;
impl R {
    #[doc = "Bit 0 - Force FIFO power-down."]
    #[inline(always)]
    pub fn fifo_force_pd(&self) -> FIFO_FORCE_PD_R {
        FIFO_FORCE_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force FIFO power-up."]
    #[inline(always)]
    pub fn fifo_force_pu(&self) -> FIFO_FORCE_PU_R {
        FIFO_FORCE_PU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force I2S memory power-down."]
    #[inline(always)]
    pub fn plc_mem_force_pd(&self) -> PLC_MEM_FORCE_PD_R {
        PLC_MEM_FORCE_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force I2S memory power-up."]
    #[inline(always)]
    pub fn plc_mem_force_pu(&self) -> PLC_MEM_FORCE_PU_R {
        PLC_MEM_FORCE_PU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Force DMA FIFO power-down."]
    #[inline(always)]
    pub fn dma_ram_force_pd(&self) -> DMA_RAM_FORCE_PD_R {
        DMA_RAM_FORCE_PD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Force DMA FIFO power-up."]
    #[inline(always)]
    pub fn dma_ram_force_pu(&self) -> DMA_RAM_FORCE_PU_R {
        DMA_RAM_FORCE_PU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to force on DMA RAM clock."]
    #[inline(always)]
    pub fn dma_ram_clk_fo(&self) -> DMA_RAM_CLK_FO_R {
        DMA_RAM_CLK_FO_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force FIFO power-down."]
    #[inline(always)]
    pub fn fifo_force_pd(&mut self) -> FIFO_FORCE_PD_W {
        FIFO_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 1 - Force FIFO power-up."]
    #[inline(always)]
    pub fn fifo_force_pu(&mut self) -> FIFO_FORCE_PU_W {
        FIFO_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 2 - Force I2S memory power-down."]
    #[inline(always)]
    pub fn plc_mem_force_pd(&mut self) -> PLC_MEM_FORCE_PD_W {
        PLC_MEM_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 3 - Force I2S memory power-up."]
    #[inline(always)]
    pub fn plc_mem_force_pu(&mut self) -> PLC_MEM_FORCE_PU_W {
        PLC_MEM_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 4 - Force DMA FIFO power-down."]
    #[inline(always)]
    pub fn dma_ram_force_pd(&mut self) -> DMA_RAM_FORCE_PD_W {
        DMA_RAM_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 5 - Force DMA FIFO power-up."]
    #[inline(always)]
    pub fn dma_ram_force_pu(&mut self) -> DMA_RAM_FORCE_PU_W {
        DMA_RAM_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to force on DMA RAM clock."]
    #[inline(always)]
    pub fn dma_ram_clk_fo(&mut self) -> DMA_RAM_CLK_FO_W {
        DMA_RAM_CLK_FO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S power-down configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd_conf](index.html) module"]
pub struct PD_CONF_SPEC;
impl crate::RegisterSpec for PD_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd_conf::R](R) reader structure"]
impl crate::Readable for PD_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd_conf::W](W) writer structure"]
impl crate::Writable for PD_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PD_CONF to value 0x2a"]
impl crate::Resettable for PD_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2a
    }
}
