#[doc = "Register `WPR` writer"]
pub type W = crate::W<WPRrs>;
#[doc = "Field `KEY` writer - Write protection key"]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<WPRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Write protection key"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<WPRrs> {
        KEY_W::new(self, 0)
    }
}
#[doc = "write protection register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPRrs;
impl crate::RegisterSpec for WPRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wpr::W`](W) writer structure"]
impl crate::Writable for WPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WPR to value 0"]
impl crate::Resettable for WPRrs {
    const RESET_VALUE: u32 = 0;
}
