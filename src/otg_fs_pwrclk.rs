#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    fs_pcgcctl: FS_PCGCCTL,
}
impl RegisterBlock {
    #[doc = "0x00 - OTG_FS power and clock gating control register"]
    #[inline(always)]
    pub const fn fs_pcgcctl(&self) -> &FS_PCGCCTL {
        &self.fs_pcgcctl
    }
}
#[doc = "FS_PCGCCTL (rw) register accessor: OTG_FS power and clock gating control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_pcgcctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_pcgcctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_pcgcctl`]
module"]
pub type FS_PCGCCTL = crate::Reg<fs_pcgcctl::FS_PCGCCTLrs>;
#[doc = "OTG_FS power and clock gating control register"]
pub mod fs_pcgcctl;
