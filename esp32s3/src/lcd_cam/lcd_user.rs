#[doc = "Register `LCD_USER` reader"]
pub struct R(crate::R<LCD_USER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_USER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_USER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_USER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCD_USER` writer"]
pub struct W(crate::W<LCD_USER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_USER_SPEC>;
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
impl From<crate::W<LCD_USER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_USER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCD_DOUT_CYCLELEN` reader - The output data cycles minus 1 of LCD module."]
pub struct LCD_DOUT_CYCLELEN_R(crate::FieldReader<u16, u16>);
impl LCD_DOUT_CYCLELEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LCD_DOUT_CYCLELEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_DOUT_CYCLELEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_DOUT_CYCLELEN` writer - The output data cycles minus 1 of LCD module."]
pub struct LCD_DOUT_CYCLELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_DOUT_CYCLELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | (value as u32 & 0x1fff);
        self.w
    }
}
#[doc = "Field `LCD_ALWAYS_OUT_EN` reader - LCD always output when LCD is in LCD_DOUT state, unless reg_lcd_start is cleared or reg_lcd_reset is set."]
pub struct LCD_ALWAYS_OUT_EN_R(crate::FieldReader<bool, bool>);
impl LCD_ALWAYS_OUT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCD_ALWAYS_OUT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_ALWAYS_OUT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_ALWAYS_OUT_EN` writer - LCD always output when LCD is in LCD_DOUT state, unless reg_lcd_start is cleared or reg_lcd_reset is set."]
pub struct LCD_ALWAYS_OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_ALWAYS_OUT_EN_W<'a> {
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
#[doc = "Field `LCD_8BITS_ORDER` reader - 1: invert every two data byte, valid in 1 byte mode. 0: Not change."]
pub struct LCD_8BITS_ORDER_R(crate::FieldReader<bool, bool>);
impl LCD_8BITS_ORDER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCD_8BITS_ORDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_8BITS_ORDER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_8BITS_ORDER` writer - 1: invert every two data byte, valid in 1 byte mode. 0: Not change."]
pub struct LCD_8BITS_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_8BITS_ORDER_W<'a> {
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
#[doc = "Field `LCD_UPDATE` reader - 1: Update LCD registers, will be cleared by hardware. 0 : Not care."]
pub struct LCD_UPDATE_R(crate::FieldReader<bool, bool>);
impl LCD_UPDATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCD_UPDATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_UPDATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_UPDATE` writer - 1: Update LCD registers, will be cleared by hardware. 0 : Not care."]
pub struct LCD_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_UPDATE_W<'a> {
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
#[doc = "Field `LCD_BIT_ORDER` reader - 1: Change data bit order, change LCD_DATA_out\\[7:0\\]
 to LCD_DATA_out\\[0:7\\]
 in one byte mode, and bits\\[15:0\\]
 to bits\\[0:15\\]
 in two byte mode. 0: Not change."]
pub struct LCD_BIT_ORDER_R(crate::FieldReader<bool, bool>);
impl LCD_BIT_ORDER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCD_BIT_ORDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_BIT_ORDER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_BIT_ORDER` writer - 1: Change data bit order, change LCD_DATA_out\\[7:0\\]
 to LCD_DATA_out\\[0:7\\]
 in one byte mode, and bits\\[15:0\\]
 to bits\\[0:15\\]
 in two byte mode. 0: Not change."]
pub struct LCD_BIT_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_BIT_ORDER_W<'a> {
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
#[doc = "Field `LCD_BYTE_ORDER` reader - 1: invert data byte order, only valid in 2 byte mode. 0: Not change."]
pub struct LCD_BYTE_ORDER_R(crate::FieldReader<bool, bool>);
impl LCD_BYTE_ORDER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCD_BYTE_ORDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_BYTE_ORDER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_BYTE_ORDER` writer - 1: invert data byte order, only valid in 2 byte mode. 0: Not change."]
pub struct LCD_BYTE_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_BYTE_ORDER_W<'a> {
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
#[doc = "Field `LCD_2BYTE_EN` reader - 1: The bit number of output LCD data is 9~16. 0: The bit number of output LCD data is 0~8."]
pub struct LCD_2BYTE_EN_R(crate::FieldReader<bool, bool>);
impl LCD_2BYTE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCD_2BYTE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_2BYTE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_2BYTE_EN` writer - 1: The bit number of output LCD data is 9~16. 0: The bit number of output LCD data is 0~8."]
pub struct LCD_2BYTE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_2BYTE_EN_W<'a> {
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
#[doc = "Field `LCD_DOUT` reader - 1: Be able to send data out in LCD sequence when LCD starts. 0: Disable."]
pub struct LCD_DOUT_R(crate::FieldReader<bool, bool>);
impl LCD_DOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCD_DOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_DOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_DOUT` writer - 1: Be able to send data out in LCD sequence when LCD starts. 0: Disable."]
pub struct LCD_DOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_DOUT_W<'a> {
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
#[doc = "Field `LCD_DUMMY` reader - 1: Enable DUMMY phase in LCD sequence when LCD starts. 0: Disable."]
pub struct LCD_DUMMY_R(crate::FieldReader<bool, bool>);
impl LCD_DUMMY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCD_DUMMY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_DUMMY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_DUMMY` writer - 1: Enable DUMMY phase in LCD sequence when LCD starts. 0: Disable."]
pub struct LCD_DUMMY_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_DUMMY_W<'a> {
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
#[doc = "Field `LCD_CMD` reader - 1: Be able to send command in LCD sequence when LCD starts. 0: Disable."]
pub struct LCD_CMD_R(crate::FieldReader<bool, bool>);
impl LCD_CMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCD_CMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_CMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_CMD` writer - 1: Be able to send command in LCD sequence when LCD starts. 0: Disable."]
pub struct LCD_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_CMD_W<'a> {
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
#[doc = "Field `LCD_START` reader - LCD start sending data enable signal, valid in high level."]
pub struct LCD_START_R(crate::FieldReader<bool, bool>);
impl LCD_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCD_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_START` writer - LCD start sending data enable signal, valid in high level."]
pub struct LCD_START_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_START_W<'a> {
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
#[doc = "Field `LCD_RESET` writer - The value of command."]
pub struct LCD_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_RESET_W<'a> {
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
#[doc = "Field `LCD_DUMMY_CYCLELEN` reader - The dummy cycle length minus 1."]
pub struct LCD_DUMMY_CYCLELEN_R(crate::FieldReader<u8, u8>);
impl LCD_DUMMY_CYCLELEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LCD_DUMMY_CYCLELEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_DUMMY_CYCLELEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_DUMMY_CYCLELEN` writer - The dummy cycle length minus 1."]
pub struct LCD_DUMMY_CYCLELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_DUMMY_CYCLELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 29)) | ((value as u32 & 3) << 29);
        self.w
    }
}
#[doc = "Field `LCD_CMD_2_CYCLE_EN` reader - The cycle length of command phase. 1: 2 cycles. 0: 1 cycle."]
pub struct LCD_CMD_2_CYCLE_EN_R(crate::FieldReader<bool, bool>);
impl LCD_CMD_2_CYCLE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCD_CMD_2_CYCLE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_CMD_2_CYCLE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_CMD_2_CYCLE_EN` writer - The cycle length of command phase. 1: 2 cycles. 0: 1 cycle."]
pub struct LCD_CMD_2_CYCLE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_CMD_2_CYCLE_EN_W<'a> {
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
    #[doc = "Bits 0:12 - The output data cycles minus 1 of LCD module."]
    #[inline(always)]
    pub fn lcd_dout_cyclelen(&self) -> LCD_DOUT_CYCLELEN_R {
        LCD_DOUT_CYCLELEN_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13 - LCD always output when LCD is in LCD_DOUT state, unless reg_lcd_start is cleared or reg_lcd_reset is set."]
    #[inline(always)]
    pub fn lcd_always_out_en(&self) -> LCD_ALWAYS_OUT_EN_R {
        LCD_ALWAYS_OUT_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: invert every two data byte, valid in 1 byte mode. 0: Not change."]
    #[inline(always)]
    pub fn lcd_8bits_order(&self) -> LCD_8BITS_ORDER_R {
        LCD_8BITS_ORDER_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: Update LCD registers, will be cleared by hardware. 0 : Not care."]
    #[inline(always)]
    pub fn lcd_update(&self) -> LCD_UPDATE_R {
        LCD_UPDATE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: Change data bit order, change LCD_DATA_out\\[7:0\\]
 to LCD_DATA_out\\[0:7\\]
 in one byte mode, and bits\\[15:0\\]
 to bits\\[0:15\\]
 in two byte mode. 0: Not change."]
    #[inline(always)]
    pub fn lcd_bit_order(&self) -> LCD_BIT_ORDER_R {
        LCD_BIT_ORDER_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: invert data byte order, only valid in 2 byte mode. 0: Not change."]
    #[inline(always)]
    pub fn lcd_byte_order(&self) -> LCD_BYTE_ORDER_R {
        LCD_BYTE_ORDER_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: The bit number of output LCD data is 9~16. 0: The bit number of output LCD data is 0~8."]
    #[inline(always)]
    pub fn lcd_2byte_en(&self) -> LCD_2BYTE_EN_R {
        LCD_2BYTE_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: Be able to send data out in LCD sequence when LCD starts. 0: Disable."]
    #[inline(always)]
    pub fn lcd_dout(&self) -> LCD_DOUT_R {
        LCD_DOUT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: Enable DUMMY phase in LCD sequence when LCD starts. 0: Disable."]
    #[inline(always)]
    pub fn lcd_dummy(&self) -> LCD_DUMMY_R {
        LCD_DUMMY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: Be able to send command in LCD sequence when LCD starts. 0: Disable."]
    #[inline(always)]
    pub fn lcd_cmd(&self) -> LCD_CMD_R {
        LCD_CMD_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LCD start sending data enable signal, valid in high level."]
    #[inline(always)]
    pub fn lcd_start(&self) -> LCD_START_R {
        LCD_START_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 29:30 - The dummy cycle length minus 1."]
    #[inline(always)]
    pub fn lcd_dummy_cyclelen(&self) -> LCD_DUMMY_CYCLELEN_R {
        LCD_DUMMY_CYCLELEN_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - The cycle length of command phase. 1: 2 cycles. 0: 1 cycle."]
    #[inline(always)]
    pub fn lcd_cmd_2_cycle_en(&self) -> LCD_CMD_2_CYCLE_EN_R {
        LCD_CMD_2_CYCLE_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - The output data cycles minus 1 of LCD module."]
    #[inline(always)]
    pub fn lcd_dout_cyclelen(&mut self) -> LCD_DOUT_CYCLELEN_W {
        LCD_DOUT_CYCLELEN_W { w: self }
    }
    #[doc = "Bit 13 - LCD always output when LCD is in LCD_DOUT state, unless reg_lcd_start is cleared or reg_lcd_reset is set."]
    #[inline(always)]
    pub fn lcd_always_out_en(&mut self) -> LCD_ALWAYS_OUT_EN_W {
        LCD_ALWAYS_OUT_EN_W { w: self }
    }
    #[doc = "Bit 19 - 1: invert every two data byte, valid in 1 byte mode. 0: Not change."]
    #[inline(always)]
    pub fn lcd_8bits_order(&mut self) -> LCD_8BITS_ORDER_W {
        LCD_8BITS_ORDER_W { w: self }
    }
    #[doc = "Bit 20 - 1: Update LCD registers, will be cleared by hardware. 0 : Not care."]
    #[inline(always)]
    pub fn lcd_update(&mut self) -> LCD_UPDATE_W {
        LCD_UPDATE_W { w: self }
    }
    #[doc = "Bit 21 - 1: Change data bit order, change LCD_DATA_out\\[7:0\\]
 to LCD_DATA_out\\[0:7\\]
 in one byte mode, and bits\\[15:0\\]
 to bits\\[0:15\\]
 in two byte mode. 0: Not change."]
    #[inline(always)]
    pub fn lcd_bit_order(&mut self) -> LCD_BIT_ORDER_W {
        LCD_BIT_ORDER_W { w: self }
    }
    #[doc = "Bit 22 - 1: invert data byte order, only valid in 2 byte mode. 0: Not change."]
    #[inline(always)]
    pub fn lcd_byte_order(&mut self) -> LCD_BYTE_ORDER_W {
        LCD_BYTE_ORDER_W { w: self }
    }
    #[doc = "Bit 23 - 1: The bit number of output LCD data is 9~16. 0: The bit number of output LCD data is 0~8."]
    #[inline(always)]
    pub fn lcd_2byte_en(&mut self) -> LCD_2BYTE_EN_W {
        LCD_2BYTE_EN_W { w: self }
    }
    #[doc = "Bit 24 - 1: Be able to send data out in LCD sequence when LCD starts. 0: Disable."]
    #[inline(always)]
    pub fn lcd_dout(&mut self) -> LCD_DOUT_W {
        LCD_DOUT_W { w: self }
    }
    #[doc = "Bit 25 - 1: Enable DUMMY phase in LCD sequence when LCD starts. 0: Disable."]
    #[inline(always)]
    pub fn lcd_dummy(&mut self) -> LCD_DUMMY_W {
        LCD_DUMMY_W { w: self }
    }
    #[doc = "Bit 26 - 1: Be able to send command in LCD sequence when LCD starts. 0: Disable."]
    #[inline(always)]
    pub fn lcd_cmd(&mut self) -> LCD_CMD_W {
        LCD_CMD_W { w: self }
    }
    #[doc = "Bit 27 - LCD start sending data enable signal, valid in high level."]
    #[inline(always)]
    pub fn lcd_start(&mut self) -> LCD_START_W {
        LCD_START_W { w: self }
    }
    #[doc = "Bit 28 - The value of command."]
    #[inline(always)]
    pub fn lcd_reset(&mut self) -> LCD_RESET_W {
        LCD_RESET_W { w: self }
    }
    #[doc = "Bits 29:30 - The dummy cycle length minus 1."]
    #[inline(always)]
    pub fn lcd_dummy_cyclelen(&mut self) -> LCD_DUMMY_CYCLELEN_W {
        LCD_DUMMY_CYCLELEN_W { w: self }
    }
    #[doc = "Bit 31 - The cycle length of command phase. 1: 2 cycles. 0: 1 cycle."]
    #[inline(always)]
    pub fn lcd_cmd_2_cycle_en(&mut self) -> LCD_CMD_2_CYCLE_EN_W {
        LCD_CMD_2_CYCLE_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD configuration register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_user]
(index.html) module"]
pub struct LCD_USER_SPEC;
impl crate::RegisterSpec for LCD_USER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_user::R]
(R) reader structure"]
impl crate::Readable for LCD_USER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_user::W]
(W) writer structure"]
impl crate::Writable for LCD_USER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCD_USER to value 0x01"]
impl crate::Resettable for LCD_USER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
