#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    pllcfgr: Pllcfgr,
    cfgr: Cfgr,
    cir: Cir,
    ahb1rstr: Ahb1rstr,
    ahb2rstr: Ahb2rstr,
    _reserved6: [u8; 0x08],
    apb1rstr: Apb1rstr,
    apb2rstr: Apb2rstr,
    _reserved8: [u8; 0x08],
    ahb1enr: Ahb1enr,
    ahb2enr: Ahb2enr,
    _reserved10: [u8; 0x08],
    apb1enr: Apb1enr,
    apb2enr: Apb2enr,
    _reserved12: [u8; 0x08],
    ahb1lpenr: Ahb1lpenr,
    ahb2lpenr: Ahb2lpenr,
    _reserved14: [u8; 0x08],
    apb1lpenr: Apb1lpenr,
    apb2lpenr: Apb2lpenr,
    _reserved16: [u8; 0x08],
    bdcr: Bdcr,
    csr: Csr,
    _reserved18: [u8; 0x08],
    sscgr: Sscgr,
    plli2scfgr: Plli2scfgr,
}
impl RegisterBlock {
    #[doc = "0x00 - clock control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - PLL configuration register"]
    #[inline(always)]
    pub const fn pllcfgr(&self) -> &Pllcfgr {
        &self.pllcfgr
    }
    #[doc = "0x08 - clock configuration register"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &Cfgr {
        &self.cfgr
    }
    #[doc = "0x0c - clock interrupt register"]
    #[inline(always)]
    pub const fn cir(&self) -> &Cir {
        &self.cir
    }
    #[doc = "0x10 - AHB1 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb1rstr(&self) -> &Ahb1rstr {
        &self.ahb1rstr
    }
    #[doc = "0x14 - AHB2 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb2rstr(&self) -> &Ahb2rstr {
        &self.ahb2rstr
    }
    #[doc = "0x20 - APB1 peripheral reset register"]
    #[inline(always)]
    pub const fn apb1rstr(&self) -> &Apb1rstr {
        &self.apb1rstr
    }
    #[doc = "0x24 - APB2 peripheral reset register"]
    #[inline(always)]
    pub const fn apb2rstr(&self) -> &Apb2rstr {
        &self.apb2rstr
    }
    #[doc = "0x30 - AHB1 peripheral clock register"]
    #[inline(always)]
    pub const fn ahb1enr(&self) -> &Ahb1enr {
        &self.ahb1enr
    }
    #[doc = "0x34 - AHB2 peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahb2enr(&self) -> &Ahb2enr {
        &self.ahb2enr
    }
    #[doc = "0x40 - APB1 peripheral clock enable register"]
    #[inline(always)]
    pub const fn apb1enr(&self) -> &Apb1enr {
        &self.apb1enr
    }
    #[doc = "0x44 - APB2 peripheral clock enable register"]
    #[inline(always)]
    pub const fn apb2enr(&self) -> &Apb2enr {
        &self.apb2enr
    }
    #[doc = "0x50 - AHB1 peripheral clock enable in low power mode register"]
    #[inline(always)]
    pub const fn ahb1lpenr(&self) -> &Ahb1lpenr {
        &self.ahb1lpenr
    }
    #[doc = "0x54 - AHB2 peripheral clock enable in low power mode register"]
    #[inline(always)]
    pub const fn ahb2lpenr(&self) -> &Ahb2lpenr {
        &self.ahb2lpenr
    }
    #[doc = "0x60 - APB1 peripheral clock enable in low power mode register"]
    #[inline(always)]
    pub const fn apb1lpenr(&self) -> &Apb1lpenr {
        &self.apb1lpenr
    }
    #[doc = "0x64 - APB2 peripheral clock enabled in low power mode register"]
    #[inline(always)]
    pub const fn apb2lpenr(&self) -> &Apb2lpenr {
        &self.apb2lpenr
    }
    #[doc = "0x70 - Backup domain control register"]
    #[inline(always)]
    pub const fn bdcr(&self) -> &Bdcr {
        &self.bdcr
    }
    #[doc = "0x74 - clock control & status register"]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    #[doc = "0x80 - spread spectrum clock generation register"]
    #[inline(always)]
    pub const fn sscgr(&self) -> &Sscgr {
        &self.sscgr
    }
    #[doc = "0x84 - PLLI2S configuration register"]
    #[inline(always)]
    pub const fn plli2scfgr(&self) -> &Plli2scfgr {
        &self.plli2scfgr
    }
}
#[doc = "CR (rw) register accessor: clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "clock control register"]
pub mod cr;
#[doc = "PLLCFGR (rw) register accessor: PLL configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pllcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllcfgr`]
module"]
#[doc(alias = "PLLCFGR")]
pub type Pllcfgr = crate::Reg<pllcfgr::PllcfgrSpec>;
#[doc = "PLL configuration register"]
pub mod pllcfgr;
#[doc = "CFGR (rw) register accessor: clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`]
module"]
#[doc(alias = "CFGR")]
pub type Cfgr = crate::Reg<cfgr::CfgrSpec>;
#[doc = "clock configuration register"]
pub mod cfgr;
#[doc = "CIR (rw) register accessor: clock interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`cir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cir`]
module"]
#[doc(alias = "CIR")]
pub type Cir = crate::Reg<cir::CirSpec>;
#[doc = "clock interrupt register"]
pub mod cir;
#[doc = "AHB1RSTR (rw) register accessor: AHB1 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1rstr`]
module"]
#[doc(alias = "AHB1RSTR")]
pub type Ahb1rstr = crate::Reg<ahb1rstr::Ahb1rstrSpec>;
#[doc = "AHB1 peripheral reset register"]
pub mod ahb1rstr;
#[doc = "AHB2RSTR (rw) register accessor: AHB2 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2rstr`]
module"]
#[doc(alias = "AHB2RSTR")]
pub type Ahb2rstr = crate::Reg<ahb2rstr::Ahb2rstrSpec>;
#[doc = "AHB2 peripheral reset register"]
pub mod ahb2rstr;
#[doc = "APB1RSTR (rw) register accessor: APB1 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1rstr`]
module"]
#[doc(alias = "APB1RSTR")]
pub type Apb1rstr = crate::Reg<apb1rstr::Apb1rstrSpec>;
#[doc = "APB1 peripheral reset register"]
pub mod apb1rstr;
#[doc = "APB2RSTR (rw) register accessor: APB2 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2rstr`]
module"]
#[doc(alias = "APB2RSTR")]
pub type Apb2rstr = crate::Reg<apb2rstr::Apb2rstrSpec>;
#[doc = "APB2 peripheral reset register"]
pub mod apb2rstr;
#[doc = "AHB1ENR (rw) register accessor: AHB1 peripheral clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1enr`]
module"]
#[doc(alias = "AHB1ENR")]
pub type Ahb1enr = crate::Reg<ahb1enr::Ahb1enrSpec>;
#[doc = "AHB1 peripheral clock register"]
pub mod ahb1enr;
#[doc = "AHB2ENR (rw) register accessor: AHB2 peripheral clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2enr`]
module"]
#[doc(alias = "AHB2ENR")]
pub type Ahb2enr = crate::Reg<ahb2enr::Ahb2enrSpec>;
#[doc = "AHB2 peripheral clock enable register"]
pub mod ahb2enr;
#[doc = "APB1ENR (rw) register accessor: APB1 peripheral clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1enr`]
module"]
#[doc(alias = "APB1ENR")]
pub type Apb1enr = crate::Reg<apb1enr::Apb1enrSpec>;
#[doc = "APB1 peripheral clock enable register"]
pub mod apb1enr;
#[doc = "APB2ENR (rw) register accessor: APB2 peripheral clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2enr`]
module"]
#[doc(alias = "APB2ENR")]
pub type Apb2enr = crate::Reg<apb2enr::Apb2enrSpec>;
#[doc = "APB2 peripheral clock enable register"]
pub mod apb2enr;
#[doc = "AHB1LPENR (rw) register accessor: AHB1 peripheral clock enable in low power mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1lpenr`]
module"]
#[doc(alias = "AHB1LPENR")]
pub type Ahb1lpenr = crate::Reg<ahb1lpenr::Ahb1lpenrSpec>;
#[doc = "AHB1 peripheral clock enable in low power mode register"]
pub mod ahb1lpenr;
#[doc = "AHB2LPENR (rw) register accessor: AHB2 peripheral clock enable in low power mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2lpenr`]
module"]
#[doc(alias = "AHB2LPENR")]
pub type Ahb2lpenr = crate::Reg<ahb2lpenr::Ahb2lpenrSpec>;
#[doc = "AHB2 peripheral clock enable in low power mode register"]
pub mod ahb2lpenr;
#[doc = "APB1LPENR (rw) register accessor: APB1 peripheral clock enable in low power mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1lpenr`]
module"]
#[doc(alias = "APB1LPENR")]
pub type Apb1lpenr = crate::Reg<apb1lpenr::Apb1lpenrSpec>;
#[doc = "APB1 peripheral clock enable in low power mode register"]
pub mod apb1lpenr;
#[doc = "APB2LPENR (rw) register accessor: APB2 peripheral clock enabled in low power mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2lpenr`]
module"]
#[doc(alias = "APB2LPENR")]
pub type Apb2lpenr = crate::Reg<apb2lpenr::Apb2lpenrSpec>;
#[doc = "APB2 peripheral clock enabled in low power mode register"]
pub mod apb2lpenr;
#[doc = "BDCR (rw) register accessor: Backup domain control register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdcr`]
module"]
#[doc(alias = "BDCR")]
pub type Bdcr = crate::Reg<bdcr::BdcrSpec>;
#[doc = "Backup domain control register"]
pub mod bdcr;
#[doc = "CSR (rw) register accessor: clock control & status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "clock control & status register"]
pub mod csr;
#[doc = "SSCGR (rw) register accessor: spread spectrum clock generation register\n\nYou can [`read`](crate::Reg::read) this register and get [`sscgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sscgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sscgr`]
module"]
#[doc(alias = "SSCGR")]
pub type Sscgr = crate::Reg<sscgr::SscgrSpec>;
#[doc = "spread spectrum clock generation register"]
pub mod sscgr;
#[doc = "PLLI2SCFGR (rw) register accessor: PLLI2S configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`plli2scfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plli2scfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plli2scfgr`]
module"]
#[doc(alias = "PLLI2SCFGR")]
pub type Plli2scfgr = crate::Reg<plli2scfgr::Plli2scfgrSpec>;
#[doc = "PLLI2S configuration register"]
pub mod plli2scfgr;