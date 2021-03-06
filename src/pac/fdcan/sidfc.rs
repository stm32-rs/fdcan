///Register `SIDFC` reader
pub struct R(crate::R<SIDFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIDFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIDFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIDFC_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SIDFC` writer
pub struct W(crate::W<SIDFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIDFC_SPEC>;
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
impl From<crate::W<SIDFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIDFC_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FLSSA` reader - Filter List Standard Start Address
pub struct FLSSA_R(crate::FieldReader<u16, u16>);
impl FLSSA_R {
    pub(crate) fn new(bits: u16) -> Self {
        FLSSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLSSA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FLSSA` writer - Filter List Standard Start Address
pub struct FLSSA_W<'a> {
    w: &'a mut W,
}
impl<'a> FLSSA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | ((value as u32 & 0x3fff) << 2);
        self.w
    }
}
///Field `LSS` reader - List Size Standard
pub struct LSS_R(crate::FieldReader<u8, u8>);
impl LSS_R {
    pub(crate) fn new(bits: u8) -> Self {
        LSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LSS` writer - List Size Standard
pub struct LSS_W<'a> {
    w: &'a mut W,
}
impl<'a> LSS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    ///Bits 2:15 - Filter List Standard Start Address
    #[inline(always)]
    pub fn flssa(&self) -> FLSSA_R {
        FLSSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    ///Bits 16:23 - List Size Standard
    #[inline(always)]
    pub fn lss(&self) -> LSS_R {
        LSS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bits 2:15 - Filter List Standard Start Address
    #[inline(always)]
    pub fn flssa(&mut self) -> FLSSA_W {
        FLSSA_W { w: self }
    }
    ///Bits 16:23 - List Size Standard
    #[inline(always)]
    pub fn lss(&mut self) -> LSS_W {
        LSS_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN Standard ID Filter Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sidfc](index.html) module
pub struct SIDFC_SPEC;
impl crate::RegisterSpec for SIDFC_SPEC {
    type Ux = u32;
}
///`read()` method returns [sidfc::R](R) reader structure
impl crate::Readable for SIDFC_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sidfc::W](W) writer structure
impl crate::Writable for SIDFC_SPEC {
    type Writer = W;
}
///`reset()` method sets SIDFC to value 0
impl crate::Resettable for SIDFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
