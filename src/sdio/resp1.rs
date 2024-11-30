#[doc = "Register `RESP1` reader"]
pub type R = crate::R<Resp1Spec>;
#[doc = "Field `CARDSTATUS1` reader - Card Status"]
pub type Cardstatus1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Card Status"]
    #[inline(always)]
    pub fn cardstatus1(&self) -> Cardstatus1R {
        Cardstatus1R::new(self.bits)
    }
}
#[doc = "response 1..4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`resp1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Resp1Spec;
impl crate::RegisterSpec for Resp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp1::R`](R) reader structure"]
impl crate::Readable for Resp1Spec {}
#[doc = "`reset()` method sets RESP1 to value 0"]
impl crate::Resettable for Resp1Spec {
    const RESET_VALUE: u32 = 0;
}
