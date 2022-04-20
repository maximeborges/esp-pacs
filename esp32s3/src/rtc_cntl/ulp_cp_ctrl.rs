#[doc = "Register `ULP_CP_CTRL` reader"]
pub struct R(crate::R<ULP_CP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ULP_CP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ULP_CP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ULP_CP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ULP_CP_CTRL` writer"]
pub struct W(crate::W<ULP_CP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ULP_CP_CTRL_SPEC>;
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
impl From<crate::W<ULP_CP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ULP_CP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ULP_CP_MEM_ADDR_INIT` reader - No public"]
pub struct ULP_CP_MEM_ADDR_INIT_R(crate::FieldReader<u16, u16>);
impl ULP_CP_MEM_ADDR_INIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ULP_CP_MEM_ADDR_INIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ULP_CP_MEM_ADDR_INIT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULP_CP_MEM_ADDR_INIT` writer - No public"]
pub struct ULP_CP_MEM_ADDR_INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> ULP_CP_MEM_ADDR_INIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
#[doc = "Field `ULP_CP_MEM_ADDR_SIZE` reader - No public"]
pub struct ULP_CP_MEM_ADDR_SIZE_R(crate::FieldReader<u16, u16>);
impl ULP_CP_MEM_ADDR_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ULP_CP_MEM_ADDR_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ULP_CP_MEM_ADDR_SIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULP_CP_MEM_ADDR_SIZE` writer - No public"]
pub struct ULP_CP_MEM_ADDR_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> ULP_CP_MEM_ADDR_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 11)) | ((value as u32 & 0x07ff) << 11);
        self.w
    }
}
#[doc = "Field `ULP_CP_MEM_OFFST_CLR` writer - No public"]
pub struct ULP_CP_MEM_OFFST_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ULP_CP_MEM_OFFST_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 22)) | ((value as u32 & 1) << 22);
        self.w
    }
}
#[doc = "Field `ULP_CP_CLK_FO` reader - ulp coprocessor clk force on"]
pub struct ULP_CP_CLK_FO_R(crate::FieldReader<bool, bool>);
impl ULP_CP_CLK_FO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ULP_CP_CLK_FO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ULP_CP_CLK_FO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULP_CP_CLK_FO` writer - ulp coprocessor clk force on"]
pub struct ULP_CP_CLK_FO_W<'a> {
    w: &'a mut W,
}
impl<'a> ULP_CP_CLK_FO_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 28)) | ((value as u32 & 1) << 28);
        self.w
    }
}
#[doc = "Field `ULP_CP_RESET` reader - ulp coprocessor clk software reset"]
pub struct ULP_CP_RESET_R(crate::FieldReader<bool, bool>);
impl ULP_CP_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ULP_CP_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ULP_CP_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULP_CP_RESET` writer - ulp coprocessor clk software reset"]
pub struct ULP_CP_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> ULP_CP_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 29)) | ((value as u32 & 1) << 29);
        self.w
    }
}
#[doc = "Field `ULP_CP_FORCE_START_TOP` reader - 1: ULP-coprocessor is started by SW"]
pub struct ULP_CP_FORCE_START_TOP_R(crate::FieldReader<bool, bool>);
impl ULP_CP_FORCE_START_TOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ULP_CP_FORCE_START_TOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ULP_CP_FORCE_START_TOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULP_CP_FORCE_START_TOP` writer - 1: ULP-coprocessor is started by SW"]
pub struct ULP_CP_FORCE_START_TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> ULP_CP_FORCE_START_TOP_W<'a> {
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
#[doc = "Field `ULP_CP_START_TOP` reader - Write 1 to start ULP-coprocessor"]
pub struct ULP_CP_START_TOP_R(crate::FieldReader<bool, bool>);
impl ULP_CP_START_TOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ULP_CP_START_TOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ULP_CP_START_TOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULP_CP_START_TOP` writer - Write 1 to start ULP-coprocessor"]
pub struct ULP_CP_START_TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> ULP_CP_START_TOP_W<'a> {
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
    #[doc = "Bits 0:10 - No public"]
    #[inline(always)]
    pub fn ulp_cp_mem_addr_init(&self) -> ULP_CP_MEM_ADDR_INIT_R {
        ULP_CP_MEM_ADDR_INIT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - No public"]
    #[inline(always)]
    pub fn ulp_cp_mem_addr_size(&self) -> ULP_CP_MEM_ADDR_SIZE_R {
        ULP_CP_MEM_ADDR_SIZE_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - ulp coprocessor clk force on"]
    #[inline(always)]
    pub fn ulp_cp_clk_fo(&self) -> ULP_CP_CLK_FO_R {
        ULP_CP_CLK_FO_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ulp coprocessor clk software reset"]
    #[inline(always)]
    pub fn ulp_cp_reset(&self) -> ULP_CP_RESET_R {
        ULP_CP_RESET_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: ULP-coprocessor is started by SW"]
    #[inline(always)]
    pub fn ulp_cp_force_start_top(&self) -> ULP_CP_FORCE_START_TOP_R {
        ULP_CP_FORCE_START_TOP_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Write 1 to start ULP-coprocessor"]
    #[inline(always)]
    pub fn ulp_cp_start_top(&self) -> ULP_CP_START_TOP_R {
        ULP_CP_START_TOP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - No public"]
    #[inline(always)]
    pub fn ulp_cp_mem_addr_init(&mut self) -> ULP_CP_MEM_ADDR_INIT_W {
        ULP_CP_MEM_ADDR_INIT_W { w: self }
    }
    #[doc = "Bits 11:21 - No public"]
    #[inline(always)]
    pub fn ulp_cp_mem_addr_size(&mut self) -> ULP_CP_MEM_ADDR_SIZE_W {
        ULP_CP_MEM_ADDR_SIZE_W { w: self }
    }
    #[doc = "Bit 22 - No public"]
    #[inline(always)]
    pub fn ulp_cp_mem_offst_clr(&mut self) -> ULP_CP_MEM_OFFST_CLR_W {
        ULP_CP_MEM_OFFST_CLR_W { w: self }
    }
    #[doc = "Bit 28 - ulp coprocessor clk force on"]
    #[inline(always)]
    pub fn ulp_cp_clk_fo(&mut self) -> ULP_CP_CLK_FO_W {
        ULP_CP_CLK_FO_W { w: self }
    }
    #[doc = "Bit 29 - ulp coprocessor clk software reset"]
    #[inline(always)]
    pub fn ulp_cp_reset(&mut self) -> ULP_CP_RESET_W {
        ULP_CP_RESET_W { w: self }
    }
    #[doc = "Bit 30 - 1: ULP-coprocessor is started by SW"]
    #[inline(always)]
    pub fn ulp_cp_force_start_top(&mut self) -> ULP_CP_FORCE_START_TOP_W {
        ULP_CP_FORCE_START_TOP_W { w: self }
    }
    #[doc = "Bit 31 - Write 1 to start ULP-coprocessor"]
    #[inline(always)]
    pub fn ulp_cp_start_top(&mut self) -> ULP_CP_START_TOP_W {
        ULP_CP_START_TOP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure ulp\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ulp_cp_ctrl]
(index.html) module"]
pub struct ULP_CP_CTRL_SPEC;
impl crate::RegisterSpec for ULP_CP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ulp_cp_ctrl::R]
(R) reader structure"]
impl crate::Readable for ULP_CP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ulp_cp_ctrl::W]
(W) writer structure"]
impl crate::Writable for ULP_CP_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ULP_CP_CTRL to value 0x0010_0200"]
impl crate::Resettable for ULP_CP_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0010_0200
    }
}
