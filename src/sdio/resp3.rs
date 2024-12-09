#[doc = "Register `RESP3` reader"]
pub type R = crate::R<RESP3rs>;
#[doc = "Field `CARDSTATUS3` reader - Card Status"]
pub type CARDSTATUS3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Card Status"]
    #[inline(always)]
    pub fn cardstatus3(&self) -> CARDSTATUS3_R {
        CARDSTATUS3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESP3")
            .field("cardstatus3", &self.cardstatus3())
            .finish()
    }
}
#[doc = "response 1..4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`resp3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESP3rs;
impl crate::RegisterSpec for RESP3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp3::R`](R) reader structure"]
impl crate::Readable for RESP3rs {}
#[doc = "`reset()` method sets RESP3 to value 0"]
impl crate::Resettable for RESP3rs {
    const RESET_VALUE: u32 = 0;
}
