#[doc = "Register `RESP4` reader"]
pub type R = crate::R<Resp4Spec>;
#[doc = "Field `CARDSTATUS4` reader - Card Status"]
pub type Cardstatus4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Card Status"]
    #[inline(always)]
    pub fn cardstatus4(&self) -> Cardstatus4R {
        Cardstatus4R::new(self.bits)
    }
}
#[doc = "response 1..4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`resp4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Resp4Spec;
impl crate::RegisterSpec for Resp4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp4::R`](R) reader structure"]
impl crate::Readable for Resp4Spec {}
#[doc = "`reset()` method sets RESP4 to value 0"]
impl crate::Resettable for Resp4Spec {
    const RESET_VALUE: u32 = 0;
}
