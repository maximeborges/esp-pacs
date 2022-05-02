#[doc = "Register `_1INT_ST` reader"]
pub struct R(crate::R<_1INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_1INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_1INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_1INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRHOST_BIT8_INT_ST` reader - "]
pub struct FRHOST_BIT8_INT_ST_R(crate::FieldReader<bool>);
impl FRHOST_BIT8_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRHOST_BIT8_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRHOST_BIT8_INT_ST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRHOST_BIT9_INT_ST` reader - "]
pub struct FRHOST_BIT9_INT_ST_R(crate::FieldReader<bool>);
impl FRHOST_BIT9_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRHOST_BIT9_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRHOST_BIT9_INT_ST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRHOST_BIT10_INT_ST` reader - "]
pub struct FRHOST_BIT10_INT_ST_R(crate::FieldReader<bool>);
impl FRHOST_BIT10_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRHOST_BIT10_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRHOST_BIT10_INT_ST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRHOST_BIT11_INT_ST` reader - "]
pub struct FRHOST_BIT11_INT_ST_R(crate::FieldReader<bool>);
impl FRHOST_BIT11_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRHOST_BIT11_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRHOST_BIT11_INT_ST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRHOST_BIT12_INT_ST` reader - "]
pub struct FRHOST_BIT12_INT_ST_R(crate::FieldReader<bool>);
impl FRHOST_BIT12_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRHOST_BIT12_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRHOST_BIT12_INT_ST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRHOST_BIT13_INT_ST` reader - "]
pub struct FRHOST_BIT13_INT_ST_R(crate::FieldReader<bool>);
impl FRHOST_BIT13_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRHOST_BIT13_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRHOST_BIT13_INT_ST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRHOST_BIT14_INT_ST` reader - "]
pub struct FRHOST_BIT14_INT_ST_R(crate::FieldReader<bool>);
impl FRHOST_BIT14_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRHOST_BIT14_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRHOST_BIT14_INT_ST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRHOST_BIT15_INT_ST` reader - "]
pub struct FRHOST_BIT15_INT_ST_R(crate::FieldReader<bool>);
impl FRHOST_BIT15_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRHOST_BIT15_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRHOST_BIT15_INT_ST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_RX_START_INT_ST` reader - "]
pub struct SLC1_RX_START_INT_ST_R(crate::FieldReader<bool>);
impl SLC1_RX_START_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_RX_START_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_RX_START_INT_ST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_TX_START_INT_ST` reader - "]
pub struct SLC1_TX_START_INT_ST_R(crate::FieldReader<bool>);
impl SLC1_TX_START_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_TX_START_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TX_START_INT_ST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_RX_UDF_INT_ST` reader - "]
pub struct SLC1_RX_UDF_INT_ST_R(crate::FieldReader<bool>);
impl SLC1_RX_UDF_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_RX_UDF_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_RX_UDF_INT_ST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_TX_OVF_INT_ST` reader - "]
pub struct SLC1_TX_OVF_INT_ST_R(crate::FieldReader<bool>);
impl SLC1_TX_OVF_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_TX_OVF_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TX_OVF_INT_ST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_TOKEN0_1TO0_INT_ST` reader - "]
pub struct SLC1_TOKEN0_1TO0_INT_ST_R(crate::FieldReader<bool>);
impl SLC1_TOKEN0_1TO0_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_TOKEN0_1TO0_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TOKEN0_1TO0_INT_ST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_TOKEN1_1TO0_INT_ST` reader - "]
pub struct SLC1_TOKEN1_1TO0_INT_ST_R(crate::FieldReader<bool>);
impl SLC1_TOKEN1_1TO0_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_TOKEN1_1TO0_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TOKEN1_1TO0_INT_ST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_TX_DONE_INT_ST` reader - "]
pub struct SLC1_TX_DONE_INT_ST_R(crate::FieldReader<bool>);
impl SLC1_TX_DONE_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_TX_DONE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TX_DONE_INT_ST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_TX_SUC_EOF_INT_ST` reader - "]
pub struct SLC1_TX_SUC_EOF_INT_ST_R(crate::FieldReader<bool>);
impl SLC1_TX_SUC_EOF_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_TX_SUC_EOF_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TX_SUC_EOF_INT_ST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_RX_DONE_INT_ST` reader - "]
pub struct SLC1_RX_DONE_INT_ST_R(crate::FieldReader<bool>);
impl SLC1_RX_DONE_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_RX_DONE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_RX_DONE_INT_ST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_RX_EOF_INT_ST` reader - "]
pub struct SLC1_RX_EOF_INT_ST_R(crate::FieldReader<bool>);
impl SLC1_RX_EOF_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_RX_EOF_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_RX_EOF_INT_ST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_TOHOST_INT_ST` reader - "]
pub struct SLC1_TOHOST_INT_ST_R(crate::FieldReader<bool>);
impl SLC1_TOHOST_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_TOHOST_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TOHOST_INT_ST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_TX_DSCR_ERR_INT_ST` reader - "]
pub struct SLC1_TX_DSCR_ERR_INT_ST_R(crate::FieldReader<bool>);
impl SLC1_TX_DSCR_ERR_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_TX_DSCR_ERR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TX_DSCR_ERR_INT_ST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_RX_DSCR_ERR_INT_ST` reader - "]
pub struct SLC1_RX_DSCR_ERR_INT_ST_R(crate::FieldReader<bool>);
impl SLC1_RX_DSCR_ERR_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_RX_DSCR_ERR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_RX_DSCR_ERR_INT_ST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_TX_DSCR_EMPTY_INT_ST` reader - "]
pub struct SLC1_TX_DSCR_EMPTY_INT_ST_R(crate::FieldReader<bool>);
impl SLC1_TX_DSCR_EMPTY_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_TX_DSCR_EMPTY_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TX_DSCR_EMPTY_INT_ST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_HOST_RD_ACK_INT_ST` reader - "]
pub struct SLC1_HOST_RD_ACK_INT_ST_R(crate::FieldReader<bool>);
impl SLC1_HOST_RD_ACK_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_HOST_RD_ACK_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_HOST_RD_ACK_INT_ST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_WR_RETRY_DONE_INT_ST` reader - "]
pub struct SLC1_WR_RETRY_DONE_INT_ST_R(crate::FieldReader<bool>);
impl SLC1_WR_RETRY_DONE_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_WR_RETRY_DONE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_WR_RETRY_DONE_INT_ST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_TX_ERR_EOF_INT_ST` reader - "]
pub struct SLC1_TX_ERR_EOF_INT_ST_R(crate::FieldReader<bool>);
impl SLC1_TX_ERR_EOF_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_TX_ERR_EOF_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TX_ERR_EOF_INT_ST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn frhost_bit8_int_st(&self) -> FRHOST_BIT8_INT_ST_R {
        FRHOST_BIT8_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn frhost_bit9_int_st(&self) -> FRHOST_BIT9_INT_ST_R {
        FRHOST_BIT9_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn frhost_bit10_int_st(&self) -> FRHOST_BIT10_INT_ST_R {
        FRHOST_BIT10_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn frhost_bit11_int_st(&self) -> FRHOST_BIT11_INT_ST_R {
        FRHOST_BIT11_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn frhost_bit12_int_st(&self) -> FRHOST_BIT12_INT_ST_R {
        FRHOST_BIT12_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn frhost_bit13_int_st(&self) -> FRHOST_BIT13_INT_ST_R {
        FRHOST_BIT13_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn frhost_bit14_int_st(&self) -> FRHOST_BIT14_INT_ST_R {
        FRHOST_BIT14_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn frhost_bit15_int_st(&self) -> FRHOST_BIT15_INT_ST_R {
        FRHOST_BIT15_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slc1_rx_start_int_st(&self) -> SLC1_RX_START_INT_ST_R {
        SLC1_RX_START_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slc1_tx_start_int_st(&self) -> SLC1_TX_START_INT_ST_R {
        SLC1_TX_START_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slc1_rx_udf_int_st(&self) -> SLC1_RX_UDF_INT_ST_R {
        SLC1_RX_UDF_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slc1_tx_ovf_int_st(&self) -> SLC1_TX_OVF_INT_ST_R {
        SLC1_TX_OVF_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc1_token0_1to0_int_st(&self) -> SLC1_TOKEN0_1TO0_INT_ST_R {
        SLC1_TOKEN0_1TO0_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc1_token1_1to0_int_st(&self) -> SLC1_TOKEN1_1TO0_INT_ST_R {
        SLC1_TOKEN1_1TO0_INT_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc1_tx_done_int_st(&self) -> SLC1_TX_DONE_INT_ST_R {
        SLC1_TX_DONE_INT_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slc1_tx_suc_eof_int_st(&self) -> SLC1_TX_SUC_EOF_INT_ST_R {
        SLC1_TX_SUC_EOF_INT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_rx_done_int_st(&self) -> SLC1_RX_DONE_INT_ST_R {
        SLC1_RX_DONE_INT_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc1_rx_eof_int_st(&self) -> SLC1_RX_EOF_INT_ST_R {
        SLC1_RX_EOF_INT_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc1_tohost_int_st(&self) -> SLC1_TOHOST_INT_ST_R {
        SLC1_TOHOST_INT_ST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc1_tx_dscr_err_int_st(&self) -> SLC1_TX_DSCR_ERR_INT_ST_R {
        SLC1_TX_DSCR_ERR_INT_ST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc1_rx_dscr_err_int_st(&self) -> SLC1_RX_DSCR_ERR_INT_ST_R {
        SLC1_RX_DSCR_ERR_INT_ST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slc1_tx_dscr_empty_int_st(&self) -> SLC1_TX_DSCR_EMPTY_INT_ST_R {
        SLC1_TX_DSCR_EMPTY_INT_ST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn slc1_host_rd_ack_int_st(&self) -> SLC1_HOST_RD_ACK_INT_ST_R {
        SLC1_HOST_RD_ACK_INT_ST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn slc1_wr_retry_done_int_st(&self) -> SLC1_WR_RETRY_DONE_INT_ST_R {
        SLC1_WR_RETRY_DONE_INT_ST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn slc1_tx_err_eof_int_st(&self) -> SLC1_TX_ERR_EOF_INT_ST_R {
        SLC1_TX_ERR_EOF_INT_ST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_1int_st](index.html) module"]
pub struct _1INT_ST_SPEC;
impl crate::RegisterSpec for _1INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_1int_st::R](R) reader structure"]
impl crate::Readable for _1INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets _1INT_ST to value 0"]
impl crate::Resettable for _1INT_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
