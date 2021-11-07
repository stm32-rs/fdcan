///Register `TOCC` reader
pub struct R(crate::R<TOCC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOCC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOCC_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TOCC` writer
pub struct W(crate::W<TOCC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOCC_SPEC>;
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
impl From<crate::W<TOCC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOCC_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ETOC` reader - Enable Timeout Counter
pub struct ETOC_R(crate::FieldReader<bool, bool>);
impl ETOC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETOC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETOC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ETOC` writer - Enable Timeout Counter
pub struct ETOC_W<'a> {
    w: &'a mut W,
}
impl<'a> ETOC_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
///Field `TOS` reader - Timeout Select
pub struct TOS_R(crate::FieldReader<u8, u8>);
impl TOS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TOS` writer - Timeout Select
pub struct TOS_W<'a> {
    w: &'a mut W,
}
impl<'a> TOS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
///Field `TOP` reader - Timeout Period
pub struct TOP_R(crate::FieldReader<u16, u16>);
impl TOP_R {
    pub(crate) fn new(bits: u16) -> Self {
        TOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TOP` writer - Timeout Period
pub struct TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    ///Bit 0 - Enable Timeout Counter
    #[inline(always)]
    pub fn etoc(&self) -> ETOC_R {
        ETOC_R::new((self.bits & 0x01) != 0)
    }
    ///Bits 1:2 - Timeout Select
    #[inline(always)]
    pub fn tos(&self) -> TOS_R {
        TOS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    ///Bits 16:31 - Timeout Period
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bit 0 - Enable Timeout Counter
    #[inline(always)]
    pub fn etoc(&mut self) -> ETOC_W {
        ETOC_W { w: self }
    }
    ///Bits 1:2 - Timeout Select
    #[inline(always)]
    pub fn tos(&mut self) -> TOS_W {
        TOS_W { w: self }
    }
    ///Bits 16:31 - Timeout Period
    #[inline(always)]
    pub fn top(&mut self) -> TOP_W {
        TOP_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN Timeout Counter Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tocc](index.html) module
pub struct TOCC_SPEC;
impl crate::RegisterSpec for TOCC_SPEC {
    type Ux = u32;
}
///`read()` method returns [tocc::R](R) reader structure
impl crate::Readable for TOCC_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tocc::W](W) writer structure
impl crate::Writable for TOCC_SPEC {
    type Writer = W;
}
///`reset()` method sets TOCC to value 0
impl crate::Resettable for TOCC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
