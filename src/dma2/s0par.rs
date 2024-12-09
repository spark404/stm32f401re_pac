#[doc = "Register `S0PAR` reader"]
pub type R = crate::R<S0PARrs>;
#[doc = "Register `S0PAR` writer"]
pub type W = crate::W<S0PARrs>;
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
        f.debug_struct("S0PAR").field("pa", &self.pa()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W<S0PARrs> {
        PA_W::new(self, 0)
    }
}
#[doc = "stream x peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s0par::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s0par::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S0PARrs;
impl crate::RegisterSpec for S0PARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s0par::R`](R) reader structure"]
impl crate::Readable for S0PARrs {}
#[doc = "`write(|w| ..)` method takes [`s0par::W`](W) writer structure"]
impl crate::Writable for S0PARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S0PAR to value 0"]
impl crate::Resettable for S0PARrs {
    const RESET_VALUE: u32 = 0;
}
