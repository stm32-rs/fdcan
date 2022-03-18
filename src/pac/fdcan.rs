// unfortunately there are two conflicting memory maps in use for these registers
#[cfg(feature = "fdcan_g0_g4_l5")]
pub use ie_g0_g4_l5 as ie;
#[cfg(feature = "fdcan_g0_g4_l5")]
pub use ile_g0_g4_l5 as ile;
#[cfg(feature = "fdcan_g0_g4_l5")]
pub use ils_g0_g4_l5 as ils;
#[cfg(feature = "fdcan_g0_g4_l5")]
pub use ir_g0_g4_l5 as ir;
#[cfg(feature = "fdcan_g0_g4_l5")]
pub use rxgfc_g0_g4_l5 as rxgfc;
#[cfg(feature = "fdcan_g0_g4_l5")]
pub use txbc_g0_g4_l5 as txbc;

#[cfg(feature = "fdcan_h7")]
pub use ie_h7 as ie;
#[cfg(feature = "fdcan_h7")]
pub use ile_h7 as ile;
#[cfg(feature = "fdcan_h7")]
pub use ils_h7 as ils;
#[cfg(feature = "fdcan_h7")]
pub use ir_h7 as ir;
#[cfg(feature = "fdcan_h7")]
pub use rxgfc_h7 as rxgfc;
#[cfg(feature = "fdcan_h7")]
pub use txbc_h7 as txbc;

