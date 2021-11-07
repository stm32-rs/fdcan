///Register `TXEFS` reader
pub struct R(crate::R<TXEFS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXEFS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXEFS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXEFS_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TXEFS` writer
pub struct W(crate::W<TXEFS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXEFS_SPEC>;
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
impl From<crate::W<TXEFS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXEFS_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EFFL` reader - Event FIFO Fill Level
pub struct EFFL_R(crate::FieldReader<u8, u8>);
impl EFFL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EFFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFFL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EFFL` writer - Event FIFO Fill Level
pub struct EFFL_W<'a> {
    w: &'a mut W,
}
impl<'a> EFFL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
///Field `EFGI` reader - Event FIFO Get Index.
pub struct EFGI_R(crate::FieldReader<u8, u8>);
impl EFGI_R {
    pub(crate) fn new(bits: u8) -> Self {
        EFGI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFGI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EFGI` writer - Event FIFO Get Index.
pub struct EFGI_W<'a> {
    w: &'a mut W,
}
impl<'a> EFGI_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
///Field `EFF` reader - Event FIFO Full.
pub struct EFF_R(crate::FieldReader<bool, bool>);
impl EFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        EFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EFF` writer - Event FIFO Full.
pub struct EFF_W<'a> {
    w: &'a mut W,
}
impl<'a> EFF_W<'a> {
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
///Field `TEFL` reader - Tx Event FIFO Element Lost.
pub struct TEFL_R(crate::FieldReader<bool, bool>);
impl TEFL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEFL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TEFL` writer - Tx Event FIFO Element Lost.
pub struct TEFL_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
///Field `EFPI` reader - Event FIFO put index
pub struct EFPI_R(crate::FieldReader<u8, u8>);
impl EFPI_R {
    pub(crate) fn new(bits: u8) -> Self {
        EFPI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFPI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EFPI` writer - Event FIFO put index
pub struct EFPI_W<'a> {
    w: &'a mut W,
}
impl<'a> EFPI_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:5 - Event FIFO Fill Level
    #[inline(always)]
    pub fn effl(&self) -> EFFL_R {
        EFFL_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:12 - Event FIFO Get Index.
    #[inline(always)]
    pub fn efgi(&self) -> EFGI_R {
        EFGI_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bit 24 - Event FIFO Full.
    #[inline(always)]
    pub fn eff(&self) -> EFF_R {
        EFF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 25 - Tx Event FIFO Element Lost.
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bits 16:20 - Event FIFO put index
    #[inline(always)]
    pub fn efpi(&self) -> EFPI_R {
        EFPI_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:5 - Event FIFO Fill Level
    #[inline(always)]
    pub fn effl(&mut self) -> EFFL_W {
        EFFL_W { w: self }
    }
    ///Bits 8:12 - Event FIFO Get Index.
    #[inline(always)]
    pub fn efgi(&mut self) -> EFGI_W {
        EFGI_W { w: self }
    }
    ///Bit 24 - Event FIFO Full.
    #[inline(always)]
    pub fn eff(&mut self) -> EFF_W {
        EFF_W { w: self }
    }
    ///Bit 25 - Tx Event FIFO Element Lost.
    #[inline(always)]
    pub fn tefl(&mut self) -> TEFL_W {
        TEFL_W { w: self }
    }
    ///Bits 16:20 - Event FIFO put index
    #[inline(always)]
    pub fn efpi(&mut self) -> EFPI_W {
        EFPI_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN Tx Event FIFO Status Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [txefs](index.html) module
pub struct TXEFS_SPEC;
impl crate::RegisterSpec for TXEFS_SPEC {
    type Ux = u32;
}
///`read()` method returns [txefs::R](R) reader structure
impl crate::Readable for TXEFS_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [txefs::W](W) writer structure
impl crate::Writable for TXEFS_SPEC {
    type Writer = W;
}
///`reset()` method sets TXEFS to value 0
impl crate::Resettable for TXEFS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
