#[doc = "Register `RLR` reader"]
pub type R = crate::R<RLRrs>;
#[doc = "Register `RLR` writer"]
pub type W = crate::W<RLRrs>;
#[doc = "Field `RL` reader - Watchdog counter reload value"]
pub type RL_R = crate::FieldReader<u16>;
#[doc = "Field `RL` writer - Watchdog counter reload value"]
pub type RL_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Watchdog counter reload value"]
    #[inline(always)]
    pub fn rl(&self) -> RL_R {
        RL_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RLR").field("rl", &self.rl()).finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Watchdog counter reload value"]
    #[inline(always)]
    pub fn rl(&mut self) -> RL_W<RLRrs> {
        RL_W::new(self, 0)
    }
}
#[doc = "Reload register\n\nYou can [`read`](crate::Reg::read) this register and get [`rlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RLRrs;
impl crate::RegisterSpec for RLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rlr::R`](R) reader structure"]
impl crate::Readable for RLRrs {}
#[doc = "`write(|w| ..)` method takes [`rlr::W`](W) writer structure"]
impl crate::Writable for RLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RLR to value 0x0fff"]
impl crate::Resettable for RLRrs {
    const RESET_VALUE: u32 = 0x0fff;
}
