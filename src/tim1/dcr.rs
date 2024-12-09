#[doc = "Register `DCR` reader"]
pub type R = crate::R<DCRrs>;
#[doc = "Register `DCR` writer"]
pub type W = crate::W<DCRrs>;
#[doc = "Field `DBA` reader - DMA base address"]
pub type DBA_R = crate::FieldReader;
#[doc = "Field `DBA` writer - DMA base address"]
pub type DBA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DBL` reader - DMA burst length"]
pub type DBL_R = crate::FieldReader;
#[doc = "Field `DBL` writer - DMA burst length"]
pub type DBL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - DMA base address"]
    #[inline(always)]
    pub fn dba(&self) -> DBA_R {
        DBA_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DMA burst length"]
    #[inline(always)]
    pub fn dbl(&self) -> DBL_R {
        DBL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCR")
            .field("dbl", &self.dbl())
            .field("dba", &self.dba())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA base address"]
    #[inline(always)]
    pub fn dba(&mut self) -> DBA_W<DCRrs> {
        DBA_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - DMA burst length"]
    #[inline(always)]
    pub fn dbl(&mut self) -> DBL_W<DCRrs> {
        DBL_W::new(self, 8)
    }
}
#[doc = "DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCRrs;
impl crate::RegisterSpec for DCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcr::R`](R) reader structure"]
impl crate::Readable for DCRrs {}
#[doc = "`write(|w| ..)` method takes [`dcr::W`](W) writer structure"]
impl crate::Writable for DCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCR to value 0"]
impl crate::Resettable for DCRrs {
    const RESET_VALUE: u32 = 0;
}
