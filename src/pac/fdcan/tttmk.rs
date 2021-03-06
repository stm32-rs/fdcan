///Register `TTTMK` reader
pub struct R(crate::R<TTTMK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTTMK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTTMK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTTMK_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TTTMK` writer
pub struct W(crate::W<TTTMK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTTMK_SPEC>;
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
impl From<crate::W<TTTMK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTTMK_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TM` reader - Time Mark
pub struct TM_R(crate::FieldReader<u16, u16>);
impl TM_R {
    pub(crate) fn new(bits: u16) -> Self {
        TM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TM` writer - Time Mark
pub struct TM_W<'a> {
    w: &'a mut W,
}
impl<'a> TM_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
///Field `TICC` reader - Time Mark Cycle Code
pub struct TICC_R(crate::FieldReader<u8, u8>);
impl TICC_R {
    pub(crate) fn new(bits: u8) -> Self {
        TICC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TICC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TICC` writer - Time Mark Cycle Code
pub struct TICC_W<'a> {
    w: &'a mut W,
}
impl<'a> TICC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
///Field `LCKM` reader - TT Time Mark Register Locked
pub struct LCKM_R(crate::FieldReader<bool, bool>);
impl LCKM_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LCKM` writer - TT Time Mark Register Locked
pub struct LCKM_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Time Mark
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:22 - Time Mark Cycle Code
    #[inline(always)]
    pub fn ticc(&self) -> TICC_R {
        TICC_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bit 31 - TT Time Mark Register Locked
    #[inline(always)]
    pub fn lckm(&self) -> LCKM_R {
        LCKM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:15 - Time Mark
    #[inline(always)]
    pub fn tm(&mut self) -> TM_W {
        TM_W { w: self }
    }
    ///Bits 16:22 - Time Mark Cycle Code
    #[inline(always)]
    pub fn ticc(&mut self) -> TICC_W {
        TICC_W { w: self }
    }
    ///Bit 31 - TT Time Mark Register Locked
    #[inline(always)]
    pub fn lckm(&mut self) -> LCKM_W {
        LCKM_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN TT Time Mark Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tttmk](index.html) module
pub struct TTTMK_SPEC;
impl crate::RegisterSpec for TTTMK_SPEC {
    type Ux = u32;
}
///`read()` method returns [tttmk::R](R) reader structure
impl crate::Readable for TTTMK_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tttmk::W](W) writer structure
impl crate::Writable for TTTMK_SPEC {
    type Writer = W;
}
///`reset()` method sets TTTMK to value 0
impl crate::Resettable for TTTMK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
