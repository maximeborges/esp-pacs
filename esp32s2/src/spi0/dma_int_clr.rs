#[doc = "Register `DMA_INT_CLR` reader"]
pub struct R(crate::R<DMA_INT_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_INT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_INT_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_INT_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_INT_CLR` writer"]
pub struct W(crate::W<DMA_INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_INT_CLR_SPEC>;
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
impl From<crate::W<DMA_INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INLINK_DSCR_EMPTY_INT_CLR` reader - The clear bit for lack of enough inlink descriptors. Can be configured in CONF state."]
pub struct INLINK_DSCR_EMPTY_INT_CLR_R(crate::FieldReader<bool, bool>);
impl INLINK_DSCR_EMPTY_INT_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INLINK_DSCR_EMPTY_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INLINK_DSCR_EMPTY_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INLINK_DSCR_EMPTY_INT_CLR` writer - The clear bit for lack of enough inlink descriptors. Can be configured in CONF state."]
pub struct INLINK_DSCR_EMPTY_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> INLINK_DSCR_EMPTY_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `OUTLINK_DSCR_ERROR_INT_CLR` reader - The clear bit for outlink descriptor error. Can be configured in CONF state."]
pub struct OUTLINK_DSCR_ERROR_INT_CLR_R(crate::FieldReader<bool, bool>);
impl OUTLINK_DSCR_ERROR_INT_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTLINK_DSCR_ERROR_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTLINK_DSCR_ERROR_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTLINK_DSCR_ERROR_INT_CLR` writer - The clear bit for outlink descriptor error. Can be configured in CONF state."]
pub struct OUTLINK_DSCR_ERROR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTLINK_DSCR_ERROR_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `INLINK_DSCR_ERROR_INT_CLR` reader - The clear bit for inlink descriptor error. Can be configured in CONF state."]
pub struct INLINK_DSCR_ERROR_INT_CLR_R(crate::FieldReader<bool, bool>);
impl INLINK_DSCR_ERROR_INT_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INLINK_DSCR_ERROR_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INLINK_DSCR_ERROR_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INLINK_DSCR_ERROR_INT_CLR` writer - The clear bit for inlink descriptor error. Can be configured in CONF state."]
pub struct INLINK_DSCR_ERROR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> INLINK_DSCR_ERROR_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `IN_DONE_INT_CLR` reader - The clear bit for completing usage of a inlink descriptor. Can be configured in CONF state."]
pub struct IN_DONE_INT_CLR_R(crate::FieldReader<bool, bool>);
impl IN_DONE_INT_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_DONE_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_DONE_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_DONE_INT_CLR` writer - The clear bit for completing usage of a inlink descriptor. Can be configured in CONF state."]
pub struct IN_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_DONE_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `IN_ERR_EOF_INT_CLR` reader - The clear bit for receiving error. Can be configured in CONF state."]
pub struct IN_ERR_EOF_INT_CLR_R(crate::FieldReader<bool, bool>);
impl IN_ERR_EOF_INT_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_ERR_EOF_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_ERR_EOF_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_ERR_EOF_INT_CLR` writer - The clear bit for receiving error. Can be configured in CONF state."]
pub struct IN_ERR_EOF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_ERR_EOF_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `IN_SUC_EOF_INT_CLR` reader - The clear bit for completing receiving all the packets from host. Can be configured in CONF state."]
pub struct IN_SUC_EOF_INT_CLR_R(crate::FieldReader<bool, bool>);
impl IN_SUC_EOF_INT_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_SUC_EOF_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_SUC_EOF_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_SUC_EOF_INT_CLR` writer - The clear bit for completing receiving all the packets from host. Can be configured in CONF state."]
pub struct IN_SUC_EOF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_SUC_EOF_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `OUT_DONE_INT_CLR` reader - The clear bit for completing usage of a outlink descriptor. Can be configured in CONF state."]
pub struct OUT_DONE_INT_CLR_R(crate::FieldReader<bool, bool>);
impl OUT_DONE_INT_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_DONE_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_DONE_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_DONE_INT_CLR` writer - The clear bit for completing usage of a outlink descriptor. Can be configured in CONF state."]
pub struct OUT_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_DONE_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `OUT_EOF_INT_CLR` reader - The clear bit for sending a packet to host done. Can be configured in CONF state."]
pub struct OUT_EOF_INT_CLR_R(crate::FieldReader<bool, bool>);
impl OUT_EOF_INT_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_EOF_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_EOF_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_EOF_INT_CLR` writer - The clear bit for sending a packet to host done. Can be configured in CONF state."]
pub struct OUT_EOF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_EOF_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `OUT_TOTAL_EOF_INT_CLR` reader - The clear bit for sending all the packets to host done. Can be configured in CONF state."]
pub struct OUT_TOTAL_EOF_INT_CLR_R(crate::FieldReader<bool, bool>);
impl OUT_TOTAL_EOF_INT_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_TOTAL_EOF_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_TOTAL_EOF_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_TOTAL_EOF_INT_CLR` writer - The clear bit for sending all the packets to host done. Can be configured in CONF state."]
pub struct OUT_TOTAL_EOF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_TOTAL_EOF_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `INFIFO_FULL_ERR_INT_CLR` reader - 1: Clear SPI_INFIFO_FULL_ERR_INT_RAW. 0: not valid. Can be changed by CONF_buf."]
pub struct INFIFO_FULL_ERR_INT_CLR_R(crate::FieldReader<bool, bool>);
impl INFIFO_FULL_ERR_INT_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INFIFO_FULL_ERR_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INFIFO_FULL_ERR_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INFIFO_FULL_ERR_INT_CLR` writer - 1: Clear SPI_INFIFO_FULL_ERR_INT_RAW. 0: not valid. Can be changed by CONF_buf."]
pub struct INFIFO_FULL_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> INFIFO_FULL_ERR_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `OUTFIFO_EMPTY_ERR_INT_CLR` reader - 1: Clear SPI_OUTFIFO_EMPTY_ERR_INT_RAW signal. 0: not valid. Can be changed by CONF_buf."]
pub struct OUTFIFO_EMPTY_ERR_INT_CLR_R(crate::FieldReader<bool, bool>);
impl OUTFIFO_EMPTY_ERR_INT_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTFIFO_EMPTY_ERR_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTFIFO_EMPTY_ERR_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTFIFO_EMPTY_ERR_INT_CLR` writer - 1: Clear SPI_OUTFIFO_EMPTY_ERR_INT_RAW signal. 0: not valid. Can be changed by CONF_buf."]
pub struct OUTFIFO_EMPTY_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTFIFO_EMPTY_ERR_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `SLV_CMD6_INT_CLR` reader - The clear bit for SPI slave CMD6 interrupt."]
pub struct SLV_CMD6_INT_CLR_R(crate::FieldReader<bool, bool>);
impl SLV_CMD6_INT_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_CMD6_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_CMD6_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_CMD6_INT_CLR` writer - The clear bit for SPI slave CMD6 interrupt."]
pub struct SLV_CMD6_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CMD6_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `SLV_CMD7_INT_CLR` reader - The clear bit for SPI slave CMD7 interrupt."]
pub struct SLV_CMD7_INT_CLR_R(crate::FieldReader<bool, bool>);
impl SLV_CMD7_INT_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_CMD7_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_CMD7_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_CMD7_INT_CLR` writer - The clear bit for SPI slave CMD7 interrupt."]
pub struct SLV_CMD7_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CMD7_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `SLV_CMD8_INT_CLR` reader - The clear bit for SPI slave CMD8 interrupt."]
pub struct SLV_CMD8_INT_CLR_R(crate::FieldReader<bool, bool>);
impl SLV_CMD8_INT_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_CMD8_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_CMD8_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_CMD8_INT_CLR` writer - The clear bit for SPI slave CMD8 interrupt."]
pub struct SLV_CMD8_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CMD8_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `SLV_CMD9_INT_CLR` reader - The clear bit for SPI slave CMD9 interrupt."]
pub struct SLV_CMD9_INT_CLR_R(crate::FieldReader<bool, bool>);
impl SLV_CMD9_INT_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_CMD9_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_CMD9_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_CMD9_INT_CLR` writer - The clear bit for SPI slave CMD9 interrupt."]
pub struct SLV_CMD9_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CMD9_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `SLV_CMDA_INT_CLR` reader - The clear bit for SPI slave CMDA interrupt."]
pub struct SLV_CMDA_INT_CLR_R(crate::FieldReader<bool, bool>);
impl SLV_CMDA_INT_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_CMDA_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_CMDA_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_CMDA_INT_CLR` writer - The clear bit for SPI slave CMDA interrupt."]
pub struct SLV_CMDA_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CMDA_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - The clear bit for lack of enough inlink descriptors. Can be configured in CONF state."]
    #[inline(always)]
    pub fn inlink_dscr_empty_int_clr(&self) -> INLINK_DSCR_EMPTY_INT_CLR_R {
        INLINK_DSCR_EMPTY_INT_CLR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The clear bit for outlink descriptor error. Can be configured in CONF state."]
    #[inline(always)]
    pub fn outlink_dscr_error_int_clr(&self) -> OUTLINK_DSCR_ERROR_INT_CLR_R {
        OUTLINK_DSCR_ERROR_INT_CLR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The clear bit for inlink descriptor error. Can be configured in CONF state."]
    #[inline(always)]
    pub fn inlink_dscr_error_int_clr(&self) -> INLINK_DSCR_ERROR_INT_CLR_R {
        INLINK_DSCR_ERROR_INT_CLR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The clear bit for completing usage of a inlink descriptor. Can be configured in CONF state."]
    #[inline(always)]
    pub fn in_done_int_clr(&self) -> IN_DONE_INT_CLR_R {
        IN_DONE_INT_CLR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The clear bit for receiving error. Can be configured in CONF state."]
    #[inline(always)]
    pub fn in_err_eof_int_clr(&self) -> IN_ERR_EOF_INT_CLR_R {
        IN_ERR_EOF_INT_CLR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - The clear bit for completing receiving all the packets from host. Can be configured in CONF state."]
    #[inline(always)]
    pub fn in_suc_eof_int_clr(&self) -> IN_SUC_EOF_INT_CLR_R {
        IN_SUC_EOF_INT_CLR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The clear bit for completing usage of a outlink descriptor. Can be configured in CONF state."]
    #[inline(always)]
    pub fn out_done_int_clr(&self) -> OUT_DONE_INT_CLR_R {
        OUT_DONE_INT_CLR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The clear bit for sending a packet to host done. Can be configured in CONF state."]
    #[inline(always)]
    pub fn out_eof_int_clr(&self) -> OUT_EOF_INT_CLR_R {
        OUT_EOF_INT_CLR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - The clear bit for sending all the packets to host done. Can be configured in CONF state."]
    #[inline(always)]
    pub fn out_total_eof_int_clr(&self) -> OUT_TOTAL_EOF_INT_CLR_R {
        OUT_TOTAL_EOF_INT_CLR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 1: Clear SPI_INFIFO_FULL_ERR_INT_RAW. 0: not valid. Can be changed by CONF_buf."]
    #[inline(always)]
    pub fn infifo_full_err_int_clr(&self) -> INFIFO_FULL_ERR_INT_CLR_R {
        INFIFO_FULL_ERR_INT_CLR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 1: Clear SPI_OUTFIFO_EMPTY_ERR_INT_RAW signal. 0: not valid. Can be changed by CONF_buf."]
    #[inline(always)]
    pub fn outfifo_empty_err_int_clr(&self) -> OUTFIFO_EMPTY_ERR_INT_CLR_R {
        OUTFIFO_EMPTY_ERR_INT_CLR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - The clear bit for SPI slave CMD6 interrupt."]
    #[inline(always)]
    pub fn slv_cmd6_int_clr(&self) -> SLV_CMD6_INT_CLR_R {
        SLV_CMD6_INT_CLR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - The clear bit for SPI slave CMD7 interrupt."]
    #[inline(always)]
    pub fn slv_cmd7_int_clr(&self) -> SLV_CMD7_INT_CLR_R {
        SLV_CMD7_INT_CLR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - The clear bit for SPI slave CMD8 interrupt."]
    #[inline(always)]
    pub fn slv_cmd8_int_clr(&self) -> SLV_CMD8_INT_CLR_R {
        SLV_CMD8_INT_CLR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - The clear bit for SPI slave CMD9 interrupt."]
    #[inline(always)]
    pub fn slv_cmd9_int_clr(&self) -> SLV_CMD9_INT_CLR_R {
        SLV_CMD9_INT_CLR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - The clear bit for SPI slave CMDA interrupt."]
    #[inline(always)]
    pub fn slv_cmda_int_clr(&self) -> SLV_CMDA_INT_CLR_R {
        SLV_CMDA_INT_CLR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The clear bit for lack of enough inlink descriptors. Can be configured in CONF state."]
    #[inline(always)]
    pub fn inlink_dscr_empty_int_clr(&mut self) -> INLINK_DSCR_EMPTY_INT_CLR_W {
        INLINK_DSCR_EMPTY_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1 - The clear bit for outlink descriptor error. Can be configured in CONF state."]
    #[inline(always)]
    pub fn outlink_dscr_error_int_clr(&mut self) -> OUTLINK_DSCR_ERROR_INT_CLR_W {
        OUTLINK_DSCR_ERROR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2 - The clear bit for inlink descriptor error. Can be configured in CONF state."]
    #[inline(always)]
    pub fn inlink_dscr_error_int_clr(&mut self) -> INLINK_DSCR_ERROR_INT_CLR_W {
        INLINK_DSCR_ERROR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3 - The clear bit for completing usage of a inlink descriptor. Can be configured in CONF state."]
    #[inline(always)]
    pub fn in_done_int_clr(&mut self) -> IN_DONE_INT_CLR_W {
        IN_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4 - The clear bit for receiving error. Can be configured in CONF state."]
    #[inline(always)]
    pub fn in_err_eof_int_clr(&mut self) -> IN_ERR_EOF_INT_CLR_W {
        IN_ERR_EOF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5 - The clear bit for completing receiving all the packets from host. Can be configured in CONF state."]
    #[inline(always)]
    pub fn in_suc_eof_int_clr(&mut self) -> IN_SUC_EOF_INT_CLR_W {
        IN_SUC_EOF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6 - The clear bit for completing usage of a outlink descriptor. Can be configured in CONF state."]
    #[inline(always)]
    pub fn out_done_int_clr(&mut self) -> OUT_DONE_INT_CLR_W {
        OUT_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7 - The clear bit for sending a packet to host done. Can be configured in CONF state."]
    #[inline(always)]
    pub fn out_eof_int_clr(&mut self) -> OUT_EOF_INT_CLR_W {
        OUT_EOF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 8 - The clear bit for sending all the packets to host done. Can be configured in CONF state."]
    #[inline(always)]
    pub fn out_total_eof_int_clr(&mut self) -> OUT_TOTAL_EOF_INT_CLR_W {
        OUT_TOTAL_EOF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 9 - 1: Clear SPI_INFIFO_FULL_ERR_INT_RAW. 0: not valid. Can be changed by CONF_buf."]
    #[inline(always)]
    pub fn infifo_full_err_int_clr(&mut self) -> INFIFO_FULL_ERR_INT_CLR_W {
        INFIFO_FULL_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 10 - 1: Clear SPI_OUTFIFO_EMPTY_ERR_INT_RAW signal. 0: not valid. Can be changed by CONF_buf."]
    #[inline(always)]
    pub fn outfifo_empty_err_int_clr(&mut self) -> OUTFIFO_EMPTY_ERR_INT_CLR_W {
        OUTFIFO_EMPTY_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 11 - The clear bit for SPI slave CMD6 interrupt."]
    #[inline(always)]
    pub fn slv_cmd6_int_clr(&mut self) -> SLV_CMD6_INT_CLR_W {
        SLV_CMD6_INT_CLR_W { w: self }
    }
    #[doc = "Bit 12 - The clear bit for SPI slave CMD7 interrupt."]
    #[inline(always)]
    pub fn slv_cmd7_int_clr(&mut self) -> SLV_CMD7_INT_CLR_W {
        SLV_CMD7_INT_CLR_W { w: self }
    }
    #[doc = "Bit 13 - The clear bit for SPI slave CMD8 interrupt."]
    #[inline(always)]
    pub fn slv_cmd8_int_clr(&mut self) -> SLV_CMD8_INT_CLR_W {
        SLV_CMD8_INT_CLR_W { w: self }
    }
    #[doc = "Bit 14 - The clear bit for SPI slave CMD9 interrupt."]
    #[inline(always)]
    pub fn slv_cmd9_int_clr(&mut self) -> SLV_CMD9_INT_CLR_W {
        SLV_CMD9_INT_CLR_W { w: self }
    }
    #[doc = "Bit 15 - The clear bit for SPI slave CMDA interrupt."]
    #[inline(always)]
    pub fn slv_cmda_int_clr(&mut self) -> SLV_CMDA_INT_CLR_W {
        SLV_CMDA_INT_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI DMA interrupt clear register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_clr]
(index.html) module"]
pub struct DMA_INT_CLR_SPEC;
impl crate::RegisterSpec for DMA_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_int_clr::R]
(R) reader structure"]
impl crate::Readable for DMA_INT_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_int_clr::W]
(W) writer structure"]
impl crate::Writable for DMA_INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_INT_CLR to value 0"]
impl crate::Resettable for DMA_INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
