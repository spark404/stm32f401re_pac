#[doc = "Register `OPTKEYR` writer"]
pub type W = crate::W<OPTKEYRrs>;
#[doc = "Field `OPTKEY` writer - Option byte key"]
pub type OPTKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<OPTKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Option byte key"]
    #[inline(always)]
    pub fn optkey(&mut self) -> OPTKEY_W<OPTKEYRrs> {
        OPTKEY_W::new(self, 0)
    }
}
#[doc = "Flash option key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPTKEYRrs;
impl crate::RegisterSpec for OPTKEYRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`optkeyr::W`](W) writer structure"]
impl crate::Writable for OPTKEYRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPTKEYR to value 0"]
impl crate::Resettable for OPTKEYRrs {
    const RESET_VALUE: u32 = 0;
}
