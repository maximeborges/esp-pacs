#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FDUMMY_OUT` reader - In the dummy phase the signal level of spi is output by the spi controller."]
pub struct FDUMMY_OUT_R(crate::FieldReader<bool>);
impl FDUMMY_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FDUMMY_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDUMMY_OUT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDUMMY_OUT` writer - In the dummy phase the signal level of spi is output by the spi controller."]
pub struct FDUMMY_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> FDUMMY_OUT_W<'a> {
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
#[doc = "Field `FCMD_DUAL` reader - Apply 2 signals during command phase 1:enable 0: disable"]
pub struct FCMD_DUAL_R(crate::FieldReader<bool>);
impl FCMD_DUAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FCMD_DUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCMD_DUAL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCMD_DUAL` writer - Apply 2 signals during command phase 1:enable 0: disable"]
pub struct FCMD_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> FCMD_DUAL_W<'a> {
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
#[doc = "Field `FCMD_QUAD` reader - Apply 4 signals during command phase 1:enable 0: disable"]
pub struct FCMD_QUAD_R(crate::FieldReader<bool>);
impl FCMD_QUAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FCMD_QUAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCMD_QUAD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCMD_QUAD` writer - Apply 4 signals during command phase 1:enable 0: disable"]
pub struct FCMD_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> FCMD_QUAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `FASTRD_MODE` reader - This bit enable the bits: spi_mem_fread_qio, spi_mem_fread_dio, spi_mem_fread_qout and spi_mem_fread_dout. 1: enable 0: disable."]
pub struct FASTRD_MODE_R(crate::FieldReader<bool>);
impl FASTRD_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FASTRD_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FASTRD_MODE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FASTRD_MODE` writer - This bit enable the bits: spi_mem_fread_qio, spi_mem_fread_dio, spi_mem_fread_qout and spi_mem_fread_dout. 1: enable 0: disable."]
pub struct FASTRD_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTRD_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `FREAD_DUAL` reader - In the read operations, read-data phase apply 2 signals. 1: enable 0: disable."]
pub struct FREAD_DUAL_R(crate::FieldReader<bool>);
impl FREAD_DUAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FREAD_DUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREAD_DUAL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREAD_DUAL` writer - In the read operations, read-data phase apply 2 signals. 1: enable 0: disable."]
pub struct FREAD_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> FREAD_DUAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u32 & 1) << 14);
        self.w
    }
}
#[doc = "Field `Q_POL` reader - The bit is used to set MISO line polarity, 1: high 0, low"]
pub struct Q_POL_R(crate::FieldReader<bool>);
impl Q_POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Q_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for Q_POL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Q_POL` writer - The bit is used to set MISO line polarity, 1: high 0, low"]
pub struct Q_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> Q_POL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 18)) | ((value as u32 & 1) << 18);
        self.w
    }
}
#[doc = "Field `D_POL` reader - The bit is used to set MOSI line polarity, 1: high 0, low"]
pub struct D_POL_R(crate::FieldReader<bool>);
impl D_POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        D_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for D_POL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `D_POL` writer - The bit is used to set MOSI line polarity, 1: high 0, low"]
pub struct D_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> D_POL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 19)) | ((value as u32 & 1) << 19);
        self.w
    }
}
#[doc = "Field `FREAD_QUAD` reader - In the read operations read-data phase apply 4 signals. 1: enable 0: disable."]
pub struct FREAD_QUAD_R(crate::FieldReader<bool>);
impl FREAD_QUAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FREAD_QUAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREAD_QUAD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREAD_QUAD` writer - In the read operations read-data phase apply 4 signals. 1: enable 0: disable."]
pub struct FREAD_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> FREAD_QUAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 20)) | ((value as u32 & 1) << 20);
        self.w
    }
}
#[doc = "Field `WP` reader - Write protect signal output when SPI is idle. 1: output high, 0: output low."]
pub struct WP_R(crate::FieldReader<bool>);
impl WP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WP_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WP` writer - Write protect signal output when SPI is idle. 1: output high, 0: output low."]
pub struct WP_W<'a> {
    w: &'a mut W,
}
impl<'a> WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 21)) | ((value as u32 & 1) << 21);
        self.w
    }
}
#[doc = "Field `FREAD_DIO` reader - In the read operations address phase and read-data phase apply 2 signals. 1: enable 0: disable."]
pub struct FREAD_DIO_R(crate::FieldReader<bool>);
impl FREAD_DIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FREAD_DIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREAD_DIO_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREAD_DIO` writer - In the read operations address phase and read-data phase apply 2 signals. 1: enable 0: disable."]
pub struct FREAD_DIO_W<'a> {
    w: &'a mut W,
}
impl<'a> FREAD_DIO_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 23)) | ((value as u32 & 1) << 23);
        self.w
    }
}
#[doc = "Field `FREAD_QIO` reader - In the read operations address phase and read-data phase apply 4 signals. 1: enable 0: disable."]
pub struct FREAD_QIO_R(crate::FieldReader<bool>);
impl FREAD_QIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FREAD_QIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREAD_QIO_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREAD_QIO` writer - In the read operations address phase and read-data phase apply 4 signals. 1: enable 0: disable."]
pub struct FREAD_QIO_W<'a> {
    w: &'a mut W,
}
impl<'a> FREAD_QIO_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 24)) | ((value as u32 & 1) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - In the dummy phase the signal level of spi is output by the spi controller."]
    #[inline(always)]
    pub fn fdummy_out(&self) -> FDUMMY_OUT_R {
        FDUMMY_OUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Apply 2 signals during command phase 1:enable 0: disable"]
    #[inline(always)]
    pub fn fcmd_dual(&self) -> FCMD_DUAL_R {
        FCMD_DUAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Apply 4 signals during command phase 1:enable 0: disable"]
    #[inline(always)]
    pub fn fcmd_quad(&self) -> FCMD_QUAD_R {
        FCMD_QUAD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 13 - This bit enable the bits: spi_mem_fread_qio, spi_mem_fread_dio, spi_mem_fread_qout and spi_mem_fread_dout. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fastrd_mode(&self) -> FASTRD_MODE_R {
        FASTRD_MODE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - In the read operations, read-data phase apply 2 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_dual(&self) -> FREAD_DUAL_R {
        FREAD_DUAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - The bit is used to set MISO line polarity, 1: high 0, low"]
    #[inline(always)]
    pub fn q_pol(&self) -> Q_POL_R {
        Q_POL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The bit is used to set MOSI line polarity, 1: high 0, low"]
    #[inline(always)]
    pub fn d_pol(&self) -> D_POL_R {
        D_POL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - In the read operations read-data phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_quad(&self) -> FREAD_QUAD_R {
        FREAD_QUAD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Write protect signal output when SPI is idle. 1: output high, 0: output low."]
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - In the read operations address phase and read-data phase apply 2 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_dio(&self) -> FREAD_DIO_R {
        FREAD_DIO_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - In the read operations address phase and read-data phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_qio(&self) -> FREAD_QIO_R {
        FREAD_QIO_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - In the dummy phase the signal level of spi is output by the spi controller."]
    #[inline(always)]
    pub fn fdummy_out(&mut self) -> FDUMMY_OUT_W {
        FDUMMY_OUT_W { w: self }
    }
    #[doc = "Bit 7 - Apply 2 signals during command phase 1:enable 0: disable"]
    #[inline(always)]
    pub fn fcmd_dual(&mut self) -> FCMD_DUAL_W {
        FCMD_DUAL_W { w: self }
    }
    #[doc = "Bit 8 - Apply 4 signals during command phase 1:enable 0: disable"]
    #[inline(always)]
    pub fn fcmd_quad(&mut self) -> FCMD_QUAD_W {
        FCMD_QUAD_W { w: self }
    }
    #[doc = "Bit 13 - This bit enable the bits: spi_mem_fread_qio, spi_mem_fread_dio, spi_mem_fread_qout and spi_mem_fread_dout. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fastrd_mode(&mut self) -> FASTRD_MODE_W {
        FASTRD_MODE_W { w: self }
    }
    #[doc = "Bit 14 - In the read operations, read-data phase apply 2 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_dual(&mut self) -> FREAD_DUAL_W {
        FREAD_DUAL_W { w: self }
    }
    #[doc = "Bit 18 - The bit is used to set MISO line polarity, 1: high 0, low"]
    #[inline(always)]
    pub fn q_pol(&mut self) -> Q_POL_W {
        Q_POL_W { w: self }
    }
    #[doc = "Bit 19 - The bit is used to set MOSI line polarity, 1: high 0, low"]
    #[inline(always)]
    pub fn d_pol(&mut self) -> D_POL_W {
        D_POL_W { w: self }
    }
    #[doc = "Bit 20 - In the read operations read-data phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_quad(&mut self) -> FREAD_QUAD_W {
        FREAD_QUAD_W { w: self }
    }
    #[doc = "Bit 21 - Write protect signal output when SPI is idle. 1: output high, 0: output low."]
    #[inline(always)]
    pub fn wp(&mut self) -> WP_W {
        WP_W { w: self }
    }
    #[doc = "Bit 23 - In the read operations address phase and read-data phase apply 2 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_dio(&mut self) -> FREAD_DIO_W {
        FREAD_DIO_W { w: self }
    }
    #[doc = "Bit 24 - In the read operations address phase and read-data phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_qio(&mut self) -> FREAD_QIO_W {
        FREAD_QIO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0x002c_2000"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x002c_2000
    }
}
