///Register `ILS` reader
pub struct R(crate::R<ILS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ILS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ILS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ILS_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ILS` writer
pub struct W(crate::W<ILS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ILS_SPEC>;
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
impl From<crate::W<ILS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ILS_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RF0NL` reader - Rx FIFO 0 New Message Interrupt Line
pub struct RF0NL_R(crate::FieldReader<bool, bool>);
impl RF0NL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF0NL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF0NL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RF0NL` writer - Rx FIFO 0 New Message Interrupt Line
pub struct RF0NL_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0NL_W<'a> {
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
///Field `RF0WL` reader - Rx FIFO 0 Watermark Reached Interrupt Line
pub struct RF0WL_R(crate::FieldReader<bool, bool>);
impl RF0WL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF0WL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF0WL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RF0WL` writer - Rx FIFO 0 Watermark Reached Interrupt Line
pub struct RF0WL_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0WL_W<'a> {
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
///Field `RF0FL` reader - Rx FIFO 0 Full Interrupt Line
pub struct RF0FL_R(crate::FieldReader<bool, bool>);
impl RF0FL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF0FL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF0FL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RF0FL` writer - Rx FIFO 0 Full Interrupt Line
pub struct RF0FL_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0FL_W<'a> {
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
///Field `RF0LL` reader - Rx FIFO 0 Message Lost Interrupt Line
pub struct RF0LL_R(crate::FieldReader<bool, bool>);
impl RF0LL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF0LL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF0LL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RF0LL` writer - Rx FIFO 0 Message Lost Interrupt Line
pub struct RF0LL_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0LL_W<'a> {
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
///Field `RF1NL` reader - Rx FIFO 1 New Message Interrupt Line
pub struct RF1NL_R(crate::FieldReader<bool, bool>);
impl RF1NL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF1NL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF1NL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RF1NL` writer - Rx FIFO 1 New Message Interrupt Line
pub struct RF1NL_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1NL_W<'a> {
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
///Field `RF1WL` reader - Rx FIFO 1 Watermark Reached Interrupt Line
pub struct RF1WL_R(crate::FieldReader<bool, bool>);
impl RF1WL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF1WL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF1WL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RF1WL` writer - Rx FIFO 1 Watermark Reached Interrupt Line
pub struct RF1WL_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1WL_W<'a> {
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
///Field `RF1FL` reader - Rx FIFO 1 Full Interrupt Line
pub struct RF1FL_R(crate::FieldReader<bool, bool>);
impl RF1FL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF1FL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF1FL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RF1FL` writer - Rx FIFO 1 Full Interrupt Line
pub struct RF1FL_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1FL_W<'a> {
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
///Field `RF1LL` reader - Rx FIFO 1 Message Lost Interrupt Line
pub struct RF1LL_R(crate::FieldReader<bool, bool>);
impl RF1LL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF1LL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF1LL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RF1LL` writer - Rx FIFO 1 Message Lost Interrupt Line
pub struct RF1LL_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1LL_W<'a> {
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
///Field `HPML` reader - High Priority Message Interrupt Line
pub struct HPML_R(crate::FieldReader<bool, bool>);
impl HPML_R {
    pub(crate) fn new(bits: bool) -> Self {
        HPML_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPML_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `HPML` writer - High Priority Message Interrupt Line
pub struct HPML_W<'a> {
    w: &'a mut W,
}
impl<'a> HPML_W<'a> {
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
///Field `TCL` reader - Transmission Completed Interrupt Line
pub struct TCL_R(crate::FieldReader<bool, bool>);
impl TCL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TCL` writer - Transmission Completed Interrupt Line
pub struct TCL_W<'a> {
    w: &'a mut W,
}
impl<'a> TCL_W<'a> {
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
///Field `TCFL` reader - Transmission Cancellation Finished Interrupt Line
pub struct TCFL_R(crate::FieldReader<bool, bool>);
impl TCFL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCFL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TCFL` writer - Transmission Cancellation Finished Interrupt Line
pub struct TCFL_W<'a> {
    w: &'a mut W,
}
impl<'a> TCFL_W<'a> {
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
///Field `TEFL` reader - Tx FIFO Empty Interrupt Line
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
///Field `TEFL` writer - Tx FIFO Empty Interrupt Line
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
///Field `TEFNL` reader - Tx Event FIFO New Entry Interrupt Line
pub struct TEFNL_R(crate::FieldReader<bool, bool>);
impl TEFNL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEFNL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEFNL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TEFNL` writer - Tx Event FIFO New Entry Interrupt Line
pub struct TEFNL_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFNL_W<'a> {
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
///Field `TEFWL` reader - Tx Event FIFO Watermark Reached Interrupt Line
pub struct TEFWL_R(crate::FieldReader<bool, bool>);
impl TEFWL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEFWL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEFWL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TEFWL` writer - Tx Event FIFO Watermark Reached Interrupt Line
pub struct TEFWL_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFWL_W<'a> {
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
///Field `TEFFL` reader - Tx Event FIFO Full Interrupt Line
pub struct TEFFL_R(crate::FieldReader<bool, bool>);
impl TEFFL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEFFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEFFL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TEFFL` writer - Tx Event FIFO Full Interrupt Line
pub struct TEFFL_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFFL_W<'a> {
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
///Field `TEFLL` reader - Tx Event FIFO Element Lost Interrupt Line
pub struct TEFLL_R(crate::FieldReader<bool, bool>);
impl TEFLL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEFLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEFLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TEFLL` writer - Tx Event FIFO Element Lost Interrupt Line
pub struct TEFLL_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFLL_W<'a> {
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
///Field `TSWL` reader - Timestamp Wraparound Interrupt Line
pub struct TSWL_R(crate::FieldReader<bool, bool>);
impl TSWL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSWL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSWL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TSWL` writer - Timestamp Wraparound Interrupt Line
pub struct TSWL_W<'a> {
    w: &'a mut W,
}
impl<'a> TSWL_W<'a> {
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
///Field `MRAFL` reader - Message RAM Access Failure Interrupt Line
pub struct MRAFL_R(crate::FieldReader<bool, bool>);
impl MRAFL_R {
    pub(crate) fn new(bits: bool) -> Self {
        MRAFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MRAFL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MRAFL` writer - Message RAM Access Failure Interrupt Line
pub struct MRAFL_W<'a> {
    w: &'a mut W,
}
impl<'a> MRAFL_W<'a> {
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
///Field `TOOL` reader - Timeout Occurred Interrupt Line
pub struct TOOL_R(crate::FieldReader<bool, bool>);
impl TOOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TOOL` writer - Timeout Occurred Interrupt Line
pub struct TOOL_W<'a> {
    w: &'a mut W,
}
impl<'a> TOOL_W<'a> {
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
///Field `DRXL` reader - Message stored to Dedicated Rx Buffer Interrupt Line
pub struct DRXL_R(crate::FieldReader<bool, bool>);
impl DRXL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DRXL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRXL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DRXL` writer - Message stored to Dedicated Rx Buffer Interrupt Line
pub struct DRXL_W<'a> {
    w: &'a mut W,
}
impl<'a> DRXL_W<'a> {
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
///Field `BECL` reader - Bit Error Corrected Interrupt Line
pub struct BECL_R(crate::FieldReader<bool, bool>);
impl BECL_R {
    pub(crate) fn new(bits: bool) -> Self {
        BECL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BECL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BECL` writer - Bit Error Corrected Interrupt Line
pub struct BECL_W<'a> {
    w: &'a mut W,
}
impl<'a> BECL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
///Field `BEUL` reader - Bit Error Uncorrected Interrupt Line
pub struct BEUL_R(crate::FieldReader<bool, bool>);
impl BEUL_R {
    pub(crate) fn new(bits: bool) -> Self {
        BEUL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BEUL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BEUL` writer - Bit Error Uncorrected Interrupt Line
pub struct BEUL_W<'a> {
    w: &'a mut W,
}
impl<'a> BEUL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
///Field `ELOL` reader - Error Logging Overflow Interrupt Line
pub struct ELOL_R(crate::FieldReader<bool, bool>);
impl ELOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ELOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ELOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ELOL` writer - Error Logging Overflow Interrupt Line
pub struct ELOL_W<'a> {
    w: &'a mut W,
}
impl<'a> ELOL_W<'a> {
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
///Field `EPL` reader - Error Passive Interrupt Line
pub struct EPL_R(crate::FieldReader<bool, bool>);
impl EPL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EPL` writer - Error Passive Interrupt Line
pub struct EPL_W<'a> {
    w: &'a mut W,
}
impl<'a> EPL_W<'a> {
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
///Field `EWL` reader - Warning Status Interrupt Line
pub struct EWL_R(crate::FieldReader<bool, bool>);
impl EWL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EWL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EWL` writer - Warning Status Interrupt Line
pub struct EWL_W<'a> {
    w: &'a mut W,
}
impl<'a> EWL_W<'a> {
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
///Field `BOL` reader - Bus_Off Status
pub struct BOL_R(crate::FieldReader<bool, bool>);
impl BOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BOL` writer - Bus_Off Status
pub struct BOL_W<'a> {
    w: &'a mut W,
}
impl<'a> BOL_W<'a> {
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
///Field `WDIL` reader - Watchdog Interrupt Line
pub struct WDIL_R(crate::FieldReader<bool, bool>);
impl WDIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WDIL` writer - Watchdog Interrupt Line
pub struct WDIL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDIL_W<'a> {
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
///Field `PEAL` reader - Protocol Error in Arbitration Phase Line
pub struct PEAL_R(crate::FieldReader<bool, bool>);
impl PEAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PEAL` writer - Protocol Error in Arbitration Phase Line
pub struct PEAL_W<'a> {
    w: &'a mut W,
}
impl<'a> PEAL_W<'a> {
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
///Field `PEDL` reader - Protocol Error in Data Phase Line
pub struct PEDL_R(crate::FieldReader<bool, bool>);
impl PEDL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEDL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PEDL` writer - Protocol Error in Data Phase Line
pub struct PEDL_W<'a> {
    w: &'a mut W,
}
impl<'a> PEDL_W<'a> {
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
///Field `ARAL` reader - Access to Reserved Address Line
pub struct ARAL_R(crate::FieldReader<bool, bool>);
impl ARAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ARAL` writer - Access to Reserved Address Line
pub struct ARAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ARAL_W<'a> {
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
    ///Bit 0 - Rx FIFO 0 New Message Interrupt Line
    #[inline(always)]
    pub fn rf0nl(&self) -> RF0NL_R {
        RF0NL_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Rx FIFO 0 Watermark Reached Interrupt Line
    #[inline(always)]
    pub fn rf0wl(&self) -> RF0WL_R {
        RF0WL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Rx FIFO 0 Full Interrupt Line
    #[inline(always)]
    pub fn rf0fl(&self) -> RF0FL_R {
        RF0FL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Rx FIFO 0 Message Lost Interrupt Line
    #[inline(always)]
    pub fn rf0ll(&self) -> RF0LL_R {
        RF0LL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Rx FIFO 1 New Message Interrupt Line
    #[inline(always)]
    pub fn rf1nl(&self) -> RF1NL_R {
        RF1NL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Rx FIFO 1 Watermark Reached Interrupt Line
    #[inline(always)]
    pub fn rf1wl(&self) -> RF1WL_R {
        RF1WL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Rx FIFO 1 Full Interrupt Line
    #[inline(always)]
    pub fn rf1fl(&self) -> RF1FL_R {
        RF1FL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Rx FIFO 1 Message Lost Interrupt Line
    #[inline(always)]
    pub fn rf1ll(&self) -> RF1LL_R {
        RF1LL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - High Priority Message Interrupt Line
    #[inline(always)]
    pub fn hpml(&self) -> HPML_R {
        HPML_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Transmission Completed Interrupt Line
    #[inline(always)]
    pub fn tcl(&self) -> TCL_R {
        TCL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - Transmission Cancellation Finished Interrupt Line
    #[inline(always)]
    pub fn tcfl(&self) -> TCFL_R {
        TCFL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - Tx FIFO Empty Interrupt Line
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Tx Event FIFO New Entry Interrupt Line
    #[inline(always)]
    pub fn tefnl(&self) -> TEFNL_R {
        TEFNL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - Tx Event FIFO Watermark Reached Interrupt Line
    #[inline(always)]
    pub fn tefwl(&self) -> TEFWL_R {
        TEFWL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Tx Event FIFO Full Interrupt Line
    #[inline(always)]
    pub fn teffl(&self) -> TEFFL_R {
        TEFFL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Tx Event FIFO Element Lost Interrupt Line
    #[inline(always)]
    pub fn tefll(&self) -> TEFLL_R {
        TEFLL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - Timestamp Wraparound Interrupt Line
    #[inline(always)]
    pub fn tswl(&self) -> TSWL_R {
        TSWL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - Message RAM Access Failure Interrupt Line
    #[inline(always)]
    pub fn mrafl(&self) -> MRAFL_R {
        MRAFL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - Timeout Occurred Interrupt Line
    #[inline(always)]
    pub fn tool(&self) -> TOOL_R {
        TOOL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - Message stored to Dedicated Rx Buffer Interrupt Line
    #[inline(always)]
    pub fn drxl(&self) -> DRXL_R {
        DRXL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 20 - Bit Error Corrected Interrupt Line
    #[inline(always)]
    pub fn becl(&self) -> BECL_R {
        BECL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 21 - Bit Error Uncorrected Interrupt Line
    #[inline(always)]
    pub fn beul(&self) -> BEUL_R {
        BEUL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 22 - Error Logging Overflow Interrupt Line
    #[inline(always)]
    pub fn elol(&self) -> ELOL_R {
        ELOL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 23 - Error Passive Interrupt Line
    #[inline(always)]
    pub fn epl(&self) -> EPL_R {
        EPL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 24 - Warning Status Interrupt Line
    #[inline(always)]
    pub fn ewl(&self) -> EWL_R {
        EWL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 25 - Bus_Off Status
    #[inline(always)]
    pub fn bol(&self) -> BOL_R {
        BOL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 26 - Watchdog Interrupt Line
    #[inline(always)]
    pub fn wdil(&self) -> WDIL_R {
        WDIL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 27 - Protocol Error in Arbitration Phase Line
    #[inline(always)]
    pub fn peal(&self) -> PEAL_R {
        PEAL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bit 28 - Protocol Error in Data Phase Line
    #[inline(always)]
    pub fn pedl(&self) -> PEDL_R {
        PEDL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bit 29 - Access to Reserved Address Line
    #[inline(always)]
    pub fn aral(&self) -> ARAL_R {
        ARAL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Rx FIFO 0 New Message Interrupt Line
    #[inline(always)]
    pub fn rf0nl(&mut self) -> RF0NL_W {
        RF0NL_W { w: self }
    }
    ///Bit 1 - Rx FIFO 0 Watermark Reached Interrupt Line
    #[inline(always)]
    pub fn rf0wl(&mut self) -> RF0WL_W {
        RF0WL_W { w: self }
    }
    ///Bit 2 - Rx FIFO 0 Full Interrupt Line
    #[inline(always)]
    pub fn rf0fl(&mut self) -> RF0FL_W {
        RF0FL_W { w: self }
    }
    ///Bit 3 - Rx FIFO 0 Message Lost Interrupt Line
    #[inline(always)]
    pub fn rf0ll(&mut self) -> RF0LL_W {
        RF0LL_W { w: self }
    }
    ///Bit 4 - Rx FIFO 1 New Message Interrupt Line
    #[inline(always)]
    pub fn rf1nl(&mut self) -> RF1NL_W {
        RF1NL_W { w: self }
    }
    ///Bit 5 - Rx FIFO 1 Watermark Reached Interrupt Line
    #[inline(always)]
    pub fn rf1wl(&mut self) -> RF1WL_W {
        RF1WL_W { w: self }
    }
    ///Bit 6 - Rx FIFO 1 Full Interrupt Line
    #[inline(always)]
    pub fn rf1fl(&mut self) -> RF1FL_W {
        RF1FL_W { w: self }
    }
    ///Bit 7 - Rx FIFO 1 Message Lost Interrupt Line
    #[inline(always)]
    pub fn rf1ll(&mut self) -> RF1LL_W {
        RF1LL_W { w: self }
    }
    ///Bit 8 - High Priority Message Interrupt Line
    #[inline(always)]
    pub fn hpml(&mut self) -> HPML_W {
        HPML_W { w: self }
    }
    ///Bit 9 - Transmission Completed Interrupt Line
    #[inline(always)]
    pub fn tcl(&mut self) -> TCL_W {
        TCL_W { w: self }
    }
    ///Bit 10 - Transmission Cancellation Finished Interrupt Line
    #[inline(always)]
    pub fn tcfl(&mut self) -> TCFL_W {
        TCFL_W { w: self }
    }
    ///Bit 11 - Tx FIFO Empty Interrupt Line
    #[inline(always)]
    pub fn tefl(&mut self) -> TEFL_W {
        TEFL_W { w: self }
    }
    ///Bit 12 - Tx Event FIFO New Entry Interrupt Line
    #[inline(always)]
    pub fn tefnl(&mut self) -> TEFNL_W {
        TEFNL_W { w: self }
    }
    ///Bit 13 - Tx Event FIFO Watermark Reached Interrupt Line
    #[inline(always)]
    pub fn tefwl(&mut self) -> TEFWL_W {
        TEFWL_W { w: self }
    }
    ///Bit 14 - Tx Event FIFO Full Interrupt Line
    #[inline(always)]
    pub fn teffl(&mut self) -> TEFFL_W {
        TEFFL_W { w: self }
    }
    ///Bit 15 - Tx Event FIFO Element Lost Interrupt Line
    #[inline(always)]
    pub fn tefll(&mut self) -> TEFLL_W {
        TEFLL_W { w: self }
    }
    ///Bit 16 - Timestamp Wraparound Interrupt Line
    #[inline(always)]
    pub fn tswl(&mut self) -> TSWL_W {
        TSWL_W { w: self }
    }
    ///Bit 17 - Message RAM Access Failure Interrupt Line
    #[inline(always)]
    pub fn mrafl(&mut self) -> MRAFL_W {
        MRAFL_W { w: self }
    }
    ///Bit 18 - Timeout Occurred Interrupt Line
    #[inline(always)]
    pub fn tool(&mut self) -> TOOL_W {
        TOOL_W { w: self }
    }
    ///Bit 19 - Message stored to Dedicated Rx Buffer Interrupt Line
    #[inline(always)]
    pub fn drxl(&mut self) -> DRXL_W {
        DRXL_W { w: self }
    }
    ///Bit 20 - Bit Error Corrected Interrupt Line
    #[inline(always)]
    pub fn becl(&mut self) -> BECL_W {
        BECL_W { w: self }
    }
    ///Bit 21 - Bit Error Uncorrected Interrupt Line
    #[inline(always)]
    pub fn beul(&mut self) -> BEUL_W {
        BEUL_W { w: self }
    }
    ///Bit 22 - Error Logging Overflow Interrupt Line
    #[inline(always)]
    pub fn elol(&mut self) -> ELOL_W {
        ELOL_W { w: self }
    }
    ///Bit 23 - Error Passive Interrupt Line
    #[inline(always)]
    pub fn epl(&mut self) -> EPL_W {
        EPL_W { w: self }
    }
    ///Bit 24 - Warning Status Interrupt Line
    #[inline(always)]
    pub fn ewl(&mut self) -> EWL_W {
        EWL_W { w: self }
    }
    ///Bit 25 - Bus_Off Status
    #[inline(always)]
    pub fn bol(&mut self) -> BOL_W {
        BOL_W { w: self }
    }
    ///Bit 26 - Watchdog Interrupt Line
    #[inline(always)]
    pub fn wdil(&mut self) -> WDIL_W {
        WDIL_W { w: self }
    }
    ///Bit 27 - Protocol Error in Arbitration Phase Line
    #[inline(always)]
    pub fn peal(&mut self) -> PEAL_W {
        PEAL_W { w: self }
    }
    ///Bit 28 - Protocol Error in Data Phase Line
    #[inline(always)]
    pub fn pedl(&mut self) -> PEDL_W {
        PEDL_W { w: self }
    }
    ///Bit 29 - Access to Reserved Address Line
    #[inline(always)]
    pub fn aral(&mut self) -> ARAL_W {
        ARAL_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN Interrupt Line Select Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ils](index.html) module
pub struct ILS_SPEC;
impl crate::RegisterSpec for ILS_SPEC {
    type Ux = u32;
}
///`read()` method returns [ils::R](R) reader structure
impl crate::Readable for ILS_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ils::W](W) writer structure
impl crate::Writable for ILS_SPEC {
    type Writer = W;
}
///`reset()` method sets ILS to value 0
impl crate::Resettable for ILS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
