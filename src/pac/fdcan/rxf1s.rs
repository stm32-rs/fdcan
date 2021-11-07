///Register `RXF1S` reader
pub struct R(crate::R<RXF1S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXF1S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXF1S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXF1S_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RXF1S` writer
pub struct W(crate::W<RXF1S_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXF1S_SPEC>;
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
impl From<crate::W<RXF1S_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXF1S_SPEC>) -> Self {
        W(writer)
    }
}
///Field `F1FL` reader - Rx FIFO 1 Fill Level
pub struct F1FL_R(crate::FieldReader<u8, u8>);
impl F1FL_R {
    pub(crate) fn new(bits: u8) -> Self {
        F1FL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1FL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `F1FL` writer - Rx FIFO 1 Fill Level
pub struct F1FL_W<'a> {
    w: &'a mut W,
}
impl<'a> F1FL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
///Field `F1GI` reader - Rx FIFO 1 Get Index
pub struct F1GI_R(crate::FieldReader<u8, u8>);
impl F1GI_R {
    pub(crate) fn new(bits: u8) -> Self {
        F1GI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1GI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `F1GI` writer - Rx FIFO 1 Get Index
pub struct F1GI_W<'a> {
    w: &'a mut W,
}
impl<'a> F1GI_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
///Field `F1PI` reader - Rx FIFO 1 Put Index
pub struct F1PI_R(crate::FieldReader<u8, u8>);
impl F1PI_R {
    pub(crate) fn new(bits: u8) -> Self {
        F1PI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1PI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `F1PI` writer - Rx FIFO 1 Put Index
pub struct F1PI_W<'a> {
    w: &'a mut W,
}
impl<'a> F1PI_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
///Field `F1F` reader - Rx FIFO 1 Full
pub struct F1F_R(crate::FieldReader<bool, bool>);
impl F1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        F1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `F1F` writer - Rx FIFO 1 Full
pub struct F1F_W<'a> {
    w: &'a mut W,
}
impl<'a> F1F_W<'a> {
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
///Field `RF1L` reader - Rx FIFO 1 Message Lost
pub struct RF1L_R(crate::FieldReader<bool, bool>);
impl RF1L_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF1L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF1L_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RF1L` writer - Rx FIFO 1 Message Lost
pub struct RF1L_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1L_W<'a> {
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
///Field `DMS` reader - Debug Message Status
pub struct DMS_R(crate::FieldReader<u8, u8>);
impl DMS_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DMS` writer - Debug Message Status
pub struct DMS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    ///Bits 0:6 - Rx FIFO 1 Fill Level
    #[inline(always)]
    pub fn f1fl(&self) -> F1FL_R {
        F1FL_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:14 - Rx FIFO 1 Get Index
    #[inline(always)]
    pub fn f1gi(&self) -> F1GI_R {
        F1GI_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bits 16:22 - Rx FIFO 1 Put Index
    #[inline(always)]
    pub fn f1pi(&self) -> F1PI_R {
        F1PI_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bit 24 - Rx FIFO 1 Full
    #[inline(always)]
    pub fn f1f(&self) -> F1F_R {
        F1F_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 25 - Rx FIFO 1 Message Lost
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bits 30:31 - Debug Message Status
    #[inline(always)]
    pub fn dms(&self) -> DMS_R {
        DMS_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    ///Bits 0:6 - Rx FIFO 1 Fill Level
    #[inline(always)]
    pub fn f1fl(&mut self) -> F1FL_W {
        F1FL_W { w: self }
    }
    ///Bits 8:14 - Rx FIFO 1 Get Index
    #[inline(always)]
    pub fn f1gi(&mut self) -> F1GI_W {
        F1GI_W { w: self }
    }
    ///Bits 16:22 - Rx FIFO 1 Put Index
    #[inline(always)]
    pub fn f1pi(&mut self) -> F1PI_W {
        F1PI_W { w: self }
    }
    ///Bit 24 - Rx FIFO 1 Full
    #[inline(always)]
    pub fn f1f(&mut self) -> F1F_W {
        F1F_W { w: self }
    }
    ///Bit 25 - Rx FIFO 1 Message Lost
    #[inline(always)]
    pub fn rf1l(&mut self) -> RF1L_W {
        RF1L_W { w: self }
    }
    ///Bits 30:31 - Debug Message Status
    #[inline(always)]
    pub fn dms(&mut self) -> DMS_W {
        DMS_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN Rx FIFO 1 Status Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rxf1s](index.html) module
pub struct RXF1S_SPEC;
impl crate::RegisterSpec for RXF1S_SPEC {
    type Ux = u32;
}
///`read()` method returns [rxf1s::R](R) reader structure
impl crate::Readable for RXF1S_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rxf1s::W](W) writer structure
impl crate::Writable for RXF1S_SPEC {
    type Writer = W;
}
///`reset()` method sets RXF1S to value 0
impl crate::Resettable for RXF1S_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
