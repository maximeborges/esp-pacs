#[doc = "Register `INT_ST` reader"]
pub struct R(crate::R<INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PER_END_INT_ST` reader - The status bit for SPI_MEM_PER_END_INT interrupt."]
pub struct PER_END_INT_ST_R(crate::FieldReader<bool>);
impl PER_END_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PER_END_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PER_END_INT_ST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PES_END_INT_ST` reader - The status bit for SPI_MEM_PES_END_INT interrupt."]
pub struct PES_END_INT_ST_R(crate::FieldReader<bool>);
impl PES_END_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PES_END_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PES_END_INT_ST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOTAL_TRANS_END_INT_ST` reader - The status bit for SPI_MEM_TOTAL_TRANS_END_INT interrupt."]
pub struct TOTAL_TRANS_END_INT_ST_R(crate::FieldReader<bool>);
impl TOTAL_TRANS_END_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOTAL_TRANS_END_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOTAL_TRANS_END_INT_ST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BROWN_OUT_INT_ST` reader - The status bit for SPI_MEM_BROWN_OUT_INT interrupt."]
pub struct BROWN_OUT_INT_ST_R(crate::FieldReader<bool>);
impl BROWN_OUT_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BROWN_OUT_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BROWN_OUT_INT_ST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - The status bit for SPI_MEM_PER_END_INT interrupt."]
    #[inline(always)]
    pub fn per_end_int_st(&self) -> PER_END_INT_ST_R {
        PER_END_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The status bit for SPI_MEM_PES_END_INT interrupt."]
    #[inline(always)]
    pub fn pes_end_int_st(&self) -> PES_END_INT_ST_R {
        PES_END_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The status bit for SPI_MEM_TOTAL_TRANS_END_INT interrupt."]
    #[inline(always)]
    pub fn total_trans_end_int_st(&self) -> TOTAL_TRANS_END_INT_ST_R {
        TOTAL_TRANS_END_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The status bit for SPI_MEM_BROWN_OUT_INT interrupt."]
    #[inline(always)]
    pub fn brown_out_int_st(&self) -> BROWN_OUT_INT_ST_R {
        BROWN_OUT_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "SPI1 interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](index.html) module"]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_st::R](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
