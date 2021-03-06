///Register `TURCF` reader
pub struct R(crate::R<TURCF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TURCF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TURCF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TURCF_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TURCF` writer
pub struct W(crate::W<TURCF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TURCF_SPEC>;
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
impl From<crate::W<TURCF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TURCF_SPEC>) -> Self {
        W(writer)
    }
}
///Field `NCL` reader - Numerator Configuration Low.
pub struct NCL_R(crate::FieldReader<u16, u16>);
impl NCL_R {
    pub(crate) fn new(bits: u16) -> Self {
        NCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NCL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `NCL` writer - Numerator Configuration Low.
pub struct NCL_W<'a> {
    w: &'a mut W,
}
impl<'a> NCL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
///Field `DC` reader - Denominator Configuration.
pub struct DC_R(crate::FieldReader<u16, u16>);
impl DC_R {
    pub(crate) fn new(bits: u16) -> Self {
        DC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DC` writer - Denominator Configuration.
pub struct DC_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 16)) | ((value as u32 & 0x3fff) << 16);
        self.w
    }
}
///Field `ELT` reader - Enable Local Time
pub struct ELT_R(crate::FieldReader<bool, bool>);
impl ELT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ELT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ELT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ELT` writer - Enable Local Time
pub struct ELT_W<'a> {
    w: &'a mut W,
}
impl<'a> ELT_W<'a> {
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
    ///Bits 0:15 - Numerator Configuration Low.
    #[inline(always)]
    pub fn ncl(&self) -> NCL_R {
        NCL_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:29 - Denominator Configuration.
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    ///Bit 31 - Enable Local Time
    #[inline(always)]
    pub fn elt(&self) -> ELT_R {
        ELT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:15 - Numerator Configuration Low.
    #[inline(always)]
    pub fn ncl(&mut self) -> NCL_W {
        NCL_W { w: self }
    }
    ///Bits 16:29 - Denominator Configuration.
    #[inline(always)]
    pub fn dc(&mut self) -> DC_W {
        DC_W { w: self }
    }
    ///Bit 31 - Enable Local Time
    #[inline(always)]
    pub fn elt(&mut self) -> ELT_W {
        ELT_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN TUR Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [turcf](index.html) module
pub struct TURCF_SPEC;
impl crate::RegisterSpec for TURCF_SPEC {
    type Ux = u32;
}
///`read()` method returns [turcf::R](R) reader structure
impl crate::Readable for TURCF_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [turcf::W](W) writer structure
impl crate::Writable for TURCF_SPEC {
    type Writer = W;
}
///`reset()` method sets TURCF to value 0
impl crate::Resettable for TURCF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
