#[doc = "Register `S2PAR` reader"]
pub type R = crate::R<S2PARrs>;
#[doc = "Register `S2PAR` writer"]
pub type W = crate::W<S2PARrs>;
#[doc = "Field `PA` reader - Peripheral address"]
pub type PA_R = crate::FieldReader<u32>;
#[doc = "Field `PA` writer - Peripheral address"]
pub type PA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S2PAR").field("pa", &self.pa()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W<S2PARrs> {
        PA_W::new(self, 0)
    }
}
#[doc = "stream x peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s2par::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s2par::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S2PARrs;
impl crate::RegisterSpec for S2PARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s2par::R`](R) reader structure"]
impl crate::Readable for S2PARrs {}
#[doc = "`write(|w| ..)` method takes [`s2par::W`](W) writer structure"]
impl crate::Writable for S2PARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S2PAR to value 0"]
impl crate::Resettable for S2PARrs {
    const RESET_VALUE: u32 = 0;
}
