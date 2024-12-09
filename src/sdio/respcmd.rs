#[doc = "Register `RESPCMD` reader"]
pub type R = crate::R<RESPCMDrs>;
#[doc = "Field `RESPCMD` reader - Response command index"]
pub type RESPCMD_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Response command index"]
    #[inline(always)]
    pub fn respcmd(&self) -> RESPCMD_R {
        RESPCMD_R::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESPCMD")
            .field("respcmd", &self.respcmd())
            .finish()
    }
}
#[doc = "command response register\n\nYou can [`read`](crate::Reg::read) this register and get [`respcmd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESPCMDrs;
impl crate::RegisterSpec for RESPCMDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`respcmd::R`](R) reader structure"]
impl crate::Readable for RESPCMDrs {}
#[doc = "`reset()` method sets RESPCMD to value 0"]
impl crate::Resettable for RESPCMDrs {
    const RESET_VALUE: u32 = 0;
}
