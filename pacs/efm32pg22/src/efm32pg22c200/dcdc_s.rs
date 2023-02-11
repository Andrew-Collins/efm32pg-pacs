#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IPVERSION"]
    pub ipversion: IPVERSION,
    #[doc = "0x04 - Enable"]
    pub en: EN,
    #[doc = "0x08 - Control"]
    pub ctrl: CTRL,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - EM01 Configurations"]
    pub em01ctrl0: EM01CTRL0,
    #[doc = "0x14 - EM23 Configurations"]
    pub em23ctrl0: EM23CTRL0,
    _reserved5: [u8; 0x0c],
    #[doc = "0x24 - Interrupt Flags"]
    pub if_: IF,
    #[doc = "0x28 - Interrupt Enable"]
    pub ien: IEN,
    #[doc = "0x2c - DCDC Status Register"]
    pub status: STATUS,
    _reserved8: [u8; 0x10],
    #[doc = "0x40 - No Description"]
    pub lock: LOCK,
    #[doc = "0x44 - No Description"]
    pub lockstatus: LOCKSTATUS,
}
#[doc = "IPVERSION (r) register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "IPVERSION"]
pub mod ipversion;
#[doc = "EN (rw) register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "Enable"]
pub mod en;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "EM01CTRL0 (rw) register accessor: an alias for `Reg<EM01CTRL0_SPEC>`"]
pub type EM01CTRL0 = crate::Reg<em01ctrl0::EM01CTRL0_SPEC>;
#[doc = "EM01 Configurations"]
pub mod em01ctrl0;
#[doc = "EM23CTRL0 (rw) register accessor: an alias for `Reg<EM23CTRL0_SPEC>`"]
pub type EM23CTRL0 = crate::Reg<em23ctrl0::EM23CTRL0_SPEC>;
#[doc = "EM23 Configurations"]
pub mod em23ctrl0;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt Flags"]
pub mod if_;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt Enable"]
pub mod ien;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "DCDC Status Register"]
pub mod status;
#[doc = "LOCK (w) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "No Description"]
pub mod lock;
#[doc = "LOCKSTATUS (r) register accessor: an alias for `Reg<LOCKSTATUS_SPEC>`"]
pub type LOCKSTATUS = crate::Reg<lockstatus::LOCKSTATUS_SPEC>;
#[doc = "No Description"]
pub mod lockstatus;
