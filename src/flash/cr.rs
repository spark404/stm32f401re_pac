#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `PG` reader - Programming"]
pub type PG_R = crate::BitReader;
#[doc = "Field `PG` writer - Programming"]
pub type PG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SER` reader - Sector Erase"]
pub type SER_R = crate::BitReader;
#[doc = "Field `SER` writer - Sector Erase"]
pub type SER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MER` reader - Mass Erase"]
pub type MER_R = crate::BitReader;
#[doc = "Field `MER` writer - Mass Erase"]
pub type MER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNB` reader - Sector number"]
pub type SNB_R = crate::FieldReader;
#[doc = "Field `SNB` writer - Sector number"]
pub type SNB_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PSIZE` reader - Program size"]
pub type PSIZE_R = crate::FieldReader;
#[doc = "Field `PSIZE` writer - Program size"]
pub type PSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STRT` reader - Start"]
pub type STRT_R = crate::BitReader;
#[doc = "Field `STRT` writer - Start"]
pub type STRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPIE` reader - End of operation interrupt enable"]
pub type EOPIE_R = crate::BitReader;
#[doc = "Field `EOPIE` writer - End of operation interrupt enable"]
pub type EOPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - Lock"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - Lock"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Programming"]
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sector Erase"]
    #[inline(always)]
    pub fn ser(&self) -> SER_R {
        SER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mass Erase"]
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - Sector number"]
    #[inline(always)]
    pub fn snb(&self) -> SNB_R {
        SNB_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Program size"]
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Start"]
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - End of operation interrupt enable"]
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 31 - Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("pg", &self.pg())
            .field("ser", &self.ser())
            .field("mer", &self.mer())
            .field("snb", &self.snb())
            .field("psize", &self.psize())
            .field("strt", &self.strt())
            .field("eopie", &self.eopie())
            .field("errie", &self.errie())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Programming"]
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W<CRrs> {
        PG_W::new(self, 0)
    }
    #[doc = "Bit 1 - Sector Erase"]
    #[inline(always)]
    pub fn ser(&mut self) -> SER_W<CRrs> {
        SER_W::new(self, 1)
    }
    #[doc = "Bit 2 - Mass Erase"]
    #[inline(always)]
    pub fn mer(&mut self) -> MER_W<CRrs> {
        MER_W::new(self, 2)
    }
    #[doc = "Bits 3:6 - Sector number"]
    #[inline(always)]
    pub fn snb(&mut self) -> SNB_W<CRrs> {
        SNB_W::new(self, 3)
    }
    #[doc = "Bits 8:9 - Program size"]
    #[inline(always)]
    pub fn psize(&mut self) -> PSIZE_W<CRrs> {
        PSIZE_W::new(self, 8)
    }
    #[doc = "Bit 16 - Start"]
    #[inline(always)]
    pub fn strt(&mut self) -> STRT_W<CRrs> {
        STRT_W::new(self, 16)
    }
    #[doc = "Bit 24 - End of operation interrupt enable"]
    #[inline(always)]
    pub fn eopie(&mut self) -> EOPIE_W<CRrs> {
        EOPIE_W::new(self, 24)
    }
    #[doc = "Bit 25 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<CRrs> {
        ERRIE_W::new(self, 25)
    }
    #[doc = "Bit 31 - Lock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<CRrs> {
        LOCK_W::new(self, 31)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0x8000_0000"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
