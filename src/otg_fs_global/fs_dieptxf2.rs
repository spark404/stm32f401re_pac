#[doc = "Register `FS_DIEPTXF2` reader"]
pub type R = crate::R<FS_DIEPTXF2rs>;
#[doc = "Register `FS_DIEPTXF2` writer"]
pub type W = crate::W<FS_DIEPTXF2rs>;
#[doc = "Field `INEPTXSA` reader - IN endpoint FIFO3 transmit RAM start address"]
pub type INEPTXSA_R = crate::FieldReader<u16>;
#[doc = "Field `INEPTXSA` writer - IN endpoint FIFO3 transmit RAM start address"]
pub type INEPTXSA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INEPTXFD` reader - IN endpoint TxFIFO depth"]
pub type INEPTXFD_R = crate::FieldReader<u16>;
#[doc = "Field `INEPTXFD` writer - IN endpoint TxFIFO depth"]
pub type INEPTXFD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint FIFO3 transmit RAM start address"]
    #[inline(always)]
    pub fn ineptxsa(&self) -> INEPTXSA_R {
        INEPTXSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    pub fn ineptxfd(&self) -> INEPTXFD_R {
        INEPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS_DIEPTXF2")
            .field("ineptxsa", &self.ineptxsa())
            .field("ineptxfd", &self.ineptxfd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - IN endpoint FIFO3 transmit RAM start address"]
    #[inline(always)]
    pub fn ineptxsa(&mut self) -> INEPTXSA_W<FS_DIEPTXF2rs> {
        INEPTXSA_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    pub fn ineptxfd(&mut self) -> INEPTXFD_W<FS_DIEPTXF2rs> {
        INEPTXFD_W::new(self, 16)
    }
}
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_dieptxf2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_dieptxf2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FS_DIEPTXF2rs;
impl crate::RegisterSpec for FS_DIEPTXF2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_dieptxf2::R`](R) reader structure"]
impl crate::Readable for FS_DIEPTXF2rs {}
#[doc = "`write(|w| ..)` method takes [`fs_dieptxf2::W`](W) writer structure"]
impl crate::Writable for FS_DIEPTXF2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FS_DIEPTXF2 to value 0x0200_0400"]
impl crate::Resettable for FS_DIEPTXF2rs {
    const RESET_VALUE: u32 = 0x0200_0400;
}
