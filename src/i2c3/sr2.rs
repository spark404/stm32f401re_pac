#[doc = "Register `SR2` reader"]
pub type R = crate::R<Sr2Spec>;
#[doc = "Field `MSL` reader - Master/slave"]
pub type MslR = crate::BitReader;
#[doc = "Field `BUSY` reader - Bus busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `TRA` reader - Transmitter/receiver"]
pub type TraR = crate::BitReader;
#[doc = "Field `GENCALL` reader - General call address (Slave mode)"]
pub type GencallR = crate::BitReader;
#[doc = "Field `SMBDEFAULT` reader - SMBus device default address (Slave mode)"]
pub type SmbdefaultR = crate::BitReader;
#[doc = "Field `SMBHOST` reader - SMBus host header (Slave mode)"]
pub type SmbhostR = crate::BitReader;
#[doc = "Field `DUALF` reader - Dual flag (Slave mode)"]
pub type DualfR = crate::BitReader;
#[doc = "Field `PEC` reader - acket error checking register"]
pub type PecR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Master/slave"]
    #[inline(always)]
    pub fn msl(&self) -> MslR {
        MslR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bus busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitter/receiver"]
    #[inline(always)]
    pub fn tra(&self) -> TraR {
        TraR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - General call address (Slave mode)"]
    #[inline(always)]
    pub fn gencall(&self) -> GencallR {
        GencallR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SMBus device default address (Slave mode)"]
    #[inline(always)]
    pub fn smbdefault(&self) -> SmbdefaultR {
        SmbdefaultR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SMBus host header (Slave mode)"]
    #[inline(always)]
    pub fn smbhost(&self) -> SmbhostR {
        SmbhostR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Dual flag (Slave mode)"]
    #[inline(always)]
    pub fn dualf(&self) -> DualfR {
        DualfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - acket error checking register"]
    #[inline(always)]
    pub fn pec(&self) -> PecR {
        PecR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Status register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sr2Spec;
impl crate::RegisterSpec for Sr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr2::R`](R) reader structure"]
impl crate::Readable for Sr2Spec {}
#[doc = "`reset()` method sets SR2 to value 0"]
impl crate::Resettable for Sr2Spec {
    const RESET_VALUE: u32 = 0;
}
