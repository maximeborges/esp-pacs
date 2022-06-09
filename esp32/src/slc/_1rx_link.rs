#[doc = "Register `_1RX_LINK` reader"]
pub struct R(crate::R<_1RX_LINK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_1RX_LINK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_1RX_LINK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_1RX_LINK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_1RX_LINK` writer"]
pub struct W(crate::W<_1RX_LINK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_1RX_LINK_SPEC>;
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
impl From<crate::W<_1RX_LINK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_1RX_LINK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC1_RXLINK_ADDR` reader - "]
pub type SLC1_RXLINK_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SLC1_RXLINK_ADDR` writer - "]
pub type SLC1_RXLINK_ADDR_W<'a> = crate::FieldWriter<'a, u32, _1RX_LINK_SPEC, u32, u32, 20, 0>;
#[doc = "Field `SLC1_BT_PACKET` reader - "]
pub type SLC1_BT_PACKET_R = crate::BitReader<bool>;
#[doc = "Field `SLC1_BT_PACKET` writer - "]
pub type SLC1_BT_PACKET_W<'a> = crate::BitWriter<'a, u32, _1RX_LINK_SPEC, bool, 20>;
#[doc = "Field `SLC1_RXLINK_STOP` reader - "]
pub type SLC1_RXLINK_STOP_R = crate::BitReader<bool>;
#[doc = "Field `SLC1_RXLINK_STOP` writer - "]
pub type SLC1_RXLINK_STOP_W<'a> = crate::BitWriter<'a, u32, _1RX_LINK_SPEC, bool, 28>;
#[doc = "Field `SLC1_RXLINK_START` reader - "]
pub type SLC1_RXLINK_START_R = crate::BitReader<bool>;
#[doc = "Field `SLC1_RXLINK_START` writer - "]
pub type SLC1_RXLINK_START_W<'a> = crate::BitWriter<'a, u32, _1RX_LINK_SPEC, bool, 29>;
#[doc = "Field `SLC1_RXLINK_RESTART` reader - "]
pub type SLC1_RXLINK_RESTART_R = crate::BitReader<bool>;
#[doc = "Field `SLC1_RXLINK_RESTART` writer - "]
pub type SLC1_RXLINK_RESTART_W<'a> = crate::BitWriter<'a, u32, _1RX_LINK_SPEC, bool, 30>;
#[doc = "Field `SLC1_RXLINK_PARK` reader - "]
pub type SLC1_RXLINK_PARK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn slc1_rxlink_addr(&self) -> SLC1_RXLINK_ADDR_R {
        SLC1_RXLINK_ADDR_R::new((self.bits & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc1_bt_packet(&self) -> SLC1_BT_PACKET_R {
        SLC1_BT_PACKET_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn slc1_rxlink_stop(&self) -> SLC1_RXLINK_STOP_R {
        SLC1_RXLINK_STOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn slc1_rxlink_start(&self) -> SLC1_RXLINK_START_R {
        SLC1_RXLINK_START_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn slc1_rxlink_restart(&self) -> SLC1_RXLINK_RESTART_R {
        SLC1_RXLINK_RESTART_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn slc1_rxlink_park(&self) -> SLC1_RXLINK_PARK_R {
        SLC1_RXLINK_PARK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn slc1_rxlink_addr(&mut self) -> SLC1_RXLINK_ADDR_W {
        SLC1_RXLINK_ADDR_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc1_bt_packet(&mut self) -> SLC1_BT_PACKET_W {
        SLC1_BT_PACKET_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn slc1_rxlink_stop(&mut self) -> SLC1_RXLINK_STOP_W {
        SLC1_RXLINK_STOP_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn slc1_rxlink_start(&mut self) -> SLC1_RXLINK_START_W {
        SLC1_RXLINK_START_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn slc1_rxlink_restart(&mut self) -> SLC1_RXLINK_RESTART_W {
        SLC1_RXLINK_RESTART_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_1rx_link](index.html) module"]
pub struct _1RX_LINK_SPEC;
impl crate::RegisterSpec for _1RX_LINK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_1rx_link::R](R) reader structure"]
impl crate::Readable for _1RX_LINK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_1rx_link::W](W) writer structure"]
impl crate::Writable for _1RX_LINK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets _1RX_LINK to value 0x0010_0000"]
impl crate::Resettable for _1RX_LINK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0010_0000
    }
}
