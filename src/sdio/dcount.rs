#[doc = "Register `DCOUNT` reader"]
pub type R = crate::R<DcountSpec>;
#[doc = "Field `DATACOUNT` reader - Data count value"]
pub type DatacountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:24 - Data count value"]
    #[inline(always)]
    pub fn datacount(&self) -> DatacountR {
        DatacountR::new(self.bits & 0x01ff_ffff)
    }
}
#[doc = "data counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcount::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcountSpec;
impl crate::RegisterSpec for DcountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcount::R`](R) reader structure"]
impl crate::Readable for DcountSpec {}
#[doc = "`reset()` method sets DCOUNT to value 0"]
impl crate::Resettable for DcountSpec {
    const RESET_VALUE: u32 = 0;
}
