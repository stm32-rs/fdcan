//! Interrupt types.

use core::ops;

#[allow(unused_imports)] // for intra-doc links only
use crate::{FdCan, Rx};

macro_rules! declare_interrupts {
    ($([$name:ident, $index:literal, $doc:expr],)*) => {
        /// FdCAN interrupt sources.
        ///
        /// These can be individually enabled and disabled in the FdCAN
        /// peripheral. Note that each FdCAN peripheral only exposes 2
        /// interrupts to the microcontroller:
        ///
        /// FDCANx_INTR0,
        /// FDCANx_INTR1,
        ///
        /// The interrupts available on each line can be configured using the
        /// `[crate::config::FdCanConfig]` struct.
        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        #[non_exhaustive]
        #[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
        pub enum Interrupt {
            $(
                #[doc = $doc]
                $name = 1 << $index
            ),*
        }

        paste::paste! {
            bitflags::bitflags! {
                /// A set of FdCAN interrupts.
                #[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
                pub struct Interrupts: u32 {
                    $(
                        #[doc = $doc]
                        const [< $name:snake:upper >] = 1 << $index;
                    )*
                }
            }
        }
    };
}

// interrupts for g0 g4 l5
#[cfg(feature = "fdcan_g0_g4_l5")]
declare_interrupts!(
    [RxFifo0NewMsg, 0, "Rx FIFO 0 has a new message"],
    [RxFifo0Full, 1, "Rx FIFO 0 is full"],
    [RxFifo0MsgLost, 2, "Rx FIFO 0 has lost a message"],
    [RxFifo1NewMsg, 3, "Rx FIFO 1 has a new message"],
    [RxFifo1Full, 4, "Rx FIFO 1 is full"],
    [RxFifo1MsgLost, 5, "Rx FIFO 1 has lost a message"],
    [
        RxHighPrio,
        6,
        "A High Priority Message has been flagged by a filter"
    ],
    [TxComplete, 7, "Transmit has been completed"],
    [TxCancel, 8, "Tx message has been cancelled"],
    [TxEmpty, 9, "Tx Fifo is empty"],
    [
        TxEventNew,
        10,
        "An new Event has been received in the Tx Event Fifo"
    ],
    [TxEventFull, 11, "The TxEvent Fifo is full"],
    [TxEventLost, 12, "An Tx Event has been lost"],
    [TsWrapAround, 13, "Timestamp wrap around has occurred"],
    [MsgRamAccessFailure, 14, "Message RAM access failure.
The flag is set when the Rx handler:
has not completed acceptance filtering or storage of an accepted message until the
arbitration field of the following message has been received. In this case acceptance
filtering or message storage is aborted and the Rx handler starts processing of the
following message. was unable to write a message to the message RAM. In this case
message storage is aborted.
In both cases the FIFO put index is not updated. The partly stored message is overwritten
when the next message is stored to this location.
The flag is also set when the Tx Handler was not able to read a message from the Message
RAM in time. In this case message transmission is aborted. In case of a Tx Handler access
failure the FDCAN is switched into Restricted operation Mode (see Restricted operation
mode)."],
    [TimeoutOccurred, 15, "Timeout Occurred"],
    [
        ErrLogOverflow,
        16,
        "Overflow of CAN error logging counter occurred"
    ],
    [ErrPassive, 17, "Errr Passive"],
    [WarningStatus, 18, "Warning Status"],
    [BusOff, 19, "Bus_Off status"],
    [WatchdogInt, 20, " Watchdog interrupt"],
    [
        ProtErrArbritation,
        21,
        "Protocol error in arbitration phase (nominal bit time is used)"
    ],
    [
        ProtErrData,
        22,
        "Protocol error in data phase (data bit time is used)"
    ],
    [ReservedAccess, 23, "Access to reserved address"],
);

// interrupts for h7
#[cfg(feature = "fdcan_h7")]
declare_interrupts!(
    [RxFifo0NewMsg, 0, "Rx FIFO 0 has a new message"],
    [RxFifo0Watermark, 1, "Rx FIFO 0 watermark reached"],
    [RxFifo0Full, 2, "Rx FIFO 0 is full"],
    [RxFifo0MsgLost, 3, "Rx FIFO 0 has lost a message"],

    [RxFifo1NewMsg, 4, "Rx FIFO 1 has a new message"],
    [RxFifo1Watermark, 5, "Rx FIFO 1 watermark reached"],
    [RxFifo1Full, 6, "Rx FIFO 1 is full"],
    [RxFifo1MsgLost, 7, "Rx FIFO 1 has lost a message"],

    [
        RxHighPrio,
        8,
        "A High Priority Message has been flagged by a filter"
    ],
    [TxComplete, 9, "Transmit has been completed"],
    [TxCancel, 10, "Tx message has been cancelled"],
    [TxEmpty, 11, "Tx Fifo is empty"],
    [
        TxEventNew,
        12,
        "An new Event has been received in the Tx Event Fifo"
    ],
    [TxWatermark, 13, "TxEvent FIFO watermark reached"],
    [TxEventFull, 14, "The TxEvent Fifo is full"],
    [TxEventLost, 15, "An Tx Event has been lost"],
    [TsWrapAround, 16, "Timestamp wrap around has occurred"],

    [MsgRamAccessFailure, 17, "Message RAM access failure.
The flag is set when the Rx handler:
has not completed acceptance filtering or storage of an accepted message until the
arbitration field of the following message has been received. In this case acceptance
filtering or message storage is aborted and the Rx handler starts processing of the
following message. was unable to write a message to the message RAM. In this case
message storage is aborted.
In both cases the FIFO put index is not updated. The partly stored message is overwritten
when the next message is stored to this location.
The flag is also set when the Tx Handler was not able to read a message from the Message
RAM in time. In this case message transmission is aborted. In case of a Tx Handler access
failure the FDCAN is switched into Restricted operation Mode (see Restricted operation
mode)."],
    [TimeoutOccurred, 18, "Timeout Occurred"],
    [
        ErrLogOverflow,
        22,
        "Overflow of CAN error logging counter occurred"
    ],
    [ErrPassive, 23, "Errr Passive"],
    [WarningStatus, 24, "Warning Status"],
    [BusOff, 25, "Bus_Off status"],
    [WatchdogInt, 26, " Watchdog interrupt"],
    [
        ProtErrArbritation,
        27,
        "Protocol error in arbitration phase (nominal bit time is used)"
    ],
    [
        ProtErrData,
        28,
        "Protocol error in data phase (data bit time is used)"
    ],
    [ReservedAccess, 29, "Access to reserved address"],
);

impl Interrupts {
    /// No Interrupt masks selected
    pub fn none() -> Self {
        Self::from_bits_truncate(0)
    }
}

impl From<Interrupt> for Interrupts {
    #[inline]
    fn from(i: Interrupt) -> Self {
        Self::from_bits_truncate(i as u32)
    }
}

/// Adds an interrupt to the interrupt set.
impl ops::BitOrAssign<Interrupt> for Interrupts {
    #[inline]
    fn bitor_assign(&mut self, rhs: Interrupt) {
        *self |= Self::from(rhs);
    }
}

/// There are two interrupt lines for the FdCan
/// The events linked to these can be configured
/// see `[config::FdCanConfig]`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum InterruptLine {
    /// Interrupt Line 0
    _0 = 0,
    /// Interrupt Line 1
    _1 = 1,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interrupt_flags() {
        assert_eq!(
            Interrupts::from(Interrupt::TxComplete),
            Interrupts::TX_COMPLETE
        );
        assert_eq!(Interrupts::from(Interrupt::TxEmpty), Interrupts::TX_EMPTY);

        let mut ints = Interrupts::RX_FIFO0_FULL;
        ints |= Interrupt::RxFifo1Full;
        assert_eq!(ints, Interrupts::RX_FIFO0_FULL | Interrupts::RX_FIFO1_FULL);
    }
}
