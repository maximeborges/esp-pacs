#[doc = "Register `DMA_APB_I_1` reader"]
pub struct R(crate::R<DMA_APB_I_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_APB_I_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_APB_I_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_APB_I_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_APB_I_1` writer"]
pub struct W(crate::W<DMA_APB_I_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_APB_I_1_SPEC>;
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
impl From<crate::W<DMA_APB_I_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_APB_I_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_APB_I_SRAM_0_R` reader - Setting to 1 grants internal DMA permission to read SRAM Block 0."]
pub struct DMA_APB_I_SRAM_0_R_R(crate::FieldReader<bool>);
impl DMA_APB_I_SRAM_0_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_APB_I_SRAM_0_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APB_I_SRAM_0_R_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APB_I_SRAM_0_R` writer - Setting to 1 grants internal DMA permission to read SRAM Block 0."]
pub struct DMA_APB_I_SRAM_0_R_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_APB_I_SRAM_0_R_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `DMA_APB_I_SRAM_0_W` reader - Setting to 1 grants internal DMA permission to write SRAM Block 0."]
pub struct DMA_APB_I_SRAM_0_W_R(crate::FieldReader<bool>);
impl DMA_APB_I_SRAM_0_W_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_APB_I_SRAM_0_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APB_I_SRAM_0_W_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APB_I_SRAM_0_W` writer - Setting to 1 grants internal DMA permission to write SRAM Block 0."]
pub struct DMA_APB_I_SRAM_0_W_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_APB_I_SRAM_0_W_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `DMA_APB_I_SRAM_1_R` reader - Setting to 1 grants internal DMA permission to read SRAM Block 1."]
pub struct DMA_APB_I_SRAM_1_R_R(crate::FieldReader<bool>);
impl DMA_APB_I_SRAM_1_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_APB_I_SRAM_1_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APB_I_SRAM_1_R_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APB_I_SRAM_1_R` writer - Setting to 1 grants internal DMA permission to read SRAM Block 1."]
pub struct DMA_APB_I_SRAM_1_R_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_APB_I_SRAM_1_R_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `DMA_APB_I_SRAM_1_W` reader - Setting to 1 grants internal DMA permission to write SRAM Block 1."]
pub struct DMA_APB_I_SRAM_1_W_R(crate::FieldReader<bool>);
impl DMA_APB_I_SRAM_1_W_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_APB_I_SRAM_1_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APB_I_SRAM_1_W_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APB_I_SRAM_1_W` writer - Setting to 1 grants internal DMA permission to write SRAM Block 1."]
pub struct DMA_APB_I_SRAM_1_W_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_APB_I_SRAM_1_W_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `DMA_APB_I_SRAM_2_R` reader - Setting to 1 grants internal DMA permission to read SRAM Block 2."]
pub struct DMA_APB_I_SRAM_2_R_R(crate::FieldReader<bool>);
impl DMA_APB_I_SRAM_2_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_APB_I_SRAM_2_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APB_I_SRAM_2_R_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APB_I_SRAM_2_R` writer - Setting to 1 grants internal DMA permission to read SRAM Block 2."]
pub struct DMA_APB_I_SRAM_2_R_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_APB_I_SRAM_2_R_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `DMA_APB_I_SRAM_2_W` reader - Setting to 1 grants internal DMA permission to write SRAM Block 2."]
pub struct DMA_APB_I_SRAM_2_W_R(crate::FieldReader<bool>);
impl DMA_APB_I_SRAM_2_W_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_APB_I_SRAM_2_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APB_I_SRAM_2_W_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APB_I_SRAM_2_W` writer - Setting to 1 grants internal DMA permission to write SRAM Block 2."]
pub struct DMA_APB_I_SRAM_2_W_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_APB_I_SRAM_2_W_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `DMA_APB_I_SRAM_3_R` reader - Setting to 1 grants internal DMA permission to read SRAM Block 3."]
pub struct DMA_APB_I_SRAM_3_R_R(crate::FieldReader<bool>);
impl DMA_APB_I_SRAM_3_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_APB_I_SRAM_3_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APB_I_SRAM_3_R_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APB_I_SRAM_3_R` writer - Setting to 1 grants internal DMA permission to read SRAM Block 3."]
pub struct DMA_APB_I_SRAM_3_R_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_APB_I_SRAM_3_R_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `DMA_APB_I_SRAM_3_W` reader - Setting to 1 grants internal DMA permission to write SRAM Block 3."]
pub struct DMA_APB_I_SRAM_3_W_R(crate::FieldReader<bool>);
impl DMA_APB_I_SRAM_3_W_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_APB_I_SRAM_3_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APB_I_SRAM_3_W_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APB_I_SRAM_3_W` writer - Setting to 1 grants internal DMA permission to write SRAM Block 3."]
pub struct DMA_APB_I_SRAM_3_W_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_APB_I_SRAM_3_W_W<'a> {
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
#[doc = "Field `DMA_APB_I_SRAM_4_SPLTADDR` reader - Configure the split address of SRAM Block 4-21 for internal DMA access."]
pub struct DMA_APB_I_SRAM_4_SPLTADDR_R(crate::FieldReader<u32>);
impl DMA_APB_I_SRAM_4_SPLTADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DMA_APB_I_SRAM_4_SPLTADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APB_I_SRAM_4_SPLTADDR_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APB_I_SRAM_4_SPLTADDR` writer - Configure the split address of SRAM Block 4-21 for internal DMA access."]
pub struct DMA_APB_I_SRAM_4_SPLTADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_APB_I_SRAM_4_SPLTADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0001_ffff << 8)) | ((value as u32 & 0x0001_ffff) << 8);
        self.w
    }
}
#[doc = "Field `DMA_APB_I_SRAM_4_L_R` reader - Setting to 1 grants internal DMA permission to read SRAM Block 4-21 low address region."]
pub struct DMA_APB_I_SRAM_4_L_R_R(crate::FieldReader<bool>);
impl DMA_APB_I_SRAM_4_L_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_APB_I_SRAM_4_L_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APB_I_SRAM_4_L_R_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APB_I_SRAM_4_L_R` writer - Setting to 1 grants internal DMA permission to read SRAM Block 4-21 low address region."]
pub struct DMA_APB_I_SRAM_4_L_R_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_APB_I_SRAM_4_L_R_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 25)) | ((value as u32 & 1) << 25);
        self.w
    }
}
#[doc = "Field `DMA_APB_I_SRAM_4_L_W` reader - Setting to 1 grants internal DMA permission to write SRAM Block 4-21 low address region."]
pub struct DMA_APB_I_SRAM_4_L_W_R(crate::FieldReader<bool>);
impl DMA_APB_I_SRAM_4_L_W_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_APB_I_SRAM_4_L_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APB_I_SRAM_4_L_W_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APB_I_SRAM_4_L_W` writer - Setting to 1 grants internal DMA permission to write SRAM Block 4-21 low address region."]
pub struct DMA_APB_I_SRAM_4_L_W_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_APB_I_SRAM_4_L_W_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 26)) | ((value as u32 & 1) << 26);
        self.w
    }
}
#[doc = "Field `DMA_APB_I_SRAM_4_H_R` reader - Setting to 1 grants internal DMA permission to read SRAM Block 4-21 high address region."]
pub struct DMA_APB_I_SRAM_4_H_R_R(crate::FieldReader<bool>);
impl DMA_APB_I_SRAM_4_H_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_APB_I_SRAM_4_H_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APB_I_SRAM_4_H_R_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APB_I_SRAM_4_H_R` writer - Setting to 1 grants internal DMA permission to read SRAM Block 4-21 high address region."]
pub struct DMA_APB_I_SRAM_4_H_R_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_APB_I_SRAM_4_H_R_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 27)) | ((value as u32 & 1) << 27);
        self.w
    }
}
#[doc = "Field `DMA_APB_I_SRAM_4_H_W` reader - Setting to 1 grants internal DMA permission to write SRAM Block 4-21 high address region."]
pub struct DMA_APB_I_SRAM_4_H_W_R(crate::FieldReader<bool>);
impl DMA_APB_I_SRAM_4_H_W_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_APB_I_SRAM_4_H_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APB_I_SRAM_4_H_W_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APB_I_SRAM_4_H_W` writer - Setting to 1 grants internal DMA permission to write SRAM Block 4-21 high address region."]
pub struct DMA_APB_I_SRAM_4_H_W_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_APB_I_SRAM_4_H_W_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Setting to 1 grants internal DMA permission to read SRAM Block 0."]
    #[inline(always)]
    pub fn dma_apb_i_sram_0_r(&self) -> DMA_APB_I_SRAM_0_R_R {
        DMA_APB_I_SRAM_0_R_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Setting to 1 grants internal DMA permission to write SRAM Block 0."]
    #[inline(always)]
    pub fn dma_apb_i_sram_0_w(&self) -> DMA_APB_I_SRAM_0_W_R {
        DMA_APB_I_SRAM_0_W_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Setting to 1 grants internal DMA permission to read SRAM Block 1."]
    #[inline(always)]
    pub fn dma_apb_i_sram_1_r(&self) -> DMA_APB_I_SRAM_1_R_R {
        DMA_APB_I_SRAM_1_R_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Setting to 1 grants internal DMA permission to write SRAM Block 1."]
    #[inline(always)]
    pub fn dma_apb_i_sram_1_w(&self) -> DMA_APB_I_SRAM_1_W_R {
        DMA_APB_I_SRAM_1_W_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Setting to 1 grants internal DMA permission to read SRAM Block 2."]
    #[inline(always)]
    pub fn dma_apb_i_sram_2_r(&self) -> DMA_APB_I_SRAM_2_R_R {
        DMA_APB_I_SRAM_2_R_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Setting to 1 grants internal DMA permission to write SRAM Block 2."]
    #[inline(always)]
    pub fn dma_apb_i_sram_2_w(&self) -> DMA_APB_I_SRAM_2_W_R {
        DMA_APB_I_SRAM_2_W_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Setting to 1 grants internal DMA permission to read SRAM Block 3."]
    #[inline(always)]
    pub fn dma_apb_i_sram_3_r(&self) -> DMA_APB_I_SRAM_3_R_R {
        DMA_APB_I_SRAM_3_R_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Setting to 1 grants internal DMA permission to write SRAM Block 3."]
    #[inline(always)]
    pub fn dma_apb_i_sram_3_w(&self) -> DMA_APB_I_SRAM_3_W_R {
        DMA_APB_I_SRAM_3_W_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:24 - Configure the split address of SRAM Block 4-21 for internal DMA access."]
    #[inline(always)]
    pub fn dma_apb_i_sram_4_spltaddr(&self) -> DMA_APB_I_SRAM_4_SPLTADDR_R {
        DMA_APB_I_SRAM_4_SPLTADDR_R::new(((self.bits >> 8) & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 25 - Setting to 1 grants internal DMA permission to read SRAM Block 4-21 low address region."]
    #[inline(always)]
    pub fn dma_apb_i_sram_4_l_r(&self) -> DMA_APB_I_SRAM_4_L_R_R {
        DMA_APB_I_SRAM_4_L_R_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Setting to 1 grants internal DMA permission to write SRAM Block 4-21 low address region."]
    #[inline(always)]
    pub fn dma_apb_i_sram_4_l_w(&self) -> DMA_APB_I_SRAM_4_L_W_R {
        DMA_APB_I_SRAM_4_L_W_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Setting to 1 grants internal DMA permission to read SRAM Block 4-21 high address region."]
    #[inline(always)]
    pub fn dma_apb_i_sram_4_h_r(&self) -> DMA_APB_I_SRAM_4_H_R_R {
        DMA_APB_I_SRAM_4_H_R_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Setting to 1 grants internal DMA permission to write SRAM Block 4-21 high address region."]
    #[inline(always)]
    pub fn dma_apb_i_sram_4_h_w(&self) -> DMA_APB_I_SRAM_4_H_W_R {
        DMA_APB_I_SRAM_4_H_W_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting to 1 grants internal DMA permission to read SRAM Block 0."]
    #[inline(always)]
    pub fn dma_apb_i_sram_0_r(&mut self) -> DMA_APB_I_SRAM_0_R_W {
        DMA_APB_I_SRAM_0_R_W { w: self }
    }
    #[doc = "Bit 1 - Setting to 1 grants internal DMA permission to write SRAM Block 0."]
    #[inline(always)]
    pub fn dma_apb_i_sram_0_w(&mut self) -> DMA_APB_I_SRAM_0_W_W {
        DMA_APB_I_SRAM_0_W_W { w: self }
    }
    #[doc = "Bit 2 - Setting to 1 grants internal DMA permission to read SRAM Block 1."]
    #[inline(always)]
    pub fn dma_apb_i_sram_1_r(&mut self) -> DMA_APB_I_SRAM_1_R_W {
        DMA_APB_I_SRAM_1_R_W { w: self }
    }
    #[doc = "Bit 3 - Setting to 1 grants internal DMA permission to write SRAM Block 1."]
    #[inline(always)]
    pub fn dma_apb_i_sram_1_w(&mut self) -> DMA_APB_I_SRAM_1_W_W {
        DMA_APB_I_SRAM_1_W_W { w: self }
    }
    #[doc = "Bit 4 - Setting to 1 grants internal DMA permission to read SRAM Block 2."]
    #[inline(always)]
    pub fn dma_apb_i_sram_2_r(&mut self) -> DMA_APB_I_SRAM_2_R_W {
        DMA_APB_I_SRAM_2_R_W { w: self }
    }
    #[doc = "Bit 5 - Setting to 1 grants internal DMA permission to write SRAM Block 2."]
    #[inline(always)]
    pub fn dma_apb_i_sram_2_w(&mut self) -> DMA_APB_I_SRAM_2_W_W {
        DMA_APB_I_SRAM_2_W_W { w: self }
    }
    #[doc = "Bit 6 - Setting to 1 grants internal DMA permission to read SRAM Block 3."]
    #[inline(always)]
    pub fn dma_apb_i_sram_3_r(&mut self) -> DMA_APB_I_SRAM_3_R_W {
        DMA_APB_I_SRAM_3_R_W { w: self }
    }
    #[doc = "Bit 7 - Setting to 1 grants internal DMA permission to write SRAM Block 3."]
    #[inline(always)]
    pub fn dma_apb_i_sram_3_w(&mut self) -> DMA_APB_I_SRAM_3_W_W {
        DMA_APB_I_SRAM_3_W_W { w: self }
    }
    #[doc = "Bits 8:24 - Configure the split address of SRAM Block 4-21 for internal DMA access."]
    #[inline(always)]
    pub fn dma_apb_i_sram_4_spltaddr(&mut self) -> DMA_APB_I_SRAM_4_SPLTADDR_W {
        DMA_APB_I_SRAM_4_SPLTADDR_W { w: self }
    }
    #[doc = "Bit 25 - Setting to 1 grants internal DMA permission to read SRAM Block 4-21 low address region."]
    #[inline(always)]
    pub fn dma_apb_i_sram_4_l_r(&mut self) -> DMA_APB_I_SRAM_4_L_R_W {
        DMA_APB_I_SRAM_4_L_R_W { w: self }
    }
    #[doc = "Bit 26 - Setting to 1 grants internal DMA permission to write SRAM Block 4-21 low address region."]
    #[inline(always)]
    pub fn dma_apb_i_sram_4_l_w(&mut self) -> DMA_APB_I_SRAM_4_L_W_W {
        DMA_APB_I_SRAM_4_L_W_W { w: self }
    }
    #[doc = "Bit 27 - Setting to 1 grants internal DMA permission to read SRAM Block 4-21 high address region."]
    #[inline(always)]
    pub fn dma_apb_i_sram_4_h_r(&mut self) -> DMA_APB_I_SRAM_4_H_R_W {
        DMA_APB_I_SRAM_4_H_R_W { w: self }
    }
    #[doc = "Bit 28 - Setting to 1 grants internal DMA permission to write SRAM Block 4-21 high address region."]
    #[inline(always)]
    pub fn dma_apb_i_sram_4_h_w(&mut self) -> DMA_APB_I_SRAM_4_H_W_W {
        DMA_APB_I_SRAM_4_H_W_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal DMA permission control register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_apb_i_1](index.html) module"]
pub struct DMA_APB_I_1_SPEC;
impl crate::RegisterSpec for DMA_APB_I_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_apb_i_1::R](R) reader structure"]
impl crate::Readable for DMA_APB_I_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_apb_i_1::W](W) writer structure"]
impl crate::Writable for DMA_APB_I_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_APB_I_1 to value 0x1e00_00ff"]
impl crate::Resettable for DMA_APB_I_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1e00_00ff
    }
}
