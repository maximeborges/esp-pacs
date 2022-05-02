#[doc = "Register `PERI_BACKUP_INT_CLR` writer"]
pub struct W(crate::W<PERI_BACKUP_INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERI_BACKUP_INT_CLR_SPEC>;
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
impl From<crate::W<PERI_BACKUP_INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERI_BACKUP_INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERI_BACKUP_DONE_INT_CLR` writer - reg_peri_backup_done_int_clr"]
pub struct PERI_BACKUP_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PERI_BACKUP_DONE_INT_CLR_W<'a> {
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
#[doc = "Field `PERI_BACKUP_ERR_INT_CLR` writer - reg_peri_backup_err_int_clr"]
pub struct PERI_BACKUP_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PERI_BACKUP_ERR_INT_CLR_W<'a> {
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
impl W {
    #[doc = "Bit 0 - reg_peri_backup_done_int_clr"]
    #[inline(always)]
    pub fn peri_backup_done_int_clr(&mut self) -> PERI_BACKUP_DONE_INT_CLR_W {
        PERI_BACKUP_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1 - reg_peri_backup_err_int_clr"]
    #[inline(always)]
    pub fn peri_backup_err_int_clr(&mut self) -> PERI_BACKUP_ERR_INT_CLR_W {
        PERI_BACKUP_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB_CTRL_PERI_BACKUP_INT_CLR_REG\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peri_backup_int_clr](index.html) module"]
pub struct PERI_BACKUP_INT_CLR_SPEC;
impl crate::RegisterSpec for PERI_BACKUP_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [peri_backup_int_clr::W](W) writer structure"]
impl crate::Writable for PERI_BACKUP_INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERI_BACKUP_INT_CLR to value 0"]
impl crate::Resettable for PERI_BACKUP_INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
