///Register `TXBC` reader
pub struct R(crate::R<TXBC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBC_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TXBC` writer
pub struct W(crate::W<TXBC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXBC_SPEC>;
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
impl From<crate::W<TXBC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXBC_SPEC>) -> Self {
        W(writer)
    }
}

///Field `TFQM` reader - Tx FIFO/Queue Mode
pub struct TFQM_R(crate::FieldReader<bool, bool>);
impl TFQM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFQM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFQM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TFQM` writer - Tx FIFO/Queue Mode
pub struct TFQM_W<'a> {
    w: &'a mut W,
}
impl<'a> TFQM_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    ///Bit 24 - Tx FIFO/Queue Mode
    #[inline(always)]
    pub fn tfqm(&self) -> TFQM_R {
        TFQM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    ///Bit 24 - Tx FIFO/Queue Mode
    #[inline(always)]
    pub fn tfqm(&mut self) -> TFQM_W {
        TFQM_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN Tx Buffer Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [txbc](index.html) module
pub struct TXBC_SPEC;
impl crate::RegisterSpec for TXBC_SPEC {
    type Ux = u32;
}
///`read()` method returns [txbc::R](R) reader structure
impl crate::Readable for TXBC_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [txbc::W](W) writer structure
impl crate::Writable for TXBC_SPEC {
    type Writer = W;
}
///`reset()` method sets TXBC to value 0
impl crate::Resettable for TXBC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
