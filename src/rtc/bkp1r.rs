#[doc = "Register `BKP1R` reader"]
pub type R = crate::R<Bkp1rSpec>;
#[doc = "Register `BKP1R` writer"]
pub type W = crate::W<Bkp1rSpec>;
#[doc = "Field `BKP` reader - BKP"]
pub type BkpR = crate::FieldReader<u32>;
#[doc = "Field `BKP` writer - BKP"]
pub type BkpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    pub fn bkp(&self) -> BkpR {
        BkpR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    pub fn bkp(&mut self) -> BkpW<Bkp1rSpec> {
        BkpW::new(self, 0)
    }
}
#[doc = "backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bkp1rSpec;
impl crate::RegisterSpec for Bkp1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkp1r::R`](R) reader structure"]
impl crate::Readable for Bkp1rSpec {}
#[doc = "`write(|w| ..)` method takes [`bkp1r::W`](W) writer structure"]
impl crate::Writable for Bkp1rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BKP1R to value 0"]
impl crate::Resettable for Bkp1rSpec {
    const RESET_VALUE: u32 = 0;
}