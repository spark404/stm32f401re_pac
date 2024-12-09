#[doc = "Register `FS_GRXFSIZ` reader"]
pub type R = crate::R<FS_GRXFSIZrs>;
#[doc = "Register `FS_GRXFSIZ` writer"]
pub type W = crate::W<FS_GRXFSIZrs>;
#[doc = "Field `RXFD` reader - RxFIFO depth"]
pub type RXFD_R = crate::FieldReader<u16>;
#[doc = "Field `RXFD` writer - RxFIFO depth"]
pub type RXFD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RxFIFO depth"]
    #[inline(always)]
    pub fn rxfd(&self) -> RXFD_R {
        RXFD_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS_GRXFSIZ")
            .field("rxfd", &self.rxfd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - RxFIFO depth"]
    #[inline(always)]
    pub fn rxfd(&mut self) -> RXFD_W<FS_GRXFSIZrs> {
        RXFD_W::new(self, 0)
    }
}
#[doc = "OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_grxfsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_grxfsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FS_GRXFSIZrs;
impl crate::RegisterSpec for FS_GRXFSIZrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_grxfsiz::R`](R) reader structure"]
impl crate::Readable for FS_GRXFSIZrs {}
#[doc = "`write(|w| ..)` method takes [`fs_grxfsiz::W`](W) writer structure"]
impl crate::Writable for FS_GRXFSIZrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FS_GRXFSIZ to value 0x0200"]
impl crate::Resettable for FS_GRXFSIZrs {
    const RESET_VALUE: u32 = 0x0200;
}
