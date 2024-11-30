#[doc = "Register `S2M1AR` reader"]
pub type R = crate::R<S2m1arSpec>;
#[doc = "Register `S2M1AR` writer"]
pub type W = crate::W<S2m1arSpec>;
#[doc = "Field `M1A` reader - Memory 1 address (used in case of Double buffer mode)"]
pub type M1aR = crate::FieldReader<u32>;
#[doc = "Field `M1A` writer - Memory 1 address (used in case of Double buffer mode)"]
pub type M1aW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub fn m1a(&self) -> M1aR {
        M1aR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub fn m1a(&mut self) -> M1aW<S2m1arSpec> {
        M1aW::new(self, 0)
    }
}
#[doc = "stream x memory 1 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s2m1ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s2m1ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S2m1arSpec;
impl crate::RegisterSpec for S2m1arSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s2m1ar::R`](R) reader structure"]
impl crate::Readable for S2m1arSpec {}
#[doc = "`write(|w| ..)` method takes [`s2m1ar::W`](W) writer structure"]
impl crate::Writable for S2m1arSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S2M1AR to value 0"]
impl crate::Resettable for S2m1arSpec {
    const RESET_VALUE: u32 = 0;
}
