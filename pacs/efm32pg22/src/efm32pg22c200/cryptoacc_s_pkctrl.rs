#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub pointer: POINTER,
    #[doc = "0x04 - No Description"]
    pub command: COMMAND,
    #[doc = "0x08 - No Description"]
    pub pkctrl: PKCTRL,
    #[doc = "0x0c - No Description"]
    pub pkstatus: PKSTATUS,
    #[doc = "0x10 - No Description"]
    pub version: VERSION,
    #[doc = "0x14 - No Description"]
    pub timer: TIMER,
}
#[doc = "POINTER (rw) register accessor: an alias for `Reg<POINTER_SPEC>`"]
pub type POINTER = crate::Reg<pointer::POINTER_SPEC>;
#[doc = "No Description"]
pub mod pointer;
#[doc = "COMMAND (rw) register accessor: an alias for `Reg<COMMAND_SPEC>`"]
pub type COMMAND = crate::Reg<command::COMMAND_SPEC>;
#[doc = "No Description"]
pub mod command;
#[doc = "PKCTRL (w) register accessor: an alias for `Reg<PKCTRL_SPEC>`"]
pub type PKCTRL = crate::Reg<pkctrl::PKCTRL_SPEC>;
#[doc = "No Description"]
pub mod pkctrl;
#[doc = "PKSTATUS (r) register accessor: an alias for `Reg<PKSTATUS_SPEC>`"]
pub type PKSTATUS = crate::Reg<pkstatus::PKSTATUS_SPEC>;
#[doc = "No Description"]
pub mod pkstatus;
#[doc = "VERSION (r) register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "No Description"]
pub mod version;
#[doc = "TIMER (r) register accessor: an alias for `Reg<TIMER_SPEC>`"]
pub type TIMER = crate::Reg<timer::TIMER_SPEC>;
#[doc = "No Description"]
pub mod timer;
