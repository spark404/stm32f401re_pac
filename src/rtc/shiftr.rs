#[doc = "Register `SHIFTR` writer"]
pub type W = crate::W<SHIFTRrs>;
#[doc = "Field `SUBFS` writer - Subtract a fraction of a second"]
pub type SUBFS_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `ADD1S` writer - Add one second"]
pub type ADD1S_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<SHIFTRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:14 - Subtract a fraction of a second"]
    #[inline(always)]
    pub fn subfs(&mut self) -> SUBFS_W<SHIFTRrs> {
        SUBFS_W::new(self, 0)
    }
    #[doc = "Bit 31 - Add one second"]
    #[inline(always)]
    pub fn add1s(&mut self) -> ADD1S_W<SHIFTRrs> {
        ADD1S_W::new(self, 31)
    }
}
#[doc = "shift control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shiftr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHIFTRrs;
impl crate::RegisterSpec for SHIFTRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`shiftr::W`](W) writer structure"]
impl crate::Writable for SHIFTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHIFTR to value 0"]
impl crate::Resettable for SHIFTRrs {
    const RESET_VALUE: u32 = 0;
}
