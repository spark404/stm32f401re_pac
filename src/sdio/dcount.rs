#[doc = "Register `DCOUNT` reader"]
pub type R = crate::R<DCOUNTrs>;
#[doc = "Field `DATACOUNT` reader - Data count value"]
pub type DATACOUNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:24 - Data count value"]
    #[inline(always)]
    pub fn datacount(&self) -> DATACOUNT_R {
        DATACOUNT_R::new(self.bits & 0x01ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCOUNT")
            .field("datacount", &self.datacount())
            .finish()
    }
}
#[doc = "data counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcount::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCOUNTrs;
impl crate::RegisterSpec for DCOUNTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcount::R`](R) reader structure"]
impl crate::Readable for DCOUNTrs {}
#[doc = "`reset()` method sets DCOUNT to value 0"]
impl crate::Resettable for DCOUNTrs {
    const RESET_VALUE: u32 = 0;
}
