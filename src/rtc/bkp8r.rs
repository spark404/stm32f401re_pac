#[doc = "Register `BKP8R` reader"]
pub type R = crate::R<BKP8Rrs>;
#[doc = "Register `BKP8R` writer"]
pub type W = crate::W<BKP8Rrs>;
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
        f.debug_struct("BKP8R").field("bkp", &self.bkp()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<BKP8Rrs> {
        BKP_W::new(self, 0)
    }
}
#[doc = "backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp8r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp8r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BKP8Rrs;
impl crate::RegisterSpec for BKP8Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkp8r::R`](R) reader structure"]
impl crate::Readable for BKP8Rrs {}
#[doc = "`write(|w| ..)` method takes [`bkp8r::W`](W) writer structure"]
impl crate::Writable for BKP8Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BKP8R to value 0"]
impl crate::Resettable for BKP8Rrs {
    const RESET_VALUE: u32 = 0;
}
