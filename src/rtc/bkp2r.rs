#[doc = "Register `BKP2R` reader"]
pub type R = crate::R<BKP2Rrs>;
#[doc = "Register `BKP2R` writer"]
pub type W = crate::W<BKP2Rrs>;
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
        f.debug_struct("BKP2R").field("bkp", &self.bkp()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<BKP2Rrs> {
        BKP_W::new(self, 0)
    }
}
#[doc = "backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BKP2Rrs;
impl crate::RegisterSpec for BKP2Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkp2r::R`](R) reader structure"]
impl crate::Readable for BKP2Rrs {}
#[doc = "`write(|w| ..)` method takes [`bkp2r::W`](W) writer structure"]
impl crate::Writable for BKP2Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BKP2R to value 0"]
impl crate::Resettable for BKP2Rrs {
    const RESET_VALUE: u32 = 0;
}
