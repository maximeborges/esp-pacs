#[doc = "Register `ID` reader"]
pub struct R(crate::R<ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ID` writer"]
pub struct W(crate::W<ID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ID_SPEC>;
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
impl From<crate::W<ID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ID` reader - This register is used to configure the uart_id."]
pub struct ID_R(crate::FieldReader<u32, u32>);
impl ID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ID` writer - This register is used to configure the uart_id."]
pub struct ID_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | (value as u32 & 0x3fff_ffff);
        self.w
    }
}
#[doc = "Field `HIGH_SPEED` reader - This bit used to select synchronize mode. 1: Registers are auto synchronized into UART Core clock and UART core should be keep the same with APB clock. 0: After configure registers, software needs to write 1 to UART_REG_UPDATE to synchronize registers."]
pub struct HIGH_SPEED_R(crate::FieldReader<bool, bool>);
impl HIGH_SPEED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HIGH_SPEED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HIGH_SPEED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIGH_SPEED` writer - This bit used to select synchronize mode. 1: Registers are auto synchronized into UART Core clock and UART core should be keep the same with APB clock. 0: After configure registers, software needs to write 1 to UART_REG_UPDATE to synchronize registers."]
pub struct HIGH_SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> HIGH_SPEED_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "Field `REG_UPDATE` reader - Software write 1 would synchronize registers into UART Core clock domain and would be cleared by hardware after synchronization is done."]
pub struct REG_UPDATE_R(crate::FieldReader<bool, bool>);
impl REG_UPDATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_UPDATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_UPDATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG_UPDATE` writer - Software write 1 would synchronize registers into UART Core clock domain and would be cleared by hardware after synchronization is done."]
pub struct REG_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_UPDATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29 - This register is used to configure the uart_id."]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x3fff_ffff) as u32)
    }
    #[doc = "Bit 30 - This bit used to select synchronize mode. 1: Registers are auto synchronized into UART Core clock and UART core should be keep the same with APB clock. 0: After configure registers, software needs to write 1 to UART_REG_UPDATE to synchronize registers."]
    #[inline(always)]
    pub fn high_speed(&self) -> HIGH_SPEED_R {
        HIGH_SPEED_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Software write 1 would synchronize registers into UART Core clock domain and would be cleared by hardware after synchronization is done."]
    #[inline(always)]
    pub fn reg_update(&self) -> REG_UPDATE_R {
        REG_UPDATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:29 - This register is used to configure the uart_id."]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W {
        ID_W { w: self }
    }
    #[doc = "Bit 30 - This bit used to select synchronize mode. 1: Registers are auto synchronized into UART Core clock and UART core should be keep the same with APB clock. 0: After configure registers, software needs to write 1 to UART_REG_UPDATE to synchronize registers."]
    #[inline(always)]
    pub fn high_speed(&mut self) -> HIGH_SPEED_W {
        HIGH_SPEED_W { w: self }
    }
    #[doc = "Bit 31 - Software write 1 would synchronize registers into UART Core clock domain and would be cleared by hardware after synchronization is done."]
    #[inline(always)]
    pub fn reg_update(&mut self) -> REG_UPDATE_W {
        REG_UPDATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART ID register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id]
(index.html) module"]
pub struct ID_SPEC;
impl crate::RegisterSpec for ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id::R]
(R) reader structure"]
impl crate::Readable for ID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [id::W]
(W) writer structure"]
impl crate::Writable for ID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ID to value 0x4000_0500"]
impl crate::Resettable for ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000_0500
    }
}
