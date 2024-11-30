#[doc = "Register `DTXFSTS3` reader"]
pub type R = crate::R<Dtxfsts3Spec>;
#[doc = "Field `INEPTFSAV` reader - IN endpoint TxFIFO space available"]
pub type IneptfsavR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space available"]
    #[inline(always)]
    pub fn ineptfsav(&self) -> IneptfsavR {
        IneptfsavR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "OTG_FS device IN endpoint transmit FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dtxfsts3Spec;
impl crate::RegisterSpec for Dtxfsts3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtxfsts3::R`](R) reader structure"]
impl crate::Readable for Dtxfsts3Spec {}
#[doc = "`reset()` method sets DTXFSTS3 to value 0"]
impl crate::Resettable for Dtxfsts3Spec {
    const RESET_VALUE: u32 = 0;
}
