#[doc = "Register `RESP2` reader"]
pub type R = crate::R<Resp2Spec>;
#[doc = "Field `CARDSTATUS2` reader - Card Status"]
pub type Cardstatus2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Card Status"]
    #[inline(always)]
    pub fn cardstatus2(&self) -> Cardstatus2R {
        Cardstatus2R::new(self.bits)
    }
}
#[doc = "response 1..4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`resp2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Resp2Spec;
impl crate::RegisterSpec for Resp2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp2::R`](R) reader structure"]
impl crate::Readable for Resp2Spec {}
#[doc = "`reset()` method sets RESP2 to value 0"]
impl crate::Resettable for Resp2Spec {
    const RESET_VALUE: u32 = 0;
}
