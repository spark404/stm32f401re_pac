#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dbgmcu_idcode: DBGMCU_IDCODE,
    dbgmcu_cr: DBGMCU_CR,
    dbgmcu_apb1_fz: DBGMCU_APB1_FZ,
    dbgmcu_apb2_fz: DBGMCU_APB2_FZ,
}
impl RegisterBlock {
    #[doc = "0x00 - IDCODE"]
    #[inline(always)]
    pub const fn dbgmcu_idcode(&self) -> &DBGMCU_IDCODE {
        &self.dbgmcu_idcode
    }
    #[doc = "0x04 - Control Register"]
    #[inline(always)]
    pub const fn dbgmcu_cr(&self) -> &DBGMCU_CR {
        &self.dbgmcu_cr
    }
    #[doc = "0x08 - Debug MCU APB1 Freeze registe"]
    #[inline(always)]
    pub const fn dbgmcu_apb1_fz(&self) -> &DBGMCU_APB1_FZ {
        &self.dbgmcu_apb1_fz
    }
    #[doc = "0x0c - Debug MCU APB2 Freeze registe"]
    #[inline(always)]
    pub const fn dbgmcu_apb2_fz(&self) -> &DBGMCU_APB2_FZ {
        &self.dbgmcu_apb2_fz
    }
}
#[doc = "DBGMCU_IDCODE (r) register accessor: IDCODE\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgmcu_idcode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgmcu_idcode`]
module"]
pub type DBGMCU_IDCODE = crate::Reg<dbgmcu_idcode::DBGMCU_IDCODErs>;
#[doc = "IDCODE"]
pub mod dbgmcu_idcode;
#[doc = "DBGMCU_CR (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgmcu_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgmcu_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgmcu_cr`]
module"]
pub type DBGMCU_CR = crate::Reg<dbgmcu_cr::DBGMCU_CRrs>;
#[doc = "Control Register"]
pub mod dbgmcu_cr;
#[doc = "DBGMCU_APB1_FZ (rw) register accessor: Debug MCU APB1 Freeze registe\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgmcu_apb1_fz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgmcu_apb1_fz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgmcu_apb1_fz`]
module"]
pub type DBGMCU_APB1_FZ = crate::Reg<dbgmcu_apb1_fz::DBGMCU_APB1_FZrs>;
#[doc = "Debug MCU APB1 Freeze registe"]
pub mod dbgmcu_apb1_fz;
#[doc = "DBGMCU_APB2_FZ (rw) register accessor: Debug MCU APB2 Freeze registe\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgmcu_apb2_fz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgmcu_apb2_fz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgmcu_apb2_fz`]
module"]
pub type DBGMCU_APB2_FZ = crate::Reg<dbgmcu_apb2_fz::DBGMCU_APB2_FZrs>;
#[doc = "Debug MCU APB2 Freeze registe"]
pub mod dbgmcu_apb2_fz;
