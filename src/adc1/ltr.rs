#[doc = "Register `LTR` reader"]
pub type R = crate::R<LTRrs>;
#[doc = "Register `LTR` writer"]
pub type W = crate::W<LTRrs>;
#[doc = "Field `LT` reader - Analog watchdog lower threshold"]
pub type LT_R = crate::FieldReader<u16>;
#[doc = "Field `LT` writer - Analog watchdog lower threshold"]
pub type LT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Analog watchdog lower threshold"]
    #[inline(always)]
    pub fn lt(&self) -> LT_R {
        LT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LTR").field("lt", &self.lt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Analog watchdog lower threshold"]
    #[inline(always)]
    pub fn lt(&mut self) -> LT_W<LTRrs> {
        LT_W::new(self, 0)
    }
}
#[doc = "watchdog lower threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`ltr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTRrs;
impl crate::RegisterSpec for LTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltr::R`](R) reader structure"]
impl crate::Readable for LTRrs {}
#[doc = "`write(|w| ..)` method takes [`ltr::W`](W) writer structure"]
impl crate::Writable for LTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LTR to value 0"]
impl crate::Resettable for LTRrs {
    const RESET_VALUE: u32 = 0;
}
