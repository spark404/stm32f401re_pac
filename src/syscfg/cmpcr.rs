#[doc = "Register `CMPCR` reader"]
pub type R = crate::R<CmpcrSpec>;
#[doc = "Field `CMP_PD` reader - Compensation cell power-down"]
pub type CmpPdR = crate::BitReader;
#[doc = "Field `READY` reader - READY"]
pub type ReadyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Compensation cell power-down"]
    #[inline(always)]
    pub fn cmp_pd(&self) -> CmpPdR {
        CmpPdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - READY"]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Compensation cell control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpcrSpec;
impl crate::RegisterSpec for CmpcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpcr::R`](R) reader structure"]
impl crate::Readable for CmpcrSpec {}
#[doc = "`reset()` method sets CMPCR to value 0"]
impl crate::Resettable for CmpcrSpec {
    const RESET_VALUE: u32 = 0;
}
