///Register `IR` reader
pub struct R(crate::R<IR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IR` writer
pub struct W(crate::W<IR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IR_SPEC>;
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
impl From<crate::W<IR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RF0N` reader - Rx FIFO 0 New Message
pub struct RF0N_R(crate::FieldReader<bool, bool>);
impl RF0N_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF0N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF0N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RF0N` writer - Rx FIFO 0 New Message
pub struct RF0N_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0N_W<'a> {
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
///Field `RF0W` reader - Rx FIFO 0 Full
pub struct RF0W_R(crate::FieldReader<bool, bool>);
impl RF0W_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF0W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF0W_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RF0W` writer - Rx FIFO 0 Full
pub struct RF0W_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0W_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
///Field `RF0F` reader - Rx FIFO 0 Full
pub struct RF0F_R(crate::FieldReader<bool, bool>);
impl RF0F_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF0F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF0F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RF0F` writer - Rx FIFO 0 Full
pub struct RF0F_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0F_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
///Field `RF0L` reader - Rx FIFO 0 Message Lost
pub struct RF0L_R(crate::FieldReader<bool, bool>);
impl RF0L_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF0L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF0L_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RF0L` writer - Rx FIFO 0 Message Lost
pub struct RF0L_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0L_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
///Field `RF1N` reader - Rx FIFO 1 New Message
pub struct RF1N_R(crate::FieldReader<bool, bool>);
impl RF1N_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF1N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF1N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RF1N` writer - Rx FIFO 1 New Message
pub struct RF1N_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1N_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
///Field `RF1W` reader - Rx FIFO 1 Watermark Reached
pub struct RF1W_R(crate::FieldReader<bool, bool>);
impl RF1W_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF1W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF1W_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RF1W` writer - Rx FIFO 1 Watermark Reached
pub struct RF1W_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1W_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
///Field `RF1F` reader - Rx FIFO 1 Watermark Reached
pub struct RF1F_R(crate::FieldReader<bool, bool>);
impl RF1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RF1F` writer - Rx FIFO 1 Watermark Reached
pub struct RF1F_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1F_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
///Field `HPM` reader - High Priority Message
pub struct HPM_R(crate::FieldReader<bool, bool>);
impl HPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        HPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `HPM` writer - High Priority Message
pub struct HPM_W<'a> {
    w: &'a mut W,
}
impl<'a> HPM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
///Field `TC` reader - Transmission Completed
pub struct TC_R(crate::FieldReader<bool, bool>);
impl TC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TC` writer - Transmission Completed
pub struct TC_W<'a> {
    w: &'a mut W,
}
impl<'a> TC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
///Field `TCF` reader - Transmission Cancellation Finished
pub struct TCF_R(crate::FieldReader<bool, bool>);
impl TCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TCF` writer - Transmission Cancellation Finished
pub struct TCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TCF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
///Field `TEF` reader - Tx FIFO Empty
pub struct TEF_R(crate::FieldReader<bool, bool>);
impl TEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TEF` writer - Tx FIFO Empty
pub struct TEF_W<'a> {
    w: &'a mut W,
}
impl<'a> TEF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
///Field `TEFN` reader - Tx Event FIFO New Entry
pub struct TEFN_R(crate::FieldReader<bool, bool>);
impl TEFN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEFN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEFN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TEFN` writer - Tx Event FIFO New Entry
pub struct TEFN_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
///Field `TEFW` reader - Tx Event FIFO Watermark Reached
pub struct TEFW_R(crate::FieldReader<bool, bool>);
impl TEFW_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEFW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEFW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TEFW` writer - Tx Event FIFO Watermark Reached
pub struct TEFW_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
///Field `TEFF` reader - Tx Event FIFO Full
pub struct TEFF_R(crate::FieldReader<bool, bool>);
impl TEFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TEFF` writer - Tx Event FIFO Full
pub struct TEFF_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
///Field `TEFL` reader - Tx Event FIFO Element Lost
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
///Field `TEFL` writer - Tx Event FIFO Element Lost
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
///Field `TSW` reader - Timestamp Wraparound
pub struct TSW_R(crate::FieldReader<bool, bool>);
impl TSW_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TSW` writer - Timestamp Wraparound
pub struct TSW_W<'a> {
    w: &'a mut W,
}
impl<'a> TSW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
///Field `MRAF` reader - Message RAM Access Failure
pub struct MRAF_R(crate::FieldReader<bool, bool>);
impl MRAF_R {
    pub(crate) fn new(bits: bool) -> Self {
        MRAF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MRAF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MRAF` writer - Message RAM Access Failure
pub struct MRAF_W<'a> {
    w: &'a mut W,
}
impl<'a> MRAF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
///Field `TOO` reader - Timeout Occurred
pub struct TOO_R(crate::FieldReader<bool, bool>);
impl TOO_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TOO` writer - Timeout Occurred
pub struct TOO_W<'a> {
    w: &'a mut W,
}
impl<'a> TOO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
///Field `DRX` reader - Message stored to Dedicated Rx Buffer
pub struct DRX_R(crate::FieldReader<bool, bool>);
impl DRX_R {
    pub(crate) fn new(bits: bool) -> Self {
        DRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DRX` writer - Message stored to Dedicated Rx Buffer
pub struct DRX_W<'a> {
    w: &'a mut W,
}
impl<'a> DRX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
///Field `ELO` reader - Error Logging Overflow
pub struct ELO_R(crate::FieldReader<bool, bool>);
impl ELO_R {
    pub(crate) fn new(bits: bool) -> Self {
        ELO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ELO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ELO` writer - Error Logging Overflow
pub struct ELO_W<'a> {
    w: &'a mut W,
}
impl<'a> ELO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
///Field `EP` reader - Error Passive
pub struct EP_R(crate::FieldReader<bool, bool>);
impl EP_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EP` writer - Error Passive
pub struct EP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
///Field `EW` reader - Warning Status
pub struct EW_R(crate::FieldReader<bool, bool>);
impl EW_R {
    pub(crate) fn new(bits: bool) -> Self {
        EW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EW` writer - Warning Status
pub struct EW_W<'a> {
    w: &'a mut W,
}
impl<'a> EW_W<'a> {
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
///Field `BO` reader - Bus_Off Status
pub struct BO_R(crate::FieldReader<bool, bool>);
impl BO_R {
    pub(crate) fn new(bits: bool) -> Self {
        BO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BO` writer - Bus_Off Status
pub struct BO_W<'a> {
    w: &'a mut W,
}
impl<'a> BO_W<'a> {
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
///Field `WDI` reader - Watchdog Interrupt
pub struct WDI_R(crate::FieldReader<bool, bool>);
impl WDI_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WDI` writer - Watchdog Interrupt
pub struct WDI_W<'a> {
    w: &'a mut W,
}
impl<'a> WDI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
///Field `PEA` reader - Protocol Error in Arbitration Phase (Nominal Bit Time is used)
pub struct PEA_R(crate::FieldReader<bool, bool>);
impl PEA_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PEA` writer - Protocol Error in Arbitration Phase (Nominal Bit Time is used)
pub struct PEA_W<'a> {
    w: &'a mut W,
}
impl<'a> PEA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
///Field `PED` reader - Protocol Error in Data Phase (Data Bit Time is used)
pub struct PED_R(crate::FieldReader<bool, bool>);
impl PED_R {
    pub(crate) fn new(bits: bool) -> Self {
        PED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PED` writer - Protocol Error in Data Phase (Data Bit Time is used)
pub struct PED_W<'a> {
    w: &'a mut W,
}
impl<'a> PED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
///Field `ARA` reader - Access to Reserved Address
pub struct ARA_R(crate::FieldReader<bool, bool>);
impl ARA_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ARA` writer - Access to Reserved Address
pub struct ARA_W<'a> {
    w: &'a mut W,
}
impl<'a> ARA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
impl R {
    ///Bit 0 - Rx FIFO 0 New Message
    #[inline(always)]
    pub fn rf0n(&self) -> RF0N_R {
        RF0N_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Rx FIFO 0 Full
    #[inline(always)]
    pub fn rf0w(&self) -> RF0W_R {
        RF0W_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Rx FIFO 0 Full
    #[inline(always)]
    pub fn rf0f(&self) -> RF0F_R {
        RF0F_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Rx FIFO 0 Message Lost
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Rx FIFO 1 New Message
    #[inline(always)]
    pub fn rf1n(&self) -> RF1N_R {
        RF1N_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Rx FIFO 1 Watermark Reached
    #[inline(always)]
    pub fn rf1w(&self) -> RF1W_R {
        RF1W_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Rx FIFO 1 Watermark Reached
    #[inline(always)]
    pub fn rf1f(&self) -> RF1F_R {
        RF1F_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Rx FIFO 1 Message Lost
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - High Priority Message
    #[inline(always)]
    pub fn hpm(&self) -> HPM_R {
        HPM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Transmission Completed
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - Transmission Cancellation Finished
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - Tx FIFO Empty
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Tx Event FIFO New Entry
    #[inline(always)]
    pub fn tefn(&self) -> TEFN_R {
        TEFN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - Tx Event FIFO Watermark Reached
    #[inline(always)]
    pub fn tefw(&self) -> TEFW_R {
        TEFW_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Tx Event FIFO Full
    #[inline(always)]
    pub fn teff(&self) -> TEFF_R {
        TEFF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Tx Event FIFO Element Lost
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - Timestamp Wraparound
    #[inline(always)]
    pub fn tsw(&self) -> TSW_R {
        TSW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - Message RAM Access Failure
    #[inline(always)]
    pub fn mraf(&self) -> MRAF_R {
        MRAF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - Timeout Occurred
    #[inline(always)]
    pub fn too(&self) -> TOO_R {
        TOO_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - Message stored to Dedicated Rx Buffer
    #[inline(always)]
    pub fn drx(&self) -> DRX_R {
        DRX_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 22 - Error Logging Overflow
    #[inline(always)]
    pub fn elo(&self) -> ELO_R {
        ELO_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 23 - Error Passive
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 24 - Warning Status
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 25 - Bus_Off Status
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 26 - Watchdog Interrupt
    #[inline(always)]
    pub fn wdi(&self) -> WDI_R {
        WDI_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 27 - Protocol Error in Arbitration Phase (Nominal Bit Time is used)
    #[inline(always)]
    pub fn pea(&self) -> PEA_R {
        PEA_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bit 28 - Protocol Error in Data Phase (Data Bit Time is used)
    #[inline(always)]
    pub fn ped(&self) -> PED_R {
        PED_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bit 29 - Access to Reserved Address
    #[inline(always)]
    pub fn ara(&self) -> ARA_R {
        ARA_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Rx FIFO 0 New Message
    #[inline(always)]
    pub fn rf0n(&mut self) -> RF0N_W {
        RF0N_W { w: self }
    }
    ///Bit 1 - Rx FIFO 0 Full
    #[inline(always)]
    pub fn rf0w(&mut self) -> RF0W_W {
        RF0W_W { w: self }
    }
    ///Bit 2 - Rx FIFO 0 Full
    #[inline(always)]
    pub fn rf0f(&mut self) -> RF0F_W {
        RF0F_W { w: self }
    }
    ///Bit 3 - Rx FIFO 0 Message Lost
    #[inline(always)]
    pub fn rf0l(&mut self) -> RF0L_W {
        RF0L_W { w: self }
    }
    ///Bit 4 - Rx FIFO 1 New Message
    #[inline(always)]
    pub fn rf1n(&mut self) -> RF1N_W {
        RF1N_W { w: self }
    }
    ///Bit 5 - Rx FIFO 1 Watermark Reached
    #[inline(always)]
    pub fn rf1w(&mut self) -> RF1W_W {
        RF1W_W { w: self }
    }
    ///Bit 6 - Rx FIFO 1 Watermark Reached
    #[inline(always)]
    pub fn rf1f(&mut self) -> RF1F_W {
        RF1F_W { w: self }
    }
    ///Bit 7 - Rx FIFO 1 Message Lost
    #[inline(always)]
    pub fn rf1l(&mut self) -> RF1L_W {
        RF1L_W { w: self }
    }
    ///Bit 8 - High Priority Message
    #[inline(always)]
    pub fn hpm(&mut self) -> HPM_W {
        HPM_W { w: self }
    }
    ///Bit 9 - Transmission Completed
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W {
        TC_W { w: self }
    }
    ///Bit 10 - Transmission Cancellation Finished
    #[inline(always)]
    pub fn tcf(&mut self) -> TCF_W {
        TCF_W { w: self }
    }
    ///Bit 11 - Tx FIFO Empty
    #[inline(always)]
    pub fn tef(&mut self) -> TEF_W {
        TEF_W { w: self }
    }
    ///Bit 12 - Tx Event FIFO New Entry
    #[inline(always)]
    pub fn tefn(&mut self) -> TEFN_W {
        TEFN_W { w: self }
    }
    ///Bit 13 - Tx Event FIFO Watermark Reached
    #[inline(always)]
    pub fn tefw(&mut self) -> TEFW_W {
        TEFW_W { w: self }
    }
    ///Bit 14 - Tx Event FIFO Full
    #[inline(always)]
    pub fn teff(&mut self) -> TEFF_W {
        TEFF_W { w: self }
    }
    ///Bit 15 - Tx Event FIFO Element Lost
    #[inline(always)]
    pub fn tefl(&mut self) -> TEFL_W {
        TEFL_W { w: self }
    }
    ///Bit 16 - Timestamp Wraparound
    #[inline(always)]
    pub fn tsw(&mut self) -> TSW_W {
        TSW_W { w: self }
    }
    ///Bit 17 - Message RAM Access Failure
    #[inline(always)]
    pub fn mraf(&mut self) -> MRAF_W {
        MRAF_W { w: self }
    }
    ///Bit 18 - Timeout Occurred
    #[inline(always)]
    pub fn too(&mut self) -> TOO_W {
        TOO_W { w: self }
    }
    ///Bit 19 - Message stored to Dedicated Rx Buffer
    #[inline(always)]
    pub fn drx(&mut self) -> DRX_W {
        DRX_W { w: self }
    }
    ///Bit 22 - Error Logging Overflow
    #[inline(always)]
    pub fn elo(&mut self) -> ELO_W {
        ELO_W { w: self }
    }
    ///Bit 23 - Error Passive
    #[inline(always)]
    pub fn ep(&mut self) -> EP_W {
        EP_W { w: self }
    }
    ///Bit 24 - Warning Status
    #[inline(always)]
    pub fn ew(&mut self) -> EW_W {
        EW_W { w: self }
    }
    ///Bit 25 - Bus_Off Status
    #[inline(always)]
    pub fn bo(&mut self) -> BO_W {
        BO_W { w: self }
    }
    ///Bit 26 - Watchdog Interrupt
    #[inline(always)]
    pub fn wdi(&mut self) -> WDI_W {
        WDI_W { w: self }
    }
    ///Bit 27 - Protocol Error in Arbitration Phase (Nominal Bit Time is used)
    #[inline(always)]
    pub fn pea(&mut self) -> PEA_W {
        PEA_W { w: self }
    }
    ///Bit 28 - Protocol Error in Data Phase (Data Bit Time is used)
    #[inline(always)]
    pub fn ped(&mut self) -> PED_W {
        PED_W { w: self }
    }
    ///Bit 29 - Access to Reserved Address
    #[inline(always)]
    pub fn ara(&mut self) -> ARA_W {
        ARA_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN Interrupt Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ir](index.html) module
pub struct IR_SPEC;
impl crate::RegisterSpec for IR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ir::R](R) reader structure
impl crate::Readable for IR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ir::W](W) writer structure
impl crate::Writable for IR_SPEC {
    type Writer = W;
}
///`reset()` method sets IR to value 0
impl crate::Resettable for IR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
