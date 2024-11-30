#[doc = "Register `RESP3` reader"]
pub type R = crate::R<Resp3Spec>;
#[doc = "Field `CARDSTATUS3` reader - Card Status"]
pub type Cardstatus3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Card Status"]
    #[inline(always)]
    pub fn cardstatus3(&self) -> Cardstatus3R {
        Cardstatus3R::new(self.bits)
    }
}
#[doc = "response 1..4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`resp3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Resp3Spec;
impl crate::RegisterSpec for Resp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp3::R`](R) reader structure"]
impl crate::Readable for Resp3Spec {}
#[doc = "`reset()` method sets RESP3 to value 0"]
impl crate::Resettable for Resp3Spec {
    const RESET_VALUE: u32 = 0;
}
