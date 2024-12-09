#[doc = "Register `BKP10R` reader"]
pub type R = crate::R<BKP10Rrs>;
#[doc = "Register `BKP10R` writer"]
pub type W = crate::W<BKP10Rrs>;
#[doc = "Field `BKP` reader - BKP"]
pub type BKP_R = crate::FieldReader<u32>;
#[doc = "Field `BKP` writer - BKP"]
pub type BKP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BKP10R").field("bkp", &self.bkp()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<BKP10Rrs> {
        BKP_W::new(self, 0)
    }
}
#[doc = "backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp10r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp10r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BKP10Rrs;
impl crate::RegisterSpec for BKP10Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkp10r::R`](R) reader structure"]
impl crate::Readable for BKP10Rrs {}
#[doc = "`write(|w| ..)` method takes [`bkp10r::W`](W) writer structure"]
impl crate::Writable for BKP10Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BKP10R to value 0"]
impl crate::Resettable for BKP10Rrs {
    const RESET_VALUE: u32 = 0;
}
