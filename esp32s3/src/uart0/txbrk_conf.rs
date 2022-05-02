#[doc = "Register `TXBRK_CONF` reader"]
pub struct R(crate::R<TXBRK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBRK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBRK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBRK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXBRK_CONF` writer"]
pub struct W(crate::W<TXBRK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXBRK_CONF_SPEC>;
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
impl From<crate::W<TXBRK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXBRK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_BRK_NUM` reader - This register is used to configure the number of 0 to be sent after the process of sending data is done. It is active when txd_brk is set to 1."]
pub struct TX_BRK_NUM_R(crate::FieldReader<u8>);
impl TX_BRK_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_BRK_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BRK_NUM_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BRK_NUM` writer - This register is used to configure the number of 0 to be sent after the process of sending data is done. It is active when txd_brk is set to 1."]
pub struct TX_BRK_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BRK_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - This register is used to configure the number of 0 to be sent after the process of sending data is done. It is active when txd_brk is set to 1."]
    #[inline(always)]
    pub fn tx_brk_num(&self) -> TX_BRK_NUM_R {
        TX_BRK_NUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is used to configure the number of 0 to be sent after the process of sending data is done. It is active when txd_brk is set to 1."]
    #[inline(always)]
    pub fn tx_brk_num(&mut self) -> TX_BRK_NUM_W {
        TX_BRK_NUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tx Break character configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbrk_conf](index.html) module"]
pub struct TXBRK_CONF_SPEC;
impl crate::RegisterSpec for TXBRK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbrk_conf::R](R) reader structure"]
impl crate::Readable for TXBRK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txbrk_conf::W](W) writer structure"]
impl crate::Writable for TXBRK_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXBRK_CONF to value 0x0a"]
impl crate::Resettable for TXBRK_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a
    }
}
