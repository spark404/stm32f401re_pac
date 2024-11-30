#[doc = "Register `RESPCMD` reader"]
pub type R = crate::R<RespcmdSpec>;
#[doc = "Field `RESPCMD` reader - Response command index"]
pub type RespcmdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Response command index"]
    #[inline(always)]
    pub fn respcmd(&self) -> RespcmdR {
        RespcmdR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "command response register\n\nYou can [`read`](crate::Reg::read) this register and get [`respcmd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RespcmdSpec;
impl crate::RegisterSpec for RespcmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`respcmd::R`](R) reader structure"]
impl crate::Readable for RespcmdSpec {}
#[doc = "`reset()` method sets RESPCMD to value 0"]
impl crate::Resettable for RespcmdSpec {
    const RESET_VALUE: u32 = 0;
}
