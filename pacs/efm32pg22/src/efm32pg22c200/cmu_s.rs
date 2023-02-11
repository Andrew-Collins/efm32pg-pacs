#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub ipversion: IPVERSION,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - No Description"]
    pub status: STATUS,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - No Description"]
    pub lock: LOCK,
    #[doc = "0x14 - No Description"]
    pub wdoglock: WDOGLOCK,
    _reserved4: [u8; 0x08],
    #[doc = "0x20 - No Description"]
    pub if_: IF,
    #[doc = "0x24 - No Description"]
    pub ien: IEN,
    _reserved6: [u8; 0x28],
    #[doc = "0x50 - No Description"]
    pub calcmd: CALCMD,
    #[doc = "0x54 - No Description"]
    pub calctrl: CALCTRL,
    #[doc = "0x58 - No Description"]
    pub calcnt: CALCNT,
    _reserved9: [u8; 0x08],
    #[doc = "0x64 - No Description"]
    pub clken0: CLKEN0,
    #[doc = "0x68 - No Description"]
    pub clken1: CLKEN1,
    _reserved11: [u8; 0x04],
    #[doc = "0x70 - No Description"]
    pub sysclkctrl: SYSCLKCTRL,
    _reserved12: [u8; 0x0c],
    #[doc = "0x80 - No Description"]
    pub traceclkctrl: TRACECLKCTRL,
    _reserved13: [u8; 0x0c],
    #[doc = "0x90 - No Description"]
    pub exportclkctrl: EXPORTCLKCTRL,
    _reserved14: [u8; 0x6c],
    #[doc = "0x100 - No Description"]
    pub dpllrefclkctrl: DPLLREFCLKCTRL,
    _reserved15: [u8; 0x1c],
    #[doc = "0x120 - No Description"]
    pub em01grpaclkctrl: EM01GRPACLKCTRL,
    #[doc = "0x124 - No Description"]
    pub em01grpbclkctrl: EM01GRPBCLKCTRL,
    _reserved17: [u8; 0x18],
    #[doc = "0x140 - No Description"]
    pub em23grpaclkctrl: EM23GRPACLKCTRL,
    _reserved18: [u8; 0x1c],
    #[doc = "0x160 - No Description"]
    pub em4grpaclkctrl: EM4GRPACLKCTRL,
    _reserved19: [u8; 0x1c],
    #[doc = "0x180 - No Description"]
    pub iadcclkctrl: IADCCLKCTRL,
    _reserved20: [u8; 0x7c],
    #[doc = "0x200 - No Description"]
    pub wdog0clkctrl: WDOG0CLKCTRL,
    _reserved21: [u8; 0x1c],
    #[doc = "0x220 - No Description"]
    pub euart0clkctrl: EUART0CLKCTRL,
    _reserved22: [u8; 0x1c],
    #[doc = "0x240 - No Description"]
    pub rtccclkctrl: RTCCCLKCTRL,
    _reserved23: [u8; 0x1c],
    #[doc = "0x260 - No Description"]
    pub cryptoaccclkctrl: CRYPTOACCCLKCTRL,
}
#[doc = "IPVERSION (r) register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "No Description"]
pub mod status;
#[doc = "LOCK (w) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "No Description"]
pub mod lock;
#[doc = "WDOGLOCK (w) register accessor: an alias for `Reg<WDOGLOCK_SPEC>`"]
pub type WDOGLOCK = crate::Reg<wdoglock::WDOGLOCK_SPEC>;
#[doc = "No Description"]
pub mod wdoglock;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "No Description"]
pub mod if_;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "No Description"]
pub mod ien;
#[doc = "CALCMD (w) register accessor: an alias for `Reg<CALCMD_SPEC>`"]
pub type CALCMD = crate::Reg<calcmd::CALCMD_SPEC>;
#[doc = "No Description"]
pub mod calcmd;
#[doc = "CALCTRL (rw) register accessor: an alias for `Reg<CALCTRL_SPEC>`"]
pub type CALCTRL = crate::Reg<calctrl::CALCTRL_SPEC>;
#[doc = "No Description"]
pub mod calctrl;
#[doc = "CALCNT (r) register accessor: an alias for `Reg<CALCNT_SPEC>`"]
pub type CALCNT = crate::Reg<calcnt::CALCNT_SPEC>;
#[doc = "No Description"]
pub mod calcnt;
#[doc = "CLKEN0 (rw) register accessor: an alias for `Reg<CLKEN0_SPEC>`"]
pub type CLKEN0 = crate::Reg<clken0::CLKEN0_SPEC>;
#[doc = "No Description"]
pub mod clken0;
#[doc = "CLKEN1 (rw) register accessor: an alias for `Reg<CLKEN1_SPEC>`"]
pub type CLKEN1 = crate::Reg<clken1::CLKEN1_SPEC>;
#[doc = "No Description"]
pub mod clken1;
#[doc = "SYSCLKCTRL (rw) register accessor: an alias for `Reg<SYSCLKCTRL_SPEC>`"]
pub type SYSCLKCTRL = crate::Reg<sysclkctrl::SYSCLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod sysclkctrl;
#[doc = "TRACECLKCTRL (rw) register accessor: an alias for `Reg<TRACECLKCTRL_SPEC>`"]
pub type TRACECLKCTRL = crate::Reg<traceclkctrl::TRACECLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod traceclkctrl;
#[doc = "EXPORTCLKCTRL (rw) register accessor: an alias for `Reg<EXPORTCLKCTRL_SPEC>`"]
pub type EXPORTCLKCTRL = crate::Reg<exportclkctrl::EXPORTCLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod exportclkctrl;
#[doc = "DPLLREFCLKCTRL (rw) register accessor: an alias for `Reg<DPLLREFCLKCTRL_SPEC>`"]
pub type DPLLREFCLKCTRL = crate::Reg<dpllrefclkctrl::DPLLREFCLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod dpllrefclkctrl;
#[doc = "EM01GRPACLKCTRL (rw) register accessor: an alias for `Reg<EM01GRPACLKCTRL_SPEC>`"]
pub type EM01GRPACLKCTRL = crate::Reg<em01grpaclkctrl::EM01GRPACLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod em01grpaclkctrl;
#[doc = "EM01GRPBCLKCTRL (rw) register accessor: an alias for `Reg<EM01GRPBCLKCTRL_SPEC>`"]
pub type EM01GRPBCLKCTRL = crate::Reg<em01grpbclkctrl::EM01GRPBCLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod em01grpbclkctrl;
#[doc = "EM23GRPACLKCTRL (rw) register accessor: an alias for `Reg<EM23GRPACLKCTRL_SPEC>`"]
pub type EM23GRPACLKCTRL = crate::Reg<em23grpaclkctrl::EM23GRPACLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod em23grpaclkctrl;
#[doc = "EM4GRPACLKCTRL (rw) register accessor: an alias for `Reg<EM4GRPACLKCTRL_SPEC>`"]
pub type EM4GRPACLKCTRL = crate::Reg<em4grpaclkctrl::EM4GRPACLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod em4grpaclkctrl;
#[doc = "IADCCLKCTRL (rw) register accessor: an alias for `Reg<IADCCLKCTRL_SPEC>`"]
pub type IADCCLKCTRL = crate::Reg<iadcclkctrl::IADCCLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod iadcclkctrl;
#[doc = "WDOG0CLKCTRL (rw) register accessor: an alias for `Reg<WDOG0CLKCTRL_SPEC>`"]
pub type WDOG0CLKCTRL = crate::Reg<wdog0clkctrl::WDOG0CLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod wdog0clkctrl;
#[doc = "EUART0CLKCTRL (rw) register accessor: an alias for `Reg<EUART0CLKCTRL_SPEC>`"]
pub type EUART0CLKCTRL = crate::Reg<euart0clkctrl::EUART0CLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod euart0clkctrl;
#[doc = "RTCCCLKCTRL (rw) register accessor: an alias for `Reg<RTCCCLKCTRL_SPEC>`"]
pub type RTCCCLKCTRL = crate::Reg<rtccclkctrl::RTCCCLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod rtccclkctrl;
#[doc = "CRYPTOACCCLKCTRL (rw) register accessor: an alias for `Reg<CRYPTOACCCLKCTRL_SPEC>`"]
pub type CRYPTOACCCLKCTRL = crate::Reg<cryptoaccclkctrl::CRYPTOACCCLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod cryptoaccclkctrl;
