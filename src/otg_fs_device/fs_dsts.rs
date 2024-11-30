#[doc = "Register `FS_DSTS` reader"]
pub type R = crate::R<FsDstsSpec>;
#[doc = "Field `SUSPSTS` reader - Suspend status"]
pub type SuspstsR = crate::BitReader;
#[doc = "Field `ENUMSPD` reader - Enumerated speed"]
pub type EnumspdR = crate::FieldReader;
#[doc = "Field `EERR` reader - Erratic error"]
pub type EerrR = crate::BitReader;
#[doc = "Field `FNSOF` reader - Frame number of the received SOF"]
pub type FnsofR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Suspend status"]
    #[inline(always)]
    pub fn suspsts(&self) -> SuspstsR {
        SuspstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Enumerated speed"]
    #[inline(always)]
    pub fn enumspd(&self) -> EnumspdR {
        EnumspdR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Erratic error"]
    #[inline(always)]
    pub fn eerr(&self) -> EerrR {
        EerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:21 - Frame number of the received SOF"]
    #[inline(always)]
    pub fn fnsof(&self) -> FnsofR {
        FnsofR::new(((self.bits >> 8) & 0x3fff) as u16)
    }
}
#[doc = "OTG_FS device status register (OTG_FS_DSTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_dsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsDstsSpec;
impl crate::RegisterSpec for FsDstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_dsts::R`](R) reader structure"]
impl crate::Readable for FsDstsSpec {}
#[doc = "`reset()` method sets FS_DSTS to value 0x10"]
impl crate::Resettable for FsDstsSpec {
    const RESET_VALUE: u32 = 0x10;
}
