#[doc = "Register `S5M0AR` reader"]
pub type R = crate::R<S5M0ARrs>;
#[doc = "Register `S5M0AR` writer"]
pub type W = crate::W<S5M0ARrs>;
#[doc = "Field `M0A` reader - Memory 0 address"]
pub type M0A_R = crate::FieldReader<u32>;
#[doc = "Field `M0A` writer - Memory 0 address"]
pub type M0A_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory 0 address"]
    #[inline(always)]
    pub fn m0a(&self) -> M0A_R {
        M0A_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S5M0AR").field("m0a", &self.m0a()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory 0 address"]
    #[inline(always)]
    pub fn m0a(&mut self) -> M0A_W<S5M0ARrs> {
        M0A_W::new(self, 0)
    }
}
#[doc = "stream x memory 0 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s5m0ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s5m0ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S5M0ARrs;
impl crate::RegisterSpec for S5M0ARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s5m0ar::R`](R) reader structure"]
impl crate::Readable for S5M0ARrs {}
#[doc = "`write(|w| ..)` method takes [`s5m0ar::W`](W) writer structure"]
impl crate::Writable for S5M0ARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S5M0AR to value 0"]
impl crate::Resettable for S5M0ARrs {
    const RESET_VALUE: u32 = 0;
}
