#[doc = "Register `FS_DAINT` reader"]
pub type R = crate::R<FsDaintSpec>;
#[doc = "Field `IEPINT` reader - IN endpoint interrupt bits"]
pub type IepintR = crate::FieldReader<u16>;
#[doc = "Field `OEPINT` reader - OUT endpoint interrupt bits"]
pub type OepintR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint interrupt bits"]
    #[inline(always)]
    pub fn iepint(&self) -> IepintR {
        IepintR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OUT endpoint interrupt bits"]
    #[inline(always)]
    pub fn oepint(&self) -> OepintR {
        OepintR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_daint::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsDaintSpec;
impl crate::RegisterSpec for FsDaintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_daint::R`](R) reader structure"]
impl crate::Readable for FsDaintSpec {}
#[doc = "`reset()` method sets FS_DAINT to value 0"]
impl crate::Resettable for FsDaintSpec {
    const RESET_VALUE: u32 = 0;
}
