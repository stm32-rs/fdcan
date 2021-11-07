///Register `TTRMC` reader
pub struct R(crate::R<TTRMC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTRMC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTRMC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTRMC_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TTRMC` writer
pub struct W(crate::W<TTRMC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTRMC_SPEC>;
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
impl From<crate::W<TTRMC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTRMC_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RID` reader - Reference Identifier.
pub struct RID_R(crate::FieldReader<u32, u32>);
impl RID_R {
    pub(crate) fn new(bits: u32) -> Self {
        RID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RID` writer - Reference Identifier.
pub struct RID_W<'a> {
    w: &'a mut W,
}
impl<'a> RID_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | (value as u32 & 0x1fff_ffff);
        self.w
    }
}
///Field `XTD` reader - Extended Identifier
pub struct XTD_R(crate::FieldReader<bool, bool>);
impl XTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        XTD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `XTD` writer - Extended Identifier
pub struct XTD_W<'a> {
    w: &'a mut W,
}
impl<'a> XTD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
///Field `RMPS` reader - Reference Message Payload Select
pub struct RMPS_R(crate::FieldReader<bool, bool>);
impl RMPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RMPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMPS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RMPS` writer - Reference Message Payload Select
pub struct RMPS_W<'a> {
    w: &'a mut W,
}
impl<'a> RMPS_W<'a> {
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
    ///Bits 0:28 - Reference Identifier.
    #[inline(always)]
    pub fn rid(&self) -> RID_R {
        RID_R::new((self.bits & 0x1fff_ffff) as u32)
    }
    ///Bit 30 - Extended Identifier
    #[inline(always)]
    pub fn xtd(&self) -> XTD_R {
        XTD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - Reference Message Payload Select
    #[inline(always)]
    pub fn rmps(&self) -> RMPS_R {
        RMPS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:28 - Reference Identifier.
    #[inline(always)]
    pub fn rid(&mut self) -> RID_W {
        RID_W { w: self }
    }
    ///Bit 30 - Extended Identifier
    #[inline(always)]
    pub fn xtd(&mut self) -> XTD_W {
        XTD_W { w: self }
    }
    ///Bit 31 - Reference Message Payload Select
    #[inline(always)]
    pub fn rmps(&mut self) -> RMPS_W {
        RMPS_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN TT Reference Message Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ttrmc](index.html) module
pub struct TTRMC_SPEC;
impl crate::RegisterSpec for TTRMC_SPEC {
    type Ux = u32;
}
///`read()` method returns [ttrmc::R](R) reader structure
impl crate::Readable for TTRMC_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ttrmc::W](W) writer structure
impl crate::Writable for TTRMC_SPEC {
    type Writer = W;
}
///`reset()` method sets TTRMC to value 0
impl crate::Resettable for TTRMC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