///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - FDCAN Core Release Register
    pub crel: crate::Reg<crel::CREL_SPEC>,
    ///0x04 - FDCAN Core Release Register
    pub endn: crate::Reg<endn::ENDN_SPEC>,
    _reserved2: [u8; 0x04],
    ///0x0c - FDCAN Data Bit Timing and Prescaler Register
    pub dbtp: crate::Reg<dbtp::DBTP_SPEC>,
    ///0x10 - FDCAN Test Register
    pub test: crate::Reg<test::TEST_SPEC>,
    ///0x14 - FDCAN RAM Watchdog Register
    pub rwd: crate::Reg<rwd::RWD_SPEC>,
    ///0x18 - FDCAN CC Control Register
    pub cccr: crate::Reg<cccr::CCCR_SPEC>,
    ///0x1c - FDCAN Nominal Bit Timing and Prescaler Register
    pub nbtp: crate::Reg<nbtp::NBTP_SPEC>,
    ///0x20 - FDCAN Timestamp Counter Configuration Register
    pub tscc: crate::Reg<tscc::TSCC_SPEC>,
    ///0x24 - FDCAN Timestamp Counter Value Register
    pub tscv: crate::Reg<tscv::TSCV_SPEC>,
    ///0x28 - FDCAN Timeout Counter Configuration Register
    pub tocc: crate::Reg<tocc::TOCC_SPEC>,
    ///0x2c - FDCAN Timeout Counter Value Register
    pub tocv: crate::Reg<tocv::TOCV_SPEC>,
    _reserved11: [u8; 0x10],
    ///0x40 - FDCAN Error Counter Register
    pub ecr: crate::Reg<ecr::ECR_SPEC>,
    ///0x44 - FDCAN Protocol Status Register
    pub psr: crate::Reg<psr::PSR_SPEC>,
    ///0x48 - FDCAN Transmitter Delay Compensation Register
    pub tdcr: crate::Reg<tdcr::TDCR_SPEC>,
    _reserved14: [u8; 0x04],
    ///0x50 - FDCAN Interrupt Register
    pub ir: crate::Reg<ir::IR_SPEC>,
    ///0x54 - FDCAN Interrupt Enable Register
    pub ie: crate::Reg<ie::IE_SPEC>,
    ///0x58 - FDCAN Interrupt Line Select Register
    pub ils: crate::Reg<ils::ILS_SPEC>,
    ///0x5c - FDCAN Interrupt Line Enable Register
    pub ile: crate::Reg<ile::ILE_SPEC>,
    _reserved18: [u8; 0x20],
    ///0x80 - FDCAN Global Filter Configuration Register
    pub rxgfc: crate::Reg<rxgfc::RXGFC_SPEC>,

    ///0x84 - FDCAN Standard ID Filter Configuration Register
    #[cfg(feature = "fdcan_h7")]
    pub sidfc: crate::Reg<sidfc::SIDFC_SPEC>,
    ///0x88 - FDCAN Extended ID Filter Configuration Register
    #[cfg(feature = "fdcan_h7")]
    pub xidfc: crate::Reg<xidfc::XIDFC_SPEC>,
    #[cfg(feature = "fdcan_h7")]
    _reserved21: [u8; 0x04],
    ///0x90 - FDCAN Extended ID and Mask Register
    #[cfg(feature = "fdcan_h7")]
    pub xidam: crate::Reg<xidam::XIDAM_SPEC>,
    ///0x94 - FDCAN High Priority Message Status Register
    #[cfg(feature = "fdcan_h7")]
    pub hpms: crate::Reg<hpms::HPMS_SPEC>,
    ///0x98 - FDCAN New Data 1 Register
    #[cfg(feature = "fdcan_h7")]
    pub ndat1: crate::Reg<ndat1::NDAT1_SPEC>,
    ///0x9c - FDCAN New Data 2 Register
    #[cfg(feature = "fdcan_h7")]
    pub ndat2: crate::Reg<ndat2::NDAT2_SPEC>,
    ///0xa0 - FDCAN Rx FIFO 0 Configuration Register
    #[cfg(feature = "fdcan_h7")]
    pub rxf0c: crate::Reg<rxf0c::RXF0C_SPEC>,
    ///0xa4 - FDCAN Rx FIFO 0 Status Register
    #[cfg(feature = "fdcan_h7")]
    pub rxf0s: crate::Reg<rxf0s::RXF0S_SPEC>,
    ///0xa8 - CAN Rx FIFO 0 Acknowledge Register
    #[cfg(feature = "fdcan_h7")]
    pub rxf0a: crate::Reg<rxf0a::RXF0A_SPEC>,
    ///0xac - FDCAN Rx Buffer Configuration Register
    #[cfg(feature = "fdcan_h7")]
    pub rxbc: crate::Reg<rxbc::RXBC_SPEC>,
    ///0xb0 - FDCAN Rx FIFO 1 Configuration Register
    #[cfg(feature = "fdcan_h7")]
    pub rxf1c: crate::Reg<rxf1c::RXF1C_SPEC>,
    ///0xb4 - FDCAN Rx FIFO 1 Status Register
    #[cfg(feature = "fdcan_h7")]
    pub rxf1s: crate::Reg<rxf1s::RXF1S_SPEC>,
    ///0xb8 - FDCAN Rx FIFO 1 Acknowledge Register
    #[cfg(feature = "fdcan_h7")]
    pub rxf1a: crate::Reg<rxf1a::RXF1A_SPEC>,
    ///0xbc - FDCAN Rx Buffer Element Size Configuration Register
    #[cfg(feature = "fdcan_h7")]
    pub rxesc: crate::Reg<rxesc::RXESC_SPEC>,

    ///0x84 - FDCAN Extended ID and Mask Register
    #[cfg(feature = "fdcan_g0_g4_l5")]
    pub xidam: crate::Reg<xidam::XIDAM_SPEC>,
    ///0x88 - FDCAN High Priority Message Status Register
    #[cfg(feature = "fdcan_g0_g4_l5")]
    pub hpms: crate::Reg<hpms::HPMS_SPEC>,
    #[cfg(feature = "fdcan_g0_g4_l5")]
    _reserved21: [u8; 0x04],
    ///0x90 - FDCAN Rx FIFO 0 Status Register
    #[cfg(feature = "fdcan_g0_g4_l5")]
    pub rxf0s: crate::Reg<rxf0s::RXF0S_SPEC>,
    ///0x94 - CAN Rx FIFO 0 Acknowledge Register
    #[cfg(feature = "fdcan_g0_g4_l5")]
    pub rxf0a: crate::Reg<rxf0a::RXF0A_SPEC>,
    ///0x98 - FDCAN Rx FIFO 1 Status Register
    #[cfg(feature = "fdcan_g0_g4_l5")]
    pub rxf1s: crate::Reg<rxf1s::RXF1S_SPEC>,
    ///0x9c - FDCAN Rx FIFO 1 Acknowledge Register
    #[cfg(feature = "fdcan_g0_g4_l5")]
    pub rxf1a: crate::Reg<rxf1a::RXF1A_SPEC>,
    #[cfg(feature = "fdcan_g0_g4_l5")]
    _reserved25: [u8; 0x20],

    ///0xc0 - FDCAN Tx Buffer Configuration Register
    pub txbc: crate::Reg<txbc::TXBC_SPEC>,
    ///0xc4 - FDCAN Tx FIFO/Queue Status Register
    pub txfqs: crate::Reg<txfqs::TXFQS_SPEC>,

    ///0xc8 - FDCAN Tx Buffer Element Size Configuration Register
    #[cfg(feature = "fdcan_h7")]
    pub txesc: crate::Reg<txesc::TXESC_SPEC>,

    ///0xc8/0xcc - FDCAN Tx Buffer Request Pending Register
    pub txbrp: crate::Reg<txbrp::TXBRP_SPEC>,
    ///0xcc/0xd0 - FDCAN Tx Buffer Add Request Register
    pub txbar: crate::Reg<txbar::TXBAR_SPEC>,
    ///0xd0/0xd4 - FDCAN Tx Buffer Cancellation Request Register
    pub txbcr: crate::Reg<txbcr::TXBCR_SPEC>,
    ///0xd4/0xd8 - FDCAN Tx Buffer Transmission Occurred Register
    pub txbto: crate::Reg<txbto::TXBTO_SPEC>,
    ///0xd8/0xdc - FDCAN Tx Buffer Cancellation Finished Register
    pub txbcf: crate::Reg<txbcf::TXBCF_SPEC>,
    ///0xdc/0xe0 - FDCAN Tx Buffer Transmission Interrupt Enable Register
    pub txbtie: crate::Reg<txbtie::TXBTIE_SPEC>,
    ///0xe0/0xe4 - FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register
    pub txbcie: crate::Reg<txbcie::TXBCIE_SPEC>,

    #[cfg(feature = "fdcan_h7")]
    _reserved43: [u8; 0x08],
    ///0xf0 - FDCAN Tx Event FIFO Configuration Register
    #[cfg(feature = "fdcan_h7")]
    pub txefc: crate::Reg<txefc::TXEFC_SPEC>,

    ///0xe4/0xf4 - FDCAN Tx Event FIFO Status Register
    pub txefs: crate::Reg<txefs::TXEFS_SPEC>,
    ///0xe8/0xf8 - FDCAN Tx Event FIFO Acknowledge Register
    pub txefa: crate::Reg<txefa::TXEFA_SPEC>,

    #[cfg(feature = "fdcan_h7")]
    _reserved46: [u8; 0x04],
    #[cfg(feature = "fdcan_g0_g4_l5")]
    _reserved36: [u8; 0x14],

    ///0x100 - FDCAN TT Trigger Memory Configuration Register
    pub tttmc: crate::Reg<tttmc::TTTMC_SPEC>,
    ///0x104 - FDCAN TT Reference Message Configuration Register
    pub ttrmc: crate::Reg<ttrmc::TTRMC_SPEC>,
    ///0x108 - FDCAN TT Operation Configuration Register
    pub ttocf: crate::Reg<ttocf::TTOCF_SPEC>,
    ///0x10c - FDCAN TT Matrix Limits Register
    pub ttmlm: crate::Reg<ttmlm::TTMLM_SPEC>,
    ///0x110 - FDCAN TUR Configuration Register
    pub turcf: crate::Reg<turcf::TURCF_SPEC>,
    ///0x114 - FDCAN TT Operation Control Register
    pub ttocn: crate::Reg<ttocn::TTOCN_SPEC>,
    ///0x118 - FDCAN TT Global Time Preset Register
    pub ttgtp: crate::Reg<ttgtp::TTGTP_SPEC>,
    ///0x11c - FDCAN TT Time Mark Register
    pub tttmk: crate::Reg<tttmk::TTTMK_SPEC>,
    ///0x120 - FDCAN TT Interrupt Register
    pub ttir: crate::Reg<ttir::TTIR_SPEC>,
    ///0x124 - FDCAN TT Interrupt Enable Register
    pub ttie: crate::Reg<ttie::TTIE_SPEC>,
    ///0x128 - FDCAN TT Interrupt Line Select Register
    pub ttils: crate::Reg<ttils::TTILS_SPEC>,
    ///0x12c - FDCAN TT Operation Status Register
    pub ttost: crate::Reg<ttost::TTOST_SPEC>,
    ///0x130 - FDCAN TUR Numerator Actual Register
    pub turna: crate::Reg<turna::TURNA_SPEC>,
    ///0x134 - FDCAN TT Local and Global Time Register
    pub ttlgt: crate::Reg<ttlgt::TTLGT_SPEC>,
    ///0x138 - FDCAN TT Cycle Time and Count Register
    pub ttctc: crate::Reg<ttctc::TTCTC_SPEC>,
    ///0x13c - FDCAN TT Capture Time Register
    pub ttcpt: crate::Reg<ttcpt::TTCPT_SPEC>,
    ///0x140 - FDCAN TT Cycle Sync Mark Register
    pub ttcsm: crate::Reg<ttcsm::TTCSM_SPEC>,
    _reserved63: [u8; 0x01bc],
    ///0x300 - FDCAN TT Trigger Select Register
    pub ttts: crate::Reg<ttts::TTTS_SPEC>,
}
///CREL register accessor: an alias for `Reg<CREL_SPEC>`
pub type CREL = crate::Reg<crel::CREL_SPEC>;
///FDCAN Core Release Register
pub mod crel;
///ENDN register accessor: an alias for `Reg<ENDN_SPEC>`
pub type ENDN = crate::Reg<endn::ENDN_SPEC>;
///FDCAN Core Release Register
pub mod endn;
///DBTP register accessor: an alias for `Reg<DBTP_SPEC>`
pub type DBTP = crate::Reg<dbtp::DBTP_SPEC>;
///FDCAN Data Bit Timing and Prescaler Register
pub mod dbtp;
///TEST register accessor: an alias for `Reg<TEST_SPEC>`
pub type TEST = crate::Reg<test::TEST_SPEC>;
///FDCAN Test Register
pub mod test;
///RWD register accessor: an alias for `Reg<RWD_SPEC>`
pub type RWD = crate::Reg<rwd::RWD_SPEC>;
///FDCAN RAM Watchdog Register
pub mod rwd;
///CCCR register accessor: an alias for `Reg<CCCR_SPEC>`
pub type CCCR = crate::Reg<cccr::CCCR_SPEC>;
///FDCAN CC Control Register
pub mod cccr;
///NBTP register accessor: an alias for `Reg<NBTP_SPEC>`
pub type NBTP = crate::Reg<nbtp::NBTP_SPEC>;
///FDCAN Nominal Bit Timing and Prescaler Register
pub mod nbtp;
///TSCC register accessor: an alias for `Reg<TSCC_SPEC>`
pub type TSCC = crate::Reg<tscc::TSCC_SPEC>;
///FDCAN Timestamp Counter Configuration Register
pub mod tscc;
///TSCV register accessor: an alias for `Reg<TSCV_SPEC>`
pub type TSCV = crate::Reg<tscv::TSCV_SPEC>;
///FDCAN Timestamp Counter Value Register
pub mod tscv;
///TOCC register accessor: an alias for `Reg<TOCC_SPEC>`
pub type TOCC = crate::Reg<tocc::TOCC_SPEC>;
///FDCAN Timeout Counter Configuration Register
pub mod tocc;
///TOCV register accessor: an alias for `Reg<TOCV_SPEC>`
pub type TOCV = crate::Reg<tocv::TOCV_SPEC>;
///FDCAN Timeout Counter Value Register
pub mod tocv;
///ECR register accessor: an alias for `Reg<ECR_SPEC>`
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
///FDCAN Error Counter Register
pub mod ecr;
///PSR register accessor: an alias for `Reg<PSR_SPEC>`
pub type PSR = crate::Reg<psr::PSR_SPEC>;
///FDCAN Protocol Status Register
pub mod psr;
///TDCR register accessor: an alias for `Reg<TDCR_SPEC>`
pub type TDCR = crate::Reg<tdcr::TDCR_SPEC>;
///FDCAN Transmitter Delay Compensation Register
pub mod tdcr;
///IR register accessor: an alias for `Reg<IR_SPEC>`
pub type IR = crate::Reg<ir::IR_SPEC>;
///FDCAN Interrupt Register
pub mod ir_g0_g4_l5;
pub mod ir_h7;
///IE register accessor: an alias for `Reg<IE_SPEC>`
pub type IE = crate::Reg<ie::IE_SPEC>;
///FDCAN Interrupt Enable Register
pub mod ie_g0_g4_l5;
pub mod ie_h7;
///ILS register accessor: an alias for `Reg<ILS_SPEC>`
pub type ILS = crate::Reg<ils::ILS_SPEC>;
///FDCAN Interrupt Line Select Register
pub mod ils_g0_g4_l5;
pub mod ils_h7;
///ILE register accessor: an alias for `Reg<ILE_SPEC>`
pub type ILE = crate::Reg<ile::ILE_SPEC>;
///FDCAN Interrupt Line Enable Register
pub mod ile_g0_g4_l5;
pub mod ile_h7;
///GFC register accessor: an alias for `Reg<GFC_SPEC>`
pub type RXGFC = crate::Reg<rxgfc::RXGFC_SPEC>;
///FDCAN Global Filter Configuration Register
pub mod rxgfc_g0_g4_l5;
pub mod rxgfc_h7;
///SIDFC register accessor: an alias for `Reg<SIDFC_SPEC>`
pub type SIDFC = crate::Reg<sidfc::SIDFC_SPEC>;
///FDCAN Standard ID Filter Configuration Register
pub mod sidfc;
///XIDFC register accessor: an alias for `Reg<XIDFC_SPEC>`
pub type XIDFC = crate::Reg<xidfc::XIDFC_SPEC>;
///FDCAN Extended ID Filter Configuration Register
pub mod xidfc;
///XIDAM register accessor: an alias for `Reg<XIDAM_SPEC>`
pub type XIDAM = crate::Reg<xidam::XIDAM_SPEC>;
///FDCAN Extended ID and Mask Register
pub mod xidam;
///HPMS register accessor: an alias for `Reg<HPMS_SPEC>`
pub type HPMS = crate::Reg<hpms::HPMS_SPEC>;
///FDCAN High Priority Message Status Register
pub mod hpms;
///NDAT1 register accessor: an alias for `Reg<NDAT1_SPEC>`
pub type NDAT1 = crate::Reg<ndat1::NDAT1_SPEC>;
///FDCAN New Data 1 Register
pub mod ndat1;
///NDAT2 register accessor: an alias for `Reg<NDAT2_SPEC>`
pub type NDAT2 = crate::Reg<ndat2::NDAT2_SPEC>;
///FDCAN New Data 2 Register
pub mod ndat2;
///RXF0C register accessor: an alias for `Reg<RXF0C_SPEC>`
pub type RXF0C = crate::Reg<rxf0c::RXF0C_SPEC>;
///FDCAN Rx FIFO 0 Configuration Register
pub mod rxf0c;
///RXF0S register accessor: an alias for `Reg<RXF0S_SPEC>`
pub type RXF0S = crate::Reg<rxf0s::RXF0S_SPEC>;
///FDCAN Rx FIFO 0 Status Register
pub mod rxf0s;
///RXF0A register accessor: an alias for `Reg<RXF0A_SPEC>`
pub type RXF0A = crate::Reg<rxf0a::RXF0A_SPEC>;
///CAN Rx FIFO 0 Acknowledge Register
pub mod rxf0a;
///RXBC register accessor: an alias for `Reg<RXBC_SPEC>`
pub type RXBC = crate::Reg<rxbc::RXBC_SPEC>;
///FDCAN Rx Buffer Configuration Register
pub mod rxbc;
///RXF1C register accessor: an alias for `Reg<RXF1C_SPEC>`
pub type RXF1C = crate::Reg<rxf1c::RXF1C_SPEC>;
///FDCAN Rx FIFO 1 Configuration Register
pub mod rxf1c;
///RXF1S register accessor: an alias for `Reg<RXF1S_SPEC>`
pub type RXF1S = crate::Reg<rxf1s::RXF1S_SPEC>;
///FDCAN Rx FIFO 1 Status Register
pub mod rxf1s;
///RXF1A register accessor: an alias for `Reg<RXF1A_SPEC>`
pub type RXF1A = crate::Reg<rxf1a::RXF1A_SPEC>;
///FDCAN Rx FIFO 1 Acknowledge Register
pub mod rxf1a;
///RXESC register accessor: an alias for `Reg<RXESC_SPEC>`
pub type RXESC = crate::Reg<rxesc::RXESC_SPEC>;
///FDCAN Rx Buffer Element Size Configuration Register
pub mod rxesc;
///TXBC register accessor: an alias for `Reg<TXBC_SPEC>`
pub type TXBC = crate::Reg<txbc::TXBC_SPEC>;
///FDCAN Tx Buffer Configuration Register
pub mod txbc_g0_g4_l5;
pub mod txbc_h7;
///TXFQS register accessor: an alias for `Reg<TXFQS_SPEC>`
pub type TXFQS = crate::Reg<txfqs::TXFQS_SPEC>;
///FDCAN Tx FIFO/Queue Status Register
pub mod txfqs;
///TXESC register accessor: an alias for `Reg<TXESC_SPEC>`
pub type TXESC = crate::Reg<txesc::TXESC_SPEC>;
///FDCAN Tx Buffer Element Size Configuration Register
pub mod txesc;
///TXBRP register accessor: an alias for `Reg<TXBRP_SPEC>`
pub type TXBRP = crate::Reg<txbrp::TXBRP_SPEC>;
///FDCAN Tx Buffer Request Pending Register
pub mod txbrp;
///TXBAR register accessor: an alias for `Reg<TXBAR_SPEC>`
pub type TXBAR = crate::Reg<txbar::TXBAR_SPEC>;
///FDCAN Tx Buffer Add Request Register
pub mod txbar;
///TXBCR register accessor: an alias for `Reg<TXBCR_SPEC>`
pub type TXBCR = crate::Reg<txbcr::TXBCR_SPEC>;
///FDCAN Tx Buffer Cancellation Request Register
pub mod txbcr;
///TXBTO register accessor: an alias for `Reg<TXBTO_SPEC>`
pub type TXBTO = crate::Reg<txbto::TXBTO_SPEC>;
///FDCAN Tx Buffer Transmission Occurred Register
pub mod txbto;
///TXBCF register accessor: an alias for `Reg<TXBCF_SPEC>`
pub type TXBCF = crate::Reg<txbcf::TXBCF_SPEC>;
///FDCAN Tx Buffer Cancellation Finished Register
pub mod txbcf;
///TXBTIE register accessor: an alias for `Reg<TXBTIE_SPEC>`
pub type TXBTIE = crate::Reg<txbtie::TXBTIE_SPEC>;
///FDCAN Tx Buffer Transmission Interrupt Enable Register
pub mod txbtie;
///TXBCIE register accessor: an alias for `Reg<TXBCIE_SPEC>`
pub type TXBCIE = crate::Reg<txbcie::TXBCIE_SPEC>;
///FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register
pub mod txbcie;
///TXEFC register accessor: an alias for `Reg<TXEFC_SPEC>`
pub type TXEFC = crate::Reg<txefc::TXEFC_SPEC>;
///FDCAN Tx Event FIFO Configuration Register
pub mod txefc;
///TXEFS register accessor: an alias for `Reg<TXEFS_SPEC>`
pub type TXEFS = crate::Reg<txefs::TXEFS_SPEC>;
///FDCAN Tx Event FIFO Status Register
pub mod txefs;
///TXEFA register accessor: an alias for `Reg<TXEFA_SPEC>`
pub type TXEFA = crate::Reg<txefa::TXEFA_SPEC>;
///FDCAN Tx Event FIFO Acknowledge Register
pub mod txefa;
///TTTMC register accessor: an alias for `Reg<TTTMC_SPEC>`
pub type TTTMC = crate::Reg<tttmc::TTTMC_SPEC>;
///FDCAN TT Trigger Memory Configuration Register
pub mod tttmc;
///TTRMC register accessor: an alias for `Reg<TTRMC_SPEC>`
pub type TTRMC = crate::Reg<ttrmc::TTRMC_SPEC>;
///FDCAN TT Reference Message Configuration Register
pub mod ttrmc;
///TTOCF register accessor: an alias for `Reg<TTOCF_SPEC>`
pub type TTOCF = crate::Reg<ttocf::TTOCF_SPEC>;
///FDCAN TT Operation Configuration Register
pub mod ttocf;
///TTMLM register accessor: an alias for `Reg<TTMLM_SPEC>`
pub type TTMLM = crate::Reg<ttmlm::TTMLM_SPEC>;
///FDCAN TT Matrix Limits Register
pub mod ttmlm;
///TURCF register accessor: an alias for `Reg<TURCF_SPEC>`
pub type TURCF = crate::Reg<turcf::TURCF_SPEC>;
///FDCAN TUR Configuration Register
pub mod turcf;
///TTOCN register accessor: an alias for `Reg<TTOCN_SPEC>`
pub type TTOCN = crate::Reg<ttocn::TTOCN_SPEC>;
///FDCAN TT Operation Control Register
pub mod ttocn;
///TTGTP register accessor: an alias for `Reg<TTGTP_SPEC>`
pub type TTGTP = crate::Reg<ttgtp::TTGTP_SPEC>;
///FDCAN TT Global Time Preset Register
pub mod ttgtp;
///TTTMK register accessor: an alias for `Reg<TTTMK_SPEC>`
pub type TTTMK = crate::Reg<tttmk::TTTMK_SPEC>;
///FDCAN TT Time Mark Register
pub mod tttmk;
///TTIR register accessor: an alias for `Reg<TTIR_SPEC>`
pub type TTIR = crate::Reg<ttir::TTIR_SPEC>;
///FDCAN TT Interrupt Register
pub mod ttir;
///TTIE register accessor: an alias for `Reg<TTIE_SPEC>`
pub type TTIE = crate::Reg<ttie::TTIE_SPEC>;
///FDCAN TT Interrupt Enable Register
pub mod ttie;
///TTILS register accessor: an alias for `Reg<TTILS_SPEC>`
pub type TTILS = crate::Reg<ttils::TTILS_SPEC>;
///FDCAN TT Interrupt Line Select Register
pub mod ttils;
///TTOST register accessor: an alias for `Reg<TTOST_SPEC>`
pub type TTOST = crate::Reg<ttost::TTOST_SPEC>;
///FDCAN TT Operation Status Register
pub mod ttost;
///TURNA register accessor: an alias for `Reg<TURNA_SPEC>`
pub type TURNA = crate::Reg<turna::TURNA_SPEC>;
///FDCAN TUR Numerator Actual Register
pub mod turna;
///TTLGT register accessor: an alias for `Reg<TTLGT_SPEC>`
pub type TTLGT = crate::Reg<ttlgt::TTLGT_SPEC>;
///FDCAN TT Local and Global Time Register
pub mod ttlgt;
///TTCTC register accessor: an alias for `Reg<TTCTC_SPEC>`
pub type TTCTC = crate::Reg<ttctc::TTCTC_SPEC>;
///FDCAN TT Cycle Time and Count Register
pub mod ttctc;
///TTCPT register accessor: an alias for `Reg<TTCPT_SPEC>`
pub type TTCPT = crate::Reg<ttcpt::TTCPT_SPEC>;
///FDCAN TT Capture Time Register
pub mod ttcpt;
///TTCSM register accessor: an alias for `Reg<TTCSM_SPEC>`
pub type TTCSM = crate::Reg<ttcsm::TTCSM_SPEC>;
///FDCAN TT Cycle Sync Mark Register
pub mod ttcsm;
///TTTS register accessor: an alias for `Reg<TTTS_SPEC>`
pub type TTTS = crate::Reg<ttts::TTTS_SPEC>;
///FDCAN TT Trigger Select Register
pub mod ttts;
