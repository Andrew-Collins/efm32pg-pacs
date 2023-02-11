#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port control"]
    pub porta_ctrl: PORTA_CTRL,
    #[doc = "0x04 - mode low"]
    pub porta_model: PORTA_MODEL,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - mode high"]
    pub porta_modeh: PORTA_MODEH,
    #[doc = "0x10 - data out"]
    pub porta_dout: PORTA_DOUT,
    #[doc = "0x14 - data in"]
    pub porta_din: PORTA_DIN,
    _reserved5: [u8; 0x18],
    #[doc = "0x30 - Port control"]
    pub portb_ctrl: PORTB_CTRL,
    #[doc = "0x34 - mode low"]
    pub portb_model: PORTB_MODEL,
    _reserved7: [u8; 0x08],
    #[doc = "0x40 - data out"]
    pub portb_dout: PORTB_DOUT,
    #[doc = "0x44 - data in"]
    pub portb_din: PORTB_DIN,
    _reserved9: [u8; 0x18],
    #[doc = "0x60 - Port control"]
    pub portc_ctrl: PORTC_CTRL,
    #[doc = "0x64 - mode low"]
    pub portc_model: PORTC_MODEL,
    _reserved11: [u8; 0x08],
    #[doc = "0x70 - data out"]
    pub portc_dout: PORTC_DOUT,
    #[doc = "0x74 - data in"]
    pub portc_din: PORTC_DIN,
    _reserved13: [u8; 0x18],
    #[doc = "0x90 - Port control"]
    pub portd_ctrl: PORTD_CTRL,
    #[doc = "0x94 - mode low"]
    pub portd_model: PORTD_MODEL,
    _reserved15: [u8; 0x08],
    #[doc = "0xa0 - data out"]
    pub portd_dout: PORTD_DOUT,
    #[doc = "0xa4 - data in"]
    pub portd_din: PORTD_DIN,
    _reserved17: [u8; 0x0258],
    #[doc = "0x300 - No Description"]
    pub lock: LOCK,
    _reserved18: [u8; 0x0c],
    #[doc = "0x310 - No Description"]
    pub gpiolockstatus: GPIOLOCKSTATUS,
    _reserved19: [u8; 0x0c],
    #[doc = "0x320 - A Bus allocation"]
    pub abusalloc: ABUSALLOC,
    #[doc = "0x324 - B Bus allocation"]
    pub bbusalloc: BBUSALLOC,
    #[doc = "0x328 - CD Bus allocation"]
    pub cdbusalloc: CDBUSALLOC,
    _reserved22: [u8; 0xd4],
    #[doc = "0x400 - External Interrupt Port Select Low"]
    pub extipsell: EXTIPSELL,
    #[doc = "0x404 - External interrupt Port Select High"]
    pub extipselh: EXTIPSELH,
    #[doc = "0x408 - External Interrupt Pin Select Low"]
    pub extipinsell: EXTIPINSELL,
    #[doc = "0x40c - External Interrupt Pin Select High"]
    pub extipinselh: EXTIPINSELH,
    #[doc = "0x410 - External Interrupt Rising Edge Trigger"]
    pub extirise: EXTIRISE,
    #[doc = "0x414 - External Interrupt Falling Edge Trigger"]
    pub extifall: EXTIFALL,
    _reserved28: [u8; 0x08],
    #[doc = "0x420 - Interrupt Flag"]
    pub if_: IF,
    #[doc = "0x424 - Interrupt Enable"]
    pub ien: IEN,
    _reserved30: [u8; 0x04],
    #[doc = "0x42c - No Description"]
    pub em4wuen: EM4WUEN,
    #[doc = "0x430 - No Description"]
    pub em4wupol: EM4WUPOL,
    _reserved32: [u8; 0x0c],
    #[doc = "0x440 - No Description"]
    pub dbgroutepen: DBGROUTEPEN,
    #[doc = "0x444 - No Description"]
    pub traceroutepen: TRACEROUTEPEN,
    _reserved34: [u8; 0x08],
    #[doc = "0x450 - CMU pin enable"]
    pub cmu_routeen: CMU_ROUTEEN,
    #[doc = "0x454 - CLKIN0 port/pin select"]
    pub cmu_clkin0route: CMU_CLKIN0ROUTE,
    #[doc = "0x458 - CLKOUT0 port/pin select"]
    pub cmu_clkout0route: CMU_CLKOUT0ROUTE,
    #[doc = "0x45c - CLKOUT1 port/pin select"]
    pub cmu_clkout1route: CMU_CLKOUT1ROUTE,
    #[doc = "0x460 - CLKOUT2 port/pin select"]
    pub cmu_clkout2route: CMU_CLKOUT2ROUTE,
    _reserved39: [u8; 0x08],
    #[doc = "0x46c - DCDC pin enable"]
    pub dcdc_routeen: DCDC_ROUTEEN,
    _reserved40: [u8; 0x20],
    #[doc = "0x490 - I2C0 pin enable"]
    pub i2c0_routeen: I2C0_ROUTEEN,
    #[doc = "0x494 - SCL port/pin select"]
    pub i2c0_sclroute: I2C0_SCLROUTE,
    #[doc = "0x498 - SDA port/pin select"]
    pub i2c0_sdaroute: I2C0_SDAROUTE,
    _reserved43: [u8; 0x04],
    #[doc = "0x4a0 - I2C1 pin enable"]
    pub i2c1_routeen: I2C1_ROUTEEN,
    #[doc = "0x4a4 - SCL port/pin select"]
    pub i2c1_sclroute: I2C1_SCLROUTE,
    #[doc = "0x4a8 - SDA port/pin select"]
    pub i2c1_sdaroute: I2C1_SDAROUTE,
    _reserved46: [u8; 0x04],
    #[doc = "0x4b0 - LETIMER pin enable"]
    pub letimer0_routeen: LETIMER0_ROUTEEN,
    #[doc = "0x4b4 - OUT0 port/pin select"]
    pub letimer0_out0route: LETIMER0_OUT0ROUTE,
    #[doc = "0x4b8 - OUT1 port/pin select"]
    pub letimer0_out1route: LETIMER0_OUT1ROUTE,
    _reserved49: [u8; 0x04],
    #[doc = "0x4c0 - EUART pin enable"]
    pub euart0_routeen: EUART0_ROUTEEN,
    #[doc = "0x4c4 - CTS port/pin select"]
    pub euart0_ctsroute: EUART0_CTSROUTE,
    #[doc = "0x4c8 - RTS port/pin select"]
    pub euart0_rtsroute: EUART0_RTSROUTE,
    #[doc = "0x4cc - RX port/pin select"]
    pub euart0_rxroute: EUART0_RXROUTE,
    #[doc = "0x4d0 - TX port/pin select"]
    pub euart0_txroute: EUART0_TXROUTE,
    _reserved54: [u8; 0x4c],
    #[doc = "0x520 - PDM pin enable"]
    pub pdm_routeen: PDM_ROUTEEN,
    #[doc = "0x524 - CLK port/pin select"]
    pub pdm_clkroute: PDM_CLKROUTE,
    #[doc = "0x528 - DAT0 port/pin select"]
    pub pdm_dat0route: PDM_DAT0ROUTE,
    #[doc = "0x52c - DAT1 port/pin select"]
    pub pdm_dat1route: PDM_DAT1ROUTE,
    _reserved58: [u8; 0x04],
    #[doc = "0x534 - PRS0 pin enable"]
    pub prs0_routeen: PRS0_ROUTEEN,
    #[doc = "0x538 - ASYNCH0 port/pin select"]
    pub prs0_asynch0route: PRS0_ASYNCH0ROUTE,
    #[doc = "0x53c - ASYNCH1 port/pin select"]
    pub prs0_asynch1route: PRS0_ASYNCH1ROUTE,
    #[doc = "0x540 - ASYNCH2 port/pin select"]
    pub prs0_asynch2route: PRS0_ASYNCH2ROUTE,
    #[doc = "0x544 - ASYNCH3 port/pin select"]
    pub prs0_asynch3route: PRS0_ASYNCH3ROUTE,
    #[doc = "0x548 - ASYNCH4 port/pin select"]
    pub prs0_asynch4route: PRS0_ASYNCH4ROUTE,
    #[doc = "0x54c - ASYNCH5 port/pin select"]
    pub prs0_asynch5route: PRS0_ASYNCH5ROUTE,
    #[doc = "0x550 - ASYNCH6 port/pin select"]
    pub prs0_asynch6route: PRS0_ASYNCH6ROUTE,
    #[doc = "0x554 - ASYNCH7 port/pin select"]
    pub prs0_asynch7route: PRS0_ASYNCH7ROUTE,
    #[doc = "0x558 - ASYNCH8 port/pin select"]
    pub prs0_asynch8route: PRS0_ASYNCH8ROUTE,
    #[doc = "0x55c - ASYNCH9 port/pin select"]
    pub prs0_asynch9route: PRS0_ASYNCH9ROUTE,
    #[doc = "0x560 - ASYNCH10 port/pin select"]
    pub prs0_asynch10route: PRS0_ASYNCH10ROUTE,
    #[doc = "0x564 - ASYNCH11 port/pin select"]
    pub prs0_asynch11route: PRS0_ASYNCH11ROUTE,
    #[doc = "0x568 - SYNCH0 port/pin select"]
    pub prs0_synch0route: PRS0_SYNCH0ROUTE,
    #[doc = "0x56c - SYNCH1 port/pin select"]
    pub prs0_synch1route: PRS0_SYNCH1ROUTE,
    #[doc = "0x570 - SYNCH2 port/pin select"]
    pub prs0_synch2route: PRS0_SYNCH2ROUTE,
    #[doc = "0x574 - SYNCH3 port/pin select"]
    pub prs0_synch3route: PRS0_SYNCH3ROUTE,
    _reserved75: [u8; 0x04],
    #[doc = "0x57c - TIMER0 pin enable"]
    pub timer0_routeen: TIMER0_ROUTEEN,
    #[doc = "0x580 - CC0 port/pin select"]
    pub timer0_cc0route: TIMER0_CC0ROUTE,
    #[doc = "0x584 - CC1 port/pin select"]
    pub timer0_cc1route: TIMER0_CC1ROUTE,
    #[doc = "0x588 - CC2 port/pin select"]
    pub timer0_cc2route: TIMER0_CC2ROUTE,
    #[doc = "0x58c - CDTI0 port/pin select"]
    pub timer0_cdti0route: TIMER0_CDTI0ROUTE,
    #[doc = "0x590 - CDTI1 port/pin select"]
    pub timer0_cdti1route: TIMER0_CDTI1ROUTE,
    #[doc = "0x594 - CDTI2 port/pin select"]
    pub timer0_cdti2route: TIMER0_CDTI2ROUTE,
    _reserved82: [u8; 0x04],
    #[doc = "0x59c - TIMER1 pin enable"]
    pub timer1_routeen: TIMER1_ROUTEEN,
    #[doc = "0x5a0 - CC0 port/pin select"]
    pub timer1_cc0route: TIMER1_CC0ROUTE,
    #[doc = "0x5a4 - CC1 port/pin select"]
    pub timer1_cc1route: TIMER1_CC1ROUTE,
    #[doc = "0x5a8 - CC2 port/pin select"]
    pub timer1_cc2route: TIMER1_CC2ROUTE,
    #[doc = "0x5ac - CDTI0 port/pin select"]
    pub timer1_cdti0route: TIMER1_CDTI0ROUTE,
    #[doc = "0x5b0 - CDTI1 port/pin select"]
    pub timer1_cdti1route: TIMER1_CDTI1ROUTE,
    #[doc = "0x5b4 - CDTI2 port/pin select"]
    pub timer1_cdti2route: TIMER1_CDTI2ROUTE,
    _reserved89: [u8; 0x04],
    #[doc = "0x5bc - TIMER2 pin enable"]
    pub timer2_routeen: TIMER2_ROUTEEN,
    #[doc = "0x5c0 - CC0 port/pin select"]
    pub timer2_cc0route: TIMER2_CC0ROUTE,
    #[doc = "0x5c4 - CC1 port/pin select"]
    pub timer2_cc1route: TIMER2_CC1ROUTE,
    #[doc = "0x5c8 - CC2 port/pin select"]
    pub timer2_cc2route: TIMER2_CC2ROUTE,
    #[doc = "0x5cc - CDTI0 port/pin select"]
    pub timer2_cdti0route: TIMER2_CDTI0ROUTE,
    #[doc = "0x5d0 - CDTI1 port/pin select"]
    pub timer2_cdti1route: TIMER2_CDTI1ROUTE,
    #[doc = "0x5d4 - CDTI2 port/pin select"]
    pub timer2_cdti2route: TIMER2_CDTI2ROUTE,
    _reserved96: [u8; 0x04],
    #[doc = "0x5dc - TIMER3 pin enable"]
    pub timer3_routeen: TIMER3_ROUTEEN,
    #[doc = "0x5e0 - CC0 port/pin select"]
    pub timer3_cc0route: TIMER3_CC0ROUTE,
    #[doc = "0x5e4 - CC1 port/pin select"]
    pub timer3_cc1route: TIMER3_CC1ROUTE,
    #[doc = "0x5e8 - CC2 port/pin select"]
    pub timer3_cc2route: TIMER3_CC2ROUTE,
    #[doc = "0x5ec - CDTI0 port/pin select"]
    pub timer3_cdti0route: TIMER3_CDTI0ROUTE,
    #[doc = "0x5f0 - CDTI1 port/pin select"]
    pub timer3_cdti1route: TIMER3_CDTI1ROUTE,
    #[doc = "0x5f4 - CDTI2 port/pin select"]
    pub timer3_cdti2route: TIMER3_CDTI2ROUTE,
    _reserved103: [u8; 0x04],
    #[doc = "0x5fc - TIMER4 pin enable"]
    pub timer4_routeen: TIMER4_ROUTEEN,
    #[doc = "0x600 - CC0 port/pin select"]
    pub timer4_cc0route: TIMER4_CC0ROUTE,
    #[doc = "0x604 - CC1 port/pin select"]
    pub timer4_cc1route: TIMER4_CC1ROUTE,
    #[doc = "0x608 - CC2 port/pin select"]
    pub timer4_cc2route: TIMER4_CC2ROUTE,
    #[doc = "0x60c - CDTI0 port/pin select"]
    pub timer4_cdti0route: TIMER4_CDTI0ROUTE,
    #[doc = "0x610 - CDTI1 port/pin select"]
    pub timer4_cdti1route: TIMER4_CDTI1ROUTE,
    #[doc = "0x614 - CDTI2 port/pin select"]
    pub timer4_cdti2route: TIMER4_CDTI2ROUTE,
    _reserved110: [u8; 0x04],
    #[doc = "0x61c - USART0 pin enable"]
    pub usart0_routeen: USART0_ROUTEEN,
    #[doc = "0x620 - CS port/pin select"]
    pub usart0_csroute: USART0_CSROUTE,
    #[doc = "0x624 - CTS port/pin select"]
    pub usart0_ctsroute: USART0_CTSROUTE,
    #[doc = "0x628 - RTS port/pin select"]
    pub usart0_rtsroute: USART0_RTSROUTE,
    #[doc = "0x62c - RX port/pin select"]
    pub usart0_rxroute: USART0_RXROUTE,
    #[doc = "0x630 - SCLK port/pin select"]
    pub usart0_clkroute: USART0_CLKROUTE,
    #[doc = "0x634 - TX port/pin select"]
    pub usart0_txroute: USART0_TXROUTE,
    _reserved117: [u8; 0x04],
    #[doc = "0x63c - USART1 pin enable"]
    pub usart1_routeen: USART1_ROUTEEN,
    #[doc = "0x640 - CS port/pin select"]
    pub usart1_csroute: USART1_CSROUTE,
    #[doc = "0x644 - CTS port/pin select"]
    pub usart1_ctsroute: USART1_CTSROUTE,
    #[doc = "0x648 - RTS port/pin select"]
    pub usart1_rtsroute: USART1_RTSROUTE,
    #[doc = "0x64c - RX port/pin select"]
    pub usart1_rxroute: USART1_RXROUTE,
    #[doc = "0x650 - SCLK port/pin select"]
    pub usart1_clkroute: USART1_CLKROUTE,
    #[doc = "0x654 - TX port/pin select"]
    pub usart1_txroute: USART1_TXROUTE,
}
#[doc = "PORTA_CTRL (rw) register accessor: an alias for `Reg<PORTA_CTRL_SPEC>`"]
pub type PORTA_CTRL = crate::Reg<porta_ctrl::PORTA_CTRL_SPEC>;
#[doc = "Port control"]
pub mod porta_ctrl;
#[doc = "PORTA_MODEL (rw) register accessor: an alias for `Reg<PORTA_MODEL_SPEC>`"]
pub type PORTA_MODEL = crate::Reg<porta_model::PORTA_MODEL_SPEC>;
#[doc = "mode low"]
pub mod porta_model;
#[doc = "PORTA_MODEH (rw) register accessor: an alias for `Reg<PORTA_MODEH_SPEC>`"]
pub type PORTA_MODEH = crate::Reg<porta_modeh::PORTA_MODEH_SPEC>;
#[doc = "mode high"]
pub mod porta_modeh;
#[doc = "PORTA_DOUT (rw) register accessor: an alias for `Reg<PORTA_DOUT_SPEC>`"]
pub type PORTA_DOUT = crate::Reg<porta_dout::PORTA_DOUT_SPEC>;
#[doc = "data out"]
pub mod porta_dout;
#[doc = "PORTA_DIN (r) register accessor: an alias for `Reg<PORTA_DIN_SPEC>`"]
pub type PORTA_DIN = crate::Reg<porta_din::PORTA_DIN_SPEC>;
#[doc = "data in"]
pub mod porta_din;
#[doc = "PORTB_CTRL (rw) register accessor: an alias for `Reg<PORTB_CTRL_SPEC>`"]
pub type PORTB_CTRL = crate::Reg<portb_ctrl::PORTB_CTRL_SPEC>;
#[doc = "Port control"]
pub mod portb_ctrl;
#[doc = "PORTB_MODEL (rw) register accessor: an alias for `Reg<PORTB_MODEL_SPEC>`"]
pub type PORTB_MODEL = crate::Reg<portb_model::PORTB_MODEL_SPEC>;
#[doc = "mode low"]
pub mod portb_model;
#[doc = "PORTB_DOUT (rw) register accessor: an alias for `Reg<PORTB_DOUT_SPEC>`"]
pub type PORTB_DOUT = crate::Reg<portb_dout::PORTB_DOUT_SPEC>;
#[doc = "data out"]
pub mod portb_dout;
#[doc = "PORTB_DIN (r) register accessor: an alias for `Reg<PORTB_DIN_SPEC>`"]
pub type PORTB_DIN = crate::Reg<portb_din::PORTB_DIN_SPEC>;
#[doc = "data in"]
pub mod portb_din;
#[doc = "PORTC_CTRL (rw) register accessor: an alias for `Reg<PORTC_CTRL_SPEC>`"]
pub type PORTC_CTRL = crate::Reg<portc_ctrl::PORTC_CTRL_SPEC>;
#[doc = "Port control"]
pub mod portc_ctrl;
#[doc = "PORTC_MODEL (rw) register accessor: an alias for `Reg<PORTC_MODEL_SPEC>`"]
pub type PORTC_MODEL = crate::Reg<portc_model::PORTC_MODEL_SPEC>;
#[doc = "mode low"]
pub mod portc_model;
#[doc = "PORTC_DOUT (rw) register accessor: an alias for `Reg<PORTC_DOUT_SPEC>`"]
pub type PORTC_DOUT = crate::Reg<portc_dout::PORTC_DOUT_SPEC>;
#[doc = "data out"]
pub mod portc_dout;
#[doc = "PORTC_DIN (r) register accessor: an alias for `Reg<PORTC_DIN_SPEC>`"]
pub type PORTC_DIN = crate::Reg<portc_din::PORTC_DIN_SPEC>;
#[doc = "data in"]
pub mod portc_din;
#[doc = "PORTD_CTRL (rw) register accessor: an alias for `Reg<PORTD_CTRL_SPEC>`"]
pub type PORTD_CTRL = crate::Reg<portd_ctrl::PORTD_CTRL_SPEC>;
#[doc = "Port control"]
pub mod portd_ctrl;
#[doc = "PORTD_MODEL (rw) register accessor: an alias for `Reg<PORTD_MODEL_SPEC>`"]
pub type PORTD_MODEL = crate::Reg<portd_model::PORTD_MODEL_SPEC>;
#[doc = "mode low"]
pub mod portd_model;
#[doc = "PORTD_DOUT (rw) register accessor: an alias for `Reg<PORTD_DOUT_SPEC>`"]
pub type PORTD_DOUT = crate::Reg<portd_dout::PORTD_DOUT_SPEC>;
#[doc = "data out"]
pub mod portd_dout;
#[doc = "PORTD_DIN (r) register accessor: an alias for `Reg<PORTD_DIN_SPEC>`"]
pub type PORTD_DIN = crate::Reg<portd_din::PORTD_DIN_SPEC>;
#[doc = "data in"]
pub mod portd_din;
#[doc = "LOCK (w) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "No Description"]
pub mod lock;
#[doc = "GPIOLOCKSTATUS (r) register accessor: an alias for `Reg<GPIOLOCKSTATUS_SPEC>`"]
pub type GPIOLOCKSTATUS = crate::Reg<gpiolockstatus::GPIOLOCKSTATUS_SPEC>;
#[doc = "No Description"]
pub mod gpiolockstatus;
#[doc = "ABUSALLOC (rw) register accessor: an alias for `Reg<ABUSALLOC_SPEC>`"]
pub type ABUSALLOC = crate::Reg<abusalloc::ABUSALLOC_SPEC>;
#[doc = "A Bus allocation"]
pub mod abusalloc;
#[doc = "BBUSALLOC (rw) register accessor: an alias for `Reg<BBUSALLOC_SPEC>`"]
pub type BBUSALLOC = crate::Reg<bbusalloc::BBUSALLOC_SPEC>;
#[doc = "B Bus allocation"]
pub mod bbusalloc;
#[doc = "CDBUSALLOC (rw) register accessor: an alias for `Reg<CDBUSALLOC_SPEC>`"]
pub type CDBUSALLOC = crate::Reg<cdbusalloc::CDBUSALLOC_SPEC>;
#[doc = "CD Bus allocation"]
pub mod cdbusalloc;
#[doc = "EXTIPSELL (rw) register accessor: an alias for `Reg<EXTIPSELL_SPEC>`"]
pub type EXTIPSELL = crate::Reg<extipsell::EXTIPSELL_SPEC>;
#[doc = "External Interrupt Port Select Low"]
pub mod extipsell;
#[doc = "EXTIPSELH (rw) register accessor: an alias for `Reg<EXTIPSELH_SPEC>`"]
pub type EXTIPSELH = crate::Reg<extipselh::EXTIPSELH_SPEC>;
#[doc = "External interrupt Port Select High"]
pub mod extipselh;
#[doc = "EXTIPINSELL (rw) register accessor: an alias for `Reg<EXTIPINSELL_SPEC>`"]
pub type EXTIPINSELL = crate::Reg<extipinsell::EXTIPINSELL_SPEC>;
#[doc = "External Interrupt Pin Select Low"]
pub mod extipinsell;
#[doc = "EXTIPINSELH (rw) register accessor: an alias for `Reg<EXTIPINSELH_SPEC>`"]
pub type EXTIPINSELH = crate::Reg<extipinselh::EXTIPINSELH_SPEC>;
#[doc = "External Interrupt Pin Select High"]
pub mod extipinselh;
#[doc = "EXTIRISE (rw) register accessor: an alias for `Reg<EXTIRISE_SPEC>`"]
pub type EXTIRISE = crate::Reg<extirise::EXTIRISE_SPEC>;
#[doc = "External Interrupt Rising Edge Trigger"]
pub mod extirise;
#[doc = "EXTIFALL (rw) register accessor: an alias for `Reg<EXTIFALL_SPEC>`"]
pub type EXTIFALL = crate::Reg<extifall::EXTIFALL_SPEC>;
#[doc = "External Interrupt Falling Edge Trigger"]
pub mod extifall;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt Flag"]
pub mod if_;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt Enable"]
pub mod ien;
#[doc = "EM4WUEN (rw) register accessor: an alias for `Reg<EM4WUEN_SPEC>`"]
pub type EM4WUEN = crate::Reg<em4wuen::EM4WUEN_SPEC>;
#[doc = "No Description"]
pub mod em4wuen;
#[doc = "EM4WUPOL (rw) register accessor: an alias for `Reg<EM4WUPOL_SPEC>`"]
pub type EM4WUPOL = crate::Reg<em4wupol::EM4WUPOL_SPEC>;
#[doc = "No Description"]
pub mod em4wupol;
#[doc = "DBGROUTEPEN (rw) register accessor: an alias for `Reg<DBGROUTEPEN_SPEC>`"]
pub type DBGROUTEPEN = crate::Reg<dbgroutepen::DBGROUTEPEN_SPEC>;
#[doc = "No Description"]
pub mod dbgroutepen;
#[doc = "TRACEROUTEPEN (rw) register accessor: an alias for `Reg<TRACEROUTEPEN_SPEC>`"]
pub type TRACEROUTEPEN = crate::Reg<traceroutepen::TRACEROUTEPEN_SPEC>;
#[doc = "No Description"]
pub mod traceroutepen;
#[doc = "CMU_ROUTEEN (rw) register accessor: an alias for `Reg<CMU_ROUTEEN_SPEC>`"]
pub type CMU_ROUTEEN = crate::Reg<cmu_routeen::CMU_ROUTEEN_SPEC>;
#[doc = "CMU pin enable"]
pub mod cmu_routeen;
#[doc = "CMU_CLKIN0ROUTE (rw) register accessor: an alias for `Reg<CMU_CLKIN0ROUTE_SPEC>`"]
pub type CMU_CLKIN0ROUTE = crate::Reg<cmu_clkin0route::CMU_CLKIN0ROUTE_SPEC>;
#[doc = "CLKIN0 port/pin select"]
pub mod cmu_clkin0route;
#[doc = "CMU_CLKOUT0ROUTE (rw) register accessor: an alias for `Reg<CMU_CLKOUT0ROUTE_SPEC>`"]
pub type CMU_CLKOUT0ROUTE = crate::Reg<cmu_clkout0route::CMU_CLKOUT0ROUTE_SPEC>;
#[doc = "CLKOUT0 port/pin select"]
pub mod cmu_clkout0route;
#[doc = "CMU_CLKOUT1ROUTE (rw) register accessor: an alias for `Reg<CMU_CLKOUT1ROUTE_SPEC>`"]
pub type CMU_CLKOUT1ROUTE = crate::Reg<cmu_clkout1route::CMU_CLKOUT1ROUTE_SPEC>;
#[doc = "CLKOUT1 port/pin select"]
pub mod cmu_clkout1route;
#[doc = "CMU_CLKOUT2ROUTE (rw) register accessor: an alias for `Reg<CMU_CLKOUT2ROUTE_SPEC>`"]
pub type CMU_CLKOUT2ROUTE = crate::Reg<cmu_clkout2route::CMU_CLKOUT2ROUTE_SPEC>;
#[doc = "CLKOUT2 port/pin select"]
pub mod cmu_clkout2route;
#[doc = "DCDC_ROUTEEN (rw) register accessor: an alias for `Reg<DCDC_ROUTEEN_SPEC>`"]
pub type DCDC_ROUTEEN = crate::Reg<dcdc_routeen::DCDC_ROUTEEN_SPEC>;
#[doc = "DCDC pin enable"]
pub mod dcdc_routeen;
#[doc = "I2C0_ROUTEEN (rw) register accessor: an alias for `Reg<I2C0_ROUTEEN_SPEC>`"]
pub type I2C0_ROUTEEN = crate::Reg<i2c0_routeen::I2C0_ROUTEEN_SPEC>;
#[doc = "I2C0 pin enable"]
pub mod i2c0_routeen;
#[doc = "I2C0_SCLROUTE (rw) register accessor: an alias for `Reg<I2C0_SCLROUTE_SPEC>`"]
pub type I2C0_SCLROUTE = crate::Reg<i2c0_sclroute::I2C0_SCLROUTE_SPEC>;
#[doc = "SCL port/pin select"]
pub mod i2c0_sclroute;
#[doc = "I2C0_SDAROUTE (rw) register accessor: an alias for `Reg<I2C0_SDAROUTE_SPEC>`"]
pub type I2C0_SDAROUTE = crate::Reg<i2c0_sdaroute::I2C0_SDAROUTE_SPEC>;
#[doc = "SDA port/pin select"]
pub mod i2c0_sdaroute;
#[doc = "I2C1_ROUTEEN (rw) register accessor: an alias for `Reg<I2C1_ROUTEEN_SPEC>`"]
pub type I2C1_ROUTEEN = crate::Reg<i2c1_routeen::I2C1_ROUTEEN_SPEC>;
#[doc = "I2C1 pin enable"]
pub mod i2c1_routeen;
#[doc = "I2C1_SCLROUTE (rw) register accessor: an alias for `Reg<I2C1_SCLROUTE_SPEC>`"]
pub type I2C1_SCLROUTE = crate::Reg<i2c1_sclroute::I2C1_SCLROUTE_SPEC>;
#[doc = "SCL port/pin select"]
pub mod i2c1_sclroute;
#[doc = "I2C1_SDAROUTE (rw) register accessor: an alias for `Reg<I2C1_SDAROUTE_SPEC>`"]
pub type I2C1_SDAROUTE = crate::Reg<i2c1_sdaroute::I2C1_SDAROUTE_SPEC>;
#[doc = "SDA port/pin select"]
pub mod i2c1_sdaroute;
#[doc = "LETIMER0_ROUTEEN (rw) register accessor: an alias for `Reg<LETIMER0_ROUTEEN_SPEC>`"]
pub type LETIMER0_ROUTEEN = crate::Reg<letimer0_routeen::LETIMER0_ROUTEEN_SPEC>;
#[doc = "LETIMER pin enable"]
pub mod letimer0_routeen;
#[doc = "LETIMER0_OUT0ROUTE (rw) register accessor: an alias for `Reg<LETIMER0_OUT0ROUTE_SPEC>`"]
pub type LETIMER0_OUT0ROUTE = crate::Reg<letimer0_out0route::LETIMER0_OUT0ROUTE_SPEC>;
#[doc = "OUT0 port/pin select"]
pub mod letimer0_out0route;
#[doc = "LETIMER0_OUT1ROUTE (rw) register accessor: an alias for `Reg<LETIMER0_OUT1ROUTE_SPEC>`"]
pub type LETIMER0_OUT1ROUTE = crate::Reg<letimer0_out1route::LETIMER0_OUT1ROUTE_SPEC>;
#[doc = "OUT1 port/pin select"]
pub mod letimer0_out1route;
#[doc = "EUART0_ROUTEEN (rw) register accessor: an alias for `Reg<EUART0_ROUTEEN_SPEC>`"]
pub type EUART0_ROUTEEN = crate::Reg<euart0_routeen::EUART0_ROUTEEN_SPEC>;
#[doc = "EUART pin enable"]
pub mod euart0_routeen;
#[doc = "EUART0_CTSROUTE (rw) register accessor: an alias for `Reg<EUART0_CTSROUTE_SPEC>`"]
pub type EUART0_CTSROUTE = crate::Reg<euart0_ctsroute::EUART0_CTSROUTE_SPEC>;
#[doc = "CTS port/pin select"]
pub mod euart0_ctsroute;
#[doc = "EUART0_RTSROUTE (rw) register accessor: an alias for `Reg<EUART0_RTSROUTE_SPEC>`"]
pub type EUART0_RTSROUTE = crate::Reg<euart0_rtsroute::EUART0_RTSROUTE_SPEC>;
#[doc = "RTS port/pin select"]
pub mod euart0_rtsroute;
#[doc = "EUART0_RXROUTE (rw) register accessor: an alias for `Reg<EUART0_RXROUTE_SPEC>`"]
pub type EUART0_RXROUTE = crate::Reg<euart0_rxroute::EUART0_RXROUTE_SPEC>;
#[doc = "RX port/pin select"]
pub mod euart0_rxroute;
#[doc = "EUART0_TXROUTE (rw) register accessor: an alias for `Reg<EUART0_TXROUTE_SPEC>`"]
pub type EUART0_TXROUTE = crate::Reg<euart0_txroute::EUART0_TXROUTE_SPEC>;
#[doc = "TX port/pin select"]
pub mod euart0_txroute;
#[doc = "PDM_ROUTEEN (rw) register accessor: an alias for `Reg<PDM_ROUTEEN_SPEC>`"]
pub type PDM_ROUTEEN = crate::Reg<pdm_routeen::PDM_ROUTEEN_SPEC>;
#[doc = "PDM pin enable"]
pub mod pdm_routeen;
#[doc = "PDM_CLKROUTE (rw) register accessor: an alias for `Reg<PDM_CLKROUTE_SPEC>`"]
pub type PDM_CLKROUTE = crate::Reg<pdm_clkroute::PDM_CLKROUTE_SPEC>;
#[doc = "CLK port/pin select"]
pub mod pdm_clkroute;
#[doc = "PDM_DAT0ROUTE (rw) register accessor: an alias for `Reg<PDM_DAT0ROUTE_SPEC>`"]
pub type PDM_DAT0ROUTE = crate::Reg<pdm_dat0route::PDM_DAT0ROUTE_SPEC>;
#[doc = "DAT0 port/pin select"]
pub mod pdm_dat0route;
#[doc = "PDM_DAT1ROUTE (rw) register accessor: an alias for `Reg<PDM_DAT1ROUTE_SPEC>`"]
pub type PDM_DAT1ROUTE = crate::Reg<pdm_dat1route::PDM_DAT1ROUTE_SPEC>;
#[doc = "DAT1 port/pin select"]
pub mod pdm_dat1route;
#[doc = "PRS0_ROUTEEN (rw) register accessor: an alias for `Reg<PRS0_ROUTEEN_SPEC>`"]
pub type PRS0_ROUTEEN = crate::Reg<prs0_routeen::PRS0_ROUTEEN_SPEC>;
#[doc = "PRS0 pin enable"]
pub mod prs0_routeen;
#[doc = "PRS0_ASYNCH0ROUTE (rw) register accessor: an alias for `Reg<PRS0_ASYNCH0ROUTE_SPEC>`"]
pub type PRS0_ASYNCH0ROUTE = crate::Reg<prs0_asynch0route::PRS0_ASYNCH0ROUTE_SPEC>;
#[doc = "ASYNCH0 port/pin select"]
pub mod prs0_asynch0route;
#[doc = "PRS0_ASYNCH1ROUTE (rw) register accessor: an alias for `Reg<PRS0_ASYNCH1ROUTE_SPEC>`"]
pub type PRS0_ASYNCH1ROUTE = crate::Reg<prs0_asynch1route::PRS0_ASYNCH1ROUTE_SPEC>;
#[doc = "ASYNCH1 port/pin select"]
pub mod prs0_asynch1route;
#[doc = "PRS0_ASYNCH2ROUTE (rw) register accessor: an alias for `Reg<PRS0_ASYNCH2ROUTE_SPEC>`"]
pub type PRS0_ASYNCH2ROUTE = crate::Reg<prs0_asynch2route::PRS0_ASYNCH2ROUTE_SPEC>;
#[doc = "ASYNCH2 port/pin select"]
pub mod prs0_asynch2route;
#[doc = "PRS0_ASYNCH3ROUTE (rw) register accessor: an alias for `Reg<PRS0_ASYNCH3ROUTE_SPEC>`"]
pub type PRS0_ASYNCH3ROUTE = crate::Reg<prs0_asynch3route::PRS0_ASYNCH3ROUTE_SPEC>;
#[doc = "ASYNCH3 port/pin select"]
pub mod prs0_asynch3route;
#[doc = "PRS0_ASYNCH4ROUTE (rw) register accessor: an alias for `Reg<PRS0_ASYNCH4ROUTE_SPEC>`"]
pub type PRS0_ASYNCH4ROUTE = crate::Reg<prs0_asynch4route::PRS0_ASYNCH4ROUTE_SPEC>;
#[doc = "ASYNCH4 port/pin select"]
pub mod prs0_asynch4route;
#[doc = "PRS0_ASYNCH5ROUTE (rw) register accessor: an alias for `Reg<PRS0_ASYNCH5ROUTE_SPEC>`"]
pub type PRS0_ASYNCH5ROUTE = crate::Reg<prs0_asynch5route::PRS0_ASYNCH5ROUTE_SPEC>;
#[doc = "ASYNCH5 port/pin select"]
pub mod prs0_asynch5route;
#[doc = "PRS0_ASYNCH6ROUTE (rw) register accessor: an alias for `Reg<PRS0_ASYNCH6ROUTE_SPEC>`"]
pub type PRS0_ASYNCH6ROUTE = crate::Reg<prs0_asynch6route::PRS0_ASYNCH6ROUTE_SPEC>;
#[doc = "ASYNCH6 port/pin select"]
pub mod prs0_asynch6route;
#[doc = "PRS0_ASYNCH7ROUTE (rw) register accessor: an alias for `Reg<PRS0_ASYNCH7ROUTE_SPEC>`"]
pub type PRS0_ASYNCH7ROUTE = crate::Reg<prs0_asynch7route::PRS0_ASYNCH7ROUTE_SPEC>;
#[doc = "ASYNCH7 port/pin select"]
pub mod prs0_asynch7route;
#[doc = "PRS0_ASYNCH8ROUTE (rw) register accessor: an alias for `Reg<PRS0_ASYNCH8ROUTE_SPEC>`"]
pub type PRS0_ASYNCH8ROUTE = crate::Reg<prs0_asynch8route::PRS0_ASYNCH8ROUTE_SPEC>;
#[doc = "ASYNCH8 port/pin select"]
pub mod prs0_asynch8route;
#[doc = "PRS0_ASYNCH9ROUTE (rw) register accessor: an alias for `Reg<PRS0_ASYNCH9ROUTE_SPEC>`"]
pub type PRS0_ASYNCH9ROUTE = crate::Reg<prs0_asynch9route::PRS0_ASYNCH9ROUTE_SPEC>;
#[doc = "ASYNCH9 port/pin select"]
pub mod prs0_asynch9route;
#[doc = "PRS0_ASYNCH10ROUTE (rw) register accessor: an alias for `Reg<PRS0_ASYNCH10ROUTE_SPEC>`"]
pub type PRS0_ASYNCH10ROUTE = crate::Reg<prs0_asynch10route::PRS0_ASYNCH10ROUTE_SPEC>;
#[doc = "ASYNCH10 port/pin select"]
pub mod prs0_asynch10route;
#[doc = "PRS0_ASYNCH11ROUTE (rw) register accessor: an alias for `Reg<PRS0_ASYNCH11ROUTE_SPEC>`"]
pub type PRS0_ASYNCH11ROUTE = crate::Reg<prs0_asynch11route::PRS0_ASYNCH11ROUTE_SPEC>;
#[doc = "ASYNCH11 port/pin select"]
pub mod prs0_asynch11route;
#[doc = "PRS0_SYNCH0ROUTE (rw) register accessor: an alias for `Reg<PRS0_SYNCH0ROUTE_SPEC>`"]
pub type PRS0_SYNCH0ROUTE = crate::Reg<prs0_synch0route::PRS0_SYNCH0ROUTE_SPEC>;
#[doc = "SYNCH0 port/pin select"]
pub mod prs0_synch0route;
#[doc = "PRS0_SYNCH1ROUTE (rw) register accessor: an alias for `Reg<PRS0_SYNCH1ROUTE_SPEC>`"]
pub type PRS0_SYNCH1ROUTE = crate::Reg<prs0_synch1route::PRS0_SYNCH1ROUTE_SPEC>;
#[doc = "SYNCH1 port/pin select"]
pub mod prs0_synch1route;
#[doc = "PRS0_SYNCH2ROUTE (rw) register accessor: an alias for `Reg<PRS0_SYNCH2ROUTE_SPEC>`"]
pub type PRS0_SYNCH2ROUTE = crate::Reg<prs0_synch2route::PRS0_SYNCH2ROUTE_SPEC>;
#[doc = "SYNCH2 port/pin select"]
pub mod prs0_synch2route;
#[doc = "PRS0_SYNCH3ROUTE (rw) register accessor: an alias for `Reg<PRS0_SYNCH3ROUTE_SPEC>`"]
pub type PRS0_SYNCH3ROUTE = crate::Reg<prs0_synch3route::PRS0_SYNCH3ROUTE_SPEC>;
#[doc = "SYNCH3 port/pin select"]
pub mod prs0_synch3route;
#[doc = "TIMER0_ROUTEEN (rw) register accessor: an alias for `Reg<TIMER0_ROUTEEN_SPEC>`"]
pub type TIMER0_ROUTEEN = crate::Reg<timer0_routeen::TIMER0_ROUTEEN_SPEC>;
#[doc = "TIMER0 pin enable"]
pub mod timer0_routeen;
#[doc = "TIMER0_CC0ROUTE (rw) register accessor: an alias for `Reg<TIMER0_CC0ROUTE_SPEC>`"]
pub type TIMER0_CC0ROUTE = crate::Reg<timer0_cc0route::TIMER0_CC0ROUTE_SPEC>;
#[doc = "CC0 port/pin select"]
pub mod timer0_cc0route;
#[doc = "TIMER0_CC1ROUTE (rw) register accessor: an alias for `Reg<TIMER0_CC1ROUTE_SPEC>`"]
pub type TIMER0_CC1ROUTE = crate::Reg<timer0_cc1route::TIMER0_CC1ROUTE_SPEC>;
#[doc = "CC1 port/pin select"]
pub mod timer0_cc1route;
#[doc = "TIMER0_CC2ROUTE (rw) register accessor: an alias for `Reg<TIMER0_CC2ROUTE_SPEC>`"]
pub type TIMER0_CC2ROUTE = crate::Reg<timer0_cc2route::TIMER0_CC2ROUTE_SPEC>;
#[doc = "CC2 port/pin select"]
pub mod timer0_cc2route;
#[doc = "TIMER0_CDTI0ROUTE (rw) register accessor: an alias for `Reg<TIMER0_CDTI0ROUTE_SPEC>`"]
pub type TIMER0_CDTI0ROUTE = crate::Reg<timer0_cdti0route::TIMER0_CDTI0ROUTE_SPEC>;
#[doc = "CDTI0 port/pin select"]
pub mod timer0_cdti0route;
#[doc = "TIMER0_CDTI1ROUTE (rw) register accessor: an alias for `Reg<TIMER0_CDTI1ROUTE_SPEC>`"]
pub type TIMER0_CDTI1ROUTE = crate::Reg<timer0_cdti1route::TIMER0_CDTI1ROUTE_SPEC>;
#[doc = "CDTI1 port/pin select"]
pub mod timer0_cdti1route;
#[doc = "TIMER0_CDTI2ROUTE (rw) register accessor: an alias for `Reg<TIMER0_CDTI2ROUTE_SPEC>`"]
pub type TIMER0_CDTI2ROUTE = crate::Reg<timer0_cdti2route::TIMER0_CDTI2ROUTE_SPEC>;
#[doc = "CDTI2 port/pin select"]
pub mod timer0_cdti2route;
#[doc = "TIMER1_ROUTEEN (rw) register accessor: an alias for `Reg<TIMER1_ROUTEEN_SPEC>`"]
pub type TIMER1_ROUTEEN = crate::Reg<timer1_routeen::TIMER1_ROUTEEN_SPEC>;
#[doc = "TIMER1 pin enable"]
pub mod timer1_routeen;
#[doc = "TIMER1_CC0ROUTE (rw) register accessor: an alias for `Reg<TIMER1_CC0ROUTE_SPEC>`"]
pub type TIMER1_CC0ROUTE = crate::Reg<timer1_cc0route::TIMER1_CC0ROUTE_SPEC>;
#[doc = "CC0 port/pin select"]
pub mod timer1_cc0route;
#[doc = "TIMER1_CC1ROUTE (rw) register accessor: an alias for `Reg<TIMER1_CC1ROUTE_SPEC>`"]
pub type TIMER1_CC1ROUTE = crate::Reg<timer1_cc1route::TIMER1_CC1ROUTE_SPEC>;
#[doc = "CC1 port/pin select"]
pub mod timer1_cc1route;
#[doc = "TIMER1_CC2ROUTE (rw) register accessor: an alias for `Reg<TIMER1_CC2ROUTE_SPEC>`"]
pub type TIMER1_CC2ROUTE = crate::Reg<timer1_cc2route::TIMER1_CC2ROUTE_SPEC>;
#[doc = "CC2 port/pin select"]
pub mod timer1_cc2route;
#[doc = "TIMER1_CDTI0ROUTE (rw) register accessor: an alias for `Reg<TIMER1_CDTI0ROUTE_SPEC>`"]
pub type TIMER1_CDTI0ROUTE = crate::Reg<timer1_cdti0route::TIMER1_CDTI0ROUTE_SPEC>;
#[doc = "CDTI0 port/pin select"]
pub mod timer1_cdti0route;
#[doc = "TIMER1_CDTI1ROUTE (rw) register accessor: an alias for `Reg<TIMER1_CDTI1ROUTE_SPEC>`"]
pub type TIMER1_CDTI1ROUTE = crate::Reg<timer1_cdti1route::TIMER1_CDTI1ROUTE_SPEC>;
#[doc = "CDTI1 port/pin select"]
pub mod timer1_cdti1route;
#[doc = "TIMER1_CDTI2ROUTE (rw) register accessor: an alias for `Reg<TIMER1_CDTI2ROUTE_SPEC>`"]
pub type TIMER1_CDTI2ROUTE = crate::Reg<timer1_cdti2route::TIMER1_CDTI2ROUTE_SPEC>;
#[doc = "CDTI2 port/pin select"]
pub mod timer1_cdti2route;
#[doc = "TIMER2_ROUTEEN (rw) register accessor: an alias for `Reg<TIMER2_ROUTEEN_SPEC>`"]
pub type TIMER2_ROUTEEN = crate::Reg<timer2_routeen::TIMER2_ROUTEEN_SPEC>;
#[doc = "TIMER2 pin enable"]
pub mod timer2_routeen;
#[doc = "TIMER2_CC0ROUTE (rw) register accessor: an alias for `Reg<TIMER2_CC0ROUTE_SPEC>`"]
pub type TIMER2_CC0ROUTE = crate::Reg<timer2_cc0route::TIMER2_CC0ROUTE_SPEC>;
#[doc = "CC0 port/pin select"]
pub mod timer2_cc0route;
#[doc = "TIMER2_CC1ROUTE (rw) register accessor: an alias for `Reg<TIMER2_CC1ROUTE_SPEC>`"]
pub type TIMER2_CC1ROUTE = crate::Reg<timer2_cc1route::TIMER2_CC1ROUTE_SPEC>;
#[doc = "CC1 port/pin select"]
pub mod timer2_cc1route;
#[doc = "TIMER2_CC2ROUTE (rw) register accessor: an alias for `Reg<TIMER2_CC2ROUTE_SPEC>`"]
pub type TIMER2_CC2ROUTE = crate::Reg<timer2_cc2route::TIMER2_CC2ROUTE_SPEC>;
#[doc = "CC2 port/pin select"]
pub mod timer2_cc2route;
#[doc = "TIMER2_CDTI0ROUTE (rw) register accessor: an alias for `Reg<TIMER2_CDTI0ROUTE_SPEC>`"]
pub type TIMER2_CDTI0ROUTE = crate::Reg<timer2_cdti0route::TIMER2_CDTI0ROUTE_SPEC>;
#[doc = "CDTI0 port/pin select"]
pub mod timer2_cdti0route;
#[doc = "TIMER2_CDTI1ROUTE (rw) register accessor: an alias for `Reg<TIMER2_CDTI1ROUTE_SPEC>`"]
pub type TIMER2_CDTI1ROUTE = crate::Reg<timer2_cdti1route::TIMER2_CDTI1ROUTE_SPEC>;
#[doc = "CDTI1 port/pin select"]
pub mod timer2_cdti1route;
#[doc = "TIMER2_CDTI2ROUTE (rw) register accessor: an alias for `Reg<TIMER2_CDTI2ROUTE_SPEC>`"]
pub type TIMER2_CDTI2ROUTE = crate::Reg<timer2_cdti2route::TIMER2_CDTI2ROUTE_SPEC>;
#[doc = "CDTI2 port/pin select"]
pub mod timer2_cdti2route;
#[doc = "TIMER3_ROUTEEN (rw) register accessor: an alias for `Reg<TIMER3_ROUTEEN_SPEC>`"]
pub type TIMER3_ROUTEEN = crate::Reg<timer3_routeen::TIMER3_ROUTEEN_SPEC>;
#[doc = "TIMER3 pin enable"]
pub mod timer3_routeen;
#[doc = "TIMER3_CC0ROUTE (rw) register accessor: an alias for `Reg<TIMER3_CC0ROUTE_SPEC>`"]
pub type TIMER3_CC0ROUTE = crate::Reg<timer3_cc0route::TIMER3_CC0ROUTE_SPEC>;
#[doc = "CC0 port/pin select"]
pub mod timer3_cc0route;
#[doc = "TIMER3_CC1ROUTE (rw) register accessor: an alias for `Reg<TIMER3_CC1ROUTE_SPEC>`"]
pub type TIMER3_CC1ROUTE = crate::Reg<timer3_cc1route::TIMER3_CC1ROUTE_SPEC>;
#[doc = "CC1 port/pin select"]
pub mod timer3_cc1route;
#[doc = "TIMER3_CC2ROUTE (rw) register accessor: an alias for `Reg<TIMER3_CC2ROUTE_SPEC>`"]
pub type TIMER3_CC2ROUTE = crate::Reg<timer3_cc2route::TIMER3_CC2ROUTE_SPEC>;
#[doc = "CC2 port/pin select"]
pub mod timer3_cc2route;
#[doc = "TIMER3_CDTI0ROUTE (rw) register accessor: an alias for `Reg<TIMER3_CDTI0ROUTE_SPEC>`"]
pub type TIMER3_CDTI0ROUTE = crate::Reg<timer3_cdti0route::TIMER3_CDTI0ROUTE_SPEC>;
#[doc = "CDTI0 port/pin select"]
pub mod timer3_cdti0route;
#[doc = "TIMER3_CDTI1ROUTE (rw) register accessor: an alias for `Reg<TIMER3_CDTI1ROUTE_SPEC>`"]
pub type TIMER3_CDTI1ROUTE = crate::Reg<timer3_cdti1route::TIMER3_CDTI1ROUTE_SPEC>;
#[doc = "CDTI1 port/pin select"]
pub mod timer3_cdti1route;
#[doc = "TIMER3_CDTI2ROUTE (rw) register accessor: an alias for `Reg<TIMER3_CDTI2ROUTE_SPEC>`"]
pub type TIMER3_CDTI2ROUTE = crate::Reg<timer3_cdti2route::TIMER3_CDTI2ROUTE_SPEC>;
#[doc = "CDTI2 port/pin select"]
pub mod timer3_cdti2route;
#[doc = "TIMER4_ROUTEEN (rw) register accessor: an alias for `Reg<TIMER4_ROUTEEN_SPEC>`"]
pub type TIMER4_ROUTEEN = crate::Reg<timer4_routeen::TIMER4_ROUTEEN_SPEC>;
#[doc = "TIMER4 pin enable"]
pub mod timer4_routeen;
#[doc = "TIMER4_CC0ROUTE (rw) register accessor: an alias for `Reg<TIMER4_CC0ROUTE_SPEC>`"]
pub type TIMER4_CC0ROUTE = crate::Reg<timer4_cc0route::TIMER4_CC0ROUTE_SPEC>;
#[doc = "CC0 port/pin select"]
pub mod timer4_cc0route;
#[doc = "TIMER4_CC1ROUTE (rw) register accessor: an alias for `Reg<TIMER4_CC1ROUTE_SPEC>`"]
pub type TIMER4_CC1ROUTE = crate::Reg<timer4_cc1route::TIMER4_CC1ROUTE_SPEC>;
#[doc = "CC1 port/pin select"]
pub mod timer4_cc1route;
#[doc = "TIMER4_CC2ROUTE (rw) register accessor: an alias for `Reg<TIMER4_CC2ROUTE_SPEC>`"]
pub type TIMER4_CC2ROUTE = crate::Reg<timer4_cc2route::TIMER4_CC2ROUTE_SPEC>;
#[doc = "CC2 port/pin select"]
pub mod timer4_cc2route;
#[doc = "TIMER4_CDTI0ROUTE (rw) register accessor: an alias for `Reg<TIMER4_CDTI0ROUTE_SPEC>`"]
pub type TIMER4_CDTI0ROUTE = crate::Reg<timer4_cdti0route::TIMER4_CDTI0ROUTE_SPEC>;
#[doc = "CDTI0 port/pin select"]
pub mod timer4_cdti0route;
#[doc = "TIMER4_CDTI1ROUTE (rw) register accessor: an alias for `Reg<TIMER4_CDTI1ROUTE_SPEC>`"]
pub type TIMER4_CDTI1ROUTE = crate::Reg<timer4_cdti1route::TIMER4_CDTI1ROUTE_SPEC>;
#[doc = "CDTI1 port/pin select"]
pub mod timer4_cdti1route;
#[doc = "TIMER4_CDTI2ROUTE (rw) register accessor: an alias for `Reg<TIMER4_CDTI2ROUTE_SPEC>`"]
pub type TIMER4_CDTI2ROUTE = crate::Reg<timer4_cdti2route::TIMER4_CDTI2ROUTE_SPEC>;
#[doc = "CDTI2 port/pin select"]
pub mod timer4_cdti2route;
#[doc = "USART0_ROUTEEN (rw) register accessor: an alias for `Reg<USART0_ROUTEEN_SPEC>`"]
pub type USART0_ROUTEEN = crate::Reg<usart0_routeen::USART0_ROUTEEN_SPEC>;
#[doc = "USART0 pin enable"]
pub mod usart0_routeen;
#[doc = "USART0_CSROUTE (rw) register accessor: an alias for `Reg<USART0_CSROUTE_SPEC>`"]
pub type USART0_CSROUTE = crate::Reg<usart0_csroute::USART0_CSROUTE_SPEC>;
#[doc = "CS port/pin select"]
pub mod usart0_csroute;
#[doc = "USART0_CTSROUTE (rw) register accessor: an alias for `Reg<USART0_CTSROUTE_SPEC>`"]
pub type USART0_CTSROUTE = crate::Reg<usart0_ctsroute::USART0_CTSROUTE_SPEC>;
#[doc = "CTS port/pin select"]
pub mod usart0_ctsroute;
#[doc = "USART0_RTSROUTE (rw) register accessor: an alias for `Reg<USART0_RTSROUTE_SPEC>`"]
pub type USART0_RTSROUTE = crate::Reg<usart0_rtsroute::USART0_RTSROUTE_SPEC>;
#[doc = "RTS port/pin select"]
pub mod usart0_rtsroute;
#[doc = "USART0_RXROUTE (rw) register accessor: an alias for `Reg<USART0_RXROUTE_SPEC>`"]
pub type USART0_RXROUTE = crate::Reg<usart0_rxroute::USART0_RXROUTE_SPEC>;
#[doc = "RX port/pin select"]
pub mod usart0_rxroute;
#[doc = "USART0_CLKROUTE (rw) register accessor: an alias for `Reg<USART0_CLKROUTE_SPEC>`"]
pub type USART0_CLKROUTE = crate::Reg<usart0_clkroute::USART0_CLKROUTE_SPEC>;
#[doc = "SCLK port/pin select"]
pub mod usart0_clkroute;
#[doc = "USART0_TXROUTE (rw) register accessor: an alias for `Reg<USART0_TXROUTE_SPEC>`"]
pub type USART0_TXROUTE = crate::Reg<usart0_txroute::USART0_TXROUTE_SPEC>;
#[doc = "TX port/pin select"]
pub mod usart0_txroute;
#[doc = "USART1_ROUTEEN (rw) register accessor: an alias for `Reg<USART1_ROUTEEN_SPEC>`"]
pub type USART1_ROUTEEN = crate::Reg<usart1_routeen::USART1_ROUTEEN_SPEC>;
#[doc = "USART1 pin enable"]
pub mod usart1_routeen;
#[doc = "USART1_CSROUTE (rw) register accessor: an alias for `Reg<USART1_CSROUTE_SPEC>`"]
pub type USART1_CSROUTE = crate::Reg<usart1_csroute::USART1_CSROUTE_SPEC>;
#[doc = "CS port/pin select"]
pub mod usart1_csroute;
#[doc = "USART1_CTSROUTE (rw) register accessor: an alias for `Reg<USART1_CTSROUTE_SPEC>`"]
pub type USART1_CTSROUTE = crate::Reg<usart1_ctsroute::USART1_CTSROUTE_SPEC>;
#[doc = "CTS port/pin select"]
pub mod usart1_ctsroute;
#[doc = "USART1_RTSROUTE (rw) register accessor: an alias for `Reg<USART1_RTSROUTE_SPEC>`"]
pub type USART1_RTSROUTE = crate::Reg<usart1_rtsroute::USART1_RTSROUTE_SPEC>;
#[doc = "RTS port/pin select"]
pub mod usart1_rtsroute;
#[doc = "USART1_RXROUTE (rw) register accessor: an alias for `Reg<USART1_RXROUTE_SPEC>`"]
pub type USART1_RXROUTE = crate::Reg<usart1_rxroute::USART1_RXROUTE_SPEC>;
#[doc = "RX port/pin select"]
pub mod usart1_rxroute;
#[doc = "USART1_CLKROUTE (rw) register accessor: an alias for `Reg<USART1_CLKROUTE_SPEC>`"]
pub type USART1_CLKROUTE = crate::Reg<usart1_clkroute::USART1_CLKROUTE_SPEC>;
#[doc = "SCLK port/pin select"]
pub mod usart1_clkroute;
#[doc = "USART1_TXROUTE (rw) register accessor: an alias for `Reg<USART1_TXROUTE_SPEC>`"]
pub type USART1_TXROUTE = crate::Reg<usart1_txroute::USART1_TXROUTE_SPEC>;
#[doc = "TX port/pin select"]
pub mod usart1_txroute;
