#[doc = "Register `FS_HPTXSTS` reader"]
pub type R = crate::R<FS_HPTXSTSrs>;
#[doc = "Register `FS_HPTXSTS` writer"]
pub type W = crate::W<FS_HPTXSTSrs>;
#[doc = "Field `PTXFSAVL` reader - Periodic transmit data FIFO space available"]
pub type PTXFSAVL_R = crate::FieldReader<u16>;
#[doc = "Field `PTXFSAVL` writer - Periodic transmit data FIFO space available"]
pub type PTXFSAVL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PTXQSAV` reader - Periodic transmit request queue space available"]
pub type PTXQSAV_R = crate::FieldReader;
#[doc = "Field `PTXQTOP` reader - Top of the periodic transmit request queue"]
pub type PTXQTOP_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Periodic transmit data FIFO space available"]
    #[inline(always)]
    pub fn ptxfsavl(&self) -> PTXFSAVL_R {
        PTXFSAVL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Periodic transmit request queue space available"]
    #[inline(always)]
    pub fn ptxqsav(&self) -> PTXQSAV_R {
        PTXQSAV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Top of the periodic transmit request queue"]
    #[inline(always)]
    pub fn ptxqtop(&self) -> PTXQTOP_R {
        PTXQTOP_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS_HPTXSTS")
            .field("ptxfsavl", &self.ptxfsavl())
            .field("ptxqsav", &self.ptxqsav())
            .field("ptxqtop", &self.ptxqtop())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Periodic transmit data FIFO space available"]
    #[inline(always)]
    pub fn ptxfsavl(&mut self) -> PTXFSAVL_W<FS_HPTXSTSrs> {
        PTXFSAVL_W::new(self, 0)
    }
}
#[doc = "OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_hptxsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_hptxsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FS_HPTXSTSrs;
impl crate::RegisterSpec for FS_HPTXSTSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_hptxsts::R`](R) reader structure"]
impl crate::Readable for FS_HPTXSTSrs {}
#[doc = "`write(|w| ..)` method takes [`fs_hptxsts::W`](W) writer structure"]
impl crate::Writable for FS_HPTXSTSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FS_HPTXSTS to value 0x0008_0100"]
impl crate::Resettable for FS_HPTXSTSrs {
    const RESET_VALUE: u32 = 0x0008_0100;
}
