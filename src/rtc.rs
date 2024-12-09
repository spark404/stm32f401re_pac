#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tr: TR,
    dr: DR,
    cr: CR,
    isr: ISR,
    prer: PRER,
    wutr: WUTR,
    calibr: CALIBR,
    alrmar: ALRMAR,
    alrmbr: ALRMBR,
    wpr: WPR,
    ssr: SSR,
    shiftr: SHIFTR,
    tstr: TSTR,
    tsdr: TSDR,
    tsssr: TSSSR,
    calr: CALR,
    tafcr: TAFCR,
    alrmassr: ALRMASSR,
    alrmbssr: ALRMBSSR,
    _reserved19: [u8; 0x04],
    bkp0r: BKP0R,
    bkp1r: BKP1R,
    bkp2r: BKP2R,
    bkp3r: BKP3R,
    bkp4r: BKP4R,
    bkp5r: BKP5R,
    bkp6r: BKP6R,
    bkp7r: BKP7R,
    bkp8r: BKP8R,
    bkp9r: BKP9R,
    bkp10r: BKP10R,
    bkp11r: BKP11R,
    bkp12r: BKP12R,
    bkp13r: BKP13R,
    bkp14r: BKP14R,
    bkp15r: BKP15R,
    bkp16r: BKP16R,
    bkp17r: BKP17R,
    bkp18r: BKP18R,
    bkp19r: BKP19R,
}
impl RegisterBlock {
    #[doc = "0x00 - time register"]
    #[inline(always)]
    pub const fn tr(&self) -> &TR {
        &self.tr
    }
    #[doc = "0x04 - date register"]
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    #[doc = "0x08 - control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x0c - initialization and status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x10 - prescaler register"]
    #[inline(always)]
    pub const fn prer(&self) -> &PRER {
        &self.prer
    }
    #[doc = "0x14 - wakeup timer register"]
    #[inline(always)]
    pub const fn wutr(&self) -> &WUTR {
        &self.wutr
    }
    #[doc = "0x18 - calibration register"]
    #[inline(always)]
    pub const fn calibr(&self) -> &CALIBR {
        &self.calibr
    }
    #[doc = "0x1c - alarm A register"]
    #[inline(always)]
    pub const fn alrmar(&self) -> &ALRMAR {
        &self.alrmar
    }
    #[doc = "0x20 - alarm B register"]
    #[inline(always)]
    pub const fn alrmbr(&self) -> &ALRMBR {
        &self.alrmbr
    }
    #[doc = "0x24 - write protection register"]
    #[inline(always)]
    pub const fn wpr(&self) -> &WPR {
        &self.wpr
    }
    #[doc = "0x28 - sub second register"]
    #[inline(always)]
    pub const fn ssr(&self) -> &SSR {
        &self.ssr
    }
    #[doc = "0x2c - shift control register"]
    #[inline(always)]
    pub const fn shiftr(&self) -> &SHIFTR {
        &self.shiftr
    }
    #[doc = "0x30 - time stamp time register"]
    #[inline(always)]
    pub const fn tstr(&self) -> &TSTR {
        &self.tstr
    }
    #[doc = "0x34 - time stamp date register"]
    #[inline(always)]
    pub const fn tsdr(&self) -> &TSDR {
        &self.tsdr
    }
    #[doc = "0x38 - timestamp sub second register"]
    #[inline(always)]
    pub const fn tsssr(&self) -> &TSSSR {
        &self.tsssr
    }
    #[doc = "0x3c - calibration register"]
    #[inline(always)]
    pub const fn calr(&self) -> &CALR {
        &self.calr
    }
    #[doc = "0x40 - tamper and alternate function configuration register"]
    #[inline(always)]
    pub const fn tafcr(&self) -> &TAFCR {
        &self.tafcr
    }
    #[doc = "0x44 - alarm A sub second register"]
    #[inline(always)]
    pub const fn alrmassr(&self) -> &ALRMASSR {
        &self.alrmassr
    }
    #[doc = "0x48 - alarm B sub second register"]
    #[inline(always)]
    pub const fn alrmbssr(&self) -> &ALRMBSSR {
        &self.alrmbssr
    }
    #[doc = "0x50 - backup register"]
    #[inline(always)]
    pub const fn bkp0r(&self) -> &BKP0R {
        &self.bkp0r
    }
    #[doc = "0x54 - backup register"]
    #[inline(always)]
    pub const fn bkp1r(&self) -> &BKP1R {
        &self.bkp1r
    }
    #[doc = "0x58 - backup register"]
    #[inline(always)]
    pub const fn bkp2r(&self) -> &BKP2R {
        &self.bkp2r
    }
    #[doc = "0x5c - backup register"]
    #[inline(always)]
    pub const fn bkp3r(&self) -> &BKP3R {
        &self.bkp3r
    }
    #[doc = "0x60 - backup register"]
    #[inline(always)]
    pub const fn bkp4r(&self) -> &BKP4R {
        &self.bkp4r
    }
    #[doc = "0x64 - backup register"]
    #[inline(always)]
    pub const fn bkp5r(&self) -> &BKP5R {
        &self.bkp5r
    }
    #[doc = "0x68 - backup register"]
    #[inline(always)]
    pub const fn bkp6r(&self) -> &BKP6R {
        &self.bkp6r
    }
    #[doc = "0x6c - backup register"]
    #[inline(always)]
    pub const fn bkp7r(&self) -> &BKP7R {
        &self.bkp7r
    }
    #[doc = "0x70 - backup register"]
    #[inline(always)]
    pub const fn bkp8r(&self) -> &BKP8R {
        &self.bkp8r
    }
    #[doc = "0x74 - backup register"]
    #[inline(always)]
    pub const fn bkp9r(&self) -> &BKP9R {
        &self.bkp9r
    }
    #[doc = "0x78 - backup register"]
    #[inline(always)]
    pub const fn bkp10r(&self) -> &BKP10R {
        &self.bkp10r
    }
    #[doc = "0x7c - backup register"]
    #[inline(always)]
    pub const fn bkp11r(&self) -> &BKP11R {
        &self.bkp11r
    }
    #[doc = "0x80 - backup register"]
    #[inline(always)]
    pub const fn bkp12r(&self) -> &BKP12R {
        &self.bkp12r
    }
    #[doc = "0x84 - backup register"]
    #[inline(always)]
    pub const fn bkp13r(&self) -> &BKP13R {
        &self.bkp13r
    }
    #[doc = "0x88 - backup register"]
    #[inline(always)]
    pub const fn bkp14r(&self) -> &BKP14R {
        &self.bkp14r
    }
    #[doc = "0x8c - backup register"]
    #[inline(always)]
    pub const fn bkp15r(&self) -> &BKP15R {
        &self.bkp15r
    }
    #[doc = "0x90 - backup register"]
    #[inline(always)]
    pub const fn bkp16r(&self) -> &BKP16R {
        &self.bkp16r
    }
    #[doc = "0x94 - backup register"]
    #[inline(always)]
    pub const fn bkp17r(&self) -> &BKP17R {
        &self.bkp17r
    }
    #[doc = "0x98 - backup register"]
    #[inline(always)]
    pub const fn bkp18r(&self) -> &BKP18R {
        &self.bkp18r
    }
    #[doc = "0x9c - backup register"]
    #[inline(always)]
    pub const fn bkp19r(&self) -> &BKP19R {
        &self.bkp19r
    }
}
#[doc = "TR (rw) register accessor: time register\n\nYou can [`read`](crate::Reg::read) this register and get [`tr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr`]
module"]
pub type TR = crate::Reg<tr::TRrs>;
#[doc = "time register"]
pub mod tr;
#[doc = "DR (rw) register accessor: date register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DRrs>;
#[doc = "date register"]
pub mod dr;
#[doc = "CR (rw) register accessor: control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "control register"]
pub mod cr;
#[doc = "ISR (rw) register accessor: initialization and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISRrs>;
#[doc = "initialization and status register"]
pub mod isr;
#[doc = "PRER (rw) register accessor: prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`prer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prer`]
module"]
pub type PRER = crate::Reg<prer::PRERrs>;
#[doc = "prescaler register"]
pub mod prer;
#[doc = "WUTR (rw) register accessor: wakeup timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`wutr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wutr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wutr`]
module"]
pub type WUTR = crate::Reg<wutr::WUTRrs>;
#[doc = "wakeup timer register"]
pub mod wutr;
#[doc = "CALIBR (rw) register accessor: calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`calibr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calibr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calibr`]
module"]
pub type CALIBR = crate::Reg<calibr::CALIBRrs>;
#[doc = "calibration register"]
pub mod calibr;
#[doc = "ALRMAR (rw) register accessor: alarm A register\n\nYou can [`read`](crate::Reg::read) this register and get [`alrmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrmar`]
module"]
pub type ALRMAR = crate::Reg<alrmar::ALRMARrs>;
#[doc = "alarm A register"]
pub mod alrmar;
#[doc = "ALRMBR (rw) register accessor: alarm B register\n\nYou can [`read`](crate::Reg::read) this register and get [`alrmbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrmbr`]
module"]
pub type ALRMBR = crate::Reg<alrmbr::ALRMBRrs>;
#[doc = "alarm B register"]
pub mod alrmbr;
#[doc = "WPR (w) register accessor: write protection register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpr`]
module"]
pub type WPR = crate::Reg<wpr::WPRrs>;
#[doc = "write protection register"]
pub mod wpr;
#[doc = "SSR (r) register accessor: sub second register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssr`]
module"]
pub type SSR = crate::Reg<ssr::SSRrs>;
#[doc = "sub second register"]
pub mod ssr;
#[doc = "SHIFTR (w) register accessor: shift control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shiftr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shiftr`]
module"]
pub type SHIFTR = crate::Reg<shiftr::SHIFTRrs>;
#[doc = "shift control register"]
pub mod shiftr;
#[doc = "TSTR (r) register accessor: time stamp time register\n\nYou can [`read`](crate::Reg::read) this register and get [`tstr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tstr`]
module"]
pub type TSTR = crate::Reg<tstr::TSTRrs>;
#[doc = "time stamp time register"]
pub mod tstr;
#[doc = "TSDR (r) register accessor: time stamp date register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsdr`]
module"]
pub type TSDR = crate::Reg<tsdr::TSDRrs>;
#[doc = "time stamp date register"]
pub mod tsdr;
#[doc = "TSSSR (r) register accessor: timestamp sub second register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsssr`]
module"]
pub type TSSSR = crate::Reg<tsssr::TSSSRrs>;
#[doc = "timestamp sub second register"]
pub mod tsssr;
#[doc = "CALR (rw) register accessor: calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`calr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calr`]
module"]
pub type CALR = crate::Reg<calr::CALRrs>;
#[doc = "calibration register"]
pub mod calr;
#[doc = "TAFCR (rw) register accessor: tamper and alternate function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tafcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tafcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tafcr`]
module"]
pub type TAFCR = crate::Reg<tafcr::TAFCRrs>;
#[doc = "tamper and alternate function configuration register"]
pub mod tafcr;
#[doc = "ALRMASSR (rw) register accessor: alarm A sub second register\n\nYou can [`read`](crate::Reg::read) this register and get [`alrmassr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmassr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrmassr`]
module"]
pub type ALRMASSR = crate::Reg<alrmassr::ALRMASSRrs>;
#[doc = "alarm A sub second register"]
pub mod alrmassr;
#[doc = "ALRMBSSR (rw) register accessor: alarm B sub second register\n\nYou can [`read`](crate::Reg::read) this register and get [`alrmbssr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmbssr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrmbssr`]
module"]
pub type ALRMBSSR = crate::Reg<alrmbssr::ALRMBSSRrs>;
#[doc = "alarm B sub second register"]
pub mod alrmbssr;
#[doc = "BKP0R (rw) register accessor: backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp0r`]
module"]
pub type BKP0R = crate::Reg<bkp0r::BKP0Rrs>;
#[doc = "backup register"]
pub mod bkp0r;
#[doc = "BKP1R (rw) register accessor: backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp1r`]
module"]
pub type BKP1R = crate::Reg<bkp1r::BKP1Rrs>;
#[doc = "backup register"]
pub mod bkp1r;
#[doc = "BKP2R (rw) register accessor: backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp2r`]
module"]
pub type BKP2R = crate::Reg<bkp2r::BKP2Rrs>;
#[doc = "backup register"]
pub mod bkp2r;
#[doc = "BKP3R (rw) register accessor: backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp3r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp3r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp3r`]
module"]
pub type BKP3R = crate::Reg<bkp3r::BKP3Rrs>;
#[doc = "backup register"]
pub mod bkp3r;
#[doc = "BKP4R (rw) register accessor: backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp4r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp4r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp4r`]
module"]
pub type BKP4R = crate::Reg<bkp4r::BKP4Rrs>;
#[doc = "backup register"]
pub mod bkp4r;
#[doc = "BKP5R (rw) register accessor: backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp5r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp5r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp5r`]
module"]
pub type BKP5R = crate::Reg<bkp5r::BKP5Rrs>;
#[doc = "backup register"]
pub mod bkp5r;
#[doc = "BKP6R (rw) register accessor: backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp6r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp6r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp6r`]
module"]
pub type BKP6R = crate::Reg<bkp6r::BKP6Rrs>;
#[doc = "backup register"]
pub mod bkp6r;
#[doc = "BKP7R (rw) register accessor: backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp7r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp7r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp7r`]
module"]
pub type BKP7R = crate::Reg<bkp7r::BKP7Rrs>;
#[doc = "backup register"]
pub mod bkp7r;
#[doc = "BKP8R (rw) register accessor: backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp8r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp8r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp8r`]
module"]
pub type BKP8R = crate::Reg<bkp8r::BKP8Rrs>;
#[doc = "backup register"]
pub mod bkp8r;
#[doc = "BKP9R (rw) register accessor: backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp9r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp9r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp9r`]
module"]
pub type BKP9R = crate::Reg<bkp9r::BKP9Rrs>;
#[doc = "backup register"]
pub mod bkp9r;
#[doc = "BKP10R (rw) register accessor: backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp10r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp10r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp10r`]
module"]
pub type BKP10R = crate::Reg<bkp10r::BKP10Rrs>;
#[doc = "backup register"]
pub mod bkp10r;
#[doc = "BKP11R (rw) register accessor: backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp11r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp11r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp11r`]
module"]
pub type BKP11R = crate::Reg<bkp11r::BKP11Rrs>;
#[doc = "backup register"]
pub mod bkp11r;
#[doc = "BKP12R (rw) register accessor: backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp12r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp12r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp12r`]
module"]
pub type BKP12R = crate::Reg<bkp12r::BKP12Rrs>;
#[doc = "backup register"]
pub mod bkp12r;
#[doc = "BKP13R (rw) register accessor: backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp13r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp13r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp13r`]
module"]
pub type BKP13R = crate::Reg<bkp13r::BKP13Rrs>;
#[doc = "backup register"]
pub mod bkp13r;
#[doc = "BKP14R (rw) register accessor: backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp14r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp14r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp14r`]
module"]
pub type BKP14R = crate::Reg<bkp14r::BKP14Rrs>;
#[doc = "backup register"]
pub mod bkp14r;
#[doc = "BKP15R (rw) register accessor: backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp15r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp15r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp15r`]
module"]
pub type BKP15R = crate::Reg<bkp15r::BKP15Rrs>;
#[doc = "backup register"]
pub mod bkp15r;
#[doc = "BKP16R (rw) register accessor: backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp16r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp16r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp16r`]
module"]
pub type BKP16R = crate::Reg<bkp16r::BKP16Rrs>;
#[doc = "backup register"]
pub mod bkp16r;
#[doc = "BKP17R (rw) register accessor: backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp17r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp17r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp17r`]
module"]
pub type BKP17R = crate::Reg<bkp17r::BKP17Rrs>;
#[doc = "backup register"]
pub mod bkp17r;
#[doc = "BKP18R (rw) register accessor: backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp18r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp18r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp18r`]
module"]
pub type BKP18R = crate::Reg<bkp18r::BKP18Rrs>;
#[doc = "backup register"]
pub mod bkp18r;
#[doc = "BKP19R (rw) register accessor: backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp19r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp19r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp19r`]
module"]
pub type BKP19R = crate::Reg<bkp19r::BKP19Rrs>;
#[doc = "backup register"]
pub mod bkp19r;
