#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub ipversion: IPVERSION,
    #[doc = "0x04 - No Description"]
    pub en: EN,
    #[doc = "0x08 - No Description"]
    pub cfg: CFG,
    #[doc = "0x0c - No Description"]
    pub cmd: CMD,
    #[doc = "0x10 - No Description"]
    pub status: STATUS,
    #[doc = "0x14 - No Description"]
    pub if_: IF,
    #[doc = "0x18 - No Description"]
    pub ien: IEN,
    #[doc = "0x1c - No Description"]
    pub precnt: PRECNT,
    #[doc = "0x20 - No Description"]
    pub cnt: CNT,
    #[doc = "0x24 - No Description"]
    pub combcnt: COMBCNT,
    #[doc = "0x28 - No Description"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x2c - No Description"]
    pub lock: LOCK,
    #[doc = "0x30 - No Description"]
    pub cc0_ctrl: CC0_CTRL,
    #[doc = "0x34 - No Description"]
    pub cc0_ocvalue: CC0_OCVALUE,
    #[doc = "0x38 - No Description"]
    pub cc0_icvalue: CC0_ICVALUE,
    #[doc = "0x3c - No Description"]
    pub cc1_ctrl: CC1_CTRL,
    #[doc = "0x40 - No Description"]
    pub cc1_ocvalue: CC1_OCVALUE,
    #[doc = "0x44 - No Description"]
    pub cc1_icvalue: CC1_ICVALUE,
    #[doc = "0x48 - No Description"]
    pub cc2_ctrl: CC2_CTRL,
    #[doc = "0x4c - No Description"]
    pub cc2_ocvalue: CC2_OCVALUE,
    #[doc = "0x50 - No Description"]
    pub cc2_icvalue: CC2_ICVALUE,
}
#[doc = "IPVERSION (r) register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "EN (rw) register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "No Description"]
pub mod en;
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "No Description"]
pub mod cfg;
#[doc = "CMD (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "No Description"]
pub mod cmd;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "No Description"]
pub mod status;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "No Description"]
pub mod if_;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "No Description"]
pub mod ien;
#[doc = "PRECNT (rw) register accessor: an alias for `Reg<PRECNT_SPEC>`"]
pub type PRECNT = crate::Reg<precnt::PRECNT_SPEC>;
#[doc = "No Description"]
pub mod precnt;
#[doc = "CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "No Description"]
pub mod cnt;
#[doc = "COMBCNT (r) register accessor: an alias for `Reg<COMBCNT_SPEC>`"]
pub type COMBCNT = crate::Reg<combcnt::COMBCNT_SPEC>;
#[doc = "No Description"]
pub mod combcnt;
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "No Description"]
pub mod syncbusy;
#[doc = "LOCK (w) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "No Description"]
pub mod lock;
#[doc = "CC0_CTRL (rw) register accessor: an alias for `Reg<CC0_CTRL_SPEC>`"]
pub type CC0_CTRL = crate::Reg<cc0_ctrl::CC0_CTRL_SPEC>;
#[doc = "No Description"]
pub mod cc0_ctrl;
#[doc = "CC0_OCVALUE (rw) register accessor: an alias for `Reg<CC0_OCVALUE_SPEC>`"]
pub type CC0_OCVALUE = crate::Reg<cc0_ocvalue::CC0_OCVALUE_SPEC>;
#[doc = "No Description"]
pub mod cc0_ocvalue;
#[doc = "CC0_ICVALUE (r) register accessor: an alias for `Reg<CC0_ICVALUE_SPEC>`"]
pub type CC0_ICVALUE = crate::Reg<cc0_icvalue::CC0_ICVALUE_SPEC>;
#[doc = "No Description"]
pub mod cc0_icvalue;
#[doc = "CC1_CTRL (rw) register accessor: an alias for `Reg<CC1_CTRL_SPEC>`"]
pub type CC1_CTRL = crate::Reg<cc1_ctrl::CC1_CTRL_SPEC>;
#[doc = "No Description"]
pub mod cc1_ctrl;
#[doc = "CC1_OCVALUE (rw) register accessor: an alias for `Reg<CC1_OCVALUE_SPEC>`"]
pub type CC1_OCVALUE = crate::Reg<cc1_ocvalue::CC1_OCVALUE_SPEC>;
#[doc = "No Description"]
pub mod cc1_ocvalue;
#[doc = "CC1_ICVALUE (r) register accessor: an alias for `Reg<CC1_ICVALUE_SPEC>`"]
pub type CC1_ICVALUE = crate::Reg<cc1_icvalue::CC1_ICVALUE_SPEC>;
#[doc = "No Description"]
pub mod cc1_icvalue;
#[doc = "CC2_CTRL (rw) register accessor: an alias for `Reg<CC2_CTRL_SPEC>`"]
pub type CC2_CTRL = crate::Reg<cc2_ctrl::CC2_CTRL_SPEC>;
#[doc = "No Description"]
pub mod cc2_ctrl;
#[doc = "CC2_OCVALUE (rw) register accessor: an alias for `Reg<CC2_OCVALUE_SPEC>`"]
pub type CC2_OCVALUE = crate::Reg<cc2_ocvalue::CC2_OCVALUE_SPEC>;
#[doc = "No Description"]
pub mod cc2_ocvalue;
#[doc = "CC2_ICVALUE (r) register accessor: an alias for `Reg<CC2_ICVALUE_SPEC>`"]
pub type CC2_ICVALUE = crate::Reg<cc2_icvalue::CC2_ICVALUE_SPEC>;
#[doc = "No Description"]
pub mod cc2_icvalue;
