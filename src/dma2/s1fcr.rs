#[doc = "Register `S1FCR` reader"]
pub type R = crate::R<S1FCRrs>;
#[doc = "Register `S1FCR` writer"]
pub type W = crate::W<S1FCRrs>;
#[doc = "Field `FTH` reader - FIFO threshold selection"]
pub type FTH_R = crate::FieldReader;
#[doc = "Field `FTH` writer - FIFO threshold selection"]
pub type FTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DMDIS` reader - Direct mode disable"]
pub type DMDIS_R = crate::BitReader;
#[doc = "Field `DMDIS` writer - Direct mode disable"]
pub type DMDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FS` reader - FIFO status"]
pub type FS_R = crate::FieldReader;
#[doc = "Field `FEIE` reader - FIFO error interrupt enable"]
pub type FEIE_R = crate::BitReader;
#[doc = "Field `FEIE` writer - FIFO error interrupt enable"]
pub type FEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - FIFO threshold selection"]
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Direct mode disable"]
    #[inline(always)]
    pub fn dmdis(&self) -> DMDIS_R {
        DMDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - FIFO status"]
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - FIFO error interrupt enable"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S1FCR")
            .field("feie", &self.feie())
            .field("fs", &self.fs())
            .field("dmdis", &self.dmdis())
            .field("fth", &self.fth())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - FIFO threshold selection"]
    #[inline(always)]
    pub fn fth(&mut self) -> FTH_W<S1FCRrs> {
        FTH_W::new(self, 0)
    }
    #[doc = "Bit 2 - Direct mode disable"]
    #[inline(always)]
    pub fn dmdis(&mut self) -> DMDIS_W<S1FCRrs> {
        DMDIS_W::new(self, 2)
    }
    #[doc = "Bit 7 - FIFO error interrupt enable"]
    #[inline(always)]
    pub fn feie(&mut self) -> FEIE_W<S1FCRrs> {
        FEIE_W::new(self, 7)
    }
}
#[doc = "stream x FIFO control register\n\nYou can [`read`](crate::Reg::read) this register and get [`s1fcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s1fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S1FCRrs;
impl crate::RegisterSpec for S1FCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s1fcr::R`](R) reader structure"]
impl crate::Readable for S1FCRrs {}
#[doc = "`write(|w| ..)` method takes [`s1fcr::W`](W) writer structure"]
impl crate::Writable for S1FCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S1FCR to value 0x21"]
impl crate::Resettable for S1FCRrs {
    const RESET_VALUE: u32 = 0x21;
}
